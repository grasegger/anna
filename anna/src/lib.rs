#[cfg(not(target_arch = "wasm32"))]
compile_error!(
    "This libary is only intended to be built for wasm32-unknown-unknown and won't work otherwise."
);

pub mod component;
pub mod event_mapping;

use component::Component;
use web_sys::window;

pub async fn attach(component: &dyn Component) -> Result<(), String> {
    let mount_point = component.get_mount_point().await.unwrap();
    let document = window().unwrap().document().unwrap();
    let element = document.query_selector(&mount_point).unwrap().unwrap();
    let content = component.render().await.unwrap();

    let wrapper = document.create_element("div").unwrap();
    wrapper.set_id("anna-app-wrapper");
    wrapper.set_inner_html(&content.to_string());

    element.set_inner_html(&wrapper.outer_html());
    Ok(())
}

pub async fn clear () -> Result<(), String> {
    let document = window().unwrap().document().unwrap();

    let element = document.query_selector("anna-app-wrapper").unwrap().unwrap();
    element.remove();
    Ok(())
}