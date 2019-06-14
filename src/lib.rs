use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement};

macro_rules! append_attrs {
    ($document:ident, $el:ident, $( $attr:expr ),* ) => {
        $(
            let attr = $document.create_attribute($attr.0).expect("Could not create attribute");
            attr.set_value($attr.1);
            $el.set_attribute_node(&attr).expect("Could not set attribute");
        )*
    }
}

macro_rules! append_text_child {
    ($document:ident, $el:ident, $text:expr ) => {
        let text = $document.create_text_node($text);
        $el.append_child(&text).expect("Could not append text node");
    };
}

macro_rules! create_element_attrs {
    ($document:ident, $type:expr, $( $attr:expr ),* ) => {{
        let el = $document.create_element($type).expect("Could not create element");
        append_attrs!($document, el, $( $attr ),*);
        el}
    }
}

macro_rules! append_element_attrs {
    ($document:ident, $parent:ident, $type:expr, $( $attr:expr ),* ) => {
        let el = create_element_attrs!($document, $type, $( $attr ),* );
        $parent.append_child(&el).expect("Could not append child");
    }
}

macro_rules! append_text_element_attrs {
    ($document:ident, $parent:ident, $type:expr, $text:expr, $( $attr:expr ),*) => {
        let el = create_element_attrs!($document, $type, $( $attr ),* );
        append_text_child!($document, el, $text);
        $parent.append_child(&el).expect("Could not append child");
    }
}

fn mount_canvas(document: &Document, parent: &Element) {
    let p = create_element_attrs!(document, "p",);
    append_element_attrs!(document, p, "canvas",);
    parent.append_child(&p).expect("Could not append child");
}

fn mount_controls(document: &Document, parent: &HtmlElement) {
    // containing div
    let div = create_element_attrs!(document, "div", ("id", "rxcanvas"));
    // span
    // TODO pass in state?  5 is hardcoded here, but you havent done state yet.
    append_text_element_attrs!(document, div, "span", "5", ("id", "size-output"));
    // input
    append_element_attrs!(
        document,
        div,
        "input",
        ("id", "size"),
        ("type", "range"),
        ("min", "5"),
        ("max", "100"),
        ("step", "5")
    );
    // label
    append_text_element_attrs!(document, div, "label", "- Size", ("for", "size"));
    // canvas
    mount_canvas(&document, &div);
    parent.append_child(&div).expect("Could not append child");
}

fn mount_app(document: &Document, body: &HtmlElement) {
    append_text_element_attrs!(document, body, "h1", "DOT",);
    mount_controls(&document, &body);
}

fn run_loop(document: &Document, body: &HtmlElement) {
    // listen for size change events

    // set initial size
    let mut size = 5;

    // add onchange listener to slider
    //document.get_element_by_id()

    // this will update the canvas, the slider itself, and the span
}

#[wasm_bindgen]
pub fn run() {
    // get window/document/body
    let window = web_sys::window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not get body");

    mount_app(&document, &body);
    run_loop(&document, &body);
}
