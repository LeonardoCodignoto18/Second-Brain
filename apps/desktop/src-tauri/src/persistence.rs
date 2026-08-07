//! SQLCipher-backed Local First operation journal and migration boundary.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, OptionalExtension, params};
use second_brain_windows_security::{protect, unprotect};

const KEY_PURPOSE: &[u8] = b"second-brain-os/sqlcipher-key/v1";
const SENTINEL: &str = "second-brain-os/database/v1";
const MIGRATION_1: &str = "CREATE TABLE operations (sequence INTEGER PRIMARY KEY AUTOINCREMENT, kind TEXT NOT NULL, payload TEXT NOT NULL, created_utc TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP);";

/// One durable application operation, ordered by its committed sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoredOperation {
    /// Monotonic local sequence.
    pub sequence: u64,
    /// Stable operation discriminator.
    pub kind: String,
    /// Versioned JSON payload owned by the native boundary.
    pub payload: String,
}

/// Runtime facts proving that the encrypted store opened correctly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StoreHealth {
    /// Runtime `SQLCipher` version, never an assumed package version.
    pub cipher_version: String,
    /// Current embedded schema version.
    pub schema_version: u32,
}

/// Fail-closed local persistence error.
#[derive(Debug)]
pub(crate) enum StoreError {
    /// File-system operation failed.
    Io(std::io::Error),
    /// Secret protection or recovery failed.
    Secret,
    /// SQLite/SQLCipher rejected an operation.
    Database(rusqlite::Error),
    /// `SQLCipher` was not present in the running binary.
    CipherUnavailable,
    /// The key opened a database without the authenticated application sentinel.
    InvalidSentinel,
    /// A migration checksum differs from the embedded migration.
    MigrationMismatch,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "file-system failure: {error}"),
            Self::Secret => formatter.write_str("secret recovery failed"),
            Self::Database(error) => write!(formatter, "database failure: {error}"),
            Self::CipherUnavailable => formatter.write_str("SQLCipher unavailable"),
            Self::InvalidSentinel => formatter.write_str("database sentinel invalid"),
            Self::MigrationMismatch => formatter.write_str("migration checksum mismatch"),
        }
    }
}
impl std::error::Error for StoreError {}
impl From<std::io::Error> for StoreError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<rusqlite::Error> for StoreError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Database(value)
    }
}

/// Exclusive owner of the encrypted `SQLite` connection.
#[derive(Debug)]
pub(crate) struct LocalStore {
    connection: Connection,
    health: StoreHealth,
}

impl LocalStore {
    /// Opens or creates the user-bound encrypted store and runs forward migrations.
    ///
    /// # Errors
    /// Fails closed for key, cipher, sentinel, integrity, or migration failures.
    pub(crate) fn open(directory: &Path) -> Result<Self, StoreError> {
        fs::create_dir_all(directory)?;
        let key = load_or_create_key(directory)?;
        let connection = Connection::open(directory.join("second-brain.db"))?;
        connection.pragma_update(None, "key", format!("x'{}'", encode_hex(&key)))?;
        let cipher_version: String = connection
            .query_row("PRAGMA cipher_version", [], |row| row.get(0))
            .optional()?
            .filter(|value: &String| !value.trim().is_empty())
            .ok_or(StoreError::CipherUnavailable)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.execute_batch("CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY, checksum TEXT NOT NULL); CREATE TABLE IF NOT EXISTS app_sentinel (id INTEGER PRIMARY KEY CHECK (id = 1), value TEXT NOT NULL);")?;
        apply_migration(&connection, 1, "operations-v1", MIGRATION_1)?;
        connection.execute(
            "INSERT OR IGNORE INTO app_sentinel (id, value) VALUES (1, ?1)",
            [SENTINEL],
        )?;
        let sentinel: String =
            connection.query_row("SELECT value FROM app_sentinel WHERE id = 1", [], |row| {
                row.get(0)
            })?;
        if sentinel != SENTINEL {
            return Err(StoreError::InvalidSentinel);
        }
        let integrity: String =
            connection.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
        if integrity != "ok" {
            return Err(StoreError::InvalidSentinel);
        }
        Ok(Self {
            connection,
            health: StoreHealth {
                cipher_version,
                schema_version: 1,
            },
        })
    }

