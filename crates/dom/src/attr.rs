use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{
    document::WeakDocumentRef, element::ElementBase, node::NodeType, AsEventTarget, AsNode,
    Element, Node,
};

/// A DOM element's attribute.
/// In most DOM methods, you will probably directly retrieve the attribute as a string (e.g., [`Element::get_attribute`], but certain functions (e.g., [`Element::get_attribute_node`] or means of iterating give Attr types.
///
/// [Element::get_attribute]: crate::Element::get_attribute
/// [Element::get_attribute_node]: crate::Element::get_attribute_node
#[derive(Debug)]
pub struct Attr {
    node: Node,
    pub(crate) __name: String,
    namespace_uri: Option<String>,
    owner_element_ref: Option<Weak<RefCell<ElementBase>>>,
    pub(crate) __value: Option<String>,
    #[deprecated]
    specified: bool,
}

impl Attr {
    #[allow(deprecated)]
    /// Create a new attribute in a document.
    pub(crate) fn in_document(local_name: &str, weak_ref: WeakDocumentRef) -> Attr {
        Self {
            node: Node::in_document(NodeType::AttributeNode, weak_ref),
            __name: local_name.to_owned(),
            namespace_uri: None,
            owner_element_ref: None,
            __value: None,
            specified: false,
        }
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
        &self.__name
    }
    #[inline(always)]
    pub fn namespace_uri(&self) -> Option<&str> {
        self.namespace_uri.as_ref().map(|x| x.as_str())
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
    pub fn prefix(&self) -> Option<String> {
        if self
            .owner_element()
            .is_some_and(|element| element.is_html())
        {
            None
        } else {
            self.__name
                .split_once(':')
                .map(|tuple| tuple.0.to_lowercase())
        }
    }
    #[inline(always)]
    pub fn value(&self) -> &str {
        match &self.__value {
            Some(value) => value,
            None => "",
        }
    }
    pub fn set_value(&mut self, value: String) {
        self.__value = Some(value);
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
            __name: self.__name.to_owned(),
            namespace_uri: self.namespace_uri.clone(),
            owner_element_ref: None,
            __value: self.__value.to_owned(),
            specified: self.specified,
        }
    }
}
