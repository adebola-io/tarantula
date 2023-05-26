use super::ElementRef;

#[derive(Clone, PartialEq, Debug)]
pub enum HtmlNode<'a> {
    Element(ElementRef<'a>),
    Text(Text<'a>),
    Comment(Comment<'a>),
}

impl<'a> HtmlNode<'a> {
    pub(crate) fn is_element(&self) -> bool {
        matches!(self, HtmlNode::Element(_))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Text<'a> {
    value: &'a str,
    parent: &'a ElementRef<'a>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Comment<'a> {
    value: &'a str,
    parent: &'a ElementRef<'a>,
}
