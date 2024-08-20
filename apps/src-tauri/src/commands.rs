#[tauri::command]
pub fn greet(name: String) -> String {
    println!("invoked: {name}");
    format!("Hello, {name}!")
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
