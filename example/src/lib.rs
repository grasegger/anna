#[macro_use]
extern crate horrorshow;

use anna::component::Component;
use anna::event_mapping::EventMapping;
use async_trait::async_trait;
use horrorshow::prelude::*;
use wasm_bindgen::prelude::*;

struct App {}

#[async_trait]
impl Component for App {
    async fn get_event_mapping(&self) -> Result<Vec<EventMapping>, String> {
        Ok(Vec::new())
    }

    async fn render(&self) -> Result<String, String> {
        let content = html! {
            h1(id="heading") {
                : "Hello! This is <html />"
            }
            ol(id="count") {
                @ for i in 0..10 {
                    li(first? = (i == 0)) {
                        : format_args!("{}", i+1)
                    }
                }
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
