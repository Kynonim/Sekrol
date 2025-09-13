use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(BerandaPage)]
pub fn beranda_page() -> Html {
  html! {
    <Link<Route> to={Route::CreatePage}>{"Buat"}</Link<Route>>
  }
}