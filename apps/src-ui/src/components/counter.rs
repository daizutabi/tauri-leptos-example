use leptos::prelude::*;
use thaw::{Button, Tag};

#[component]
pub fn SimpleCounter(
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let value = RwSignal::new(initial_value);
    let dec = move |_| *value.write() -= step;
    let inc = move |_| value.update(|value| *value += step);

    view! {
        <div>
            <Button on_click=dec>"-1"</Button>
            <Tag>"Value: " {value} "!"</Tag>
            <Button on_click=inc>"+1"</Button>
            <Button on_click=move |_| value.set(0)>"Clear"</Button>
        </div>
    }
}
