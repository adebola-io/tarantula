#[derive(Debug)]
pub enum DOMException {
    HierarchyRequestError(&'static str),
}
