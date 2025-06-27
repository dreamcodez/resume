use crate::components::router::history;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub to: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let onclick = {
        let to = props.to.clone();
        let user_onclick = props.onclick.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // Call user's onclick handler if provided
            user_onclick.emit(event);

            // Navigate to the target route
            if let Err(e) = history::push_state(&to) {
                web_sys::console::error_1(&format!("Failed to navigate to {}: {}", to, e).into());
            }
        })
    };

    html! {
        <a
            href={props.to.clone()}
            class={props.class.clone()}
            {onclick}
        >
            {props.children.clone()}
        </a>
    }
}

/// Create a link that replaces the current history entry
#[function_component(ReplaceLink)]
pub fn replace_link(props: &LinkProps) -> Html {
    let onclick = {
        let to = props.to.clone();
        let user_onclick = props.onclick.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            // Call user's onclick handler if provided
            user_onclick.emit(event);

            // Replace current history entry
            if let Err(e) = history::replace_state(&to) {
                web_sys::console::error_1(
                    &format!("Failed to replace route with {}: {}", to, e).into(),
                );
            }
        })
    };

    html! {
        <a
            href={props.to.clone()}
            class={props.class.clone()}
            {onclick}
        >
            {props.children.clone()}
        </a>
    }
}
