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
            <div class="top-0 py-60 px-4 sm:px-6 md:px-8 border-b dark:border-secondary-dark/40 shadow-2xl bg-[url('/images/curry.jpg')] bg-cover object-cover bg-center">
            <div class="relative max-w-6xl mx-auto">
                <Label>
                    <h1 class="font-extrabold text-8xl sm:text-5xl lg:text-6xl m-5">
                        {"Allheim WMA/HEMA"}
                    </h1>
                </Label>

                <Label>
                    <h2 class="text-4xl">
                        {"Come with famiy and friends to feel the joy of hitting them with swords"}
                    </h2>
                </Label>

                <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
                    <a href="/contact">
                        <Button btn_type="button">
                            <p>{"Contact Us!"}</p>
                        </Button>
                    </a>
                    <a href="/about">
                        <Button btn_type="button">
                            <p>{"Who Are We?"}</p>
                        </Button>
                    </a>
                </div>
            </div>
        </div>

            // Various Menus Options
            <div class="justify-center py-20 mx-40 bg-black">
                <div class="p-8 pb-12 mx-auto max-w-md text-center">
                        <h4 class="p-4 text-sm font-light text-primary-200">
                            {
                                "FLAVOURS FOR ROYALTY"
                            }
                        </h4>
                        <SeperatorIcon />
                        <h1 class="p-6 text-5xl font-normal text-white">
                            {"We Offer Top Notch"}
                        </h1>
                        <p class="text-sm font-light leading-6 text-white">
                            {
                                "Lorem Ipsum is simply dummy text of the printing and typesetting
                                industry lorem Ipsum has been the industrys standard dummy text ever."
                            }
                        </p>

                    </div>
                    <div class="grid grid-cols-3 gap-x-8 p-6 text-center">
                        <div class="p-6 pt-4">
                            <img src="/images/pancakes.jpeg" class="object-cover h-full w-fit" alt="" />
                            <h1 class="p-6 text-3xl text-white">
                                {"Breakfast"}
                            </h1>
                            <TextLink text="VIEW MENU" href="/Menu"/>
                        </div>
                        <div class="p-6">
                            <img src="/images/pancakes.jpeg" class="object-cover h-full w-fit" alt="" />
                            <h1 class="p-6 text-3xl text-white">
                                {"Desserts"}
                            </h1>
                            <TextLink text="VIEW MENU" href="/Menu"/>
                        </div>
                        <div class="p-6 pt-4">
                            <img src="/images/pancakes.jpeg" class="object-cover h-full w-fit" alt="" />
                            <h1 class="p-6 text-3xl text-white">
                                {"Drinks"}
                            </h1>
                            <TextLink text="VIEW MENU" href="/Menu"/>
                        </div>
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
