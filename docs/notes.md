# Notes to Future Self

This file will contain things to note regarding the implementation, such as changes that have been made to the API (for the sake of clarity / idiomatic cohesion), current issues and drawbacks, or just stuff deemed important enough in regards to the project.

## Compromises

Due to the differences between Rust and Javascript/Typescript, a number of changes have been made to achieve a balance between the architecture of the DOM and Rust semantics.

### Casing

The `camelCase` style of the DOM is rewritten with `pascal_case`. So

```js
let rootElement = document.querySelector("html");
console.log(rootElement.getAttribute("lang"));
```

now equals:

```rust
let root_element = document.query_selector("html");
println!("{:?}", root_element.get_attribute("lang"));
```

### Object properties

All properties on the DOM objects are now methods, because:
- Most properties are read-only anyways, like `Element.attributes` or `Document.documentRoot`.
- It is harder and less efficient to keep redundant references in structs. Nobody likes fighting lifetimes.
- No way to inherit properties in Rust, so if `Attr.nodeName` and `Node.nodeName` must be two distinct things, despite being essentially the same for an attribute node.
- Some properties are actually getters and setters, meaning they do more than just assign a value during an assignment operation.

In summary,
```js
element.parentElement?.classList.length;
```

is equivalent to:

```rust
element.parent_element()?.class_list().len();
```

> Note: All instances of `length` are also now `len`, because that's what the Rust Standard Library uses.

### Setters and getters

A runoff of the last compromise. Mutable properties on objects are now split into two separate methods: One to set the property, and another to return it.

Ergo,
```js
button.id = "hero-button";
button.className = "bg-black text-white";
console.log(button.id, button.className);
```

is transformed to:

```rust
button.set_id("hero-button");
button.set_class_name("bg-black text-white");
println!("{}, {}", button.id(), button.class_name());
```


### Upcasting and Downcasting
### Mutability