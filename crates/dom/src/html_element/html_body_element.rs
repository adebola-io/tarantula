use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};

/// The [`HTMLBodyElement`] struct provides special methods (beyond the regular [`HTMLElement`] struct) for manipulating `<area>` elements.
///
/// MDN Reference: [`HTMLBodyElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBodyElement).
pub struct HTMLBodyElement {
    html_element: HTMLElement,
}

#[deprecated]
impl HTMLBodyElement {
    pub fn a_link(&self) -> &str {
        todo!()
    }
    pub fn set_a_link(&mut self, value: &str) {
        todo!()
    }
    pub fn background(&self) -> &str {
        todo!()
    }
    pub fn set_background(&mut self, value: &str) {
        todo!()
    }
    pub fn bg_color(&self) -> &str {
        todo!()
    }
    pub fn set_bg_color(&mut self, value: &str) {
        todo!()
    }
    pub fn link(&self) -> &str {
        todo!()
    }
    pub fn set_link(&mut self, value: &str) {
        todo!()
    }
    pub fn text(&self) -> &str {
        todo!()
    }
    pub fn set_text(&mut self, value: &str) {
        todo!()
    }
    pub fn v_link(&self) -> &str {
        todo!()
    }
    pub fn set_v_link(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLBodyElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLBodyElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLBodyElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLBodyElement {}
impl AsChildNode for HTMLBodyElement {}
impl AsNode for HTMLBodyElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLBodyElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLBodyElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLBodyElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLBodyElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().inner_ref.borrow().tag, Tag::A) {
            Ok(HTMLBodyElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLBodyElement"
            )))
        }
    }
}
