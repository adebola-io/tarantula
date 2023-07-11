use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};

#[deprecated]
pub struct HTMLFontElement {
    html_element: HTMLElement,
}

impl HTMLFontElement {
    #[deprecated]
    pub fn color(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_color(&mut self, value: &str) {
        todo!()
    }
    #[deprecated]
    pub fn face(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_face(&mut self, value: &str) {
        todo!()
    }
    #[deprecated]
    pub fn size(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_size(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLFontElement {
    fn cast(&self) -> &HTMLElement {
        &self.html_element
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.html_element
    }
}
impl AsElement for HTMLFontElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.html_element)
    }
}
impl InnerHtml for HTMLFontElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLFontElement {}
impl AsChildNode for HTMLFontElement {}
impl AsNode for HTMLFontElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.html_element)
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLFontElement {
            html_element: self.html_element.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLFontElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLFontElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.html_element)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.html_element)
    }
}

impl TryFrom<HTMLElement> for HTMLFontElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if matches!(value.element().base.borrow().tag, Tag::A) {
            Ok(HTMLFontElement {
                html_element: value,
            })
        } else {
            Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an  HTMLFontElement"
            )))
        }
    }
}
