use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{document::Document, EventTarget, WeakDocumentRef};

use super::{HTMLElementRef, IntoEventTarget};

#[derive(Debug, PartialEq, Clone)]
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
    Unknown,
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

#[derive(Clone, PartialEq)]
/// Node is an interface from which a number of DOM API object types inherit. It allows those types to be treated similarly; for example, inheriting the same set of methods, or being tested in the same way.
pub struct Node {
    pub node_type: NodeType,
    pub event_target: EventTarget,
    pub owner_document: Option<WeakDocumentRef>,
    /// Tuple containing the parent node and the index of this node in the parent's child list.
    pub parent: Option<(WeakNodeRef, usize)>,
    pub children: Vec<NodeRef>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("node_type", &self.node_type)
            .field("event_target", &self.event_target)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct WeakNodeRef {
    pub(crate) inner: Weak<RefCell<Node>>,
}

impl PartialEq for WeakNodeRef {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T: IntoNode> From<&T> for WeakNodeRef {
    fn from(node: &T) -> Self {
        WeakNodeRef {
            inner: Rc::downgrade(&IntoNode::cast(node).inner),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeRef {
    pub(crate) inner: Rc<RefCell<Node>>,
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl NodeRef {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Node::new())),
        }
    }
}

impl IntoNode for NodeRef {
    fn cast(&self) -> &NodeRef {
        self
    }

    fn cast_mut(&mut self) -> &mut NodeRef {
        self
    }
}

impl IntoEventTarget for NodeRef {
    fn cast(&self) -> &EventTarget {
        unsafe { &(*self.inner.as_ptr()).event_target }
    }

