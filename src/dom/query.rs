use super::{ElementRef, HtmlTag};

pub trait Query<'a> {
    /// Traverse tree and find the first element that matches a selector, if it exists.
    fn query_selector(&'a self, selector: &str) -> Option<&ElementRef>;

    /// Traverse tree and find all the elements that matches a selector.
    fn query_selector_all(&'a self, selector: &str) -> Vec<&ElementRef>;

    /// Traverse element or tree and return all elements that have a particular class.
    fn get_elements_by_class_name(&'a self, class_name: &str) -> Vec<&ElementRef>;

    /// Return the first element in the tree/subtree that has a specified id, if it exists.
    fn get_element_by_id(&'a self, id: &str) -> Option<&ElementRef>;

    /// Return all elements in the tree/subtree that have a specified tag name.
    fn get_elements_by_tag_name(&'a self, tag: &HtmlTag) -> Vec<&ElementRef>;

    /// Traverses the node/tree and returns the first child that matches a predicate.
    fn find(&'a self, predicate: fn(element: &ElementRef) -> bool) -> Option<&ElementRef>;
}
