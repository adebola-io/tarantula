mod html_anchor_element;
mod html_button_element;
mod html_div_element;
mod html_form_element;
mod html_label_element;

pub use html_anchor_element::HTMLAnchorElement;
pub use html_button_element::{HTMLButtonElement, ValidityState};
pub use html_div_element::HTMLDivElement;
pub use html_form_element::HTMLFormElement;
pub use html_label_element::HTMLLabelElement;
use unicode_bidi::{bidi_class, BidiClass};

use std::{cell::RefCell, rc::Rc};

use crate::{
    document::WeakDocumentRef, tag::Tag, AsChildNode, AsElement, AsEventTarget, AsNode,
    AsParentNode, Element, InnerHtml, Node,
};

#[derive(Debug)]
pub(crate) struct HTMLElementBase {
    pub(crate) element: Element,
    value: String,
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
            debug_assert!(document
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

    fn is_document_element(&self) -> bool {
        if let Some(parent) = self.parent_node() {
            return parent == self.owner_document().unwrap();
        }
        false
    }

    pub(crate) fn in_document(tagname: &str, weak_ref: WeakDocumentRef) -> Self {
        HTMLElement {
            inner: Rc::new(RefCell::new(HTMLElementBase {
                element: Element::in_document(tagname, true, weak_ref),
                value: String::new(),
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
                value: String::new(),
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

impl AsHTMLElement for HTMLElement {
    fn cast(&self) -> &HTMLElement {
        self
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        self
    }
}

/// This trait defines all the functions pertaining to [`HTMLElement`] or any of its "descendants".
pub trait AsHTMLElement: AsElement {
    fn cast(&self) -> &HTMLElement;
    fn cast_mut(&mut self) -> &mut HTMLElement;
    // PROPERTIES
    /// Returns the keystroke which a user can press to jump to this element.
    ///
    /// MDN Reference: [HTMLElement.accessKey](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)
    fn access_key(&self) -> &str {
        self.get_attribute("accesskey").unwrap_or("")
    }
    /// Sets the keystroke which a user can press to jump to this element.
    ///
    /// MDN Reference: [HTMLElement.accessKey](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKey)
    fn set_access_key(&mut self, value: &str) {
        self.set_attribute("accesskey", value);
        todo!()
    }
    /// Returns a string with this element's assigned key.
    ///
    /// MDN Reference: [HTMLElement.accessKeyLabel](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/accessKeyLabel)
    fn access_key_label(&self) -> &str {
        match self.get_attribute("accesskey") {
            Some(assigned_access_key) => todo!(),
            None => "",
        }
    }
    /// Returns the value that controls whether and how text input is automatically capitalized as it is entered by the user.
    ///
    /// It returns an empty string if the value has not been set.
    ///
    /// MDN Reference: [autocapitalize](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autocapitalize)
    fn autocapitalize(&self) -> &str {
        let state = helpers::get_own_capitalization_hint(self);
        match state {
            "default" => "",
            "none" | "sentences" => state,
            _ => todo!(),
        }
    }
    /// Sets the value that controls whether and how text input is automatically capitalized as it is entered by the user.
    ///
    /// MDN Reference: [autocapitalize](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autocapitalize)
    fn set_autocapitalize(&mut self, value: &str) {
        self.set_attribute("autocapitalize", value);
        todo!()
    }
    /// Returns a value indicating the writing direction of the content in this element.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/HTMLElement/dir)
    fn dir(&self) -> &str {
        todo!()
    }
    fn set_dir(&mut self, value: &str) {
        todo!()
    }
}

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

mod helpers {
    use crate::{element, AsHTMLElement, HTMLElement, HTMLFormElement};

    pub fn get_own_capitalization_hint<'a>(element: &'a impl AsHTMLElement) -> &'a str {
        match element.get_attribute("autocapitalize") {
            Some(value) if value != "" => value,
            _ => {
                if is_autocapitalize_inheriting_element(element) {
                    match form_owner(element) {
                        Some(form) => get_own_capitalization_hint(form),
                        _ => "default",
                    }
                } else {
                    "default"
                }
            }
        }
    }

    pub fn is_autocapitalize_inheriting_element(element: &impl AsHTMLElement) -> bool {
        todo!()
    }

    pub fn form_owner<'a>(element: &'a impl AsHTMLElement) -> Option<&'a HTMLElement> {
        todo!()
    }
}
