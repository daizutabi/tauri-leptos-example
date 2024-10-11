use leptos::prelude::*;
use thaw::{Button, Theme, Tooltip, TooltipPosition};

#[component]
pub fn Theme() -> impl IntoView {
    let theme = Theme::use_rw_theme();
    let theme_name = Memo::new(move |_| {
        theme.with(|theme| {
            if theme.name == "light" {
                "dark".to_string()
            } else {
                "light".to_string()
            }
        })
    });

    let change_theme = move |_| {
        if theme_name.get_untracked() == "light" {
            theme.set(Theme::light());
        } else {
            theme.set(Theme::dark());
        }
    };

    view! {
        <Tooltip
            content=Memo::new(move |_| { format!("Switch to {} mode", theme_name.get()) })
            position=TooltipPosition::Bottom
        >
            <Button
                icon=Memo::new(move |_| {
                    theme
                        .with(|theme| {
                            if theme.name == "light" {
                                icondata::BiSunRegular
                            } else {
                                icondata::BiMoonRegular
                            }
                        })
                })
                on_click=change_theme
            />
        </Tooltip>
    }
}
