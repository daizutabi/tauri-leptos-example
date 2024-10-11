use futures::stream::StreamExt;
use leptos::prelude::*;
use thaw::{Button, Tag};

#[component]
pub fn EventFrontend() -> impl IntoView {
    let count = RwSignal::new(0);

    let emit_frontend_event = move |_| {
        leptos::task::spawn_local(async move {
            let n = count.get_untracked();
            tauri_sys::event::emit("frontend", &n).await.unwrap();
        });
    };

    leptos::task::spawn_local(async move {
        let mut listener = tauri_sys::event::listen::<i32>("frontend")
            .await
            .unwrap();

        while let Some(event) = listener.next().await {
            count.set(event.payload + 1);
        }
    });

    view! {
        <div>
            <Button on_click=emit_frontend_event>"Emit fontend event"</Button>
            <Tag>"Number of emitted events: "{count}</Tag>
        </div>
    }
}
