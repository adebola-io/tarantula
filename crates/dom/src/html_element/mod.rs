mod html_anchor_element;
mod html_div_element;
pub use html_anchor_element::*;
pub use html_div_element::*;

use std::{cell::RefCell, rc::Rc};

use crate::{
    document::WeakDocumentRef, AsChildNode, AsElement, AsEventTarget, AsNode, AsParentNode,
    Element, InnerHtml, Node,
};

#[derive(Debug)]
pub(crate) struct HTMLElementBase {
    pub(crate) element: Element,
}

/// Any HTML element. Some elements directly implement this interface, while others implement it via an interface that inherits it.
///
/// MDN Reference: [`HTMLElement`](https://developer.mozilla.org/docs/Web/API/HTMLElement)
#[derive(Debug)]
pub struct HTMLElement {
    pub(crate) inner: Rc<RefCell<HTMLElementBase>>,
}

impl Drop for HTMLElement {
    fn drop(&mut self) {
        // Disconnect node from document.
        if self.parent_node().is_none() && Rc::strong_count(&self.inner) == 2 {
            let mut document = self.owner_document().unwrap();

            document.drop_node(AsNode::cast(self).get_base_ptr());
            assert!(document
                .lookup_node(AsNode::cast(self).get_base_ptr())
                .is_none())
        }
    }
}

impl<T: AsNode> PartialEq<T> for HTMLElement {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}

impl HTMLElement {
    fn inner(&self) -> &mut HTMLElementBase {
        unsafe { &mut *self.inner.as_ptr() }
    }

    pub(crate) fn in_document(tagname: &str, weak_ref: WeakDocumentRef) -> Self {
        HTMLElement {
            inner: Rc::new(RefCell::new(HTMLElementBase {
                element: Element::in_document(tagname, true, weak_ref),
            })),
        }
    }

    pub fn clone_ref(&self) -> HTMLElement {
        HTMLElement {
            inner: self.inner.clone(),
        }
    }
}

impl AsElement for HTMLElement {
    fn cast(&self) -> &Element {
        &self.inner().element
    }

    fn cast_mut(&mut self) -> &mut Element {
        &mut self.inner().element
    }
}

impl AsParentNode for HTMLElement {}
impl AsChildNode for HTMLElement {}

impl InnerHtml for HTMLElement {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), crate::DOMException> {
        todo!()
    }
}

impl AsNode for HTMLElement {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(AsElement::cast(self))
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(AsElement::cast_mut(self))
    }

    fn clone_node(&self, deep: bool) -> Self {
        HTMLElement {
            inner: Rc::new(RefCell::new(HTMLElementBase {
                element: self.inner().element.clone_node(deep),
            })),
        }
    }

    fn node_name(&self) -> String {
        self.tag_name()
    }
}

impl AsEventTarget for HTMLElement {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(AsNode::cast(self))
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(AsNode::cast_mut(self))
    }
}

impl AsHTMLElement for HTMLElement {}

pub trait AsHTMLElement: AsElement {}

// #[cfg(test)]
// mod tests {
//     use crate::{Document, AsChildNode};

//     #[test]
//     fn stuff () {
//         let document = Document::new();

//         let element = document.create_element("div");
//         element.remove();
//     }
// }
