#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {}

pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            _ => true
        }
    }
}

impl Model {
    fn shout_out_loud() -> String {
        js! { @(no_return) console.log("shouting out loudly on the console"); }
        return String::from("shouting out loudly for a string");
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
             <div>{"Rust + WebAssembly = 💙💙💙💙♥"}</div>
             <div>{Model::shout_out_loud()}</div>
        }
    }
}