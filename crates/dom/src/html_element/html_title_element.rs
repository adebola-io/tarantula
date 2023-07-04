use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};
pub struct HTMLTitleElement {
    value: HTMLElement,
}

impl AsHTMLElement for HTMLTitleElement {
    fn cast(&self) -> &HTMLElement {
        &self.value
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.value
    }
}
impl AsElement for HTMLTitleElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for HTMLTitleElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLTitleElement {}
impl AsChildNode for HTMLTitleElement {}
impl AsNode for HTMLTitleElement {
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
        HTMLTitleElement {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLTitleElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLTitleElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLTitleElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::A) {
            Ok(HTMLTitleElement { value })
        } else {
            Err(DOMException::TypeError(
                "Cannot convert element to an HTMLTitleElement",
            ))
        }
    }
}