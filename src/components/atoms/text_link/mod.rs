use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn TextLink(props: &Props) -> Html {
    let Props { text, href } = props;
    html! {
    <Label>
                            <a class="inline dark:text-primary-200 dark:hover:text-accent-300"
                                href={href.to_owned()}>
                                {text.to_owned()}
                            </a>
                    </Label>
        }
}
