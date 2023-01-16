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
            <div class="max-w-7xl mx-auto p-8 pb-12">
                <div class="text-center justify-center">
                    <h4 class="text-sm font-light text-primary-200 p-4">
                        {
                            "STARTER MENU"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class=" text-6xl font-normal text-white p-6">
                        {"Appetizers"}
                    </h1>
                </div>
                <div class="grid grid-cols-3 ">
                    <img src="images/chicken_rice.jpg" class="col-span-2 object-contain p-6"/>
                    <div class="col-span-1 p-6 justify-items-between place-content-between text-left">
                        <MenuItem name="Greek Salad" ingredients="Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese." cost=25.00 />
                        <MenuItem name="Lasagne" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=40.50 />
                        <MenuItem name="Tokusen Wagyu" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=150.50 />
                        <MenuItem name="Butternut Pumpkin" ingredients="Typesetting industry lorem Lorem Ipsum is simply dummy text of the priand." cost=10.20 />
                    </div>
                </div>
            </div>
            <div class="max-w-7xl mx-auto p-8 pb-12">
                <div class="text-center items-center">
                    <h4 class="text-sm font-light text-primary-200 p-4">
                        {
                            "DELICIOUS"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class=" text-6xl font-normal text-white p-6">
                        {"Main Dishes"}
                    </h1>
                </div>
                <div class="grid grid-cols-3 max-w-6xl mx-auto">
                    <div class="col-span-1 p-6 justify-items-between place-content-between text-left">
                        <MenuItem name="Greek Salad" ingredients="Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese." cost=25.00 />
                        <MenuItem name="Lasagne" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=40.50 />
                        <MenuItem name="Tokusen Wagyu" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=150.50 />
                        <MenuItem name="Butternut Pumpkin" ingredients="Typesetting industry lorem Lorem Ipsum is simply dummy text of the priand." cost=10.20 />
                    </div>
                    <img src="images/chicken_rice.jpg" class="col-span-2 object-contain p-6"/>
                </div>
            </div>

            

            <div class="max-w-7xl mx-auto p-8 pb-12">
                <div class="text-center">
                    <h4 class="text-sm font-light text-primary-200 p-4">
                        {
                            "SWEET & SWEET"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class=" text-6xl font-normal text-white p-6">
                        {"Desserts"}
                    </h1>
                </div>
                <div class="grid grid-cols-3 max-w-6xl mx-auto">
                    <div class="col-span-1 p-6 justify-items-between place-content-between text-left">
                        <MenuItem name="Greek Salad" ingredients="Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese." cost=25.00 />
                        <MenuItem name="Lasagne" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=40.50 />
                        <MenuItem name="Tokusen Wagyu" ingredients="Vegetables, cheeses, ground meats, tomato sauce, seasonings and spices." cost=150.50 />
                        <MenuItem name="Butternut Pumpkin" ingredients="Typesetting industry lorem Lorem Ipsum is simply dummy text of the priand." cost=10.20 />
                    </div>
                    <img src="images/chicken_rice.jpg" class="col-span-2 object-contain p-6"/>
                </div>
            </div>

            
        </div>

        </main>

        }
    }
}
