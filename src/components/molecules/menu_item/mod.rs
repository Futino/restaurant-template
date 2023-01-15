use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn MenuItem(props: &Props) -> Html {
    let Props {
        name,
        ingredients,
        cost,
    } = props;

    let cost = format!("${:.2}", cost);
    html! {
        <div class="pb-6">
            <div class="flex flex-row pb-2">
            <h1 class="text-white text-lg pr-4">
                {name}
            </h1>
            <div class="self-center grow inline-flex h-2 border-gray-500 border-t-1 border-b-1">
            </div>
            <h1 class="text-primary-200 text-xl pl-4">
                {cost}
            </h1>
            </div>
        <h2 class="text-gray-400 text-base font-light">
            {ingredients}
        </h2>

    </div>
        }
}
