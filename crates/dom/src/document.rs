use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{Attr, Element, HTMLElement, Node, Tag};

pub(crate) struct DocumentBase {
    pub url: String,
    node_to_element_map: HashMap<Node, Element>,
}

impl std::fmt::Debug for DocumentBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocumentBase")
            .field("url", &self.url)
            .finish()
    }
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
pub(crate) struct WeakDocumentRef {
    pub(crate) inner: Weak<RefCell<DocumentBase>>,
}

impl PartialEq for WeakDocumentRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(DocumentBase {
                url: String::new(),
                node_to_element_map: HashMap::new(),
            })),
        }
    }
    /// Create an HTML attribute with the specified `local_name`.
    pub fn create_attribute(&self, local_name: &str) -> Attr {
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&self.inner),
        };
        Attr::in_document(local_name, weak_ref)
    }
    /// Create an HTML element with the specified `tagname`.
    pub fn create_element(&self, tagname: &str) -> HTMLElement {
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&self.inner),
        };
        HTMLElement::in_document(tagname, weak_ref)
    }
}
