# Reactive Canvas with Rust/WebAssembly and web-sys

## Or How I learn To Stop Worrying And Love Macros

It's been a little while since I built a resizable dot with a slider in some esoteric stack.  Time for Chapter 3!  I guess it's a series now.

The last two demos used languages that transpile the whole app to regular ol' JavaScript to be interpreted.  This time around, we're going to be compiling our app to WebAssembly first, and then having JavaScript load that.

As per usual with these dot demos, this is overkill for this app.  This one perhaps especially so.

## The Pipeline

This section is largely copped straight outta the [RustWasm Book](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html).  If you plan to do further work with Rust and WebAssembly, head straight there next (or now).  You'll need a Rust toolchain and Node/NPM to follow along.

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

## The Layout

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

To make sure it's all groovy, we're going to build a DOM node ourselves, JS-style but, like, also Rust-style.  Remove the `alert()` test in `src/lib.rs` and add:

```rust
#[wasm_bindgen]
pub fn run() {
    // get window/document/body
    let window = web_sys::window().expect("Could not get window");
    let document = window.document().expect("Could not get document");
    let body = document.body().expect("Could not get body");

    mount_app(&document, &body);
}

fn mount_app(document: &Document, body: &HtmlElement) {
    mount_title(&document, &body);
}

// Create a title
fn mount_title(document: &Document, body: &HtmlElement) {
    // create title element
    let title = document
        .create_element("h1")
        .expect("Could not create element");
    let title_text = document.create_text_node("DOT"); // always succeeds
    title
        .append_child(&title_text)
        .expect("Could not append child to title");

    // append to body
    body.append_child(&title)
        .expect("Could not append title to body");
}
```

Now instead of `say_hi()` we'll need to call `run()` in `www/index.js`:

```js
import * as wasm from "wasm-dot";

wasm.run();
```

See if it works by running `wasm-pack build` and reloading `localhost:8080`:

