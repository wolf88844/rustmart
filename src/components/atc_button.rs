use crate::types::Product;
use yew::prelude::*;

pub struct AtcButton{
    props: Props,
    link:ComponentLink<Self>,
}

#[derive(Properties,Clone)]
pub struct Props{
    pub product:Product,
    pub on_add_to_cart: Callback<Product>,
}

pub enum Msg{
    AddToCart,
}

impl Component for AtcButton{
    
}