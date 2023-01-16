use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Header(props: &Props) -> Html {
    html! {
        <div class="sticky z-40 top-0 w-full backdrop-blur flex-none transition-colors duration-500 bg-black/5">

            // Top header
            <div class="py-3 border-b border-secondary-300/10 ">
                <div class="xl:mx-1 lg:mx-2 md:mx-3 md:px-auto relative flex items-center text-sm text-white">

                    // Left side
                    <a class="px-3 flex overflow-hidden" href="/">
                        <ul class="flex space-x-3 items-center">
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
                    <div class="flex items-center ml-auto ">
                        <nav>
                            <ul class="px-3 flex space-x-8 items-center">
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
                <div class="mx-auto xl:mx-3 lg:4 md:mx-5 relative flex items-center">

                    // Left part
                        <Logo style="labeled"/>

                    {for props.children.iter()}

                    // Right part
                    <div class="flex items-center ml-auto">
                        <button class="text-white bg-white-700 hover:bg-white-800 focus:ring-4 focus:ring-white-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-white-600 dark:hover:bg-primary-300 focus:outline-none dark:focus:ring-white-800" type="button" data-drawer-target="side-nav" data-drawer-show="side-nav" aria-controls="side-nav">
                            <MenuIcon />
                            </button>
                    </div>
                </div>

            </div>
        </div>

    }
}
