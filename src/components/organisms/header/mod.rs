use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Header(props: &Props) -> Html {
    html! {
        <div class="sticky top-0 z-40 flex-none w-full transition-colors duration-500 backdrop-blur bg-black/5">

            // Top header
            <div class="py-3 border-b border-secondary-300/10">
                <div class="flex relative items-center text-sm text-white md:mx-3 lg:mx-2 xl:mx-1 md:px-auto">

                    // Left side
                    <a class="flex overflow-hidden px-3" href="/">
                        <ul class="flex items-center space-x-3">
                            <p class="inline-flex">
                                <span><LocationIcon/></span>
                                {"Mau Lam Building, G/F, 16 Mau Lam St"}
                                </p>

                                <SquareIcon />

                                <p class="inline-flex">
                                    <span><ClockIcon/></span>
                                    {"Daily : 8.00 am to 10.00 pm"}
                                    </p>
                        </ul>

                    </a>
                    {for props.children.iter()}

                    // Right side
                    <div class="flex items-center ml-auto">
                        <nav>
                            <ul class="flex items-center px-3 space-x-8">
                                <li>
                                    {"+1 123 456 7890"}
                                </li>
                                <li>
                                <SquareIcon />
                                </li>
                                <li>
                                    {"template@restaurant.com"}
                                </li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </div>

            // Bottom header
            <div class="py-4 mx-5">
                <div class="flex relative items-center mx-auto md:mx-5 xl:mx-3 lg:4">

                    // Left part
                        <Logo style="labeled"/>

                    {for props.children.iter()}

                    // Right part
                    <div class="flex items-center ml-auto">
                        <button class="py-2.5 px-5 mr-2 mb-2 text-sm font-medium text-white rounded-lg focus:ring-4 focus:outline-none bg-white-700 dark:bg-white-600 dark:hover:bg-primary-300 dark:focus:ring-white-800 hover:bg-white-800 focus:ring-white-300" type="button" data-drawer-target="side-nav" data-drawer-show="side-nav" aria-controls="side-nav">
                            <MenuIcon />
                            </button>
                    </div>
                </div>

            </div>
        </div>

    }
}
