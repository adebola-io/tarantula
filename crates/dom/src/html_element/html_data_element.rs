use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};
pub struct HTMLDataElement {
    html_element: HTMLElement,
}

impl HTMLDataElement {
    pub fn value(&self) -> &str {
        todo!()
    }
    pub fn set_value(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLDataElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLDataElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLDataElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLDataElement {}
impl AsChildNode for HTMLDataElement {}
impl AsNode for HTMLDataElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLDataElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLDataElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLDataElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLDataElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLDataElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLDataElement"
            )))
        }
    }
}
