use leptos::prelude::*;
use thaw::ConfigProvider;

use crate::components::counter::SimpleCounter;
use crate::components::event_backend::EventBackend;
use crate::components::event_frontend::EventFrontend;
use crate::components::greet::Greet;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <SimpleCounter initial_value=0 step=1 />
            <Greet />
            <EventFrontend />
            <EventBackend />
        </ConfigProvider>
    }
}
