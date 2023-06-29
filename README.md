# Tarantula 🕷

[![status](https://img.shields.io/badge/status-stable-blue.svg)](https://github.com/adebola-io/tarantula/tree/master)

This (experimental) project hopes be a (somewhat) faithful implementation of the [WHATWG Document Object Model](https://dom.spec.whatwg.org/) in Rust. 

The purpose is to see how possible it is to wrangle the DOM out of Javascript and squeeze it into a Rust API, possibly for GUI clients, who knows.

Because of the differences in the languages, The API **will** diverge from the specification in situations where there is no feasible implementation. A list of all concessions and compromises so far can be seen [here](https://github.com/adebola-io/tarantula/blob/master/docs/notes.md).

## Theoretical Usage

```rust
use tarantula::prelude::*;

fn main() {
   if let Err(e) = Window::instance("index.html", context, None).run() {
      eprintln!("{}", e);
      std::process::exit(1);
   };
}

fn context(mut window: Window) -> DOMResult {
  
   let mut document = window.document();

   let mut element = document.create_element("div");
   element.set_inner_text("Hello, there!");
   element.style_mut().set_property("color", "green");

   document.body_mut().append(&mut element)?;
}
```
