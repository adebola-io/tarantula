// #![allow(unused)]
mod animatable;
mod document;
// mod dom_token_list;
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
mod others;
mod tag;
// mod html_node;
// mod named_node_map;
// mod node;
mod node;
mod nodelist;
// mod query;
// mod tag;

pub use animatable::*;
pub use document::*;
pub use dom_token_list::*;
pub use domexception::*;
pub use inner_html::InnerHtml;
// pub use dom_token_list::DOMTokenList;
pub use document_type::*;
pub use element::*;
pub use event::*;
pub use event_target::*;
pub use html_collection::*;
pub use html_collection::*;
pub use html_element::HTMLElement;
pub use named_node_map::*;
pub use node::*;
pub use nodelist::*;
pub use others::*;
pub use tag::*;
// pub use html_node::*;
// pub use named_node_map::NamedNodeMap;
// pub use node::Node;
// pub use query::Query;
// pub use tag::HtmlTag;

// fn test() {
//     let col = HtmlCollection { items: vec![] };
// }
