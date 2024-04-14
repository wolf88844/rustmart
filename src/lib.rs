mod pages;
mod api;
mod types;
mod components;
mod app;
mod route;

use yew::prelude::*;
use wasm_bindgen::prelude::*;

// struct Hello{}

// impl Component for Hello{
//     type Message = ();
//     type Properties = ();
//     fn create(_: Self::Properties, _: yew::prelude::ComponentLink<Self>) -> Self {
//         Self{}
//     }
//     fn update(&mut self,_:Self::Message)->ShouldRender{
//         true
//     }
//     fn change(&mut self,_:Self::Properties)->ShouldRender{
//         true
//     }
//     fn view(&self) -> Html{
//         html!(<span>{"Hello World!"}</span>)
//     }
// }

// #[wasm_bindgen(start)]
// pub fn run_app(){
//     App::<Hello>::new().mount_to_body();
// }


//use pages::Home;

#[wasm_bindgen(start)]
pub fn run_app(){
    //App::<Home>::new().mount_to_body();
    //wasm_logger::init(wasm_logger::Config::default());
    App::<app::App>::new().mount_to_body();
}