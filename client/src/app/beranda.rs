use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlVideoElement};
use yew::prelude::*;
use crate::{app::VideoDatabase, components::icons::*};

#[function_component(BerandaPage)]
pub fn beranda_page() -> Html {
  let videos = use_state(|| Vec::<VideoDatabase>::new());
  let video_index = use_state(|| 0);

  {
    let videos = videos.clone();
    use_effect_with((), move |_| {
      spawn_local(async move {
        match Request::get("/api/videos").send().await {
          Ok(res) => {
            match res.json::<Vec<VideoDatabase>>().await {
              Ok(data) => videos.set(data),
              Err(e) => error_message_handle("Database error", e.to_string()),
            }
          },
          Err(e) => error_message_handle("Gagal fetching", e.to_string()),
        }
      });
      || ()
    });
  }

  let onwheel = {
    let videos = videos.clone();
    let video_index = video_index.clone();
    Callback::from(move |e: WheelEvent| {
      if e.delta_y() > 30.0 && *video_index < videos.len() - 1 {
        video_index.set(*video_index + 1);
      } else if e.delta_y() < -30.0 && *video_index > 0 {
        video_index.set(*video_index - 1);
      }
      e.prevent_default();
    })
  };

  let current_video = videos.get(*video_index).cloned();

  html! {
    <div class="relative w-screen h-screen max-w-[480px] mx-auto overflow-hidden bg-black text-white pb-16 md:pb-0" {onwheel}>
      {
        if let Some(video) = current_video {
          html! { <VideoContainer {video}/> }
        } else {
          html! { <h1>{"Video kosong!"}</h1> }
        }
      }
      <Navigasi/>
    </div>
  }
}

#[function_component(Navigasi)]
pub fn navigasi() -> Html {
  html! {
    <>
    <div class="fixed bottom-0 left-0 w-full flex justify-around items-center h-16 bg-black z-50 md:hidden">
      <IconHome/>
      <IconCreate/>
      <IconUser/>
    </div>
    <div class="hidden md:flex fixed top-0 left-0 h-screen w-16 flex-col justify-start items-center bg-black z-50 space-y-4 p-2">
      <IconHome/>
      <IconCreate/>
      <IconUser/>
    </div>
    </>
  }
}

#[derive(Clone, Properties, PartialEq)]
pub struct VideoContainerProps {
  pub video: VideoDatabase
}

#[function_component(VideoContainer)]
pub fn video_container(props: &VideoContainerProps) -> Html {
  let video_ref = use_node_ref();
  let is_playing = use_state(|| true);

  {
    let key = props.video.id.clone();
    let is_playing = is_playing.clone();
    use_effect_with(key, move |_| {
      is_playing.set(true);
      || ()
    });
  }

  let onclick = {
    let video_ref = video_ref.clone();
    let is_playing = is_playing.clone();
    Callback::from(move |_e: MouseEvent| {
      if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
        if !*is_playing {
          let _ = video.play();
          is_playing.set(true);
        } else {
          let _ = video.pause();
          is_playing.set(false);
        }
      }
    })
  };

  html! {
    <div key={props.video.id.clone()} class="relative w-full h-full">
      <video
        ref={video_ref}
        autoplay=true
        loop=true
        muted=true
        playsinline=true
        name="media"
        class="w-full h-full object-cover"
        src={format!("/videos/{}", &props.video.filename)}
      />

      {
        if !*is_playing {
          html! {
            <div class="absolute inset-0 flex items-center justify-center z-20 pointer-events-auto">
              <button {onclick} class="h-[200px] w-[200px] justify-center items-center flex"><IconPlayVideo width={"100"} height={"100"}/></button>
            </div>
          }
        } else {
          html! {
            <div class="absolute inset-0 flex items-center justify-center z-20 pointer-events-auto">
              <button {onclick} class="h-[250px] w-[250px]"></button>
            </div>
          }
        }
      }

      <div class="absolute inset-0 z-10 flex flex-col justify-between p-4">
        <div class="flex flex-col items-center space-y-6 self-end justify-end flex-1">
          <button class="flex flex-col items-center">
            <IconLike/>
            <span class="text-xs font-bold mt-1">{"60"}</span>
          </button>
          <button class="flex flex-col items-center">
            <IconComment/>
            <span class="text-xs font-bold mt-1">{"12"}</span>
          </button>
          <button class="flex flex-col items-center">
            <IconShare/>
            <span class="text-xs font-bold mt-1">{"20"}</span>
          </button>
        </div>

        <div class="flex flex-col space-y-2">
          <div class="flex space-x-2 items-center">
            <IconUserCircle/>
            <h2 class="font-bold text-lg">{&props.video.username}</h2>
          </div>
          <p class="text-sm">{ &props.video.description }</p>
          <p class="text-xs font-semibold">{"Audio: Bawaan Video"}</p>
        </div>
      </div>
    </div>
  }
}

fn error_message_handle(info: &str, e: String) {
  web_sys::console::log_1(&format!("{}: {}", info, e).into());
}