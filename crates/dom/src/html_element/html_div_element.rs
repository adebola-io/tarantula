use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};

/// The [`HTMLDivElement`] struct provides special methods (beyond the regular [`HTMLElement`] struct) for manipulating `<div>` elements.
///
/// MDN Reference: [`HTMLDivElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement)
pub struct HTMLDivElement {
    value: HTMLElement,
}

impl HTMLDivElement {
    /// Returns a string representing how the elements content are aligned with respect to the surrounding context. Possible values are "left", "right", "justify", and "center".
    #[deprecated]
    pub fn align(&self) -> &str {
        todo!()
    }
    /// Sets a string representing how the elements content are aligned with respect to the surrounding context. Possible values are "left", "right", "justify", and "center".
    #[deprecated]
    pub fn set_align(&self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLDivElement {}
impl AsElement for HTMLDivElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for HTMLDivElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLDivElement {}
impl AsChildNode for HTMLDivElement {}
impl AsNode for HTMLDivElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.value)
    }

    fn node_name(&self) -> String {
        self.value.tag_name()
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLDivElement {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLDivElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLDivElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLDivElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::Div) {
            Ok(HTMLDivElement { value })
        } else {
            Err(DOMException::TypeError(
                "Cannot convert element to an HTMLDivElement",
            ))
        }
    }
}
