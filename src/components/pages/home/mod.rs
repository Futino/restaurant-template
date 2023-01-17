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
        
        // Top section
        <div class="bg-black w-full relative flex flex-col items-center justify-center">
            <div class="h-full bg-black opacity-40">
                <img src="/images/curry.jpg" class="h-full bg-cover" alt="Tailwind Play" />
              </div>
              <div class="max-w-3xl mx-auto items-center px-4 py-36 flex justify-between absolute">
                <div class="text-center">
                    <h4 class="text-lg font-light text-primary-200 p-6">
                        {
                            "DELIGHTFUL EXPERIENCE"
                        }
                    </h4>
                    <h1 class=" text-8xl font-normal text-white p-6">
                        {"Flavors Inspired by the Seasons"}
                    </h1>
                    <h3 class="text-xl font-based text-white p-2">
                        {"Come with family & feel the joy of mouthwatering food"}
                    </h3>
                    <div class="grid grid-cols-2 gap-x-4 p-4 max-w-md mx-auto">
                    <a href="/Menu" class="text-primary-200 border-4 dark:border-primary-200 bg-transparent focus:ring-2 focus:outline-none font-medium text-base px-6 py-3.5 dark:bg-transparent dark:hover:bg-primary-200 dark:hover:text-black">
                        {"View our Menu"}
                    </a>
                    <a href="/Contact" class="text-primary-200 border-4 dark:border-primary-200 bg-transparent focus:ring-2 focus:outline-none font-medium text-base px-6 py-3.5 dark:bg-transparent dark:hover:bg-primary-200 dark:hover:text-black">
                        {"FIND A TABLE"}
                        </a>
                        </div>
                </div>

            </div>
        </div>

        // Various Menus Options
        <div class="bg-black justify-center py-20 mx-40">
            <div class="max-w-md mx-auto p-8 pb-12 text-center ">
                    <h4 class="text-sm font-light text-primary-200 p-4">
                        {
                            "FLAVOURS FOR ROYALTY"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class=" text-5xl font-normal text-white p-6">
                        {"We Offer Top Notch"}
                    </h1>
                    <p class="text-white text-sm leading-6 font-light">
                        {
                            "Lorem Ipsum is simply dummy text of the printing and typesetting 
                            industry lorem Ipsum has been the industrys standard dummy text ever."
                        }
                    </p>
                    
                </div>
                <div class="grid grid-cols-3 gap-x-8 text-center p-6">
                    <div class="p-6 pt-4">
                        <img src="/images/pancakes.jpeg" class="object-cover w-fit h-full " alt="" />
                        <h1 class="text-white text-3xl p-6"> 
                            {"Breakfast"}
                        </h1>
                        <TextLink text="VIEW MENU" href="/Menu"/>
                    </div>
                    <div class="p-6">
                        <img src="/images/pancakes.jpeg" class="object-cover w-fit h-full " alt="" />
                        <h1 class="text-white text-3xl p-6"> 
                            {"Desserts"}
                        </h1>
                        <TextLink text="VIEW MENU" href="/Menu"/>
                    </div>
                    <div class="p-6 pt-4">
                        <img src="/images/pancakes.jpeg" class="object-cover w-fit h-full " alt="" />
                        <h1 class="text-white text-3xl p-6"> 
                            {"Drinks"}
                        </h1>
                        <TextLink text="VIEW MENU" href="/Menu"/>
                    </div>
                </div>
        </div>

        // About section
        <div class="bg-stone-900">
            <div class="my-20 mx-20 grid grid-cols-2 gap-x-3 items-center justify-center">
                <div class="p-8 text-center max-w-lg">
                    <h4 class="text-base font-light text-primary-200 p-4">
                        {
                            "Our Story"
                        }
                    </h4>
                    <SeperatorIcon />
                    <h1 class=" text-5xl font-normal text-white p-6">
                        {"We Offer Top Notch"}
                    </h1>
                    <p class="text-white text-base leading-6 font-light p-4">
                        {
                            "Lorem Ipsum is simply dummy text of the printingand typesetting industry lorem Ipsum has 
                            been the industrys standard dummy text ever since the when an unknown printer took a galley
                            of type and scrambled it to make a type specimen book It has survived not only five centuries,
                            but also the leap into.
                            "
                        }
                    </p>
                    <a href="/About" class="text-primary-200 border-4 dark:border-primary-200 bg-transparent focus:ring-2 focus:outline-none font-medium text-base px-6 py-3.5 dark:bg-transparent dark:hover:bg-primary-200 dark:hover:text-black">
                        {"READ MORE"}
                        </a>
                </div>
                <img src="/images/interior.jpeg" class="object-cover w-auto p-8"/>
            </div>
            
        </div>
        </main>

        }
    }
}
