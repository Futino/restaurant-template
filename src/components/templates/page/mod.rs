use yew::{function_component, html, Html};

use crate::components::*;

mod props;
use props::Props;

#[function_component]
pub fn Page(props: &Props) -> Html {
    html! {
        <div class="min-h-screen dark:bg-black">
            <Header />
            {for props.children.iter()}
            <Footer />
        </div>

    }
}