    fn cast_mut(&mut self) -> &mut EventTarget {
        unsafe { &mut (*self.inner.as_ptr()).event_target }
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_type: NodeType::Unknown,
            event_target: EventTarget::new(),
            parent: None,
            owner_document: None,
            children: vec![],
        }
    }
}

pub trait IntoNode: IntoEventTarget {
    /// Convert to a reference to Node.
    fn cast(&self) -> &NodeRef;
    /// Convert to a mutable reference to Node.
    fn cast_mut(&mut self) -> &mut NodeRef;
    /// Returns node's node document's document base URL.
    fn base_uri(&self) -> &str {
        todo!()
    }
    /// Returns the children.
    fn child_nodes(&self) {
        todo!()
    }
    /// Returns the children mutably.
    fn child_nodes_mut(&mut self) {
        todo!()
    }
    /// Returns the first child.
    fn first_child(&self) -> Option<&NodeRef> {
        unsafe { &*IntoNode::cast(self).inner.as_ptr() }
            .children
            .get(0)
    }
    /// Returns a mutable reference to the first child.
    fn first_child_mut(&mut self) -> Option<&mut NodeRef> {
        unsafe { &mut *IntoNode::cast(self).inner.as_ptr() }
            .children
            .get_mut(0)
    }
    /// Returns true if node is connected and false otherwise.
    fn is_connected(&self) -> bool {
        todo!()
    }
    /// Returns the last child.
    fn last_child(&self) -> Option<&NodeRef> {
        unsafe { &*IntoNode::cast(self).inner.as_ptr() }
            .children
            .last()
    }
    /// Returns the last child mutably.
    fn last_child_mut(&mut self) -> Option<&mut NodeRef> {
        unsafe { &mut *IntoNode::cast(self).inner.as_ptr() }
            .children
            .last_mut()
    }
    /// Returns the next sibling.
    fn next_sibling(&self) -> Option<&NodeRef> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => get_node_at_index(&tuple.0, tuple.1 + 1),
            _ => None,
        }
    }
    /// Returns the next sibling mutably.
    fn next_sibling_mut(&mut self) -> Option<&mut NodeRef> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => get_mut_node_at_index(&tuple.0, tuple.1 + 1),
            _ => None,
        }
    }

    /// Returns a string appropriate for the type of node.
    fn node_name(&self) -> &str {
        todo!()
    }
    /// Returns the type of node.
    fn node_type(&self) -> &NodeType {
        &unsafe { &*IntoNode::cast(self).inner.as_ptr() }.node_type
    }
    fn node_value(&self) -> Option<&str> {
        match self.node_type() {
            NodeType::Element => None,
            _ => todo!(),
        }
    }
    fn set_node_value(&mut self, value: &str) {
        if !self.node_value().is_none() {
            todo!()
        }
    }
    /// Returns the node document. Returns None for documents.
    fn owner_document(&self) -> Option<&Document> {
        todo!()
    }
    /// Returns the node document mutably. Returns None for documents.
    fn owner_document_mut(&mut self) -> Option<&mut Document> {
        todo!()
    }
    /// Returns the parent element.
    fn parent_element(&self) -> Option<&HTMLElementRef> {
        todo!()
    }
    /// Returns the parent element mutably.
    fn parent_element_mut(&mut self) -> Option<&mut HTMLElementRef> {
        todo!()
    }
    /// Returns the parent.
    fn parent_node(&self) -> Option<NodeRef> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => tuple.0.inner.upgrade().map(|inner| NodeRef { inner }),
            _ => None,
        }
    }
    /// Returns the previous sibling.
    fn previous_sibling(&self) -> Option<&NodeRef> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => {
                let is_first_node = tuple.1 == 0;
                if is_first_node {
                    None
                } else {
                    get_node_at_index(&tuple.0, tuple.1 - 1)
                }
            }
            _ => None,
        }
    }
    /// Returns the previous sibling mutably.
    fn previous_sibling_mut(&mut self) -> Option<&mut NodeRef> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => {
                let is_first_node = tuple.1 == 0;
                if is_first_node {
                    None
                } else {
                    get_mut_node_at_index(&tuple.0, tuple.1 - 1)
                }
            }
            _ => None,
        }
    }
    fn text_content(&self) -> Option<&str> {
        todo!()
    }
    fn set_text_content(&mut self, value: &str) {
        todo!()
    }
    fn append_child<'a>(&mut self, node: &'a mut NodeRef) -> &'a NodeRef {
        let weak_reference = WeakNodeRef::from(&*self);
        let index = get_children_length(self);
        node.inner.borrow_mut().parent = Some((weak_reference, index));
        IntoNode::cast(self)
            .inner
            .borrow_mut()
            .children
            .push(node.clone());
        node
    }
    /// Returns a copy of node. If deep is true, the copy also includes the node's descendants.
    fn clone_node(&self, deep: bool) -> NodeRef {
        let noderef = clone_with_parent(IntoNode::cast(self), deep);
        noderef.inner.borrow_mut().parent = None;
        noderef
    }
    /// Returns a bitmask indicating the position of other relative to node.
    fn compare_document_position(&self) -> u8 {
        todo!()
    }
    /// Returns true if other is an inclusive descendant of node, and false otherwise.
    fn contains(&self, other: &NodeRef) -> bool {
        for child in &IntoNode::cast(self).inner.borrow().children {
            if child == other {
                return true;
            }
            if child.contains(other) {
                return true;
            }
        }
        false
    }
    /// Returns node's root.
    fn get_root_node(&self, options: Option<GetRootNodeOptions>) -> Option<&Node> {
        todo!()
    }
    /// Returns whether node has children.
    fn has_child_nodes(&self) -> bool {
        IntoNode::cast(self).inner.borrow().children.len() > 0
    }
    fn insert_before(&mut self, node: Rc<RefCell<Node>>, child: Option<&Node>) -> &Node {
        todo!()
    }
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        todo!()
    }
    /// Returns whether node and otherNode have the same properties.
    fn is_equal_node(&self, other_node: &NodeRef) -> bool {
        let inner_node = IntoNode::cast(self).inner.borrow();
        let other_inner_node = IntoNode::cast(other_node).inner.borrow();
        *inner_node == *other_inner_node
    }
    fn is_same_node(&self, other_node: &NodeRef) -> bool {
        IntoNode::cast(self) == IntoNode::cast(other_node)
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
    fn remove_child(&mut self, node: &mut NodeRef) -> NodeRef {
        todo!()
    }
    fn replace_child(&mut self, node: &mut NodeRef, child: &mut NodeRef) -> &NodeRef {
        todo!()
    }
}

