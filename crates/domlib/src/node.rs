use std::{any::Any, cell::RefCell, ops::Index, rc::Rc, slice::Iter};

use super::{DocumentRef, Element, HTMLElementRef, HtmlCollection, IntoEventTarget};

// /// node is an element.
// const ELEMENT_NODE: u8 = 1;
// const ATTRIBUTE_NODE: u8 = 2;
// /// node is a Text node.
// const TEXT_NODE: u8 = 3;
// /// node is a CDATASection node.
// const CDATA_SECTION_NODE: u8 = 4;
// const ENTITY_REFERENCE_NODE: u8 = 5;
// const ENTITY_NODE: u8 = 6;
// /// node is a ProcessingInstruction node.
// const PROCESSING_INSTRUCTION_NODE: u8 = 7;
// /// node is a Comment node.
// const COMMENT_NODE: u8 = 8;
// /// node is a document.
// const DOCUMENT_NODE: u8 = 9;
// /// node is a doctype.
// const DOCUMENT_TYPE_NODE: u8 = 10;
// /// node is a DocumentFragment node.
// const DOCUMENT_FRAGMENT_NODE: u8 = 11;
// const NOTATION_NODE: u8 = 12;
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
pub type ChildNodeRef = Rc<RefCell<ChildNode>>;
pub type ParentNode = dyn IntoParentNode;
pub type ParentNodeRef = Rc<RefCell<ParentNode>>;
/// Node is an interface from which a number of DOM API object types inherit. It allows those types to be treated similarly; for example, inheriting the same set of methods, or being tested in the same way.
pub type Node = dyn IntoNode;
pub type NodeRef = Rc<RefCell<Node>>;

pub trait IntoNode: IntoEventTarget + internal::AsNodeInner {
    /// Returns node's node document's document base URL.
    fn base_uri(&self) -> &str {
        self.__as_node().base_uri.as_str()
    }
    /// Returns the children.
    fn child_nodes(&self) -> &NodeListOf<ChildNode> {
        &self.__as_node().nodelist
    }
    /// Returns the children mutably.
    fn child_nodes_mut(&mut self) -> &mut NodeListOf<ChildNode> {
        &mut self.__as_node_mut().nodelist
    }
    /// Returns the first child.
    fn first_child(&self) -> Option<&ChildNodeRef> {
        self.__as_node().first_child.as_ref()
    }
    /// Returns the first child mutably.
    fn first_child_mut(&mut self) -> Option<&mut ChildNodeRef> {
        self.__as_node_mut().first_child.as_mut()
    }
    /// Returns true if node is connected and false otherwise.
    fn is_connected(&self) -> bool {
        self.__as_node().is_connected
    }
    /// Returns the last child.
    fn last_child(&self) -> Option<&ChildNodeRef> {
        self.__as_node().last_child.as_ref()
    }
    /// Returns the last child mutably.
    fn last_child_mut(&mut self) -> Option<&mut ChildNodeRef> {
        self.__as_node_mut().last_child.as_mut()
    }
    /// Returns the next sibling.
    fn next_sibling(&self) -> Option<&ChildNodeRef> {
        self.__as_node().next_sibling.as_ref()
    }
    /// Returns the next sibling mutably.
    fn next_sibling_mut(&mut self) -> Option<&mut ChildNodeRef> {
        self.__as_node_mut().next_sibling.as_mut()
    }
    /// Returns a string appropriate for the type of node.
    fn node_name(&self) -> &str {
        &self.__as_node().node_name
    }
    /// Returns the type of node.
    fn node_type(&self) -> &u8 {
        &self.__as_node().node_type
    }
    fn node_value(&self) -> Option<&str> {
        self.__as_node().node_value.as_ref().map(|x| x.as_str())
    }
    fn set_node_value(&mut self, value: &str) {
        self.__as_node_mut().node_value = Some(value.to_string());
    }
    /// Returns the node document. Returns None for documents.
    fn owner_document(&self) -> Option<&DocumentRef> {
        self.__as_node().owner_document.as_ref()
    }
    /// Returns the node document mutably. Returns None for documents.
    fn owner_document_mut(&mut self) -> Option<&mut DocumentRef> {
        self.__as_node_mut().owner_document.as_mut()
    }
    /// Returns the parent element.
    fn parent_element(&self) -> Option<&HTMLElementRef> {
        self.__as_node().parent_element.as_ref()
    }
    /// Returns the parent element mutably.
    fn parent_element_mut(&mut self) -> Option<&mut HTMLElementRef> {
        self.__as_node_mut().parent_element.as_mut()
    }
    /// Returns the parent.
    fn parent_node(&self) -> Option<&ParentNodeRef> {
        self.__as_node().parent_node.as_ref()
    }
    /// Returns the parent mutably.
    fn parent_node_mut(&mut self) -> Option<&mut ParentNodeRef> {
        self.__as_node_mut().parent_node.as_mut()
    }
    /// Returns the previous sibling.
    fn previous_sibling(&self) -> Option<&ChildNodeRef> {
        self.__as_node().previous_sibling.as_ref()
    }
    /// Returns the previous sibling mutably.
    fn previous_sibling_mut(&mut self) -> Option<&mut ChildNodeRef> {
        self.__as_node_mut().previous_sibling.as_mut()
    }
    fn text_content(&self) -> Option<&str> {
        self.__as_node().text_content.as_ref().map(|x| x.as_str())
    }
    fn set_text_content(&mut self, value: &str) {
        self.__as_node_mut().text_content = Some(value.to_string())
    }
    fn append_child(&mut self, node: Rc<RefCell<dyn IntoNode>>) -> &dyn IntoNode {
        todo!()
    }
    /// Returns a copy of node. If deep is true, the copy also includes the node's descendants.
    fn clone_node(&self, deep: bool) -> Rc<RefCell<dyn IntoNode>> {
        todo!()
    }
    /// Returns a bitmask indicating the position of other relative to node.
    fn compare_document_position(&self) -> u8 {
        todo!()
    }
    /// Returns true if other is an inclusive descendant of node, and false otherwise.
    fn contains(&self, other: Option<Rc<RefCell<dyn IntoNode>>>) -> bool {
        todo!()
    }
    /// Returns node's root.
    fn get_root_node(&self, options: Option<GetRootNodeOptions>) -> Option<&Node> {
        todo!()
    }
    /// Returns whether node has children.
    fn has_child_nodes(&self) -> bool {
        self.child_nodes().len() > 0
    }
    fn insert_before(&mut self, node: Rc<RefCell<Node>>, child: Option<&Node>) -> &Node {
        todo!()
    }
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        todo!()
    }
    /// Returns whether node and otherNode have the same properties.
    fn is_equal_node(&self, other_node: Option<&dyn IntoNode>) -> bool {
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
    fn remove_child(&mut self, node: &Rc<RefCell<dyn IntoNode>>) -> &dyn IntoNode {
        todo!()
    }
    fn replace_child(
        &mut self,
        node: &Rc<RefCell<dyn IntoNode>>,
        child: &Rc<RefCell<dyn IntoNode>>,
    ) -> &dyn IntoNode {
        todo!()
    }
}

