use std::ops::Index;

use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, DOMTokenList, Element, HTMLElement, InnerHtml,
};
pub struct HTMLFormControlsCollection;

pub struct HTMLFormElement {
    html_element: HTMLElement,
}

// "Properties"
impl HTMLFormElement {
    pub fn accept_charset(&self) -> &str {
        todo!()
    }
    pub fn set_accept_charset(&mut self, value: &str) {
        todo!()
    }
    pub fn action(&self) -> &str {
        todo!()
    }
    pub fn set_action(&mut self, value: &str) {
        todo!()
    }
    pub fn auto_complete(&self) -> &str {
        todo!()
    }
    pub fn set_auto_complete(&mut self, value: &str) {
        todo!()
    }
    pub fn elements(&self) -> HTMLFormControlsCollection {
        todo!()
    }
    pub fn encoding(&self) -> &str {
        todo!()
    }
    pub fn set_encoding(&mut self, value: &str) {
        todo!()
    }
    pub fn enctype(&self) -> &str {
        todo!()
    }
    pub fn set_enctype(&mut self, value: &str) {
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    pub fn method(&self) -> &str {
        todo!()
    }
    pub fn set_method(&mut self, value: &str) {
        todo!()
    }
    pub fn name(&self) -> &str {
        todo!()
    }
    pub fn set_name(&mut self, value: &str) {
        todo!()
    }
    pub fn no_validate(&self) -> bool {
        todo!()
    }
    pub fn set_no_validate(&mut self, value: bool) {
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
    pub fn target(&self) -> &str {
        todo!()
    }
    pub fn set_target(&mut self, value: &str) {
        todo!()
    }
}

// "Methods"
impl HTMLFormElement {
    pub fn check_validity(&self) -> bool {
        todo!()
    }
    pub fn report_validity(&self) -> bool {
        todo!()
    }
    pub fn request_submit(&mut self, submitter: Option<HTMLElement>) {
        todo!()
    }
    pub fn reset(&mut self) {
        todo!()
    }
    pub fn submit(&mut self) {
        todo!()
    }
}

impl Index<usize> for HTMLFormElement {
    type Output = Element;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl Index<&str> for HTMLFormElement {
    type Output = Element;

    fn index(&self, index: &str) -> &Self::Output {
        todo!()
    }
}

impl AsHTMLElement for HTMLFormElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLFormElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLFormElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLFormElement {}
impl AsChildNode for HTMLFormElement {}
impl AsNode for HTMLFormElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLFormElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLFormElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLFormElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLFormElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().inner_ref.borrow().tag, Tag::Form) {
            Ok(HTMLFormElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLFormElement"
            )))
        }
    }
}
