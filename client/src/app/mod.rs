pub mod beranda;
pub mod create;

use serde::Deserialize;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  BerandaPage,
  #[at("/create")]
  CreatePage,
  #[at("/chats")]
  ChatsPage,
  #[at("/account")]
  AccountPage,
  #[at("/404")]
  #[not_found]
  NotFoundPage
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct VideoDatabase {
  pub id: String,
  pub filename: String,
  pub username: String,
  pub description: String
}