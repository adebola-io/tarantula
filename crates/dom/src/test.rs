use crate::{traits::*, Document, HTMLButtonElement};

#[test]
fn test() {
    let document = Document::new();
    let button: HTMLButtonElement = document.create_element("button").try_into().unwrap();
}
