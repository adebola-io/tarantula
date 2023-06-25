use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{Attr, HTMLElement, Node};

#[derive(Debug)]
pub struct DocumentBase {
    pub url: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) inner: Rc<RefCell<DocumentBase>>,
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

#[derive(Debug, Clone)]
pub struct WeakDocumentRef {
    pub(crate) inner: Weak<RefCell<DocumentBase>>,
}

impl PartialEq for WeakDocumentRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}

impl Document {
    /// Create an HTML attribute with the specified `local_name`.
    pub fn create_attribute(&self, local_name: &str) -> Attr {
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&self.inner),
        };
        Attr::in_document(local_name, weak_ref)
    }
    /// Create an HTML element with the specified `tagname`.
    pub fn create_element(&self, tagname: &str) -> HTMLElement {
        todo!()
    }
}
