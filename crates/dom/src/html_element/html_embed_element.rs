use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, Document, HTMLElement, InnerHtml,
};
pub struct HTMLEmbedElement {
    html_element: HTMLElement,
}
impl HTMLEmbedElement {
    #[deprecated]
    pub fn align(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_align(&self, value: &str) {
        todo!()
    }
    pub fn height(&self) -> &str {
        todo!()
    }
    pub fn set_height(&mut self, value: &str) {
        todo!()
    }
    #[deprecated]
    pub fn name(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_name(&mut self, value: &str) {
        todo!()
    }
    pub fn src(&self) -> &str {
        todo!()
    }
    pub fn set_src(&mut self, value: &str) {
        todo!()
    }
    pub fn r#type(&self) -> &str {
        todo!()
    }
    pub fn set_type(&mut self, value: &str) {
        todo!()
    }
    pub fn width(&self) -> &str {
        todo!()
    }
    pub fn set_width(&mut self, value: &str) {
        todo!()
    }
}

impl HTMLEmbedElement {
    pub fn get_svg_document(&self) -> Option<Document> {
        todo!()
    }
}

impl AsHTMLElement for HTMLEmbedElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLEmbedElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLEmbedElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLEmbedElement {}
impl AsChildNode for HTMLEmbedElement {}
impl AsNode for HTMLEmbedElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLEmbedElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLEmbedElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLEmbedElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLEmbedElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLEmbedElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLEmbedElement"
            )))
        }
    }
}
