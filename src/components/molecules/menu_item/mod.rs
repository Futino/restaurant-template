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
            <h1 class="pr-4 text-lg text-white">
                {name}
            </h1>
            <div class="inline-flex self-center h-2 border-gray-500 grow border-t-1 border-b-1">
            </div>
            <h1 class="pl-4 text-xl text-primary-200">
                {cost}
            </h1>
            </div>
        <h2 class="text-base font-light text-gray-400">
            {ingredients}
        </h2>

    </div>
        }
}
