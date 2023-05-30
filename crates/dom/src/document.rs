use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::Node;

#[derive(Debug)]
pub struct Document {
    pub url: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct DocumentRef {
    pub(crate) inner: Rc<RefCell<Document>>,
}

impl PartialEq for DocumentRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

#[derive(Debug, Clone)]
pub struct WeakDocumentRef {
    pub(crate) inner: Weak<RefCell<Document>>,
}

impl PartialEq for WeakDocumentRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}