    /// Returns verified runtime store facts.
    #[must_use]
    pub(crate) fn health(&self) -> &StoreHealth {
        &self.health
    }

    /// Commits one operation atomically.
    ///
    /// # Errors
    /// Returns a database failure without a partial row.
    pub(crate) fn append(&mut self, kind: &str, payload: &str) -> Result<u64, StoreError> {
        let transaction = self.connection.transaction()?;
        transaction.execute(
            "INSERT INTO operations (kind, payload) VALUES (?1, ?2)",
            params![kind, payload],
        )?;
        let sequence = u64::try_from(transaction.last_insert_rowid())
            .map_err(|_| StoreError::InvalidSentinel)?;
        transaction.commit()?;
        Ok(sequence)
    }

    /// Loads the ordered journal for deterministic state reconstruction.
    ///
    /// # Errors
    /// Returns a database failure for any unreadable row.
    pub(crate) fn operations(&self) -> Result<Vec<StoredOperation>, StoreError> {
        let mut statement = self
            .connection
            .prepare("SELECT sequence, kind, payload FROM operations ORDER BY sequence")?;
        let rows = statement.query_map([], |row| {
            Ok(StoredOperation {
                sequence: u64::try_from(row.get::<_, i64>(0)?)
                    .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, -1))?,
                kind: row.get(1)?,
                payload: row.get(2)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(StoreError::from)
    }
}

fn apply_migration(
    connection: &Connection,
    version: u32,
    checksum: &str,
    sql: &str,
) -> Result<(), StoreError> {
    let existing: Option<String> = connection
        .query_row(
            "SELECT checksum FROM schema_migrations WHERE version = ?1",
            [version],
            |row| row.get(0),
        )
        .optional()?;
    if let Some(existing) = existing {
        return if existing == checksum {
            Ok(())
        } else {
            Err(StoreError::MigrationMismatch)
        };
    }
    let transaction = connection.unchecked_transaction()?;
    transaction.execute_batch(sql)?;
    transaction.execute(
        "INSERT INTO schema_migrations (version, checksum) VALUES (?1, ?2)",
        params![version, checksum],
    )?;
    transaction.commit()?;
    Ok(())
}

fn load_or_create_key(directory: &Path) -> Result<Vec<u8>, StoreError> {
    let path = directory.join("database-key.dpapi");
    if path.exists() {
        return unprotect(&fs::read(path)?, KEY_PURPOSE).map_err(|_| StoreError::Secret);
    }
    let mut key = [0_u8; 32];
    getrandom::fill(&mut key).map_err(|_| StoreError::Secret)?;
    let protected = protect(&key, KEY_PURPOSE).map_err(|_| StoreError::Secret)?;
    write_new_atomic(&path, &protected)?;
    Ok(key.to_vec())
}

fn write_new_atomic(path: &Path, bytes: &[u8]) -> Result<(), StoreError> {
    let temporary = PathBuf::from(format!("{}.new", path.display()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(temporary, path)?;
    Ok(())
}
fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_directory(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join("target/local-store-tests")
            .join(format!("{name}-{}", std::process::id()))
    }

    #[test]
    fn encrypted_journal_survives_a_real_reopen() {
        let directory = test_directory("reopen");
        if directory.exists() {
            fs::remove_dir_all(&directory).expect("remove stale test data");
        }
        {
            let mut store = LocalStore::open(&directory).expect("open encrypted store");
            assert!(!store.health().cipher_version.is_empty());
            store
                .append("test.created", "{\"value\":1}")
                .expect("append");
        }
        {
            let store = LocalStore::open(&directory).expect("reopen encrypted store");
            let operations = store.operations().expect("operations");
            assert_eq!(operations.len(), 1);
            assert_eq!(operations[0].kind, "test.created");
            assert_eq!(operations[0].payload, "{\"value\":1}");
        }
        fs::remove_dir_all(directory).expect("remove test data");
    }
}
