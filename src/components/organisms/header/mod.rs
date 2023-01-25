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
        <div class="fixed z-40 top-0 w-full backdrop-blur flex-none ">
            <div class="py-4 border-b border-secondary-light/10 mx-auto justify-items-center bg-surface-light/5">
                // Top Header
                <div class="px-auto xl:px-40 lg:px-20 md:px-10 md:px-auto relative flex items-center">

                    // Left side
                    <div class="flex items-center mr-auto">
                        <div class="text-sm leading-6 font-semibold text-primary-on-light">
                            <ul class="m-auto flex space-x-8 items-center">
                                <p class="inline-flex">
                                    <span ><LocationIcon/></span>
                                    {"Mui Wo, Lantau"}
                                </p>

                                <SquareIcon />

                                <p class="inline-flex items-center">
                                    <span class="p-1"><ClockIcon /></span>
                                    {"Monday to Friday : 17:00 to 21:00"}
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
                                    {"template@southside.com"}
                                </li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </div>
            // Event header
                <div class="relative flex bg-primary-container-light">

                    // Event Message
                    <div class="flex mx-auto transition ease-in-out delay-150 duration-300 hover:scale-110">
                        <div class="p-6 text-xl leading-6 font-semibold text-primary-container-on-light">
                            <p>
                                {"Today is Taco Tuesday!!! Choose between crowd favorites Guijillo Roasted Chicken and Fried Clams!"}
                            </p>
                        </div>
                    </div>
                </div>
        </div>

    }
}
