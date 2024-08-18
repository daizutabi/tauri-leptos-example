use app::App;
use leptos::prelude::*;

mod app;

pub fn run() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
