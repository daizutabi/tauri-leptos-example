use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn greet(name: String) -> String {
    println!("invoked: {name}");
    format!("Hello, {name}!")
}

#[tauri::command]
pub async fn trigger_backend_event(app: AppHandle) {
    let emit = |x| app.emit("backend", x).unwrap();

    for i in 1..=1000 {
        emit(Some(i));
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    emit(None);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello() {
        let result = greet("Rust".to_string());
        assert_eq!(result, "Hello, Rust!");
    }
}
