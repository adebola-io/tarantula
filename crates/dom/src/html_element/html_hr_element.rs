use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};
pub struct HTMLHRElement {
    html_element: HTMLElement,
}

#[deprecated]
impl HTMLHRElement {
    pub fn align(&self) -> &str {
        todo!()
    }
    pub fn set_align(&mut self, value: &str) {
        todo!()
    }
    pub fn color(&self) -> &str {
        todo!()
    }
    pub fn set_color(&mut self, value: &str) {
        todo!()
    }
    pub fn shade(&self) -> bool {
        todo!()
    }
    pub fn set_shade(&mut self, value: bool) {
        todo!()
    }
    pub fn size(&self) -> &str {
        todo!()
    }
    pub fn set_size(&mut self, value: &str) {
        todo!()
    }
    pub fn width(&self) -> &str {
        todo!()
    }
    pub fn set_width(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLHRElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLHRElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLHRElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLHRElement {}
impl AsChildNode for HTMLHRElement {}
impl AsNode for HTMLHRElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLHRElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLHRElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLHRElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLHRElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLHRElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLHrElement"
            )))
        }
    }
}
