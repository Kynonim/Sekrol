mod app;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::wasm_bindgen::{self, prelude::*};

use crate::app::{beranda::BerandaPage, create::VideoCreatePage, Route};

#[function_component(App)]
fn app() -> Html {
	html! {
		<BrowserRouter>
		  <Switch<Route> render={switch} />
		</BrowserRouter>
	}
}

fn switch(routes: Route) -> Html {
	match routes {
		Route::BerandaPage => html! { <BerandaPage/> },
		Route::CreatePage => html! { <VideoCreatePage/> },
		Route::ChatsPage => html! { <div>{"chats"}</div> },
		Route::AccountPage => html! { <div>{"account"}</div> },
		Route::NotFoundPage => html! { <div>{"404"}</div> },
	}
}

#[wasm_bindgen(start)]
fn imphnen() {
	yew::Renderer::<App>::new().render();
}