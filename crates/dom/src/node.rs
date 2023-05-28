use std::{
    any::Any,
    cell::RefCell,
    ops::Index,
    rc::{Rc, Weak},
};

use crate::{Attr, DocumentType};

use super::{DocumentRef, Element, HTMLElementRef, HtmlCollection, IntoEventTarget};

pub struct CastError;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    /// node is an element.
    Element = 1,
    Attribute = 2,
    /// node is a Text node.
    Text = 3,
    /// node is a CDATASection node.
    CDATASection = 4,
    EntityReference = 5,
    Entity = 6,
    /// node is a ProcessingInstruction node.
    ProcessingInstruction = 7,
    /// node is a Comment node.
    Comment = 8,
    /// node is a document.
    Document = 9,
    /// node is a doctype.
    DocumentType = 10,
    /// node is a DocumentFragment node.
    DocumentFragment = 11,
    Notation = 12,
}

// /// Set when node and other are not in the same tree.
// const DOCUMENT_POSITION_DISCONNECTED: u8 = 0x01;
// /// Set when other is preceding node.
// const DOCUMENT_POSITION_PRECEDING: u8 = 0x02;
// /// Set when other is following node.
// const DOCUMENT_POSITION_FOLLOWING: u8 = 0x03;
// /// Set when other is an ancestor of node.
// const DOCUMENT_POSITION_CONTAINS: u8 = 0x08;
// /// Set when other is a descendant of node.
// const DOCUMENT_POSITION_CONTAINED_BY: u8 = 0x10;
// const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u8 = 0x20;

pub struct GetRootNodeOptions;

pub type ChildNode = dyn IntoChildNode;
pub type ChildNodeRef = Weak<RefCell<ChildNode>>;
pub type ParentNode = dyn IntoParentNode;
pub type ParentNodeRef = Weak<RefCell<ParentNode>>;
/// Node is an interface from which a number of DOM API object types inherit. It allows those types to be treated similarly; for example, inheriting the same set of methods, or being tested in the same way.
pub type Node = dyn IntoNode;
pub type NodeRef = Rc<RefCell<Node>>;

pub trait IntoNode: IntoEventTarget + internal::AsNodeInner {
    fn as_node(&self) -> &Node;
    fn as_node_mut(&mut self) -> &mut Node;
    /// Returns node's node document's document base URL.
    fn base_uri(&self) -> &str {
        todo!()
    }
    /// Returns the children.
    fn child_nodes(&self) -> &NodeListOf<ChildNode> {
        &self.as_node_inner().nodelist
    }
    /// Returns the children mutably.
    fn child_nodes_mut(&mut self) -> &mut NodeListOf<ChildNode> {
        &mut self.as_node_inner_mut().nodelist
    }
    /// Returns the first child.
    fn first_child(&self) -> Option<&ChildNode> {
        unsafe {
            match self.as_node_inner().nodelist.items.get(0) {
                Some(node) => Some(&*(node.as_ptr())),
                None => None,
            }
        }
    }
    /// Returns a mutable reference to the first child.
    fn first_child_mut(&mut self) -> Option<&mut ChildNode> {
        unsafe {
            match self.as_node_inner_mut().nodelist.items.get_mut(0) {
                Some(node) => Some(&mut *(node.as_ptr())),
                None => None,
            }
        }
    }
    /// Returns true if node is connected and false otherwise.
    fn is_connected(&self) -> bool {
        self.as_node_inner().is_connected
    }
    /// Returns the last child.
    fn last_child(&self) -> Option<&ChildNode> {
        unsafe {
            match self.as_node_inner().nodelist.items.last() {
                Some(node) => Some(&*(node.as_ptr())),
                None => None,
            }
        }
    }
    /// Returns the last child mutably.
    fn last_child_mut(&mut self) -> Option<&mut ChildNode> {
        unsafe {
            match self.as_node_inner_mut().nodelist.items.last_mut() {
                Some(node) => Some(&mut *(node.as_ptr())),
                None => None,
            }
        }
    }
    /// Returns the next sibling.
    fn next_sibling(&self) -> Option<&ChildNode> {
        todo!()
    }
    /// Returns the next sibling mutably.
    fn next_sibling_mut(&mut self) -> Option<&mut ChildNode> {
        todo!()
    }
    /// Try to represent node as an Element.
    fn to_element(&self) -> Option<&Element> {
        None
    }
    /// Try to represent node as an Attr.
    fn to_attr(&self) -> Option<&Attr> {
        None
    }
    fn to_document_type(&self) -> Option<&DocumentType> {
        None
    }

