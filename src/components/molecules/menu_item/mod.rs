use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn MenuItem(props: &Props) -> Html {
    let Props {
        img_path,
        name,
        ingredients,
        cost,
    } = props;

    let cost = format!("${:.2}", cost);
    html! {
        <div class="flex flex-col pb-2 w-fit">
                <img src={img_path.to_owned()} class="object-cover object-center" alt="Tailwind Play" />
                <div class="py-2 flex flex-row">
                    <h1 class="ml-auto pr-2 text-xl text-surface-on-light dark:text-surface-on-dark">
                        {name}
                    </h1>
                    <div class="inline-flex self-center h-2 border-primary-light dark:border-primary-dark grow border-t-1 border-b-1">
                    </div>
                    <h1 class="mr-auto pl-2 text-xl text-surface-on-light dark:text-surface-on-dark">
                        {cost}
                    </h1>
                </div>


            <h2 class="py-1 text-base font-light text-surface-on-light dark:text-surface-on-dark">
                {ingredients}
            </h2>
    </div>
        }
}
