use yew::prelude::*;

use crate::components::*;

pub struct Menu;

impl Component for Menu {
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
        <div class="max-w-3xl mx-auto items-center text-center px-4 py-36 flex justify-between absolute ">
            <div class="">
                <h4 class="text-lg font-light text-primary-200 p-6">
                    {
                        "DELICIOUS & AMAZING"
                    }
                </h4>
                <h1 class=" text-8xl font-normal text-white p-6">
                    {"Our Menu"}
                </h1>
            </div>

        </div>
        </div>
        <div class="">
        </div>
        </main>

        }
    }
}
