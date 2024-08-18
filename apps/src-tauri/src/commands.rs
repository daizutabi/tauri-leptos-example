#[tauri::command]
pub fn greet(name: String) -> String {
    println!("invoked: {name}");
    format!("Hello, {name}!")
}
