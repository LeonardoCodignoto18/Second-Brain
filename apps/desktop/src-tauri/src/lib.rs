#![allow(clippy::needless_pass_by_value)]

//! Tauri composition root and allowlisted IPC adapter.

mod persistence;
mod runtime;

use std::sync::{Arc, Mutex};

use second_brain_application::Application;
use second_brain_contracts::{
    ArchiveProjectRequest, CreateProjectRequest, CreateTaskRequest, FoundationStatus, IpcError,
    TransitionTaskRequest, WorkspaceSnapshot,
};
use tauri::{Manager, State};

use runtime::Runtime;

#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
fn foundation_status(application: State<'_, Arc<Application>>) -> FoundationStatus {
    application.foundation_status()
}

#[tauri::command]
fn workspace_snapshot(runtime: State<'_, Mutex<Runtime>>) -> Result<WorkspaceSnapshot, IpcError> {
    runtime
        .lock()
        .map_err(|_| unavailable())
        .map(|runtime| runtime.snapshot())
}

#[tauri::command]
fn create_project(
    request: CreateProjectRequest,
    runtime: State<'_, Mutex<Runtime>>,
) -> Result<WorkspaceSnapshot, IpcError> {
    runtime
        .lock()
        .map_err(|_| unavailable())?
        .create_project(request)
}

#[tauri::command]
fn archive_project(
    request: ArchiveProjectRequest,
    runtime: State<'_, Mutex<Runtime>>,
) -> Result<WorkspaceSnapshot, IpcError> {
    runtime
        .lock()
        .map_err(|_| unavailable())?
        .archive_project(request)
}

#[tauri::command]
fn create_task(
    request: CreateTaskRequest,
    runtime: State<'_, Mutex<Runtime>>,
) -> Result<WorkspaceSnapshot, IpcError> {
    runtime
        .lock()
        .map_err(|_| unavailable())?
        .create_task(request)
}

#[tauri::command]
fn transition_task(
    request: TransitionTaskRequest,
    runtime: State<'_, Mutex<Runtime>>,
) -> Result<WorkspaceSnapshot, IpcError> {
    runtime
        .lock()
        .map_err(|_| unavailable())?
        .transition_task(request)
}

fn unavailable() -> IpcError {
    IpcError {
        code: "runtime.unavailable".to_owned(),
        message: "O nucleo local esta temporariamente indisponivel.".to_owned(),
    }
}

/// Starts the desktop host.
///
/// # Panics
/// Panics when the native host cannot initialize its encrypted store or event loop.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let application = Arc::new(Application::new(
        "Second Brain OS",
        env!("CARGO_PKG_VERSION"),
    ));
    tauri::Builder::default()
        .manage(application)
        .setup(|app| {
            let directory = std::env::var_os("SECOND_BRAIN_DATA_DIR")
                .map(std::path::PathBuf::from)
                .map_or_else(|| app.path().app_local_data_dir(), Ok)?;
            let runtime =
                Runtime::open(&directory).map_err(|error| std::io::Error::other(error.message))?;
            app.manage(Mutex::new(runtime));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            foundation_status,
            workspace_snapshot,
            create_project,
            archive_project,
            create_task,
            transition_task
        ])
        .run(tauri::generate_context!())
        .expect("the Second Brain OS desktop host must start");
}
