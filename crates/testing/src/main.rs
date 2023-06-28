use dom::{AsElement, Document, HTMLDivElement};

fn main() {
    let document = Document::new();

    let element: HTMLDivElement = document.create_element("div").try_into().unwrap();
    println!("{}", element.tag_name());
}
