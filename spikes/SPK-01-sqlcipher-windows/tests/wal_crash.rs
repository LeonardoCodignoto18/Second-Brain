use rand::RngCore;
use std::process::Command;

#[test]
fn scenario_e_recovers_committed_wal_after_abrupt_process_abort() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("crash-wal.db");
    let mut key = [0_u8; 32];
    rand::rng().fill_bytes(&mut key);
    drop(spk01_sqlcipher_windows::create_v1(&path, &key).unwrap());

    let status = Command::new(env!("CARGO_BIN_EXE_wal-crash-writer"))
        .env("SPK01_DB_PATH", &path)
        .env("SPK01_TEST_KEY", hex::encode(key))
        .status()
        .unwrap();
    assert!(
        !status.success(),
        "o processo auxiliar deveria encerrar abruptamente"
    );

    let reopened = spk01_sqlcipher_windows::open_existing(&path, &key).unwrap();
    let count: i64 = reopened
        .query_row(
            "SELECT COUNT(*) FROM items WHERE body='wal-apos-abort'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let integrity: String = reopened
        .pragma_query_value(None, "quick_check", |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(integrity, "ok");
}
