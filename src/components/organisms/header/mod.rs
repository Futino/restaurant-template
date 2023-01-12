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
                <div class="xl:mx-1 lg:mx-2 md:mx-3 md:px-auto relative flex items-center">

                    // Left side
                    <a class="px-3 flex overflow-hidden" href="/">
                        <ul class="flex space-x-3">
                            <p class="inline-flex text-white">
                                <span><LocationIcon/></span>
                                {"Mau Lam Building, G/F, 16 Mau Lam St"}
                                </p>

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
                                    <TextLink text="template@restaurant.com" href="/contact" />
                                </li>
                                <li class="border-l border-secondary-300/10"/>
                                <li class="hover:text-accent-400">
                                    <ThemeButton />
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
                    <a class="px-3 flex overflow-hidden text-white" href="/">
                        <MenuIcon />
                    </a>

                    {for props.children.iter()}

                    // Right part
                    <div class="flex items-center ml-auto">
                    <button type="button" class="text-black hover:text-white dark:border-primary-200 bg-primary-200 hover:bg-accent-300 focus:ring-4 focus:outline-none focus:ring-primary-200 font-medium text-base px-6 py-3.5 text-center dark:bg-primary-200 dark:hover:bg-black dark:focus:ring-primary-400">
                    {"FIND A TABLE"}
                    </button>
                    </div>
                </div>

            </div>
        </div>

    }
}
