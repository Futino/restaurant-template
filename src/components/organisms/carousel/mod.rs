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
        Self {
            image_paths: vec![
                "/images/burger1.jpg".to_string(),
                "/images/fries.jpg".to_string(),
                "/images/wings.jpg".to_string(),
            ],
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
            <div>
                <img src={self.image_paths[self.current_image].to_owned()} />
                <button onclick={on_prev}>{ "Prev" }</button>
                <button onclick={on_next}>{ "Next" }</button>
            </div>
        }
    }
}
