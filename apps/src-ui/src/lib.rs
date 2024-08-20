use app::App;
use leptos::prelude::*;

mod app;
pub mod components;

pub fn run() {
    console_log::init().expect("error initializing logger");
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
