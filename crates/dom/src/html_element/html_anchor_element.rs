use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, DOMTokenList, HTMLElement, HTMLHyperlinkElementUtils, InnerHtml,
};
pub struct HTMLAnchorElement {
    value: HTMLElement,
}

// Properties
impl HTMLAnchorElement {
    #[deprecated]
    pub fn charset(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_charset(&mut self, value: &str) {
        todo!()
    }
    #[deprecated]
    pub fn coords(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_coords(&mut self, value: &str) {
        todo!()
    }
    pub fn download(&self) -> &str {
        todo!()
    }
    pub fn set_download(&mut self, value: &str) {
        todo!()
    }
    pub fn hreflang(&self) -> &str {
        todo!()
    }
    pub fn set_hreflang(&mut self, value: &str) {
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
    pub fn ping(&self) -> &str {
        todo!()
    }
    pub fn set_ping(&mut self, value: &str) {
        todo!()
    }
    pub fn referrer_policy(&self) -> &str {
        todo!()
    }
    pub fn set_referrer_policy(&mut self, value: &str) {
        todo!()
    }
    pub fn rel(&self) -> &str {
        todo!()
    }
    pub fn set_rel(&mut self, value: &str) {
        todo!()
    }
    pub fn rel_list(&self) -> DOMTokenList {
        todo!()
    }
    pub fn rev(&self) -> &str {
        todo!()
    }
    pub fn set_rev(&mut self, value: &str) {
        todo!()
    }
    #[deprecated]
    pub fn shape(&self) -> &str {
        todo!()
    }
    #[deprecated]
    pub fn set_shape(&mut self, value: &str) {
        todo!()
    }
    pub fn target(&self) -> &str {
        todo!()
    }
    pub fn set_target(&mut self, value: &str) {
        todo!()
    }
    pub fn text(&self) -> &str {
        todo!()
    }
    pub fn set_text(&mut self, value: &str) {
        todo!()
    }
    pub fn r#type(&self) -> &str {
        todo!()
    }
    pub fn set_type(&mut self, value: &str) {
        todo!()
    }
}

impl AsHTMLElement for HTMLAnchorElement {
    fn cast(&self) -> &HTMLElement {
        &self.value
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.value
    }
}
impl HTMLHyperlinkElementUtils for HTMLAnchorElement {}

impl AsElement for HTMLAnchorElement {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for HTMLAnchorElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for HTMLAnchorElement {}
impl AsChildNode for HTMLAnchorElement {}
impl AsNode for HTMLAnchorElement {
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
        HTMLAnchorElement {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for HTMLAnchorElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for HTMLAnchorElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for HTMLAnchorElement {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
        let tag = value.tag();
        if !matches!(tag, Tag::A) {
            return Err(DOMException::TypeError(format!(
                "Cannot convert element with tag {tag} to an HTMLAnchorElement"
            )));
        }
        return Ok(HTMLAnchorElement { value });
    }
}
