<h1 align=center>Tarantula</h1>

This (experimental) project hopes be a (somewhat) faithful implementation of the [WHATWG Document Object Model](https://dom.spec.whatwg.org/) in Rust. 

The purpose is to see how possible it is to wrangle the DOM out of Javascript and squeeze it into a Rust API, possibly for GUI clients, who knows.

Because of the differences in the languages, The API **will** diverge from the specification in situations where there is no feasible implementation. A list of all concessions and compromises so far can be seen [here](http://link-to-doc-file).

## Theoretical Usage

```rust
use tarantula::prelude::*;

fn main() {
   let html = std::fs::read_to_string("index.html");

   Window::instance(&html,|window| {
      let mut document = window.document();

      let mut element = document.create_element("div");
      element.set_inner_text("Hello, there!");
      element.style_mut().set_property("color", "green");

      document.body_mut().append(&mut element);
   });
}
```
