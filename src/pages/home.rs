use yew::prelude::*;
use crate::api;
use crate::types::{CartProduct,Product};
use crate::components::ProductCard;
use anyhow::Error;
use yew::format::Json;
use yew::services::fetch::FetchTask;

// #[derive(Clone)]
// struct Product{
//     id: i32,
//     name: String,
//     description: String,
//     image: String,
//     price: f64,
// }

// struct CartProduct{
//     product:Product,
//     quantity:i32,
// }

struct State{
    products: Vec<Product>,
    //cart_products: Vec<CartProduct>,
    get_products_error: Option<Error>,
    get_products_loaded: bool,
}

#[derive(Properties,Clone)]
pub struct Props{
    pub cart_products:Vec<CartProduct>,
    pub on_add_to_cart:Callback<Product>,
}

pub struct Home{
    props:Props,
    state:State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg{
    //AddToCart(i32),
    GetProducts,
    GetProductsSuccess(Vec<Product>),
    GetProductsError(Error),
}

impl Component for Home{
    type Message = Msg;
    type Properties = Props;

    fn create(props:Self::Properties,link:ComponentLink<Self>)->Self{
        // let products: Vec<Product> = vec![
        //     Product{
        //         id:1,
        //         name:"Apple".to_string(),
        //         description:"An apple a day keeps the doctor away".to_string(),
        //         image:"/products/apple.png".to_string(),
        //         price:3.65,
        //     },
        //     Product{
        //         id:2,
        //         name:"Banana".to_string(),
        //         description:"An old banana leaf was one young and green".to_string(),
        //         image:"/products/banana.png".to_string(),
        //         price:7.99,
        //     }
        // ];

        let products = vec![];
        //let cart_products = vec![];

        link.send_message(Msg::GetProducts);

        Self{
            props,
            state:State{
                products,
                //cart_products,
                get_products_error: None,
                get_products_loaded: false,
            },
            link,
            task: None,
        }
    }
    fn update(&mut self,message:Self::Message)->ShouldRender{
        match message {
            Msg::GetProducts =>{
                self.state.get_products_loaded = false;
                let handler = self.link.callback(move |response: api::FetchResponse<Vec<Product>>|{
                    let (_,Json(data)) = response.into_parts();
                    match data{
                        Ok(products) => Msg::GetProductsSuccess(products),
                        Err(err) => Msg::GetProductsError(err),
                    }
                });
                self.task = Some(api::get_products(handler));
                true
            }
            Msg::GetProductsSuccess(products) =>{
                self.state.products = products;
                self.state.get_products_loaded = true;
                true
            }
            Msg::GetProductsError(error)=>{
                self.state.get_products_loaded =true;
                self.state.get_products_error = Some(error);
                true
            }
            // Msg::AddToCart(product_id) =>{
            //     let product = self
            //     .state
            //     .products
            //     .iter()
            //     .find(|p:&&Product| p.id==product_id)
            //     .unwrap();

            //     let cart_product = self.state
            //     .cart_products
            //     .iter_mut()
            //     .find(|cp:&&mut CartProduct| cp.product.id==product_id);

            //     if let Some(cp)=cart_product{
            //         cp.quantity+=1;
            //     }else{
            //         self.state.cart_products.push(CartProduct{
            //             product:product.clone(),
            //             quantity:1,
            //         })
            //     }
            //     true
            // } 
        }
    }
    fn change(&mut self,props:Self::Properties)->ShouldRender{
        self.props = props;
        true
    }
    fn view(&self) -> Html{
        let products: Vec<Html> = self
        .state
        .products
        .iter()
        .map(|product:&Product|{
            //let product_id = product.id;
            html!{
                // <div>
                //     <img src={&product.image}/>
                //     <div>{&product.name}</div>
                //     <div>{"$"}{&product.price}</div>
                //     <button onclick=self.link.callback(move|_| Msg::AddToCart(product_id))>{"Add To Cart"}</button>
                // </div>
                <ProductCard product={product} on_add_to_cart=self.props.on_add_to_cart.clone()></ProductCard>
            }
        })
        .collect();

        // let cart_value = self.state.cart_products
        // .iter()
        // .fold(0.0,|acc,cp| acc+(cp.quantity as f64 * cp.product.price));

        if !self.state.get_products_loaded{
            html!{
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"loading ..."}</div>
                </div>
            }
        }else if let Some(_) = self.state.get_products_error{
            html!{
                <div>
                    <span>{"Error loading products! :("}</span>
                </div>
            }
        }else{
            html!(
                //<div>
                // <span>{format!("Cart Value: ${:.2}",cart_value)}</span>
                // <span>{products}</span>
                // <div class="navbar">
                //     <div class="navbar_title"> {"RustMart"}</div>
                //     <div class="navbar_cart_value">{format!("Cart Value: ${:.2}",cart_value)}</div>
                // </div>
                <div class="product_card_list">{products}</div>
                //</div>
            )
        }
    }


}