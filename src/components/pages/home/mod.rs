use yew::prelude::*;

use crate::lorc::generic::*;

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
            <div class="bg-surface-variant-light mx-28 py-10">
                <Label>
                    <h1 class="py-6 text-left font-ubuntu text-3xl text-surface-variant-on-light dark:text-surface-variant-on-dark">
                        {"Burgers"}
                    </h1>
                </Label>
                <div class="grid grid-cols-3 gap-x-5">
                    <MenuItem img_path="/images/single_burger.jpg" name="Southside Burger" ingredients="Home-style cheeseburger, Served with cheese, lettuce, tomato, dill pickles, mayo, and ketchup" cost=97.0 />
                    <MenuItem img_path="/images/single_burger.jpg" name="Double Double" ingredients="Home-style cheeseburger, Served with cheese, lettuce, tomato, dill pickles, mayo, and ketchup" cost=97.0 />
                    <MenuItem img_path="/images/single_burger.jpg" name="Double Double" ingredients="Home-style cheeseburger, Served with cheese, lettuce, tomato, dill pickles, mayo, and ketchup" cost=97.0 />
                </div>
            </div>

        </div>
            </>
        }
    }
}
