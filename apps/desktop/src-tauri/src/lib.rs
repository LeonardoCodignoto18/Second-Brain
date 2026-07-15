//! Tauri composition root and IPC adapter.

use std::sync::Arc;

use second_brain_application::Application;
use second_brain_contracts::FoundationStatus;
use tauri::State;

#[tauri::command]
#[allow(clippy::needless_pass_by_value)] // Required by Tauri command extraction.
fn foundation_status(application: State<'_, Arc<Application>>) -> FoundationStatus {
    application.foundation_status()
}

/// Starts the desktop host.
///
/// # Panics
///
/// Panics when the native host cannot be initialized or its event loop fails.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let application = Arc::new(Application::new(
        "Second Brain OS",
        env!("CARGO_PKG_VERSION"),
    ));

    tauri::Builder::default()
        .manage(application)
        .invoke_handler(tauri::generate_handler![foundation_status])
        .run(tauri::generate_context!())
        .expect("the Second Brain OS desktop host must start");
}