// PARENT NODE.

pub trait IntoParentNode: IntoNode {
    fn child_element_count(&self) -> usize {
        todo!()
    }
    fn children(&self) {
        todo!()
    }
    /// Returns the first child that is an element.
    fn first_element_child(&self) {
        todo!()
    }
    /// Returns the last child that is an element.
    fn last_element_child(&self) {
        todo!()
    }
    /// Inserts nodes after the last child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn append(&mut self, node: &mut NodeRef) {
        self.append_child(node);
    }
    /// Inserts nodes before the first child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn prepend(&mut self, node: &mut NodeRef) {
        let weak_reference = WeakNodeRef::from(&*self);
        node.inner.borrow_mut().parent = Some((weak_reference, 0));
        IntoNode::cast(self)
            .inner
            .borrow_mut()
            .children
            .insert(0, node.clone());
    }
    /// Traverse tree and find the first element that matches a selector, if it exists.
    fn query_selector(&self, selector: &str) {
        todo!()
    }
    fn query_selector_mut(&mut self, selector: &str) {
        todo!()
    }
    /// Traverse tree and find all the elements that matches a selector.
    fn query_selector_all(&self, selector: &str) {
        todo!()
    }
    // /// Replace all children of node with nodes, while replacing strings in nodes with equivalent Text nodes.
    // /// # Panics
    // /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_children(&mut self, nodes: Vec<Node>) {
        todo!()
    }
}

// CHILD NODE.

pub trait ChildNode: IntoNode {
    /// Inserts nodes just after node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn after(&mut self, node: &mut Node) {
        todo!()
    }
    /// Inserts nodes just before node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn before(&mut self, node: &mut Node) {
        todo!()
    }
    /// Removes node.
    fn remove(&mut self) {
        todo!()
    }
    /// Replaces node with nodes, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_with(&mut self, node: &mut Node) {
        todo!()
    }
}

// /// NodeList objects are collections of nodes, usually returned by properties such as [`Node::child_nodes`] and methods such as [`Document::query_selector_all()`].
// pub trait NodeList: Index<usize> {
//     /// Returns the number of nodes in the collection.
//     fn len(&self) -> usize;
//     /// Returns the node with index index from the collection. The nodes are sorted in tree order.
//     fn item(&self, index: usize) -> Option<&NodeRef>;
//     fn item_mut(&mut self, index: usize) -> Option<&mut NodeRef>;
//     /// Performs the specified action for each node in an list.<br><br>
//     /// _@param_ `callbackfn`  A function that accepts up to three arguments. forEach calls the callbackfn function one time for each element in the list.<br><br>
//     /// _@param_ `thisArg`  An object to which the this keyword can refer in the callbackfn function. If thisArg is omitted, undefined is used as the this value.
//     fn for_each(&self, callbackfn: fn(NodeRef, usize, &Self) -> (), this_arg: Option<Box<dyn Any>>);
// }

// pub struct NodeListOf<'a, TNode: IntoNode + ?Sized> {
//     owner: &'a mut dyn IntoNode,
//     __: PhantomData<TNode>,
// }

// impl<'a, TNode> NodeListOf<'a, TNode>
// where
//     TNode: IntoNode,
// {
//     /// Returns the length of the list.
//     pub fn len(&self) -> usize {
//         IntoNode::cast(self.owner).children_ids.len()
//     }
//     /// Returns the node with index index from the collection. The nodes are sorted in tree order.
//     pub fn item(&self, index: usize) -> Option<&TNode> {
//         todo!()
//     }
//     pub fn item_mut(&mut self, index: usize) -> Option<&mut TNode> {
//         todo!()
//     }
//     fn iter(self) {
//         todo!()
//     }
// }

