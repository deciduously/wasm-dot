use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};

// Create a title
fn mount_a_title(document: &Document, body: &HtmlElement) {
    // create title element
    let title = document
        .create_element("h1")
        .expect("Could not create element");
    let title_text = document.create_text_node("DOTS"); // always succeeds
    title
        .append_child(&title_text)
        .expect("Could not append child to title");

    // append to body
    body.append_child(&title)
        .expect("Could not append title to body");
}

// Mount the slider
fn mount_slider(document: &Document, body: &HtmlElement) {
    let slider = document
        .create_element("input")
        .expect("Could not create input element");
    let sliderType = document
        .create_attribute("type")
        .expect("Could not create attribute");
    sliderType.set_value("range");
    slider
        .set_attribute_node(&sliderType)
        .expect("Could not add attribute");

    // min/max/step/id
    body.append_child(&slider).expect("Could not create slider");
}

#[wasm_bindgen]
pub fn mount_app() {
    // get window/document/body
    let window = web_sys::window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not get body");

    mount_a_title(&document, &body);
    mount_slider(&document, &body);
}