pub trait IntoParentNode: IntoNode + internal::AsParentNodeInner {
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
    fn append(&mut self, node: Rc<RefCell<Node>>) {
        todo!()
    }
    /// Inserts nodes before the first child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn prepend(&mut self, node: Rc<RefCell<Node>>) {
        todo!()
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

pub trait IntoChildNode: IntoNode + internal::AsChildNodeInner {
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
        todo!()
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
    items: Vec<Rc<RefCell<TNode>>>,
}

impl<TNode> NodeListOf<TNode>
where
    TNode: IntoNode,
{
    pub fn iter(&self) -> Iter<Rc<RefCell<TNode>>> {
        self.items.as_slice().iter()
    }
}

impl NodeListOf<ChildNode> {
    fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    fn item(&self, index: usize) -> Option<&ChildNodeRef> {
        self.items.get(index)
    }
    fn item_mut(&mut self, index: usize) -> Option<&mut ChildNodeRef> {
        self.items.get_mut(index)
    }
    /// Performs the specified action for each node in an list.<br><br>
    /// _@param_ `callbackfn`  A function that accepts up to three arguments. forEach calls the callbackfn function one time for each element in the list.<br><br>
    /// _@param_ `thisArg`  An object to which the this keyword can refer in the callbackfn function. If thisArg is omitted, undefined is used as the this value.
    fn for_each(
        &self,
        callbackfn: fn(NodeRef, usize, &Self) -> (),
        this_arg: Option<Box<dyn Any>>,
    ) {
        todo!()
    }
}

impl Index<usize> for NodeListOf<ChildNode> {
    type Output = Rc<RefCell<ChildNode>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

pub(crate) mod internal {
    use crate::{DocumentRef, HTMLElementRef};

    use super::{ChildNode, ChildNodeRef, NodeListOf, ParentNode, ParentNodeRef};

    pub struct NodeInner {
        pub __nodes: Vec<ChildNodeRef>,
        pub nodelist: NodeListOf<ChildNode>,
        pub base_uri: String,
        pub first_child: Option<ChildNodeRef>,
        pub is_connected: bool,
        pub last_child: Option<ChildNodeRef>,
        pub next_sibling: Option<ChildNodeRef>,
        pub node_name: String,
        pub node_type: u8,
        pub node_value: Option<String>,
        pub owner_document: Option<DocumentRef>,
        pub parent_element: Option<HTMLElementRef>,
        pub parent_node: Option<ParentNodeRef>,
        pub previous_sibling: Option<ChildNodeRef>,
        pub text_content: Option<String>,
    }

    pub trait AsNodeInner {
        fn __as_node(&self) -> &NodeInner;
        fn __as_node_mut(&mut self) -> &mut NodeInner;
    }

    pub trait AsChildNodeInner {
        fn __as_child_node(&self) -> &ChildNode;
        fn __as_child_node_mut(&mut self) -> &mut ChildNode;
    }

    pub trait AsParentNodeInner {
        fn __as_parent_node(&self) -> &ParentNode;
        fn __as_parent_node_mut(&mut self) -> &mut ParentNode;
    }
}
