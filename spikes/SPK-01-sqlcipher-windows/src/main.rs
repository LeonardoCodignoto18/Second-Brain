#[tauri::command]
fn db_probe() -> Result<spk01_sqlcipher_windows::ProbeResult, String> {
    spk01_sqlcipher_windows::db_probe()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![db_probe])
        .run(tauri::generate_context!())
        .expect("falha ao executar o invólucro descartável do SPK-01");
}
