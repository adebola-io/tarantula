use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{AsEventTarget, AsNode, DocumentBase, Element, ElementBase, Node, WeakDocumentRef};

/// A DOM element's attribute.
/// In most DOM methods, you will probably directly retrieve the attribute as a string (e.g., [`Element::get_attribute`], but certain functions (e.g., [`Element::get_attribute_node`] or means of iterating give Attr types.
///
/// [Element::get_attribute]: crate::Element::get_attribute
/// [Element::get_attribute_node]: crate::Element::get_attribute_node
pub struct Attr {
    node: Node,
    pub(crate) name: String,
    namespace_uri: Option<String>,
    owner_document: WeakDocumentRef,
    owner_element_ref: Option<Weak<RefCell<ElementBase>>>,
    prefix: Option<String>,
    pub(crate) value: String,
    #[deprecated]
    specified: bool,
}

impl Attr {
    /// Create a new attribute in a document.
    pub(crate) fn in_document(local_name: &str, weak_ref: WeakDocumentRef) -> Attr {
        todo!()
    }
    /// Set the owner element of the attribute.
    pub(crate) fn set_owner_element(&mut self, element: Weak<RefCell<ElementBase>>) {
        self.owner_element_ref = Some(element)
    }
}

impl Attr {
    #[inline(always)]
    pub fn local_name(&self) -> &str {
        todo!()
    }
    #[inline(always)]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[inline(always)]
    pub fn namespace_uri(&self) -> Option<&str> {
        self.namespace_uri.as_ref().map(|x| x.as_str())
    }
    pub fn owner_document(&self) -> &DocumentBase {
        unsafe { &*(self.owner_document.inner.upgrade().unwrap().as_ptr()) }
    }
    pub fn owner_document_mut(&self) -> &mut DocumentBase {
        unsafe { &mut *(self.owner_document.inner.upgrade().unwrap().as_ptr()) }
    }
    /// Returns the element that owns the attribute.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/Attr/ownerElement)
    pub fn owner_element(&self) -> Option<Element> {
        if let Some(weak_ref) = &self.owner_element_ref {
            weak_ref.upgrade().map(|inner_ref| Element { inner_ref })
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }
    #[inline(always)]
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }
    #[allow(deprecated)]
    #[deprecated]
    pub fn specified(&self) -> bool {
        self.specified
    }
}

impl AsEventTarget for Attr {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.node)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.node)
    }
}

impl AsNode for Attr {
    fn cast(&self) -> &crate::Node {
        &self.node
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        &mut self.node
    }

    #[allow(deprecated)]
    fn clone_node(&self, deep: bool) -> Self {
        Self {
            node: self.node.clone_node(deep),
            name: self.name.to_owned(),
            namespace_uri: self.namespace_uri.clone(),
            owner_document: self.owner_document.clone(),
            owner_element_ref: None,
            prefix: self.prefix.clone(),
            value: self.value.to_owned(),
            specified: self.specified,
        }
    }
}
