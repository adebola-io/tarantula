use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};
pub struct HTMLDialogElement {
    html_element: HTMLElement,
}

// Properties
impl HTMLDialogElement {
    pub fn open(&self) -> bool {
        todo!()
    }
    pub fn set_open(&mut self, value: bool) {
        todo!()
    }
    pub fn return_value(&self) -> &str {
        todo!()
    }
    pub fn set_return_value(&mut self, value: &str) {
        todo!()
    }
}

// Methods
impl HTMLDialogElement {
    pub fn close(&mut self, return_value: &str) {
        todo!()
    }
    pub fn show(&mut self) {
        todo!()
    }
    pub fn show_modal(&mut self) {
        todo!()
    }
}

impl AsHTMLElement for HTMLDialogElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLDialogElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLDialogElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLDialogElement {}
impl AsChildNode for HTMLDialogElement {}
impl AsNode for HTMLDialogElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLDialogElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLDialogElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLDialogElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLDialogElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLDialogElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLDialogElement"
            )))
        }
    }
}
