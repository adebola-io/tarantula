#![allow(unused, deprecated)]
mod animatable;
mod attr;
mod document;
mod document_type;
mod dom_token_list;
mod domexception;
mod domitem;
mod element;
mod event;
mod event_target;
mod html_collection;
mod html_element;
mod inner_html;
mod misc;
mod named_node_map;
mod node;
mod nodelist;
mod range;
mod slot;
mod tag;
#[cfg(test)]
mod test;
mod window;

pub use animatable::*;
pub use attr::Attr;
pub use document::{AsDocument, Document};
pub use document_type::*;
pub use dom_token_list::{DOMTokenList, MutDOMTokenList};
pub use domexception::*;
pub use element::{
    AsElement, CheckVisibilityOptions, DOMRect, DOMRectList, Element, FullscreenOptions,
    InsertPosition, ScrollIntoView, ScrollToOptions,
};
pub use event::{AsEvent, DOMHighResTimeStamp, Event};
pub use event_target::{AsEventTarget, EventTarget};
pub use html_collection::{HTMLCollection, HTMLCollectionOf};
pub use html_element::*;
pub use inner_html::InnerHtml;
pub use misc::*;
pub use named_node_map::NamedNodeMap;
pub use node::{
    AsChildNode, AsNode, AsParentNode, ChildNode, GetRootNodeOptions, Node, ParentNode,
};
pub use nodelist::{MutNodeListOf, NodeListOf};
pub use range::Range;
pub use slot::*;
pub use window::WindowEventHandlers;

pub type DOMResult = Result<(), DOMException>;

pub mod traits {
    pub use crate::{
        AsChildNode, AsDocument, AsElement, AsEvent, AsEventTarget, AsHTMLElement, AsNode,
        AsParentNode, HTMLHyperlinkElementUtils, HTMLMediaElement, WindowEventHandlers,
    };
}

// fn test() {
//     let col = HtmlCollection { items: vec![] };
// }
