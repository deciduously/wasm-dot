use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement};

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
fn mount_slider_input(document: &Document, parent: &Element) {
    // element
    let slider = document
        .create_element("input")
        .expect("Could not create input element");

    // type attr
    let slider_type = document
        .create_attribute("type")
        .expect("Could not create attribute");
    slider_type.set_value("range");
    slider
        .set_attribute_node(&slider_type)
        .expect("Could not set attribute");

    // min attr
    let min = document
        .create_attribute("min")
        .expect("Could not create attribute");
    min.set_value("5");
    slider.set_attribute_node(&min).expect("");

    // max attr
    let max = document
        .create_attribute("max")
        .expect("Could not create attribute");
    max.set_value("100");
    slider
        .set_attribute_node(&max)
        .expect("Could not set attribute");

    // step attr
    let step = document
        .create_attribute("step")
        .expect("Could not create attribute");
    step.set_value("5");
    slider
        .set_attribute_node(&step)
        .expect("Could not set attribute");

    // id attr
    let id = document
        .create_attribute("id")
        .expect("Could not create attribute");
    id.set_value("size");
    slider
        .set_attribute_node(&id)
        .expect("Could not set attribute");

    // min/max/step/id
    parent
        .append_child(&slider)
        .expect("Could not create slider");
}

fn mount_slider_label(document: &Document, parent: &Element) {
    let label = document
        .create_element("label")
        .expect("Could not create element");

    let for_attr = document
        .create_attribute("for")
        .expect("Could not create attribute");
    for_attr.set_value("size");
    label
        .set_attribute_node(&for_attr)
        .expect("Could not append child");

    parent.append_child(&label).expect("Could not append child");
}

fn mount_size_span(document: &Document, parent: &Element) {
    let span = document
        .create_element("span")
        .expect("Could not create element");
    let id = document
        .create_attribute("id")
        .expect("Could not create attribute");
    id.set_value("size-output");
    span.set_attribute_node(&id)
        .expect("Could not set attribute");
    parent.append_child(&span).expect("Could not append child");
}

fn mount_canvas(document: &Document, parent: &Element) {
    // this is wrapping in a <p>
    let p = document
        .create_element("p")
        .expect("Could not create element");
    let canvas = document
        .create_element("canvas")
        .expect("Could not create element");
    p.append_child(&canvas).expect("Could not append child");
    parent.append_child(&p).expect("Could not append child");
}

fn mount_controls(document: &Document, parent: &HtmlElement) {
    /*
    <div>
        <span>
        <input>
        <label>
        <canvas>
    </div>
    */
    let div = document
        .create_element("div")
        .expect("Could not create element");

    mount_size_span(&document, &div);
    mount_slider_input(&document, &div);
    mount_slider_label(&document, &div);
    mount_canvas(&document, &div);
    parent.append_child(&div).expect("Could not append child");
}

#[wasm_bindgen]
pub fn mount_app() {
    // get window/document/body
    let window = web_sys::window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not get body");

    mount_a_title(&document, &body);
    mount_controls(&document, &body);
}
