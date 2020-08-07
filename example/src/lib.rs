#[macro_use]
extern crate horrorshow;

use anna::component::Component;
use anna::event_mapping::EventMapping;
use async_trait::async_trait;
use wasm_bindgen::prelude::*;
use web_sys::Event;

struct App {}

impl App {
    pub async fn remove_anna(event: Event) {

    }
}

#[async_trait]
impl Component for App {
    async fn get_event_mapping(&self) -> Result<Vec<EventMapping<'_>>, String> {
        let remove_anna = EventMapping::new(
            "remove-anna".into(),
             "click".into(), 
             &Self::remove_anna
            );
        Ok(Vec::new())
    }

    async fn render(&self) -> Result<String, String> {
        let content = html! {
            h1 {
                : "Hello, Anna"
            }
            button(id="remove-anna"){
                : "remove"
            }
        };
        Ok(format!("{}", content))
    }

    async fn get_mount_point(&self) -> Result<String, String> {
        Ok("body".to_string())
    }
}

#[wasm_bindgen(start)]
pub async fn main() {
    let my_app = App {};
    anna::attach(&my_app).await.unwrap();
}