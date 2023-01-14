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
                <div class="xl:mx-1 lg:mx-2 md:mx-3 md:px-auto relative flex items-center text-sm">

                    // Left side
                    <a class="px-3 flex overflow-hidden" href="/">
                        <ul class="flex space-x-3">
                            <p class="inline-flex  text-white">
                                <span><LocationIcon/></span>
                                {"Mau Lam Building, G/F, 16 Mau Lam St"}
                                </p>

                                <SquareIcon />

                                <p class="inline-flex text-white">
                                    <span><ClockIcon/></span>
                                    {"Daily : 8.00 am to 10.00 pm"}
                                    </p>
                        </ul>

                    </a>
                    {for props.children.iter()}

                    // Right side
                    <div class="flex items-center ml-auto ">
                        <nav class="text-sm leading-6 font-semibold text-white">
                            <ul class="px-3 flex space-x-8 text-white">
                                <li>
                                    <TextLink text="+1 123 456 7890" href="/" />
                                </li>
                                <li>
                                <SquareIcon />
                                </li>
                                <li>
                                    <TextLink text="template@restaurant.com" href="/contact" />
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
                        <button class="text-white bg-white-700 hover:bg-white-800 focus:ring-4 focus:ring-white-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-white-600 dark:hover:bg-primary-300 focus:outline-none dark:focus:ring-white-800" type="button" data-drawer-target="side-nav" data-drawer-show="side-nav" aria-controls="side-nav">
                            <MenuIcon />
                            </button>

                    {for props.children.iter()}

                    // Right part
                    <div class="flex items-center ml-auto">

                    </div>
                </div>

            </div>
        </div>

    }
}
