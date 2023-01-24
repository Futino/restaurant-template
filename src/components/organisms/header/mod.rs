use yew::{function_component, html, Html};

use crate::components::*;
use crate::lorc::generic::atoms::*;
use crate::lorc::generic::molecules::*;
use crate::lorc::generic::organisms::*;

mod props;
use props::Props;

#[function_component]
pub fn Header(props: &Props) -> Html {
    html! {
        <div class="fixed z-40 top-0 w-full backdrop-blur flex-none transition-colors duration-500 bg-surface-light/5">
            <div class="py-4 border-b border-secondary-light/10 mx-5">
                <div class="px-auto xl:px-40 lg:px-20 md:px-10 md:px-auto relative flex items-center">

                    // Left side
                    <div class="flex items-center mr-auto">
                        <div class="text-sm leading-6 font-semibold text-primary-on-light">
                            <ul class="m-auto flex space-x-8 items-center">
                                <p class="inline-flex">
                                    <span ><LocationIcon/></span>
                                    {"Stavanger, Norway"}
                                </p>

                                <SquareIcon />

                                <p class="inline-flex items-center">
                                    <span class="p-1"><ClockIcon /></span>
                                    {"Wednessday and Friday : 18:00 to 21:00"}
                                </p>
                            </ul>
                        </div>
                    </div>

                    // Anything in the middle
                    {for props.children.iter()}

                    // Right side
                    <div class="flex items-center ml-auto">
                        <nav class="text-sm leading-6 font-semibold text-primary-on-light">
                            <ul class="m-auto flex space-x-8 items-center">
                                <li>
                                    {"+1 123 456 7890"}
                                </li>
                                <li>
                                <SquareIcon />
                                </li>
                                <li>
                                    {"template@allheim.com"}
                                </li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </div>
        </div>

    }
}
