use futures::stream::StreamExt;
use leptos::prelude::*;
use thaw::{Button, Tag};

#[component]
pub fn EventBackend() -> impl IntoView {
    let count = RwSignal::new(0);
    let disabled = RwSignal::new(false);

    let trigger_backend_event = move |_| {
        disabled.set(true);
        leptos::spawn::spawn_local(async move {
            tauri_sys::core::invoke::<()>("trigger_backend_event", ()).await;
        });
    };

    leptos::spawn::spawn_local(async move {
        let mut listener = tauri_sys::event::listen::<Option<i32>>("backend")
            .await
            .unwrap();

        while let Some(event) = listener.next().await {
            match event.payload {
                Some(x) => count.set(x),
                None => {
                    count.set(0);
                    disabled.set(false);
                }
            }
        }
    });

    view! {
        <div>
            <Button on_click=trigger_backend_event disabled>
                "Trigger backend event"
            </Button>
            <Tag>"Number of received events: "{count}</Tag>
        </div>
    }
}