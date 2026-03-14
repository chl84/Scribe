pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;

#[cfg(test)]
mod tests;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let editor_service = application::services::EditorService::new(
        infrastructure::filesystem::LocalFileSystem,
    );

    tauri::Builder::default()
        .manage(std::sync::Mutex::new(editor_service))
        .invoke_handler(tauri::generate_handler![
            interface::ipc::handlers::create_document,
            interface::ipc::handlers::open_document,
            interface::ipc::handlers::get_document,
            interface::ipc::handlers::edit_document,
            interface::ipc::handlers::undo_document,
            interface::ipc::handlers::redo_document,
            interface::ipc::handlers::save_document,
            interface::ipc::handlers::close_document,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Scribe");
}
