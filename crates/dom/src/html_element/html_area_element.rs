use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, DOMTokenList, HTMLElement, HTMLHyperlinkElementUtils, InnerHtml,
};

/// The [`HTMLAreaElement`] struct provides special methods (beyond the regular [`HTMLElement`] struct) for manipulating `<area>` elements.
///
/// MDN Reference: [`HTMLAreaElement`](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAreaElement).
pub struct HTMLAreaElement {
    html_element: HTMLElement,
}

// Properties
impl HTMLAreaElement {
    pub fn alt(&self) -> &str {
        todo!()
    }
    pub fn set_alt(&mut self, value: &str) {
        todo!()
    }
    pub fn coords(&self) -> &str {
        todo!()
    }
    pub fn set_coords(&mut self, value: &str) {
        todo!()
    }
    pub fn download(&self) -> &str {
        todo!()
    }
    pub fn set_download(&mut self, value: &str) {
        todo!()
    }
    #[deprecated]
    pub fn no_href(&self) -> bool {
        todo!()
    }
    #[deprecated]
    pub fn set_no_href(&mut self, value: bool) {
        todo!()
    }
    pub fn ping(&self) -> &str {
        todo!()
    }
    pub fn set_ping(&mut self, value: &str) {
        todo!()
    }
    pub fn referrer_policy(&self) -> &str {
        todo!()
    }
    pub fn set_referrer_policy(&mut self, value: &str) {
        todo!()
    }
    pub fn rel(&self) -> &str {
        todo!()
    }
    pub fn set_rel(&mut self, value: &str) {
        todo!()
    }
    pub fn rel_list(&self) -> DOMTokenList {
        todo!()
    }
    pub fn shape(&self) -> &str {
        todo!()
    }
    pub fn set_shape(&mut self, value: &str) {
        todo!()
    }
    pub fn target(&self) -> &str {
        todo!()
    }
    pub fn set_target(&mut self, value: &str) {
        todo!()
    }
}

impl HTMLHyperlinkElementUtils for HTMLAreaElement {}
impl AsHTMLElement for HTMLAreaElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}

impl AsElement for HTMLAreaElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLAreaElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLAreaElement {}
impl AsChildNode for HTMLAreaElement {}
impl AsNode for HTMLAreaElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLAreaElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLAreaElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLAreaElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLAreaElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().inner_ref.borrow().tag, Tag::Area) {
            Ok(HTMLAreaElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an HTMLAreaElement"
            )))
        }
    }
}