![dom node screenshot](https://i.imgur.com/7yiqu7f.png)

Whoa.  Did you see how blazing-fast and WASM-infused that title was?!

No, you didn't, but still.  Neat.

Next we need to define the DOM tree we want.  Here's what we're aiming for in HTML:

```html
  <div id="rxcanvas">
    <span id="size-output"></span>
    <input id="size" type="range" min="1" max="100" step="5">
    <label for="size">- Size</label>
    <p>
      <canvas></canvas>
    </p>
  </div>
```

If you've ever manipulated the DOM via JavaScript, you're pretty much good to go.  In Rust, though, this is *so verbose*.  I promised up above there would be macros - here we go.

For the uninitiated, a macro is a bit of code that expands into other code *before* everything else is evaluated.  In Rust, they look like function calls but with an exclamation point at the end.  They aren't function calls at all though - when the compiler comes through your module, it expands all of these anywhere they find them into the full Rust code you (or a library) defined.

The syn

Rust actually has another type of macro called a [procedural macro](https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html) that's *even more powerful and arcane* but for now `macro_rules!` will do us just fine.

This is the only place in Rust you'll see that `macro_rules! thing { () => {} }` bracket pattern.  It's it's own special syntax.  The parameters in parens are copied in to the Rust code in the curly braces during expansion, right in place in your code.  For example, this is ``

```rust
macro_rules! append_attrs {
    ($document:ident, $el:ident, $( $attr:expr ),* ) => {
        $(
            let attr = $document.create_attribute($attr.0).expect("Could not create attribute");
            attr.set_value($attr.1);
            $el.set_attribute_node(&attr).expect("Could not set attribute");
        )*
    }
}
```

When called, each one will just paste this block of Rust into our function in place, using what we pass in.  The first one, `append_attrs!()`, is variadic.  The `$( $name:expr ),*` syntax means that it will carry out this block for zero or more arguments given, pasting the block in the curly braces to match.  Each time through, the arg we're processing gets the name $attr.

You can call it like this, with as many trailing tuple arguments as needed for each attribute:

```rust
append_attrs!(document, label, ("for", "size"));
```

We can do better, though - macros can call other macros!  We can boil everything down to the bare minimum by defining a few more helpers:

```rust
macro_rules! append_text_child {
    ($document:ident, $el:ident, $text:expr ) => {
        let text = $document.create_text_node($text);
        $el.append_child(&text).expect("Could not append text node");
    }
}

macro_rules! create_element_attrs {
    ($document:ident, $type:expr, $( $attr:expr ),* ) => {{
        let el = $document.create_element($type).expect("Could not create element");
        append_attrs!($document, el, $( $attr),*);
        el}
    }
}

macro_rules! append_element_attrs {
    ($document:ident, $parent:ident, $type:expr, $( $attr:expr ),* ) => {
        let el = create_element_attrs!($document, $type, $( $attr),* );
        $parent.append_child(&el).expect("Could not append child");
    }
}

macro_rules! append_text_element_attrs {
    ($document:ident, $parent:ident, $type:expr, $text:expr, $( $attr:expr ),*) => {
        let el = create_element_attrs!($document, $type, $( $attr),* );
        append_text_child!($document, el, $text);
        $parent.append_child(&el).expect("Could not append child");
    }
}

```

There are two "top-level" macros, `append_element_attrs` and `append_text_element_attrs`.  The former will append a childless element with the given attributes to the parent provided and the latter will include a text node child.  Note that to pass the variadic trailing arguments down you just use the same syntax inside the curly brace expansion but omit the `expr` type:

```rust
let el = create_element_attrs!($document, $type, $( $attr ),* );
```

Now we can replace the entirety of `mount_title()` with a macro invocation:

```rust
fn mount_app(document: &Document, body: &HtmlElement) {
    append_text_element_attrs!(document, body, "h1", "DOT",);
}
```

Note the trailing comma is mandatory - that's the "zero or more" attributes the macro accepts.  That's so much boilerplate we've avoided though.  The above function is what the compiler sees when building the binary, we just saved ourselves the hassle of typing it all.  Thanks, macros!  Thacros.

Here's the whole f#@%!^g owl:

```rust
fn mount_canvas(document: &Document, parent: &Element) {
    let p = create_element_attrs!(document, "p",);
    append_element_attrs!(document, p, "canvas",);
    parent.append_child(&p).expect("Could not append child");
}

fn mount_controls(document: &Document, parent: &HtmlElement) {
    // containing div
    let div = create_element_attrs!(document, "div", ("id", "rxcanvas"));
    // span
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
```

Fully expanded, that slider input alone would be clunky:

```rust
// Mount the slider
fn mount_slider_input(document: &Document, parent: &Element) {
    // element
    let slider = document
        .create_element("input")
        .expect("Could not create input element");

    // id attr
    let id = document
        .create_attribute("id")
        .expect("Could not create attribute");
    id.set_value("size");
    slider
        .set_attribute_node(&id)
        .expect("Could not set attribute");

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
    slider.set_attribute_node(&min).expect("Could not set attribute");

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

    parent
        .append_child(&slider)
        .expect("Could not append child");
}
```

It comes out verbose in Rust, but you get a) type safety and b) hipster cred.  All the `web_sys` calls look very familiar if you're coming from JavaScript.  If you want a Web API function, just try looking for it in the [`web-sys` API docs](https://rustwasm.github.io/wasm-bindgen/api/web_sys/).  Each listing will conveniently link to the corresponding MDN page, too!  Leveraging crates or writing your own abstractions to make this smoother is both quite possible and left as an exercise for the reader.

Rebuild with `wasm-pack build`, and if you have `webpack-dev-server` running (via `npm run start`) you can reload `localhost:8080`:

![DOM tree screenshot](https://i.imgur.com/yTLnwg0.png)

Good stuff.

## The Loop
