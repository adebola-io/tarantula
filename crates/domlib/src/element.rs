use std::{cell::RefCell, rc::Rc};

use super::{IntoChildNode, IntoNode, IntoParentNode};

pub type Element = dyn IntoElement;
pub type ElementRef = Rc<RefCell<Element>>;

pub struct NamedNodeMap;
pub struct DOMTokenList;
pub struct ShadowRootInit;
pub struct ShadowRootRef;
pub struct CheckVisibilityOptions;

pub trait IntoElement:
    IntoNode + IntoChildNode + IntoParentNode + elements_internal::AsElementInner
{
}

#[allow(unused)]
pub(crate) mod elements_internal {
    use crate::{
        CheckVisibilityOptions, DOMTokenList, DocumentRef, Event, NamedNodeMap, ShadowRootInit,
        ShadowRootRef,
    };

    use super::{Element, ElementRef};

    pub struct ElementInner {
        pub attributes: NamedNodeMap,
        pub class_list: DOMTokenList,
        pub class_name: String,
        pub client_height: usize,
        pub client_top: usize,
        pub client_width: usize,
        pub id: String,
        pub local_name: String,
        pub namespace_uri: Option<String>,
        pub onfullscreenchange: fn(Self, Event),
        pub onfullscreenerror: fn(Self, Event),
        pub outer_html: String,
        pub owner_document: DocumentRef,
        pub part: DOMTokenList,
        pub prefix: String,
        pub scroll_height: usize,
        pub scroll_left: usize,
        pub scroll_top: usize,
        pub scroll_width: usize,
        pub shadow_root: Option<ShadowRootRef>,
        pub slot: String,
        pub tag_name: String,
    }

    impl ElementInner {
        pub fn __attach_shadow(&mut self, init: ShadowRootInit) {
            todo!()
        }
        pub fn __check_visibility(&self, options: Option<CheckVisibilityOptions>) {
            todo!()
        }
        pub fn __closest(&self, selector: &str) -> &ElementRef {
            todo!()
        }
        pub fn __closest_mut(&mut self, selector: &str) -> &mut ElementRef {
            todo!()
        }
        pub fn __get_attribute(&self, qualified_name: &str) -> Option<&str> {
            todo!()
        }
        pub fn __get_attribute_ns(
            &self,
            namespace: Option<&str>,
            local_name: &str,
        ) -> Option<&str> {
            todo!()
        }
    }

    pub trait AsElementInner {
        fn __as_node(&self) -> &ElementInner;
        fn __as_node_mut(&mut self) -> &mut ElementInner;
    }
}
