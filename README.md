<h1 align=center>Tarantula</h1>

This (experimental) project hopes be a (somewhat) faithful implementation of the [WHATWG Document Object Model](https://dom.spec.whatwg.org/) in Rust. 

The purpose is to see how possible it is to wrangle the DOM out of Javascript and squeeze it into a Rust API, possibly for GUI clients or server-side rendering, who knows.

Because of the differences in the languages, The API **will** diverge from the specification in situations where there is no feasible implementation. A list of all concessions and compromises so far can be seen [here](http://link-to-doc-file).

## Theoretical Usage

```rust
use tarantula_dom::{Window, AsElement};

fn main() {
   Window::create_instance(|window| {
      let mut document = window.document();

      let mut element = document.create_element("div");
      element.set_inner_text("Hello, there!");

      document.body_mut().append(element);
   });
}
```
