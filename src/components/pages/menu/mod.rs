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
        <div class="w-full relative flex flex-col items-center justify-center">
            <div class="h-96 bg-black opacity-50 w-full">
                <img src="/images/dark_vegs.jpg" class="object-none object-center h-full w-full" alt="Tailwind Play" />
              </div>
            <div class="max-w-3xl mx-auto items-center text-center px-4 py-36 flex justify-between absolute ">
                <div class="">
                    <h4 class="text-base font-light text-primary-200 p-6">
                        {
                            "DELICIOUS & AMAZING"
                        }
                    </h4>
                    <h1 class=" text-6xl font-normal text-white p-6">
                        {"Our Menu"}
                    </h1>
                </div>

            </div>

        </div>

        <div class="bg-stone-800 relative justify-center w-full">
            <div class="text-center">
                <h4 class="text-sm font-light text-primary-200 p-6">
                    {
                        "STARTER MENU"
                    }
                </h4>
                <h1 class=" text-6xl font-normal text-white p-6">
                    {"Appetizers"}
                </h1>
            </div>
            <div class="flex flex-row max-w-5xl mx-auto">
                <img src="images/chicken_rice.jpg" class="object-contain h-96 p-8"/>
                <div class="max-w-2xl p-8 text-left">
                    <MenuItem name="Greek Salad" ingredients="Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese." cost=25.50 />
                    </div>
            </div>
        </div>

        </main>

        }
    }
}