    /// Returns a string appropriate for the type of node.
    fn node_name(&self) -> &str {
        match self.node_type() {
            NodeType::Element => self.to_element().unwrap().z_as_element().tag.as_str(),
            NodeType::Attribute => self.to_attr().unwrap().name.as_str(),
            NodeType::Text => "#text",
            NodeType::CDATASection => "#cdata-section",
            NodeType::Comment => "#comment",
            NodeType::Document => "#document",
            NodeType::DocumentFragment => "#document-fragment",
            NodeType::DocumentType => self.to_document_type().unwrap().name.as_str(),
            NodeType::ProcessingInstruction => todo!(),
            _ => unimplemented!(),
        }
    }
    /// Returns the type of node.
    fn node_type(&self) -> &NodeType {
        &self.as_node_inner().node_type
    }
    fn node_value(&self) -> Option<&str> {
        match self.node_type() {
            NodeType::Element => None,
            NodeType::Attribute => todo!(),
            NodeType::Text => todo!(),
            NodeType::CDATASection => todo!(),
            NodeType::EntityReference => todo!(),
            NodeType::Entity => todo!(),
            NodeType::ProcessingInstruction => todo!(),
            NodeType::Comment => todo!(),
            NodeType::Document => None,
            NodeType::DocumentType => None,
            NodeType::DocumentFragment => todo!(),
            NodeType::Notation => todo!(),
        }
    }
    fn set_node_value(&mut self, value: &str) {
        if !self.node_value().is_none() {
            todo!()
        }
    }
    /// Returns the node document. Returns None for documents.
    fn owner_document(&self) -> Option<&DocumentRef> {
        self.as_node_inner().owner_document.as_ref()
    }
    /// Returns the node document mutably. Returns None for documents.
    fn owner_document_mut(&mut self) -> Option<&mut DocumentRef> {
        self.as_node_inner_mut().owner_document.as_mut()
    }
    /// Returns the parent element.
    fn parent_element(&self) -> Option<&HTMLElementRef> {
        self.as_node_inner().parent_element.as_ref()
    }
    /// Returns the parent element mutably.
    fn parent_element_mut(&mut self) -> Option<&mut HTMLElementRef> {
        self.as_node_inner_mut().parent_element.as_mut()
    }
    /// Returns the parent.
    fn parent_node(&self) -> Option<&ParentNodeRef> {
        self.as_node_inner().parent_node.as_ref()
    }
    /// Returns the parent mutably.
    fn parent_node_mut(&mut self) -> Option<&mut ParentNodeRef> {
        self.as_node_inner_mut().parent_node.as_mut()
    }
    /// Returns the previous sibling.
    fn previous_sibling(&self) -> Option<&ChildNode> {
        todo!()
    }
    /// Returns the previous sibling mutably.
    fn previous_sibling_mut(&mut self) -> Option<&mut ChildNode> {
        todo!()
    }
    fn text_content(&self) -> Option<&str> {
        self.as_node_inner()
            .text_content
            .as_ref()
            .map(|x| x.as_str())
    }
    fn set_text_content(&mut self, value: &str) {
        self.as_node_inner_mut().text_content = Some(value.to_string())
    }
    fn append_child(&mut self, node: Rc<RefCell<Node>>) -> &Node {
        todo!()
    }
    /// Returns a copy of node. If deep is true, the copy also includes the node's descendants.
    fn clone_node(&self, deep: bool) -> Rc<RefCell<Node>> {
        todo!()
    }
    /// Returns a bitmask indicating the position of other relative to node.
    fn compare_document_position(&self) -> u8 {
        todo!()
    }
    /// Returns true if other is an inclusive descendant of node, and false otherwise.
    fn contains(&self, other: Option<&Node>) -> bool {
        todo!()
    }
    /// Returns node's root.
    fn get_root_node(&self, options: Option<GetRootNodeOptions>) -> Option<&Node> {
        todo!()
    }
    /// Returns whether node has children.
    fn has_child_nodes(&self) -> bool {
        self.last_child().is_some()
    }
    fn insert_before(&mut self, node: Rc<RefCell<Node>>, child: Option<&Node>) -> &Node {
        todo!()
    }
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        todo!()
    }
    /// Returns whether node and otherNode have the same properties.
    fn is_equal_node(&self, other_node: Option<&Node>) -> bool {
        todo!()
    }
    fn is_same_node(&self, other_node: Option<&Node>) -> bool {
        todo!()
    }
    fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<&str> {
        todo!()
    }
    fn lookup_prefix(&self, namespace: Option<&str>) -> Option<&str> {
        todo!()
    }
    /// Removes empty exclusive Text nodes and concatenates the data of remaining contiguous exclusive Text nodes into the first of their nodes.
    fn normalize(&mut self) {
        todo!()
    }
    fn remove_child(&mut self, node: &Rc<RefCell<Node>>) -> &Node {
        todo!()
    }
    fn replace_child(&mut self, node: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) -> &Node {
        todo!()
    }
}

