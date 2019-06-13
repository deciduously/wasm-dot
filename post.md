# Reactive Canvas with Rust/WebAssembly and web-sys

It's been a little while since I built a resizable dot with a slider in some esoteric stack.  Time for Chapter 3!  I guess it's a series now.

The last two demos used languages that transpile the whole app to regular ol' JavaScript to be interpreted.  This time around, we're going to be compiling our app to WebAssembly first, and then having JavaScript load that.

As per usual with these dots, this is overkill for this app.  Way, *way* overkill.

## Setup

This section is largely copped straight outta the [RustWasm Book](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html).  If you plan to do further work with Rust and WebAssembly, head straight there next (or now).  You'll need a Rust toolchain and NPM to follow along.

First, create a new library-type crate:

```
$ cargo new wasm-dot --lib
$ cd wasm-dot
```

We need to add `wasm-bindgen`.  This crate auto-generates all the JS <-> Rust FFI glue for us, and is much of the reason Rust is such a phenomenal choice for writing WebAssembly.  Open up `Cargo.toml` in the crate root and make it look like this:

```toml
[package]
name = "wasm-dot"
description = "Demo canvas wasm app"
license = "MIT"
version = "0.1.0"
authors = ["You <you@yourcoolsite.you>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
```

The `cdylib` crate type will produce a dynamic system library for loading into another language.  Now open up `src/lib.rs` and make it look like this:

```rust
use wasm_bindgen::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    // import the alert() js function
    pub fn alert(s: &str);
}

// export a Rust function that uses the imported JS function
#[wasm_bindgen]
pub fn say_hi() {
    alert("Hi from Rust/Wasm!");
}
```

We're importing the JavaScript `alert()` function, and exporting our own `say_hi()` Rust function that calls it.  That's all we need to do, `wasm_bindgen` is taking care of the details.  This will just ensure both directions work as intended.

The rustwasm team also provides a tool called [`wasm-pack`](https://github.com/rustwasm/wasm-pack) to automateWebAssembly packaging and integration with npm.  You'll need to install it once with `cargo install wasm-pack`, and then you can use it to build your package:

```rust
$ wasm-pack build
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
    Finished release [optimized] target(s) in 0.00s
[INFO]: :-) Done in 0.05s
[INFO]: :-) Your wasm pkg is ready to publish at ./pkg.
```

Inside `pkg/`, you'll find everything you need to deploy, ready to be imported into any npm project.  All swe need now is a project in which to use it!  Because the rustwasm group thought of everything, there's a template ready to go - use it to create a new project:

```
$ npm init wasm-app www
```

This folder now contains a webpage with all the machinery set up to load your wasm library and call it from `index.js`:

```js
import * as wasm from "hello-wasm-pack";

wasm.greet();
```

There's a stub included so that it will run as is, but we don't want to import from `hello-wasm-pack`, we want to use the app we're developing.  To point it in the right direction, open up `www/package.json` and add a `dependencies` key, pointing directly at the output dir from `wasm-pack`:

```json
  // ..
  "dependencies": {
    "wasm-dot": "file:../pkg"
  }
```

Now we can point `www/index.js` there:

```js
import * as wasm from "wasm-dot";

wasm.say_hi();
```

Let's see if it does the thing:

```
$ npm install // because we added a dependency
$ npm run start
```

You should see the requested alert:

![hello wasm alert box screenshot](https://i.imgur.com/INNppw6.png)

Huzzah!  Now we can iterate.  I recommend opening a second terminal at this point.  In one, run `npm run start` and keep it open, and in the other invoke `wasm-pack build` whenever you make a change to the Rust.

One thing to keep in mind when programming this way is that the memory you're working with from Rust is wholly separate and distinct from the garbage-collected head that JS is using.  Instead, you're working with a linear tape.  I can't put this any better than [the book](https://rustwasm.github.io/docs/book/game-of-life/implementing.html) does:

> As a general rule of thumb, a good JavaScript↔WebAssembly interface design is often one where large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory, and are exposed to JavaScript as opaque handles. JavaScript calls exported WebAssembly functions that take these opaque handles, transform their data, perform heavy computations, query the data, and ultimately return a small, copy-able result. By only returning the small result of the computation, we avoid copying and/or serializing everything back and forth between the JavaScript garbage-collected heap and the WebAssembly linear memory.

Before we look at the JavaScript side, let's build the Rust app.  This will contain the slider element, the canvas element, and the logic.

## Rust

To deal with the JavaScript universe, the `wasm-bindgen` project provides two important crates: `web-sys` provides bindings for all the Web APIS (!!) and `js-sys` provides all the ECMAScript stuff like `Array` and `Date` (!!).  Yeah, they already did the hard work.  It's pretty cool, you don't need to manually define a `Document.createElement` extern or anything.  Instead, just pull in what we need from `web-sys` in `Cargo.toml`:

```toml
[dependencies]
wasm-bindgen = "0.2"

[dependencies.web-sys]
version = "0.3"
features = [
    "Attr",
    "Document",
    "Element",
    "HtmlElement",
    "Node",
    "Text",
    "Window"
]
```

It's a huge crate, so each interface is feature-gated.  You only use what you need.  If you're trying to call a function and it's telling you it doesn't exist, double check the API docs.  It always tells you which features a given method needs:

![feature gate screenshot](https://i.imgur.com/thITZh3.png)

To make sure it's all groovy, we're going to *very verbosely* build a DOM node.  Remove the `alert()` test in `src/lib.rs` and add:

```rust
// Create a title
#[wasm_bindgen]
pub fn mount_a_title() {
    // get
    let window = web_sys::window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not get body");

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
```

Now instead of `say_hi()` we'll need to call `mount_a_title()` in `www/index.js`:

```js
import * as wasm from "wasm-dot";

wasm.mount_a_title();
```

See if it works by running `wasm-pack build` and reloading `localhost:8080`:

![dom node screenshot](https://i.imgur.com/ywMxgDS.png)

Whoa.  Did you see how blazing-fast and WASM-infused that title element was?!

No, you definitely didn't, but still.  Neat.

Now, for the rest of the f&#%*ng owl, we just need to create the slider and the canvas:

```rust
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
```
Now we just call `mount_controls` inside of `mount_app`.  Yes, this is tedious and verbose - building abstractions or leveraging crates to make this task easier is left as an exercise for the reader :)

For now, though, we've defined the DOM tree we need and refactored so that everything is called from `mount_app()`.  You'll also need to adjust `www/index.js` to call this function instead:

```js
import * as wasm from "wasm-dot";

wasm.mount_app();
```

