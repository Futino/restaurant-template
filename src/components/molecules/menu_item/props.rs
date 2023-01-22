use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub name: String,
    pub ingredients: String,
    pub cost: f32,
}