pub trait IntoParentNode: IntoNode {
    fn as_parent_node(&self) -> &ParentNode;
    fn as_parent_node_mut(&mut self) -> &mut ParentNode;
    fn child_element_count(&self) -> usize {
        todo!()
    }
    fn children(&self) -> HtmlCollection {
        todo!()
    }
    /// Returns the first child that is an element.
    fn first_element_child(&self) -> Option<&Element> {
        todo!()
    }
    /// Returns the last child that is an element.
    fn last_element_child(&self) -> Option<Rc<RefCell<Element>>> {
        todo!()
    }
    /// Inserts nodes after the last child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn append(&mut self, node: Rc<RefCell<ChildNode>>) {
        self.child_nodes_mut().items.push(node);
    }
    /// Inserts nodes before the first child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn prepend(&mut self, node: Rc<RefCell<ChildNode>>) {
        self.child_nodes_mut().items.insert(0, node);
    }
    /// Traverse tree and find the first element that matches a selector, if it exists.
    fn query_selector(&self, selector: &str) -> Option<&Rc<RefCell<Element>>> {
        todo!()
    }
    fn query_selector_mut(&mut self, selector: &str) -> Option<&mut Rc<RefCell<Element>>> {
        todo!()
    }
    /// Traverse tree and find all the elements that matches a selector.
    fn query_selector_all(&self, selector: &str) -> NodeListOf<Element> {
        todo!()
    }
    // /// Replace all children of node with nodes, while replacing strings in nodes with equivalent Text nodes.
    // /// # Panics
    // /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_children(&mut self, nodes: Vec<NodeRef>) {
        todo!()
    }
}

pub trait IntoChildNode: IntoNode {
    /// Inserts nodes just after node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn after(&mut self, node: NodeRef) {
        todo!()
    }
    /// Inserts nodes just before node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn before(&mut self, node: NodeRef) {
        todo!()
    }
    /// Removes node.
    fn remove(&mut self) {
        // Disconnect from parent.
        if let Some(parentref) = self.parent_node_mut() {
            match parentref.upgrade() {
                Some(parent) => parent
                    .borrow_mut()
                    .as_node_inner_mut()
                    .nodelist
                    .items
                    .retain(|node| node.borrow().as_node_inner() != self.as_node_inner()),
                None => {}
            }
        }

        self.as_node_inner_mut().parent_node = None;
    }
    /// Replaces node with nodes, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_with(&mut self, node: NodeRef) {
        todo!()
    }
}

/// NodeList objects are collections of nodes, usually returned by properties such as [`Node::child_nodes`] and methods such as [`Document::query_selector_all()`].
pub trait NodeList: Index<usize> {
    /// Returns the number of nodes in the collection.
    fn len(&self) -> usize;
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    fn item(&self, index: usize) -> Option<&NodeRef>;
    fn item_mut(&mut self, index: usize) -> Option<&mut NodeRef>;
    /// Performs the specified action for each node in an list.<br><br>
    /// _@param_ `callbackfn`  A function that accepts up to three arguments. forEach calls the callbackfn function one time for each element in the list.<br><br>
    /// _@param_ `thisArg`  An object to which the this keyword can refer in the callbackfn function. If thisArg is omitted, undefined is used as the this value.
    fn for_each(&self, callbackfn: fn(NodeRef, usize, &Self) -> (), this_arg: Option<Box<dyn Any>>);
}

