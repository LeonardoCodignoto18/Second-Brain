use std::path::Path;

fn main() {
    let path = std::env::var("SPK01_DB_PATH").expect("caminho temporário ausente");
    let encoded = std::env::var("SPK01_TEST_KEY").expect("chave descartável ausente");
    let bytes = hex::decode(encoded).expect("chave descartável inválida");
    let key: [u8; 32] = bytes.try_into().expect("tamanho de chave inválido");
    let connection = spk01_sqlcipher_windows::open_existing(Path::new(&path), &key)
        .expect("falha ao abrir banco temporário");
    connection
        .execute("INSERT INTO items(body) VALUES ('wal-apos-abort')", [])
        .expect("falha ao confirmar escrita WAL");
    std::process::abort();
}
