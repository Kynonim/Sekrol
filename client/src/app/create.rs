use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{js_sys, wasm_bindgen::{prelude::Closure, JsCast}, HtmlInputElement, HtmlVideoElement, IntersectionObserver, IntersectionObserverEntry};
use yew::prelude::*;

use crate::app::VideoDatabase;

#[function_component(VideoCreatePage)]
pub fn video_create_page() -> Html {
  let videos = use_state(Vec::<VideoDatabase>::new);
  let file_input_ref = use_node_ref();
  let is_uploading = use_state(|| false);

  {
    let videos = videos.clone();
    use_effect_with((), move |_| {
      spawn_local(async move {
        match Request::get("http://localhost:2000/api/videos").send().await {
          Ok(res) => {
            if let Ok(data) = res.json::<Vec<VideoDatabase>>().await {
              videos.set(data);
            }
          },
          Err(e) => web_sys::console::log_1(&format!("fetcing gagal : {:?}", e).into()),
        }
      });
    });
  }

  let onupload = {
    let videos = videos.clone();
    let file_input_ref = file_input_ref.clone();
    let is_uploading = is_uploading.clone();

    Callback::from(move |_: MouseEvent| {
      let input = file_input_ref.cast::<HtmlInputElement>().unwrap();
      let file_list = input.files();
      if file_list.is_none() || file_list.as_ref().unwrap().length() == 0 {
        web_sys::console::log_1(&"tidak ada file yang dipilih".into());
        return;
      }

      let file = file_list.unwrap().get(0).unwrap();
      is_uploading.set(true);
      let videos = videos.clone();

      spawn_local(async move {
        let form = web_sys::FormData::new().unwrap();
        form.append_with_blob("video", &file).unwrap();
        form.append_with_str("title", &file.name()).unwrap();

        match Request::post("http://localhost:2000/api/videos").body(form).unwrap().send().await {
          Ok(_) => {
            match Request::get("http://localhost:2000/api/videos").send().await {
              Ok(res) => {
                if let Ok(data) = res.json::<Vec<VideoDatabase>>().await {
                  videos.set(data);
                }
              },
              Err(e) => web_sys::console::log_1(&format!("gagal upload: {:?}", e).into()),
            }
          },
          Err(e) => web_sys::console::log_1(&format!("gagal upload: {:?}", e).into()),
        }
      });
      is_uploading.set(false);
    })
  };

  html! {
    <div>
      <h1>{"Sekrol"}</h1>
      <div>
        <input type="file" ref={file_input_ref} accept="video/*"/>
        <button onclick={onupload} disabled={*is_uploading}>{ if *is_uploading {"Uploading..."} else {"Upload Now"}}</button>
      </div>
      <div>
      {
        for videos.iter().map(|i| html! {
          <VideoCard video={i.clone()}/>
        })
      }
      </div>
    </div>
  }
}

#[derive(Properties, PartialEq)]
pub struct VideoCardProps {
  video: VideoDatabase,
}

#[function_component(VideoCard)]
pub fn video_card(props: &VideoCardProps) -> Html {
  let video_ref = use_node_ref();
  {
    let video_ref = video_ref.clone();
    use_effect(move || {
      if let Some(element) = video_ref.cast::<HtmlVideoElement>() {
        let element = &element;
        let closure = Closure::wrap(Box::new(move |entries: js_sys::Array, _obs: IntersectionObserver| {
          let entry = entries.get(0).unchecked_into::<IntersectionObserverEntry>();
          let elm = element.clone();
          if entry.is_intersecting() {
            elm.set_playback_rate(1.0);
            let _ = elm.play();
          } else {
            let _ = elm.pause();
          }
        }) as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);
        let observer = IntersectionObserver::new(closure.as_ref().unchecked_ref()).unwrap();
        observer.observe(element);
        closure.forget();
      }
    });
  }

  html! {
    <div>
      <video
        ref={video_ref}
        src={format!("/videos/{}", props.video.filename)}
        controls=false
        muted=true
        autoplay=true
        loop=true
      />
      <div>
        {&props.video.username}
      </div>
    </div>
  }
}