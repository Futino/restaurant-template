use yew::prelude::*;

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
        <main>
        <div class="bg-black w-full relative flex flex-col items-center justify-center">
            <div class="h-full bg-black opacity-40">
                <img src="/images/curry.jpg" class="relative h-full bg-cover" alt="Tailwind Play" />
              </div>
        <div class="max-w-3xl mx-auto items-center text-center px-4 py-36 absolute ">
            <h4 class="text-lg font-light text-primary-600 p-6">
                {
                    "DELIGHTFUL EXPERIENCE"
                }
            </h4>
            <h1 class=" text-8xl font-normal text-white p-6">
                {"Flavors Inspired by the Seasons"}
            </h1>
            <h3 class="text-xl font-based text-white p-6">
                {"Come with family & feel the joy of mouthwatering food"}
            </h3>
            <button type="button" class="text-primary-400 border-4 dark:border-primary-400 bg-transparent focus:ring-2 focus:outline-none font-medium text-base px-6 py-3.5 dark:bg-transparent dark:hover:bg-primary-400 dark:hover:text-black">
                {"View our Menu"}
            </button>

        </div>
        </div>
        <div class="">
        </div>
        </main>

        }
    }
}
