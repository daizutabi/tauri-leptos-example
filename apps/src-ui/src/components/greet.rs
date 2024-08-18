use leptos::prelude::*;
use serde::Serialize;
use thaw::{Button, ButtonAppearance, Input, Tag};

#[derive(Serialize)]
struct GreetArgs {
    name: String,
}

#[component]
pub fn Greet() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let greet_msg = RwSignal::new(String::new());

    let on_click = move |_| {
        leptos::spawn::spawn_local(async move {
            let name = name.get_untracked();
            log::info!("invoke: {name}");
            let args = GreetArgs { name };
            let msg: String = tauri_sys::core::invoke("greet", args).await;
            greet_msg.set(msg);
        })
    };

    view! {
        <div>
            <Input placeholder="Enter a name..." value=name />
            <Button appearance=ButtonAppearance::Primary on_click>
                "Greet"
            </Button>
            <Tag>{greet_msg}</Tag>
        </div>
    }
}
