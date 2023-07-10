use crate::AsEventTarget;
#[warn(unused)]
use crate::{Document, HTMLAnchorElement};

#[test]
fn test() {
    let document = Document::new();
    let element: HTMLAnchorElement = document.create_element("a").try_into().unwrap();
}
