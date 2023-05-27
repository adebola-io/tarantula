use std::{any::Any, ops::Index};

use super::{DocumentRef, Element, EventTarget, HTMLElementRef, HtmlCollection};

pub trait ParentNode: Node {
    fn child_element_count(&self) -> usize;
    fn children(&self) -> HtmlCollection;
    /// Returns the first child that is an element.
    fn first_element_child(&self) -> Option<&Element>;
    /// Returns the last child that is an element.
    fn last_element_child(&self) -> Option<&Element>;

    /// Inserts nodes after the last child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn append<T: Node>(&mut self, node: T);
    /// Inserts nodes before the first child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn prepend<T: Node>(&mut self, node: T);

    /// Traverse tree and find the first element that matches a selector, if it exists.
    fn query_selector(&self, selector: &str) -> Option<&Element>;
    /// Traverse tree and find all the elements that matches a selector.
    fn query_selector_all(&self, selector: &str) -> NodeListOf<&Element>;

    /// Replace all children of node with nodes, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_children<T>(&mut self, nodes: Vec<Element>);
}

pub trait ChildNode: Node {
    /// Inserts nodes just after node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn after<T: Node>(&mut self, node: T);
    /// Inserts nodes just before node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn before<T: Node>(&mut self, node: T);
    /// Removes node.
    fn remove(&mut self);
    /// Replaces node with nodes, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_with<T: Node>(&mut self, node: T);
}

/// NodeList objects are collections of nodes, usually returned by properties such as [`Node::child_nodes`] and methods such as [`Document::query_selector_all()`].
pub trait NodeList<'a, T: Node>: Sized + Index<usize> {
    /// Returns the number of nodes in the collection.
    fn len(&self) -> usize;
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    fn item(&self, index: usize) -> Option<T>;
    /// Performs the specified action for each node in an list.<br><br>
    /// _@param_ `callbackfn`  A function that accepts up to three arguments. forEach calls the callbackfn function one time for each element in the list.<br><br>
    /// _@param_ `thisArg`  An object to which the this keyword can refer in the callbackfn function. If thisArg is omitted, undefined is used as the this value.
    fn for_each(&self, callbackfn: fn(T, usize, &Self) -> (), this_arg: Option<Box<dyn Any>>);
}

pub struct NodeListOf<T> {
    items: Vec<T>,
}

impl<T> NodeListOf<T> {
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> Index<usize> for NodeListOf<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

/// Node is an interface from which a number of DOM API object types inherit. It allows those types to be treated similarly; for example, inheriting the same set of methods, or being tested in the same way.
pub trait Node: EventTarget {
    /// Returns node's node document's document base URL.
    fn base_uri(&self) -> &str;
    /// Returns the children.
    fn child_nodes(&self) -> &NodeListOf<&Self>;
    /// Returns the first child.
    fn first_child(&self) -> Option<&Self>;
    /// Returns true if node is connected and false otherwise.
    fn is_connected(&self) -> bool;
    /// Returns the last child.
    fn last_child(&self) -> Option<&Self>;
    /// Returns the next sibling.
    fn next_sibling(&self) -> Option<&Self>;
    /// Returns a string appropriate for the type of node.
    fn node_name(&self) -> &str;
    /// Returns the type of node.
    fn node_type(&self) -> u8;
    fn node_value(&self) -> Option<String>;
    /// Returns the node document. Returns null for documents.
    fn owner_document(&self) -> Option<DocumentRef>;
    /// Returns the parent element.
    fn parent_element(&self) -> Option<HTMLElementRef>;
    /// Returns the parent.
    fn parent_node(&self) -> Option<&Self>;
    /// Returns the previous sibling.
    fn previous_sibling(&self) -> Option<&Self>;
    fn text_content(&self) -> Option<String>;
    fn append_child<T: Node>(&mut self) -> T;
    /// Returns a copy of node. If deep is true, the copy also includes the node's descendants.
    fn clone_node(&self, deep: bool) -> Self;
    /// Returns a bitmask indicating the position of other relative to node.
    fn compare_document_position(&self) -> u8;
    /// Returns true if other is an inclusive descendant of node, and false otherwise.
    fn contains(&self, other: Option<&Self>) -> bool
    where
        Self: PartialEq,
    {
        match other {
            Some(node) => {
                for child in self.child_nodes().items.as_slice() {
                    if *child == node {
                        return true;
                    }
                    if child.contains(other) {
                        return true;
                    }
                }
                false
            }
            None => false,
        }
    }
    /// Returns node's root.
    fn get_node_root(&self) -> Option<&Self>;
    /// Returns whether node has children.
    fn has_child_nodes(&self) -> bool {
        self.child_nodes().len() > 0
    }
    fn insert_before<T: Node>(&mut self, node: T, child: Option<&mut Self>) -> &T;
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool;
    /// Returns whether node and otherNode have the same properties.
    fn is_equal_node(&self, other_node: Option<&Self>) -> bool;
    fn is_same_node(&self, other_node: Option<&Self>) -> bool
    where
        Self: PartialEq,
    {
        other_node.is_some() && other_node.unwrap() == self
    }
    fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<&str>;
    fn lookup_prefix(&self, namespace: Option<&str>) -> Option<&str>;
    /// Removes empty exclusive Text nodes and concatenates the data of remaining contiguous exclusive Text nodes into the first of their nodes.
    fn normalize(&mut self);
    fn remove_child<T: Node>(&mut self, node: &mut T) -> &T;
    fn replace_child<T: Node>(&mut self, node: &mut T, child: &mut T) -> &T;

    /// node is an element.
    const ELEMENT_NODE: u8 = 1;
    const ATTRIBUTE_NODE: u8 = 2;
    /// node is a Text node.
    const TEXT_NODE: u8 = 3;
    /// node is a CDATASection node.
    const CDATA_SECTION_NODE: u8 = 4;
    const ENTITY_REFERENCE_NODE: u8 = 5;
    const ENTITY_NODE: u8 = 6;
    /// node is a ProcessingInstruction node.
    const PROCESSING_INSTRUCTION_NODE: u8 = 7;
    /// node is a Comment node.
    const COMMENT_NODE: u8 = 8;
    /// node is a document.
    const DOCUMENT_NODE: u8 = 9;
    /// node is a doctype.
    const DOCUMENT_TYPE_NODE: u8 = 10;
    /// node is a DocumentFragment node.
    const DOCUMENT_FRAGMENT_NODE: u8 = 11;
    const NOTATION_NODE: u8 = 12;
    /// Set when node and other are not in the same tree.
    const DOCUMENT_POSITION_DISCONNECTED: u8 = 0x01;
    /// Set when other is preceding node.
    const DOCUMENT_POSITION_PRECEDING: u8 = 0x02;
    /// Set when other is following node.
    const DOCUMENT_POSITION_FOLLOWING: u8 = 0x03;
    /// Set when other is an ancestor of node.
    const DOCUMENT_POSITION_CONTAINS: u8 = 0x08;
    /// Set when other is a descendant of node.
    const DOCUMENT_POSITION_CONTAINED_BY: u8 = 0x10;
    const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u8 = 0x20;
}
