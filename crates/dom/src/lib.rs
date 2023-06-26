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

pub use animatable::*;
pub use attr::*;
pub use document::*;
pub use document_type::*;
pub use dom_token_list::{DOMTokenList, MutDOMTokenList};
pub use domexception::*;
pub use element::*;
pub use event::*;
pub use event_target::*;
pub use html_collection::*;
pub use html_element::*;
pub use inner_html::*;
pub use named_node_map::*;
pub use node::*;
pub use nodelist::*;
pub use others::*;
pub use tag::*;

// fn test() {
//     let col = HtmlCollection { items: vec![] };
// }
