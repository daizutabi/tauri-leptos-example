use leptos::{prelude::*, spawn::tick};
use wasm_bindgen_test::*;

use src_ui_lib::components::counter::SimpleCounter;

mod common;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn increment() {
    open_counter();
    common::click_text("+1");
    common::click_text("+1");
    tick().await;

    assert_eq!(get_text(), Some("Value: 12!".to_string()));
}

#[wasm_bindgen_test]
async fn decrement() {
    open_counter();
    common::click_text("-1");
    common::click_text("-1");
    tick().await;

    assert_eq!(get_text(), Some("Value: 8!".to_string()));
}

#[wasm_bindgen_test]
async fn clear() {
    open_counter();
    common::click_text("Clear");
    tick().await;

    assert_eq!(get_text(), Some("Value: 0!".to_string()));
}

fn open_counter() {
    common::remove_top_div();
    mount_to_body(move || view! { <SimpleCounter initial_value=10 step=1 /> });
}

fn get_text() -> Option<String> {
    common::find_by_text("Value: ").text_content()
}