// impl Index<usize> for NodeListOf<'_, ChildNode> {
//     type Output = Rc<RefCell<ChildNode>>;

//     fn index(&self, index: usize) -> &Self::Output {
//         todo!()
//     }
// }

fn get_mut_node_at_index<'a>(parentref: &WeakNodeRef, index: usize) -> Option<&'a mut NodeRef> {
    match parentref.inner.upgrade() {
        Some(parent_node_ref) => unsafe { &mut *parent_node_ref.as_ptr() }
            .children
            .get_mut(index),
        None => None,
    }
}

/// Return the child node at a particular index, if it exists.
fn get_node_at_index<'a>(parentref: &WeakNodeRef, index: usize) -> Option<&'a NodeRef> {
    match parentref.inner.upgrade() {
        Some(parent_node_ref) => unsafe { &*parent_node_ref.as_ptr() }.children.get(index),
        None => None,
    }
}

/// Create a copy of a node still attached to the parent node.
fn clone_with_parent(noderef: &NodeRef, deep: bool) -> NodeRef {
    let inner_node = noderef.inner.borrow();
    if deep {
        NodeRef {
            inner: Rc::new(RefCell::new(Node {
                node_type: inner_node.node_type.clone(),
                event_target: inner_node.event_target.clone(),
                owner_document: inner_node.owner_document.clone(),
                parent: inner_node.parent.clone(),
                children: inner_node
                    .children
                    .as_slice()
                    .iter()
                    .map(|noderef| clone_with_parent(noderef, deep))
                    .collect(),
            })),
        }
    } else {
        NodeRef {
            inner: Rc::new(RefCell::new(inner_node.clone())),
        }
    }
}

/// Get the number of children a node has.
fn get_children_length<T: IntoNode>(parent: &T) -> usize {
    IntoNode::cast(parent).inner.borrow().children.len()
}

#[cfg(test)]
mod tests {
    use crate::{IntoNode, Node, NodeRef, NodeType};

    #[test]
    fn node_size() {
        let node = Node::new();
        println!("{:?}", std::mem::size_of_val(&node));
    }

    #[test]
    fn parent_child_node_check() {
        let mut parent = NodeRef::new();
        let mut child = NodeRef::new();
        let mut grandchild = NodeRef::new();
        parent.append_child(&mut child);
        parent
            .first_child_mut()
            .unwrap()
            .append_child(&mut grandchild);

        assert_eq!(parent.first_child(), Some(&child));
        assert_eq!(parent.last_child(), Some(&child));
        assert_eq!(child.parent_node().as_ref(), Some(&parent));

        assert_eq!(child.first_child(), Some(&grandchild));

        assert!(parent.contains(&grandchild));
    }

    #[test]
    fn sibling_node_check() {
        let mut parent = NodeRef::new();
        let mut child1 = NodeRef::new();
        let mut child2 = NodeRef::new();
        let mut child3 = NodeRef::new();
        parent.append_child(&mut child1);
        parent.append_child(&mut child2);
        parent.append_child(&mut child3);

        assert_eq!(child1.next_sibling(), Some(&child2));
        assert_eq!(child2.next_sibling(), Some(&child1));
        assert_eq!(child2.previous_sibling(), Some(&child1));
        assert_eq!(child3.previous_sibling(), Some(&child2));
    }

    #[test]
    fn equality_node_check() {
        let node1 = NodeRef::new();
        let node2 = NodeRef::new();
        let node1clone = node1.clone();

        assert!(node1.is_equal_node(&node2));
        assert!(node1.is_same_node(&node1clone));
        assert!(!node1.is_same_node(&node2));

        node1.inner.borrow_mut().node_type = NodeType::Element;

        assert!(!node1.is_equal_node(&node2));
    }
}
