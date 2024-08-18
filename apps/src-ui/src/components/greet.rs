use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thaw::{Button, Input};

#[derive(Serialize, Deserialize)]
struct GreetArgs {
    name: String,
}

#[component]
pub fn Greet() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let greet_msg = RwSignal::new(String::new());

    let trigger_invoke = move |_| {
        leptos::spawn::spawn_local(async move {
            let name = name.get_untracked();
            log::info!("trigger_invoke: {name}");
            let args = GreetArgs { name };
            let msg: String = tauri_sys::core::invoke("greet", args).await;
            greet_msg.set(msg);
        })
    };

    view! {
        <div>
            <Input value=name />
            <Button on_click=trigger_invoke>"Greet"</Button>
            <p>{greet_msg}</p>
        </div>
    }
}
