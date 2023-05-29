use std::{cell::RefCell, rc::Weak};

use crate::Node;

#[derive(Debug)]
pub struct Document {
    pub url: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct WeakDocumentRef {
    inner: Weak<RefCell<Document>>,
}

impl PartialEq for WeakDocumentRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}
