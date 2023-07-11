use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};

/// The [`HTMLDivElement`] struct provides special methods (beyond the regular [`HTMLElement`] struct) for manipulating `<div>` elements.
///
/// MDN Reference: [`HTMLDivElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement)
pub struct HTMLDivElement {
    html_element: HTMLElement,
}

impl HTMLDivElement {
    /// Returns a string representing how the elements content are aligned with respect to the surrounding context. Possible values are "left", "right", "justify", and "center".
    #[deprecated]
    pub fn align(&self) -> &str {
        todo!()
    }
    /// Sets a string representing how the elements content are aligned with respect to the surrounding context. Possible values are "left", "right", "justify", and "center".
    #[deprecated]
    pub fn set_align(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLDivElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLDivElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
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
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLDivElement {
            html_element: self.html_element.clone_node(deep),
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
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLDivElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::Div) {
            Ok(HTMLDivElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLDivElement"
            )))
        }
    }
}
