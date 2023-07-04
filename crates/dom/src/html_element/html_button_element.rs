use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, HTMLFormElement, InnerHtml, NodeListOf,
};

pub struct ValidityState {}

use super::HTMLLabelElement;

pub struct HTMLButtonElement {
    value: HTMLElement,
}

// "Properties".
impl HTMLButtonElement {
    pub fn disabled(&self) -> bool {
        todo!()
    }
    pub fn set_disabled(&mut self, value: bool) {
        todo!()
    }
    pub fn form(&self) -> Option<HTMLFormElement> {
        todo!()
    }
    pub fn form_action(&self) -> &str {
        todo!()
    }
    pub fn set_form_action(&mut self, value: &str) {
        todo!()
    }
    pub fn form_enctype(&self) -> &str {
        todo!()
    }
    pub fn set_form_enctype(&mut self, value: &str) {
        todo!()
    }
    pub fn form_method(&self) -> &str {
        todo!()
    }
    pub fn set_form_method(&mut self, value: &str) {
        todo!()
    }
    pub fn form_no_validate(&self) -> bool {
        todo!()
    }
    pub fn set_form_no_validate(&mut self, value: bool) {
        todo!()
    }
    pub fn form_target(&self) -> &str {
        todo!()
    }
    pub fn set_form_target(&mut self, value: &str) {
        todo!()
    }
    pub fn labels(&self) -> NodeListOf<HTMLLabelElement> {
        todo!()
    }
    pub fn name(&self) -> &str {
        todo!()
    }
    pub fn set_name(&mut self, value: &str) {
        todo!()
    }
    pub fn r#type(&self) -> &str {
        todo!()
    }
    pub fn set_type(&mut self, value: &str) -> &str {
        todo!()
    }
    pub fn validation_message(&self) -> &str {
        todo!()
    }
    pub fn validity(&self) -> ValidityState {
        todo!()
    }
    pub fn value(&self) -> &str {
        todo!()
    }
    pub fn set_value(&mut self, value: &str) {
        todo!()
    }
    pub fn will_validate(&self) -> bool {
        todo!()
    }
}

// Methods.
impl HTMLButtonElement {
    pub fn check_validity(&self) -> bool {
        todo!()
    }
    pub fn report_validity(&self) -> bool {
        todo!()
    }
    pub fn set_custom_validity(&mut self, error: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLButtonElement {
    fn cast(&self) -> &HTMLElement {
        &self.value
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.value
    }
}
impl AsElement for HTMLButtonElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for HTMLButtonElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLButtonElement {}
impl AsChildNode for HTMLButtonElement {}
impl AsNode for HTMLButtonElement {
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
        HTMLButtonElement {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLButtonElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLButtonElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLButtonElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::Button) {
            Ok(HTMLButtonElement { value })
        } else {
            Err(DOMException::TypeError(
                "Cannot convert element to an HTMLButtonElement",
            ))
        }
    }
}
