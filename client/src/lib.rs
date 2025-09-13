use yew::prelude::*;
use wasm_bindgen_futures::wasm_bindgen::{self, prelude::*};

#[function_component(App)]
fn app() -> Html {
	html! {
		<div>
		  <h1>{"Pewrubahan dikit"}</h1>
		</div>
	}
}

#[wasm_bindgen(start)]
fn main() {
	yew::Renderer::<App>::new().render();
}