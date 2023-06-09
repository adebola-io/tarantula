use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, Document, HTMLElement, InnerHtml,
};

#[deprecated]
pub struct HTMLFrameElement {
    html_element: HTMLElement,
}

pub struct WindowProxy;

#[deprecated]
impl HTMLFrameElement {
    pub fn content_document(&self) -> Option<Document> {
        todo!()
    }
    pub fn content_window(&self) -> Option<WindowProxy> {
        todo!()
    }
    pub fn frame_border(&self) -> &str {
        todo!()
    }
    pub fn set_frame_border(&mut self, value: &str) {
        todo!()
    }
    pub fn long_desc(&self) -> &str {
        todo!()
    }
    pub fn set_long_desc(&mut self, value: &str) {
        todo!()
    }
    pub fn margin_height(&self) -> &str {
        todo!()
    }
    pub fn set_margin_height(&mut self, value: &str) {
        todo!()
    }
    pub fn name(&self) -> &str {
        todo!()
    }
    pub fn set_name(&mut self, value: &str) {
        todo!()
    }
    pub fn no_resize(&self) -> bool {
        todo!()
    }
    pub fn set_no_resize(&mut self, value: bool) {
        todo!()
    }
    pub fn scrolling(&self) -> &str {
        todo!()
    }
    pub fn set_scrolling(&mut self, value: &str) {
        todo!()
    }
    pub fn src(&self) -> &str {
        todo!()
    }
    pub fn set_src(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLFrameElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLFrameElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLFrameElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLFrameElement {}
impl AsChildNode for HTMLFrameElement {}
impl AsNode for HTMLFrameElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLFrameElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLFrameElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLFrameElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLFrameElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLFrameElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLFrameElement"
            )))
        }
    }
}
