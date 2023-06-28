use crate::{
    AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode, DOMException,
    HTMLElement, InnerHtml, Tag,
};

/// The [`HTMLAnchorElement`] struct represents hyperlink elements and provides special methods (beyond those of the regular [`HTMLElement`]) for manipulating the layout and presentation of such elements.
///
/// This struct corresponds to `<a>` elements; not to be confused with `<link>`, which is represented by [`HTMLLinkElement`].
///
/// MDN Reference: [`HTMLAnchorElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement)
pub struct HTMLAnchorElement {
    value: HTMLElement,
}

impl AsHTMLElement for HTMLAnchorElement {}
impl AsElement for HTMLAnchorElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for HTMLAnchorElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLAnchorElement {}
impl AsChildNode for HTMLAnchorElement {}
impl AsNode for HTMLAnchorElement {
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
        HTMLAnchorElement {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLAnchorElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLAnchorElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLAnchorElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::A) {
            Ok(HTMLAnchorElement { value })
        } else {
            Err(DOMException::TypeError(
                "Cannot convert element to an HTMLAnchorElement",
            ))
        }
    }
}
