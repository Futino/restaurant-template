use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Testimonial(props: &Props) -> Html {
    let Props { author, quote } = props;

    html! {
        <div class="p-2 flex flex-col items-center">
            <h1 class="text-center text-2xl font-bold text-surface-on-light dark:text-surface-on-dark">
                {format!("\" {} \"",quote)}
            </h1>
            <h1 class="text-center text-xl text-surface-on-light max-w-fit dark:text-surface-on-dark border-b border-primary-light dark:border-primary-dark">
                {format!("- {}", author)}
            </h1>
        </div>
    }
}
