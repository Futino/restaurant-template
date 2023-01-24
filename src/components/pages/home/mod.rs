use yew::prelude::*;

use crate::lorc::generic::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>

            // Top section
            <div class="top-0 py-60 px-4 sm:px-6 md:px-8 border-b dark:border-secondary-dark/40 shadow-2xl bg-[url('/images/burger.jpg')] bg-cover object-contain  bg-center">
            <div class="relative max-w-6xl mx-auto space-y-4">
                <Label>
                    <h1 class="font-normal font-bilbo text-8xl text-primary-light">
                        {"Welcome"}
                    </h1>
                </Label>

                <Label>
                    <h2 class="text-8xl tracking-wider font-ubuntu text-primary-on-light border-y border-primary-on-light max-w-fit mx-auto">
                        {"SOUTHSIDE LANTAU"}
                    </h2>
                </Label>
                <Label>
                    <h2 class="text-2xl text-primary-on-light ">
                        {"AMERICAN SOUL FOOD"}
                    </h2>
                </Label>
            </div>
        </div>

            // About section
            <div class="bg-stone-900">
                <div class="grid grid-cols-2 gap-x-3 justify-center items-center my-20 mx-20">
                    <div class="p-8 max-w-lg text-center">
                        <h4 class="p-4 text-base font-light text-primary-200">
                            {
                                "Our Story"
                            }
                        </h4>
                        <SeperatorIcon />
                        <h1 class="p-6 text-5xl font-normal text-white">
                            {"We Offer Top Notch"}
                        </h1>
                        <p class="p-4 text-base font-light leading-6 text-white">
                            {
                                "Lorem Ipsum is simply dummy text of the printingand typesetting industry lorem Ipsum has
                            been the industrys standard dummy text ever since the when an unknown printer took a galley
                            of type and scrambled it to make a type specimen book It has survived not only five centuries,
                            but also the leap into.
                            "
                            }
                        </p>
                        <a href="/About" class="py-3.5 px-6 text-base font-medium bg-transparent border-4 dark:bg-transparent focus:ring-2 focus:outline-none text-primary-200 dark:border-primary-200 dark:hover:bg-primary-200 dark:hover:text-black">
                            {"READ MORE"}
                            </a>
                    </div>
                    <img src="/images/interior.jpeg" class="object-cover p-8 w-auto"/>
                </div>

            </div>
            </main>

            }
    }
}
