use tauri::Listener;

mod commands;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.listen("frontend", move |event| {
                println!("frontend event: {event:?}");
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::trigger_backend_event
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
