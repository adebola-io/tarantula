#[warn(unused)]
use crate::{traits::*, Document, HTMLAnchorElement};

#[test]
fn test() {
    let time = std::time::Instant::now();
    let document = Document::new();
    let mut element: HTMLAnchorElement = document.create_element("a").try_into().unwrap();
    let mut child = document.create_element("span");

    element.append(&mut child);
    println!("{}", element.first_child().unwrap().node_name());

    drop(element);

    println!("{}", child.parent_node().is_some());
    println!("{:?}", time.elapsed());
}
