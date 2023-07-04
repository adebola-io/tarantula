#![allow(unused)]
mod animatable;
mod attr;
mod document;
mod document_type;
mod dom_token_list;
mod domexception;
mod element;
mod event;
mod event_target;
mod html_collection;
mod html_element;
mod inner_html;
mod named_node_map;
mod node;
mod nodelist;
mod others;
mod tag;
#[cfg(test)]
mod test;

pub use animatable::*;
pub use attr::Attr;
pub use document::Document;
pub use document_type::*;
pub use dom_token_list::{DOMTokenList, MutDOMTokenList};
pub use domexception::*;
pub use element::{
    AsElement, CheckVisibilityOptions, DOMRect, DOMRectList, Element, FullscreenOptions,
    InsertPosition, ScrollIntoView, ScrollToOptions, ShadowRoot, ShadowRootInit,
};
pub use event::{AsEvent, DOMHighResTimeStamp, Event};
pub use event_target::{AsEventTarget, EventTarget};
pub use html_collection::{HTMLCollection, HTMLCollectionOf};
pub use html_element::{
    AsHTMLElement, HTMLAnchorElement, HTMLButtonElement, HTMLDivElement, HTMLElement,
    HTMLFormElement, HTMLLabelElement,
};
pub use inner_html::*;
pub use named_node_map::*;
pub use node::{
    AsChildNode, AsNode, AsParentNode, ChildNode, GetRootNodeOptions, Node, ParentNode,
};
pub use nodelist::*;
pub use others::*;

pub type DOMResult = Result<(), DOMException>;

pub mod traits {
    pub use crate::{
        AsChildNode, AsElement, AsEvent, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    };
}

// fn test() {
//     let col = HtmlCollection { items: vec![] };
// }
