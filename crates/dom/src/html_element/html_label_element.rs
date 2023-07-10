use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, HTMLFormElement, InnerHtml,
};

/// Gives access to properties specific to `<label>` elements. It inherits methods and properties from the base [`HTMLElement`] struct.
///
///  MDN Reference: [`HTMLLabelElement`](https://developer.mozilla.org/docs/Web/API/HTMLLabelElement)
pub struct HTMLLabelElement {
    value: HTMLElement,
}

impl HTMLLabelElement {
    pub fn control(&self) -> Option<HTMLElement> {
        todo!()
    }
    pub fn form(&self) -> Option<HTMLFormElement> {
        todo!()
    }
    pub fn html_for(&self) -> &str {
        todo!()
    }
    pub fn set_html_for(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLLabelElement {
    fn cast(&self) -> &HTMLElement {
        &self.value
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.value
    }
}
impl AsElement for HTMLLabelElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for HTMLLabelElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLLabelElement {}
impl AsChildNode for HTMLLabelElement {}
impl AsNode for HTMLLabelElement {
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
        HTMLLabelElement {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLLabelElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLLabelElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLLabelElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::Label) {
            Ok(HTMLLabelElement { value })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLLabelElement"
            )))
        }
    }
}
