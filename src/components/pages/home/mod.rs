use yew::prelude::*;

use crate::lorc::generic::atoms::*;

use crate::components::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>

            // Top section
            <div class="top-0 py-60 px-4 sm:px-6 md:px-8 border-b border-secondary-light/40 dark:border-secondary-dark/40 shadow-2xl bg-[url('/images/burger.jpg')] bg-cover object-contain  bg-center">
                <div class="relative max-w-6xl mx-auto my-20 space-y-6">
                    <Label>
                        <h2 class="text-2xl text-primary-light ">
                            {"AMERICAN SOUL FOOD"}
                        </h2>
                    </Label>
                    <Label>
                        <h2 class="text-9xl tracking-wider font-bilbo text-primary-on-light dark:text-primary-on-dark border-y border-primary-on-light dark:border-primary-on-dark max-w-fit mx-auto">
                            {"Southside"}
                        </h2>
                    </Label>
                    <Label>
                        <h2 class="text-6xl font-ubuntu text-primary-on-light ">
                            {"LANTAU"}
                        </h2>
                    </Label>
                </div>
            </div>

        // Menu Section
        <div class="w-full bg-surface-light dark:bg-surface-dark">
            <Label>
                <h1 class="p-24 font-ubuntu text-6xl text-center text-surface-on-light dark:text-surface-on-dark">
                    {"OUR FOOD"}
                </h1>
            </Label>

            // Burger section
            <div class="mx-28 py-5">
                <Label>
                    <h1 class="py-6 text-left font-ubuntu text-3xl text-primary-light dark:text-primary-light">
                        {"Burgers"}
                    </h1>
                </Label>
                <div class=" grid grid-cols-3 gap-x-5">
                    <MenuItem img_path="/images/single_burger.jpg" name="Southside Burger" ingredients="Home-style cheeseburger, Served with cheese, lettuce, tomato, dill pickles, mayo, and ketchup." cost=97.0 />
                    <MenuItem img_path="/images/single_burger.jpg" name="Double Double" ingredients="More protein than you can shake a stick at. Same burger with double beef and cheese." cost=97.0 />
                    <MenuItem img_path="/images/single_burger.jpg" name="Cowboy Burger" ingredients="Texas style with smooky barbeque, crispy fried onions, lettuce, tomato, and guacamole." cost=97.0 />
                </div>
            </div>

            // Burger section
            <div class=" mx-28 py-5">
                <Label>
                    <h1 class="py-6 text-left font-ubuntu text-3xl text-surface-variant-on-light dark:text-surface-variant-on-dark">
                        {"Wings & Tenders"}
                    </h1>
                </Label>
                <div class="grid grid-cols-3 gap-x-5">
                    <MenuItem img_path="/images/tenders.jpg" name="Teriyaki" ingredients="Sweet Japanese sauce with sesame and soy." cost=97.0 />
                    <MenuItem img_path="/images/wings.jpg" name="Honey Barbeque" ingredients="Sweet, smoky barbeque" cost=97.0 />
                    <MenuItem img_path="/images/tenders.jpg" name="Sweet Thai Chili" ingredients="Sweet and mild" cost=97.0 />
                </div>
            </div>

        </div>

        // Find us section
        <div class="">
            <Label>
                <h1 class="p-14 font-ubuntu text-6xl text-center text-surface-on-light dark:text-surface-on-dark">
                    {"FIND US!"}
                </h1>
            </Label>
            <div class="grid grid-cols-2 gap-x-10 justify-center gap-y-7 mx-36">
                <div class="flex flex-col justify-center gap-y-7">
                    //TODO need to scale maps based on images to the right of it
                    <iframe class="mx-auto w-full" src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d1126.2156921238252!2d114.00061451484943!3d22.26462049824039!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x3401572b43cb03a9%3A0x9066091be70524b5!2sSouthside%20Lantau!5e1!3m2!1sen!2shk!4v1674646720734!5m2!1sen!2shk"
                    height="600" style="border:0;" allowfullscreen=true loading="lazy" referrerpolicy="no-referrer-when-downgrade">
                    </iframe>
                    <div class="max-w-2xl mx-auto">
                        <Button btn_type="button">
                            <h1 class="text-center text-2xl">
                                {"Find us on google maps"}
                            </h1>
                        </Button>
                    </div>
                </div>
                <div class="flex flex-col justify-center gap-y-7">
                    <Carousel image_paths={vec![
                        "/images/outside.png".to_string(),
                        "/images/outside1.png".to_string(),
                        "/images/outside2.png".to_string(),
                    ]} height="h-600"/>
                    <Label>
                        <h1 class="font-bold text-3xl text-surface-on-light dark:text-surface-on-dark">
                            {"Can you spot us?"}
                        </h1>
                        </Label>
                </div>
            </div>

        </div>

        // Our Story Section
        <div class="w-full bg-surface-light dark:bg-surface-dark">
            <Label>
                <h1 class="p-8 pt-0 font-bold text-3xl text-surface-on-light dark:text-surface-on-dark">
                    {"Our Story"}
                </h1>
                </Label>
        </div>
        </>
        }
    }
}
