use rand::RngCore;
use rusqlite::{Connection, ErrorCode, TransactionBehavior};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum ProbeError {
    #[error("chave inválida ou arquivo não é um banco SQLCipher válido")]
    InvalidKeyOrFormat,
    #[error("erro de persistência cifrada: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("verificação de segurança falhou: {0}")]
    Verification(&'static str),
}

#[derive(Debug, Serialize)]
pub struct ProbeResult {
    pub ok: bool,
    pub cipher_family: &'static str,
}

fn test_key() -> [u8; 32] {
    let mut key = [0_u8; 32];
    rand::rng().fill_bytes(&mut key);
    key
}

fn apply_key(connection: &Connection, key: &[u8; 32]) -> Result<(), ProbeError> {
    let key_literal = format!("x'{}'", hex::encode(key));
    connection.pragma_update(None, "key", key_literal)?;
    Ok(())
}

fn classify_open_error(error: rusqlite::Error) -> ProbeError {
    match &error {
        rusqlite::Error::SqliteFailure(inner, _)
            if matches!(
                inner.code,
                ErrorCode::NotADatabase | ErrorCode::DatabaseCorrupt
            ) =>
        {
            ProbeError::InvalidKeyOrFormat
        }
        _ => ProbeError::Database(error),
    }
}

pub fn open_existing(path: &Path, key: &[u8; 32]) -> Result<Connection, ProbeError> {
    if !path.is_file() {
        return Err(ProbeError::Verification(
            "o banco existente não foi encontrado",
        ));
    }
    let connection = Connection::open_with_flags(
        path,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;
    apply_key(&connection, key)?;
    connection
        .query_row(
            "SELECT value FROM metadata WHERE key = 'sentinel'",
            [],
            |_| Ok(()),
        )
        .map_err(classify_open_error)?;
    Ok(connection)
}

pub fn create_v1(path: &Path, key: &[u8; 32]) -> Result<Connection, ProbeError> {
    if path.exists() {
        return Err(ProbeError::Verification(
            "recusa de sobrescrever banco existente",
        ));
    }
    let mut connection = Connection::open(path)?;
    apply_key(&connection, key)?;
    connection.pragma_update(None, "journal_mode", "WAL")?;
    let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    tx.execute_batch(
        "CREATE TABLE metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL);
         CREATE TABLE items (id INTEGER PRIMARY KEY, body TEXT NOT NULL);
         INSERT INTO metadata(key, value) VALUES ('sentinel', 'spk01');
         PRAGMA user_version = 1;",
    )?;
    tx.commit()?;
    Ok(connection)
}

pub fn migrate_to_v2(connection: &mut Connection) -> Result<(), ProbeError> {
    let version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    if version == 1 {
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "ALTER TABLE items ADD COLUMN completed INTEGER NOT NULL DEFAULT 0;
             CREATE TABLE migration_history (
               version INTEGER PRIMARY KEY,
               applied_marker TEXT NOT NULL
             );
             INSERT INTO migration_history(version, applied_marker) VALUES (2, '002_items_completed');
             PRAGMA user_version = 2;",
        )?;
        tx.commit()?;
    }
    Ok(())
}

pub fn cipher_version(connection: &Connection) -> Result<String, ProbeError> {
    Ok(connection.pragma_query_value(None, "cipher_version", |row| row.get(0))?)
}

pub fn db_probe() -> Result<ProbeResult, String> {
    let temp = tempfile::tempdir().map_err(|_| "não foi possível criar área temporária")?;
    let path = temp.path().join("tauri-probe.db");
    let key = test_key();
    let connection = create_v1(&path, &key).map_err(|_| "falha controlada ao criar banco")?;
    let version = cipher_version(&connection).map_err(|_| "SQLCipher indisponível")?;
    connection
        .execute("INSERT INTO items(body) VALUES (?1)", ["probe-tauri"])
        .map_err(|_| "falha controlada na escrita")?;
    drop(connection);
    let reopened = open_existing(&path, &key).map_err(|_| "falha controlada na reabertura")?;
    let count: i64 = reopened
        .query_row("SELECT COUNT(*) FROM items", [], |row| row.get(0))
        .map_err(|_| "falha controlada na leitura")?;
    if count != 1 || version.is_empty() {
        return Err("verificação do probe falhou".into());
    }
    if let Ok(marker) = std::env::var("SPK01_EVIDENCE_MARKER") {
        std::fs::write(marker, "TAURI_IPC_SQLCIPHER_OK\n")
            .map_err(|_| "não foi possível gravar marcador não sensível")?;
    }
    Ok(ProbeResult {
        ok: true,
        cipher_family: "SQLCipher 4",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture() -> (tempfile::TempDir, std::path::PathBuf, [u8; 32]) {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("spk01.db");
        let key = test_key();
        (temp, path, key)
    }

    #[test]
    fn scenario_a_create_commit_reopen_and_read() {
        let (_temp, path, key) = fixture();
        let connection = create_v1(&path, &key).unwrap();
        let cipher = cipher_version(&connection).unwrap();
        eprintln!("SQLCIPHER_VERSION={cipher}");
        assert!(!cipher.is_empty());
        connection
            .execute(
                "INSERT INTO items(body) VALUES (?1)",
                ["evidencia-descartavel"],
            )
            .unwrap();
        drop(connection);
        let reopened = open_existing(&path, &key).unwrap();
        let body: String = reopened
            .query_row("SELECT body FROM items", [], |row| row.get(0))
            .unwrap();
        assert_eq!(body, "evidencia-descartavel");
    }

    #[test]
    fn scenario_b_wrong_key_fails_without_modifying_file() {
        let (_temp, path, key) = fixture();
        drop(create_v1(&path, &key).unwrap());
        let before = fs::read(&path).unwrap();
        let wrong_key = test_key();
        assert!(matches!(
            open_existing(&path, &wrong_key),
            Err(ProbeError::InvalidKeyOrFormat)
        ));
        assert_eq!(before, fs::read(&path).unwrap());
    }

    #[test]
    fn scenario_c_uncommitted_transaction_is_rolled_back() {
        let (_temp, path, key) = fixture();
        let mut connection = create_v1(&path, &key).unwrap();
        {
            let tx = connection.transaction().unwrap();
            tx.execute("INSERT INTO items(body) VALUES ('nao-deve-persistir')", [])
                .unwrap();
        }
        drop(connection);
        let reopened = open_existing(&path, &key).unwrap();
        let count: i64 = reopened
            .query_row("SELECT COUNT(*) FROM items", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn scenario_d_numbered_migration_preserves_data() {
        let (_temp, path, key) = fixture();
        let connection = create_v1(&path, &key).unwrap();
        connection
            .execute("INSERT INTO items(body) VALUES ('pre-migracao')", [])
            .unwrap();
        drop(connection);
        let mut reopened = open_existing(&path, &key).unwrap();
        migrate_to_v2(&mut reopened).unwrap();
        let version: i64 = reopened
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        let body: String = reopened
            .query_row("SELECT body FROM items", [], |row| row.get(0))
            .unwrap();
        let marker: String = reopened
            .query_row(
                "SELECT applied_marker FROM migration_history WHERE version=2",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(
            (version, body.as_str(), marker.as_str()),
            (2, "pre-migracao", "002_items_completed")
        );
    }

    #[test]
    fn scenario_e_wal_checkpoint_reopen_and_integrity() {
        let (_temp, path, key) = fixture();
        let connection = create_v1(&path, &key).unwrap();
        let journal: String = connection
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert_eq!(journal.to_ascii_lowercase(), "wal");
        connection
            .execute("INSERT INTO items(body) VALUES ('wal')", [])
            .unwrap();
        let checkpoint: (i64, i64, i64) = connection
            .query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .unwrap();
        assert_eq!(checkpoint.0, 0);
        drop(connection);
        let reopened = open_existing(&path, &key).unwrap();
        let mut statement = reopened.prepare("PRAGMA cipher_integrity_check").unwrap();
        let integrity_errors: Vec<String> = statement
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        let quick: String = reopened
            .pragma_query_value(None, "quick_check", |row| row.get(0))
            .unwrap();
        assert!(
            integrity_errors.is_empty(),
            "cipher_integrity_check reportou erros"
        );
        assert_eq!(quick, "ok");
    }

    #[test]
    fn h6_file_is_not_plain_sqlite_and_plain_open_fails() {
        let (_temp, path, key) = fixture();
        drop(create_v1(&path, &key).unwrap());
        let bytes = fs::read(&path).unwrap();
        assert_ne!(&bytes[..16], b"SQLite format 3\0");
        let plain =
            Connection::open_with_flags(&path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();
        assert!(
            plain
                .query_row("SELECT name FROM sqlite_master LIMIT 1", [], |row| row
                    .get::<_, String>(
                    0
                ))
                .is_err()
        );
    }

    #[test]
    fn h10_invalid_file_is_rejected() {
        let (temp, path, key) = fixture();
        fs::write(&path, b"arquivo deliberadamente invalido para o spike").unwrap();
        assert!(open_existing(&path, &key).is_err());
        drop(temp);
    }

    #[test]
    fn rekey_changes_required_key_and_preserves_data() {
        let (_temp, path, old_key) = fixture();
        let connection = create_v1(&path, &old_key).unwrap();
        connection
            .execute("INSERT INTO items(body) VALUES ('rekey')", [])
            .unwrap();
        let new_key = test_key();
        let literal = format!("x'{}'", hex::encode(new_key));
        connection.pragma_update(None, "rekey", literal).unwrap();
        drop(connection);
        assert!(open_existing(&path, &old_key).is_err());
        let reopened = open_existing(&path, &new_key).unwrap();
        let count: i64 = reopened
            .query_row("SELECT COUNT(*) FROM items", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}
