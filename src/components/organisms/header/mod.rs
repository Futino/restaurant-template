use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn Header(props: &Props) -> Html {
    html! {
        <div class="sticky z-40 top-0 w-full backdrop-blur flex-none transition-colors duration-500 bg-black/5">
            <div id="side-nav" class="fixed z-40 h-screen p-4 overflow-y-auto bg-white w-80 dark:bg-gray-800 transition-transform left-0 top-0 -translate-x-full" tabindex="-1" aria-labelledby="side-nav-label">
                <h5 id="side-nav-label" class="text-base font-semibold text-gray-500 uppercase dark:text-gray-400">{"Menu"}</h5>
                <button type="button" data-side-hide="side-nav" aria-controls="side-nav" class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 absolute top-2.5 right-2.5 inline-flex items-center dark:hover:bg-gray-600 dark:hover:text-white" >
                    <svg aria-hidden="true" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                    <span class="sr-only">{"Close menu"}</span>
                </button>
              <div class="py-4 overflow-y-auto">
                  <ul class="space-y-2">
                     <li>
                        <a href="#" class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                           <svg aria-hidden="true" class="w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M2 10a8 8 0 018-8v8h8a8 8 0 11-16 0z"></path><path d="M12 2.252A8.014 8.014 0 0117.748 8H12V2.252z"></path></svg>
                           <span class="ml-3">{"Dashboard"}</span>
                        </a>
                     </li>
                     <li>
                        <button type="button" class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700" aria-controls="dropdown-example" data-collapse-toggle="dropdown-example">
                              <svg aria-hidden="true" class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 2a4 4 0 00-4 4v1H5a1 1 0 00-.994.89l-1 9A1 1 0 004 18h12a1 1 0 00.994-1.11l-1-9A1 1 0 0015 7h-1V6a4 4 0 00-4-4zm2 5V6a2 2 0 10-4 0v1h4zm-6 3a1 1 0 112 0 1 1 0 01-2 0zm7-1a1 1 0 100 2 1 1 0 000-2z" clip-rule="evenodd"></path></svg>
                              <span class="flex-1 ml-3 text-left whitespace-nowrap">{"E-commerce"}</span>
                              <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                        </button>
                        <ul id="dropdown-example" class="hidden py-2 space-y-2">
                              <li>
                                 <a href="#" class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">{"Products"}</a>
                              </li>
                              <li>
                                 <a href="#" class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">{"Billing"}</a>
                              </li>
                              <li>
                                 <a href="#" class="flex items-center w-full p-2 text-base font-normal text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700">{"Invoice"}</a>
                              </li>
                        </ul>
                     </li>
                     <li>
                        <a href="#" class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                           <svg aria-hidden="true" class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M5 3a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2V5a2 2 0 00-2-2H5zM5 11a2 2 0 00-2 2v2a2 2 0 002 2h2a2 2 0 002-2v-2a2 2 0 00-2-2H5zM11 5a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V5zM11 13a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path></svg>
                           <span class="flex-1 ml-3 whitespace-nowrap">{"Kanban"}</span>
                           <span class="inline-flex items-center justify-center px-2 ml-3 text-sm font-medium text-gray-800 bg-gray-200 rounded-full dark:bg-gray-700 dark:text-gray-300">{"Pro"}</span>
                        </a>
                     </li>
                     <li>
                        <a href="#" class="flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700">
                           <svg aria-hidden="true" class="flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M8.707 7.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l2-2a1 1 0 00-1.414-1.414L11 7.586V3a1 1 0 10-2 0v4.586l-.293-.293z"></path><path d="M3 5a2 2 0 012-2h1a1 1 0 010 2H5v7h2l1 2h4l1-2h2V5h-1a1 1 0 110-2h1a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2V5z"></path></svg>
                           <span class="flex-1 ml-3 whitespace-nowrap">{"Inbox"}</span>
                           <span class="inline-flex items-center justify-center w-3 h-3 p-3 ml-3 text-sm font-medium text-blue-600 bg-blue-200 rounded-full dark:bg-blue-900 dark:text-blue-200">{"3"}</span>
                        </a>
                     </li>
                  </ul>
               </div>
            </div>
            // Top header
            <div class="py-3 border-b border-secondary-300/10 ">
                <div class="xl:mx-1 lg:mx-2 md:mx-3 md:px-auto relative flex items-center text-sm">

                    // Left side
                    <a class="px-3 flex overflow-hidden" href="/">
                        <ul class="flex space-x-3">
                            <p class="inline-flex  text-white">
                                <span><LocationIcon/></span>
                                {"Mau Lam Building, G/F, 16 Mau Lam St"}
                                </p>

                                <SquareIcon />

                                <p class="inline-flex text-white">
                                    <span><ClockIcon/></span>
                                    {"Daily : 8.00 am to 10.00 pm"}
                                    </p>
                        </ul>

                    </a>
                    {for props.children.iter()}

                    // Right side
                    <div class="flex items-center ml-auto ">
                        <nav class="text-sm leading-6 font-semibold text-white">
                            <ul class="px-3 flex space-x-8 text-white">
                                <li>
                                    <TextLink text="+1 123 456 7890" href="/" />
                                </li>
                                <li>
                                <SquareIcon />
                                </li>
                                <li>
                                    <TextLink text="template@restaurant.com" href="/contact" />
                                </li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </div>

            // Bottom header
            <div class="py-4 mx-5">
                <div class="mx-auto xl:mx-3 lg:4 md:mx-5 relative flex items-center">

                    // Left part
                        <button class="text-white bg-white-700 hover:bg-white-800 focus:ring-4 focus:ring-white-300 font-medium rounded-lg text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-white-600 dark:hover:bg-primary-300 focus:outline-none dark:focus:ring-white-800" type="button" data-drawer-target="side-nav" data-drawer-show="side-nav" aria-controls="side-nav">
                            <MenuIcon />
                            </button>

                    {for props.children.iter()}

                    // Right part
                    <div class="flex items-center ml-auto">
                    <button type="button" class="text-black hover:text-white dark:border-primary-200 bg-primary-200 hover:bg-accent-300 focus:ring-4 focus:outline-none focus:ring-primary-200 font-medium text-base px-6 py-3.5 text-center dark:bg-primary-200 dark:hover:bg-black dark:focus:ring-primary-200">
                    {"FIND A TABLE"}
                    </button>
                    </div>
                </div>

            </div>
        </div>

    }
}
