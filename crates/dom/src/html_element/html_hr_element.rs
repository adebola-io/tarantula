use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};
pub struct HTMLHRElement {
    value: HTMLElement,
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
        &self.value
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.value
    }
}
impl AsElement for HTMLHRElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
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
        AsNode::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.value)
    }

    fn node_name(&self) -> String {
        self.value.tag_name()
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLHRElement {
            value: self.value.clone_node(deep),
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
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLHRElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::A) {
            Ok(HTMLHRElement { value })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLHrElement"
            )))
        }
    }
}
