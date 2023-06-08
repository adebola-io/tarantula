use crate::{AsEventTarget, AsNode, Element, Node, WeakDocumentRef};

/// A DOM element's attribute.
/// In most DOM methods, you will probably directly retrieve the attribute as a string (e.g., [`Element::get_attribute`], but certain functions (e.g., [`Element::get_attribute_node`] or means of iterating give Attr types.
///
/// [Element::get_attribute]: crate::Element::get_attribute
/// [Element::get_attribute_node]: crate::Element::get_attribute_node
pub struct Attr {
    node: Node,
    pub(crate) name: String,
    owner_document: Option<WeakDocumentRef>,
    // owner_element: Option< Element>,
    prefix: Option<String>,
    pub(crate) value: String,
    #[deprecated]
    specified: bool,
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

    fn clone_node(&self, deep: bool) -> Self {
        todo!()
    }
}
