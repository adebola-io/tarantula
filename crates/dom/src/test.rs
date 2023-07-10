#[warn(unused)]
use crate::{traits::*, Document, HTMLAnchorElement};

#[test]
fn test() {
    let document = Document::new();
    let element: HTMLAnchorElement = document.create_element("a").try_into().unwrap();
}
