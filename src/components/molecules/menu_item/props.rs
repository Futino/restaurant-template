use yew::{Children, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    // Name of food item (e.g. "Greek Salad")
    pub name: String,

    // List of ingredients (e.g. "Tomatoes, green bell pepper, sliced cucumber onion, olives, and feta cheese.")
    pub ingredients: String,

    // Cost of item (e.g. 25.50)
    pub cost: f32,
}