pub struct NodeListOf<TNode: IntoNode + ?Sized> {
    pub(crate) items: Vec<Rc<RefCell<TNode>>>,
}

impl<TNode> NodeListOf<TNode>
where
    TNode: IntoNode,
{
    /// Returns an iterator over the slice.
    pub fn iter(&self) -> impl Iterator<Item = &TNode> {
        unsafe { self.items.as_slice().iter().map(|x| &*x.as_ptr()) }
    }
    /// Returns an iterator that allows modifying each value.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TNode> {
        unsafe {
            self.items
                .as_mut_slice()
                .iter_mut()
                .map(|x| &mut *x.as_ptr())
        }
    }
    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    pub fn item(&self, index: usize) -> Option<&TNode> {
        unsafe { self.items.get(index).map(|x| &*x.as_ptr()) }
    }
    pub fn item_mut(&mut self, index: usize) -> Option<&mut TNode> {
        unsafe { self.items.get_mut(index).map(|x| &mut *x.as_ptr()) }
    }
    /// Performs the specified action for each node in an list.<br><br>
    /// _@param_ `callbackfn`  A function that accepts up to three arguments. forEach calls the callbackfn function one time for each element in the list.<br><br>
    /// _@param_ `thisArg`  An object to which the this keyword can refer in the callbackfn function. If thisArg is omitted, undefined is used as the this value.
    pub fn for_each(
        &self,
        callbackfn: fn(&TNode, usize, &Self) -> (),
        // this_arg: Option<Box<dyn Any>>,
    ) {
        self.iter().enumerate().for_each(|item| {
            callbackfn(item.1, item.0, self);
        })
    }
}

impl Index<usize> for NodeListOf<ChildNode> {
    type Output = Rc<RefCell<ChildNode>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

#[doc(hidden)]
pub(crate) mod internal {
    use crate::{DocumentRef, HTMLElementRef, NodeType};

    use super::{ChildNode, NodeListOf, ParentNodeRef};

    pub struct NodeInner {
        pub nodelist: NodeListOf<ChildNode>,
        pub is_connected: bool,
        pub node_type: NodeType,
        pub owner_document: Option<DocumentRef>,
        pub parent_element: Option<HTMLElementRef>,
        pub parent_node: Option<ParentNodeRef>,
        pub text_content: Option<String>,
    }

    impl NodeInner {
        pub fn get_next_sibling(&self) -> &mut ChildNode {
            todo!()
        }
    }

    impl PartialEq for NodeInner {
        fn eq(&self, other: &Self) -> bool {
            std::ptr::addr_of!(self) == std::ptr::addr_of!(other)
        }
    }

    pub trait AsNodeInner {
        fn as_node_inner(&self) -> &NodeInner;
        fn as_node_inner_mut(&mut self) -> &mut NodeInner;
    }
}

fn get_mut_node_at<'a>(
    parent: Option<&'a ParentNodeRef>,
    index: usize,
) -> Option<&'a mut ChildNode> {
    match parent {
        Some(parent) => match parent.upgrade() {
            Some(p) => unsafe {
                let parent = &*(p.as_ptr());
                parent
                    .as_node_inner()
                    .nodelist
                    .items
                    .get(index)
                    .map(|siblingref| &mut *(siblingref.as_ptr()))
            },
            None => None,
        },
        None => None,
    }
}

fn get_node_at<'a>(parent: Option<&'a ParentNodeRef>, index: usize) -> Option<&'a ChildNode> {
    match parent {
        Some(parent) => match parent.upgrade() {
            Some(p) => unsafe {
                let parent = &*(p.as_ptr());
                parent
                    .as_node_inner()
                    .nodelist
                    .items
                    .get(index)
                    .map(|siblingref| &*(siblingref.as_ptr()))
            },
            None => None,
        },
        None => None,
    }
}
