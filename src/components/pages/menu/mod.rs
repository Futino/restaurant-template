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
        <div class="flex relative flex-col justify-center items-center w-full">
            <div class="w-full h-96 bg-black opacity-50">
                <img src="/images/dark_vegs.jpg" class="object-none object-center w-full h-full" alt="Tailwind Play" />
              </div>
            <div class="flex absolute justify-between items-center py-36 px-4 mx-auto max-w-3xl text-center">
                <div class="">
                    <h4 class="p-6 text-base font-light text-primary-200">
                        {
                            "DELICIOUS & AMAZING"
                        }
                    </h4>
                    <h1 class="p-6 text-6xl font-normal text-white">
                        {"Our Menu"}
                    </h1>
                </div>

            </div>

        </div>

        <div class="relative justify-center w-full bg-stone-800">
            <div class="p-8 pb-12 mx-auto max-w-7xl">
                <div class="justify-center text-center">
                    <h4 class="p-4 text-sm font-light text-primary-200">
                        {
                            "STARTER MENU"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class="p-6 text-6xl font-normal text-white">
                        {"Appetizers"}
                    </h1>
                </div>
                <div class="grid grid-cols-3">
                    <img src="images/chicken_rice.jpg" class="object-contain col-span-2 p-6"/>
                    <div class="col-span-1 place-content-between p-6 text-left justify-items-between">
                        <MenuItem name="Greek Salad" ingredients="Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese." cost=25.00 />
                        <MenuItem name="Lasagne" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=40.50 />
                        <MenuItem name="Tokusen Wagyu" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=150.50 />
                        <MenuItem name="Butternut Pumpkin" ingredients="Typesetting industry lorem Lorem Ipsum is simply dummy text of the priand." cost=10.20 />
                    </div>
                </div>
            </div>
            <div class="p-8 pb-12 mx-auto max-w-7xl">
                <div class="items-center text-center">
                    <h4 class="p-4 text-sm font-light text-primary-200">
                        {
                            "DELICIOUS"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class="p-6 text-6xl font-normal text-white">
                        {"Main Dishes"}
                    </h1>
                </div>
                <div class="grid grid-cols-3 mx-auto max-w-6xl">
                    <div class="col-span-1 place-content-between p-6 text-left justify-items-between">
                        <MenuItem name="Greek Salad" ingredients="Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese." cost=25.00 />
                        <MenuItem name="Lasagne" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=40.50 />
                        <MenuItem name="Tokusen Wagyu" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=150.50 />
                        <MenuItem name="Butternut Pumpkin" ingredients="Typesetting industry lorem Lorem Ipsum is simply dummy text of the priand." cost=10.20 />
                    </div>
                    <img src="images/chicken_rice.jpg" class="object-contain col-span-2 p-6"/>
                </div>
            </div>

            

            <div class="p-8 pb-12 mx-auto max-w-7xl">
                <div class="text-center">
                    <h4 class="p-4 text-sm font-light text-primary-200">
                        {
                            "SWEET & SWEET"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class="p-6 text-6xl font-normal text-white">
                        {"Desserts"}
                    </h1>
                </div>
                <div class="grid grid-cols-3 mx-auto max-w-6xl">
                    <div class="col-span-1 place-content-between p-6 text-left justify-items-between">
                        <MenuItem name="Greek Salad" ingredients="Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese." cost=25.00 />
                        <MenuItem name="Lasagne" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=40.50 />
                        <MenuItem name="Tokusen Wagyu" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=150.50 />
                        <MenuItem name="Butternut Pumpkin" ingredients="Typesetting industry lorem Lorem Ipsum is simply dummy text of the priand." cost=10.20 />
                    </div>
                    <img src="images/chicken_rice.jpg" class="object-contain col-span-2 p-6"/>
                </div>
            </div>

            
        </div>

        </main>

        }
    }
}
