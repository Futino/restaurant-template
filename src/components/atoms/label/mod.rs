use yew::{function_component, html, Html};

mod props;
use props::Props;

#[function_component]
pub fn Label(props: &Props) -> Html {
    html! {
        <div class="text-center text-black align-middle dark:text-white">
            {for props.children.iter()}
        </div>
    }
}
