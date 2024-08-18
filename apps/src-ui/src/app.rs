use leptos::prelude::*;
use thaw::ConfigProvider;

use crate::components::counter::SimpleCounter;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <SimpleCounter initial_value=0 step=1 />
        </ConfigProvider>
    }
}
