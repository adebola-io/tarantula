use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLCollection, HTMLElement, HTMLFormElement, InnerHtml, ValidityState,
};
pub struct HTMLFieldsetElement {
    html_element: HTMLElement,
}

impl HTMLFieldsetElement {
    pub fn disabled(&self) -> bool {
        todo!()
    }
    pub fn set_disabled(&mut self, value: bool) {
        todo!()
    }
    pub fn elements(&self) -> HTMLCollection {
        todo!()
    }
    pub fn form(&self) -> Option<HTMLFormElement> {
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
    pub fn validation_message(&self) -> &str {
        todo!()
    }
    pub fn validity(&self) -> ValidityState {
        todo!()
    }
    pub fn will_validate(&self) -> bool {
        todo!()
    }
}

impl HTMLFieldsetElement {
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

impl AsHTMLElement for HTMLFieldsetElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLFieldsetElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLFieldsetElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLFieldsetElement {}
impl AsChildNode for HTMLFieldsetElement {}
impl AsNode for HTMLFieldsetElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLFieldsetElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLFieldsetElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLFieldsetElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLFieldsetElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLFieldsetElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLFieldsetElement"
            )))
        }
    }
}
