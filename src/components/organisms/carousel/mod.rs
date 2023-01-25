use gloo::console::log;
use web_sys::console::log;
use yew::prelude::*;

use crate::lorc::generic::*;

mod props;
use props::Props;

pub enum Msg {
    Next,
    Prev,
}

pub struct Carousel {
    image_paths: Vec<String>,
    current_image: usize,
}
impl Component for Carousel {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let paths = &_ctx.props().image_paths;
        Self {
            image_paths: paths.to_vec(),
            current_image: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Next => {
                log!("Next");
                if self.current_image == self.image_paths.len() - 1 {
                    self.current_image = 0;
                } else {
                    self.current_image += 1;
                }
            }
            Msg::Prev => {
                log!("Prev");
                if self.current_image == 0 {
                    self.current_image = self.image_paths.len() - 1;
                } else {
                    self.current_image -= 1;
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let on_prev = _ctx.link().callback(|_| Msg::Prev);
        let on_next = _ctx.link().callback(|_| Msg::Next);
        html! {
            <div class="relative">
                <img src={self.image_paths[self.current_image].to_owned()} class="w-600 h-600"/>
                <button onclick={on_prev} class="absolute top-0 left-0 z-30 flex items-center justify-center h-full px-4 cursor-pointer group focus:outline-none">
                    <span class="inline-flex items-center justify-center w-8 h-8 rounded-full sm:w-10 sm:h-10 bg-white/30 dark:bg-gray-800/30 group-hover:bg-white/50 dark:group-hover:bg-gray-800/60 group-focus:ring-4 group-focus:ring-white dark:group-focus:ring-gray-800/70 group-focus:outline-none">
                        <svg aria-hidden="true" class="w-5 h-5 text-white sm:w-6 sm:h-6 dark:text-gray-800" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path></svg>   
                    </span>
                </button>
                <button onclick={on_next} class="absolute top-0 right-0 z-30 flex items-center justify-center h-full px-4 cursor-pointer group focus:outline-none">
                    <span class="inline-flex items-center justify-center w-8 h-8 rounded-full sm:w-10 sm:h-10 bg-white/30 dark:bg-gray-800/30 group-hover:bg-white/50 dark:group-hover:bg-gray-800/60 group-focus:ring-4 group-focus:ring-white dark:group-focus:ring-gray-800/70 group-focus:outline-none">
                        <svg aria-hidden="true" class="w-5 h-5 text-white sm:w-6 sm:h-6 dark:text-gray-800" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path></svg>
                    </span>
                </button>
            </div>
        }
    }
}
