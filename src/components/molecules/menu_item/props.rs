use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub img_path: String,
    pub name: String,
    pub ingredients: String,
    pub cost: f32,
}