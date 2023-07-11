use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};
pub struct HTMLDetailsElement {
    html_element: HTMLElement,
}

impl HTMLDetailsElement {
    pub fn open(&self) -> bool {
        todo!()
    }
    pub fn set_open(&mut self, value: bool) {
        todo!()
    }
}

impl AsHTMLElement for HTMLDetailsElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLDetailsElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLDetailsElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLDetailsElement {}
impl AsChildNode for HTMLDetailsElement {}
impl AsNode for HTMLDetailsElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLDetailsElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLDetailsElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLDetailsElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLDetailsElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLDetailsElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLDetailsElement"
            )))
        }
    }
}
