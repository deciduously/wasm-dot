use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement};

type Result<T> = std::result::Result<T, JsValue>;

macro_rules! append_attrs {
    ($document:ident, $el:ident, $( $attr:expr ),* ) => {
        $(
            let attr = $document.create_attribute($attr.0)?;
            attr.set_value($attr.1);
            $el.set_attribute_node(&attr)?;
        )*
    }
}

macro_rules! append_text_child {
    ($document:ident, $el:ident, $text:expr ) => {
        let text = $document.create_text_node($text);
        $el.append_child(&text)?;
    };
}

macro_rules! create_element_attrs {
    ($document:ident, $type:expr, $( $attr:expr ),* ) => {{
        let el = $document.create_element($type)?;
        append_attrs!($document, el, $( $attr ),*);
        el}
    }
}

macro_rules! append_element_attrs {
    ($document:ident, $parent:ident, $type:expr, $( $attr:expr ),* ) => {
        let el = create_element_attrs!($document, $type, $( $attr ),* );
        $parent.append_child(&el)?;
    }
}

macro_rules! append_text_element_attrs {
    ($document:ident, $parent:ident, $type:expr, $text:expr, $( $attr:expr ),*) => {
        let el = create_element_attrs!($document, $type, $( $attr ),* );
        append_text_child!($document, el, $text);
        $parent.append_child(&el)?;
    }
}

fn mount_canvas(document: &Document, parent: &Element) -> Result<()> {
    let p = create_element_attrs!(document, "p",);
    append_element_attrs!(document, p, "canvas", ("id", "dot-canvas"));
    parent.append_child(&p)?;
    Ok(())
}

fn mount_controls(document: &Document, parent: &HtmlElement) -> Result<()> {
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
    mount_canvas(&document, &div)?;
    parent.append_child(&div)?;
    Ok(())
}

fn mount_app(document: &Document, body: &HtmlElement) -> Result<()> {
    append_text_element_attrs!(document, body, "h1", "DOT",);
    mount_controls(&document, &body)?;
    Ok(())
}

// given a new size, sets all relevant DOM elements
fn update_all(document: &Document, new_size: u32) -> Result<()> {
    update_canvas(document, new_size)?;
    Ok(())
}

// draw dot
fn update_canvas(document: &Document, size: u32) -> Result<()> {
    // grab canvas
    let canvas = document
        .get_element_by_id("dot-canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    // resize canvas to size * 2
    let canvas_dim = size * 2;
    canvas.set_width(canvas_dim);
    canvas.set_height(canvas_dim);
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // draw

    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
    // create shape around center point (radius, radius)
    const PI: f64 = 3.14159265;
    context.begin_path();
    context.arc(size.into(), size.into(), size.into(), 0.0, 2.0 * PI)?;
    context.fill();
    context.stroke();

    Ok(())
}

fn run_loop(document: &Document, _body: &HtmlElement) -> Result<()> {
    // listen for size change events

    // set initial size
    let size = 50;
    update_all(document, size)?;

    // add onchange listener to slider
    //document.get_element_by_id("size").unwrap().add_event_listener("onChange", )

    // this will update the canvas, the slider itself, and the span

    Ok(())
}

#[wasm_bindgen]
pub fn run() -> Result<()> {
    // get window/document/body
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    mount_app(&document, &body)?;
    run_loop(&document, &body)?;
    Ok(())
}
