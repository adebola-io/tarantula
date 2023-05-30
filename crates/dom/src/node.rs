use std::{
    cell::RefCell,
    ops::Index,
    rc::{Rc, Weak},
};

use crate::{DocumentRef, EventTarget, HTMLCollection, MutHTMLCollection, WeakDocumentRef};

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

impl NodeType {
    /// Returns `true` if the node type is [`Document`].
    ///
    /// [`Document`]: NodeType::Document
    #[must_use]
    pub fn is_document(&self) -> bool {
        matches!(self, Self::Document)
    }

    /// Returns `true` if the node type is [`DocumentType`].
    ///
    /// [`DocumentType`]: NodeType::DocumentType
    #[must_use]
    pub fn is_document_type(&self) -> bool {
        matches!(self, Self::DocumentType)
    }

    /// Returns `true` if the node type is [`Text`].
    ///
    /// [`Text`]: NodeType::Text
    #[must_use]
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }

    /// Returns `true` if the node type is [`Attribute`].
    ///
    /// [`Attribute`]: NodeType::Attribute
    #[must_use]
    pub fn is_attribute(&self) -> bool {
        matches!(self, Self::Attribute)
    }

    /// Returns `true` if the node type is [`Element`].
    ///
    /// [`Element`]: NodeType::Element
    #[must_use]
    pub fn is_element(&self) -> bool {
        matches!(self, Self::Element)
    }
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

pub(crate) struct NodeBase {
    pub(crate) node_type: NodeType,
    /// The inner event target.
    pub(crate) event_target: EventTarget,
    pub(crate) owner_document: Option<WeakDocumentRef>,
    /// Tuple containing the parent node and the index of this node in the parent's child list.
    pub(crate) parent: Option<(WeakNodeRef, usize)>,
    pub(crate) children: Vec<ChildNode>,
}

impl std::fmt::Debug for NodeBase {
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
    pub(crate) inner: Weak<RefCell<NodeBase>>,
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

/// Node is an interface from which a number of DOM API object types inherit. It allows those types to be treated similarly; for example, inheriting the same set of methods, or being tested in the same way.
#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) inner: Rc<RefCell<NodeBase>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}
impl PartialEq<ChildNode> for Node {
    fn eq(&self, other: &ChildNode) -> bool {
        self == &(*other).inner
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(NodeBase::new())),
        }
    }
}

impl IntoNode for Node {
    fn cast(&self) -> &Node {
        self
    }

    fn cast_mut(&mut self) -> &mut Node {
        self
    }
}

impl IntoEventTarget for Node {
    fn cast(&self) -> &EventTarget {
        unsafe { &(*self.inner.as_ptr()).event_target }
    }

    fn cast_mut(&mut self) -> &mut EventTarget {
        unsafe { &mut (*self.inner.as_ptr()).event_target }
    }
}

impl NodeBase {
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
    fn cast(&self) -> &Node;
    /// Convert to a mutable reference to Node.
    fn cast_mut(&mut self) -> &mut Node;
    /// Returns node's node document's document base URL.
    fn base_uri(&self) -> &str {
        todo!()
    }
    /// Returns the children.
    fn child_nodes(&self) -> NodeListOf<'_, ChildNode> {
        NodeListOf {
            items: unsafe { &(*IntoNode::cast(self).inner.as_ptr()).children },
        }
    }
    /// Returns the children mutably.
    fn child_nodes_mut(&mut self) -> MutNodeListOf<'_, ChildNode> {
        MutNodeListOf {
            items: unsafe { &mut (*IntoNode::cast(self).inner.as_ptr()).children },
        }
    }
    /// Returns the first child.
    fn first_child(&self) -> Option<&ChildNode> {
        unsafe { &*IntoNode::cast(self).inner.as_ptr() }
            .children
            .get(0)
    }
    /// Returns a mutable reference to the first child.
    fn first_child_mut(&mut self) -> Option<&mut ChildNode> {
        unsafe { &mut *IntoNode::cast(self).inner.as_ptr() }
            .children
            .get_mut(0)
    }
    /// Returns true if node is connected and false otherwise.
    fn is_connected(&self) -> bool {
        todo!()
    }
    /// Returns the last child.
    fn last_child(&self) -> Option<&ChildNode> {
        unsafe { &*IntoNode::cast(self).inner.as_ptr() }
            .children
            .last()
    }
    /// Returns the last child mutably.
    fn last_child_mut(&mut self) -> Option<&mut ChildNode> {
        unsafe { &mut *IntoNode::cast(self).inner.as_ptr() }
            .children
            .last_mut()
    }
    /// Returns the next sibling.
    fn next_sibling(&self) -> Option<&ChildNode> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => helpers::get_node_at_index(&tuple.0, tuple.1 + 1),
            _ => None,
        }
    }
    /// Returns the next sibling mutably.
    fn next_sibling_mut(&mut self) -> Option<&mut ChildNode> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => helpers::get_mut_node_at_index(&tuple.0, tuple.1 + 1),
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
    fn owner_document(&self) -> Option<DocumentRef> {
        if self.node_type().is_document() {
            return None;
        }
        if let Some(weak_document_ref) = &IntoNode::cast(self).inner.borrow().owner_document {
            weak_document_ref
                .inner
                .upgrade()
                .map(|inner| DocumentRef { inner })
        } else {
            None
        }
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
    fn parent_node(&self) -> Option<ParentNode> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => tuple.0.inner.upgrade().map(|inner| ParentNode {
                inner: Node { inner },
            }),
            _ => None,
        }
    }
    /// Returns the previous sibling.
    fn previous_sibling(&self) -> Option<&ChildNode> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => {
                let is_first_node = tuple.1 == 0;
                if is_first_node {
                    None
                } else {
                    helpers::get_node_at_index(&tuple.0, tuple.1 - 1)
                }
            }
            _ => None,
        }
    }
    /// Returns the previous sibling mutably.
    fn previous_sibling_mut(&mut self) -> Option<&mut ChildNode> {
        match &IntoNode::cast(self).inner.borrow().parent {
            Some(tuple) => {
                let is_first_node = tuple.1 == 0;
                if is_first_node {
                    None
                } else {
                    helpers::get_mut_node_at_index(&tuple.0, tuple.1 - 1)
                }
            }
            _ => None,
        }
    }
    fn text_content(&self) -> Option<&str> {
        unimplemented!()
    }
    fn set_text_content(&mut self, value: &str) {
        unimplemented!()
    }
    fn append_child<'a>(&mut self, node: &'a mut Node) -> &'a Node {
        let childnode = ChildNode {
            inner: node.clone(),
        };
        let weak_reference = WeakNodeRef::from(&*self);
        let index = helpers::get_children_length(self);
        IntoNode::cast(&childnode).inner.borrow_mut().parent = Some((weak_reference, index));
        IntoNode::cast(self)
            .inner
            .borrow_mut()
            .children
            .push(childnode);
        node
    }
    /// Returns a copy of node. If deep is true, the copy also includes the node's descendants.
    fn clone_node(&self, deep: bool) -> Node {
        let noderef = helpers::clone_node(IntoNode::cast(self), deep);
        noderef.inner.borrow_mut().parent = None;
        noderef
    }
    /// Returns a bitmask indicating the position of other relative to node.
    fn compare_document_position(&self) -> u8 {
        todo!()
    }
    /// Returns true if other is an inclusive descendant of node, and false otherwise.
    fn contains(&self, other: &Node) -> bool {
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
    fn insert_before(&mut self, node: &mut Node, child: &mut Node) -> Node {
        todo!()
    }
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        todo!()
    }
    /// Returns whether node and otherNode have the same properties.
    fn is_equal_node(&self, other_node: &Node) -> bool {
        let inner_node = IntoNode::cast(self).inner.borrow();
        let other_inner_node = IntoNode::cast(other_node).inner.borrow();
        *inner_node == *other_inner_node
    }
    fn is_same_node(&self, other_node: &Node) -> bool {
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
    fn remove_child(&mut self, node: &mut Node) -> Node {
        todo!()
    }
    fn replace_child(&mut self, node: &mut Node, child: &mut Node) -> &Node {
        todo!()
    }
}

// PARENT NODE.

#[derive(Debug, PartialEq, Clone)]
pub struct ParentNode {
    inner: Node,
}
impl PartialEq<Node> for ParentNode {
    fn eq(&self, other: &Node) -> bool {
        self.inner == *other
    }
}
impl IntoEventTarget for ParentNode {
    fn cast(&self) -> &EventTarget {
        IntoEventTarget::cast(&self.inner)
    }

    fn cast_mut(&mut self) -> &mut EventTarget {
        IntoEventTarget::cast_mut(&mut self.inner)
    }
}
impl IntoNode for ParentNode {
    fn cast(&self) -> &Node {
        &self.inner
    }

    fn cast_mut(&mut self) -> &mut Node {
        &mut self.inner
    }
}
impl IntoParentNode for ParentNode {}

pub trait IntoParentNode: IntoNode {
    fn child_element_count(&self) -> usize {
        todo!()
    }
    fn children(&self) -> HTMLCollection {
        todo!()
    }
    fn children_mut(&mut self) -> MutHTMLCollection {
        todo!()
    }
    /// Returns the first child that is an element.
    fn first_element_child(&self) {
        todo!()
    }
    /// Returns the first child that is an element, mutably.
    fn first_element_child_mut(&mut self) {
        todo!()
    }
    /// Returns the last child that is an element.
    fn last_element_child(&self) {
        todo!()
    }
    /// Returns the last child that is an element, mutably.
    fn last_element_child_mut(&mut self) {
        todo!()
    }
    /// Inserts nodes after the last child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn append(&mut self, node: &mut Node) {
        self.append_child(node);
    }
    /// Inserts nodes before the first child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn prepend(&mut self, node: &mut Node) {
        let mut self_node = IntoNode::cast(self).inner.borrow_mut();
        let child_node = ChildNode {
            inner: node.clone(),
        };
        let weak_reference = WeakNodeRef::from(&*self);
        node.inner.borrow_mut().parent = Some((weak_reference, 0));
        // TODO: Shift all indexes and insert node with one pass.
        // Shift all indexes.
        for child in &mut self_node.children {
            if let Some(tuple) = IntoNode::cast(child).inner.borrow_mut().parent.as_mut() {
                tuple.1 += 1;
            }
        }
        self_node.children.insert(0, child_node);
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

#[derive(Debug, Clone, PartialEq)]
pub struct ChildNode {
    inner: Node,
}
impl PartialEq<Node> for ChildNode {
    fn eq(&self, other: &Node) -> bool {
        self.inner == *other
    }
}
impl IntoEventTarget for ChildNode {
    fn cast(&self) -> &EventTarget {
        IntoEventTarget::cast(&self.inner)
    }

    fn cast_mut(&mut self) -> &mut EventTarget {
        IntoEventTarget::cast_mut(&mut self.inner)
    }
}
impl IntoNode for ChildNode {
    fn cast(&self) -> &Node {
        &self.inner
    }

    fn cast_mut(&mut self) -> &mut Node {
        &mut self.inner
    }
}
impl IntoChildNode for ChildNode {}

pub trait IntoChildNode: IntoNode {
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
        if let Some(parent) = &mut node.parent_node() {
            parent.replace_child(IntoNode::cast_mut(self), node);
        }
    }
}

pub struct NodeListOf<'a, TNode: IntoNode> {
    pub(crate) items: &'a Vec<TNode>,
}

pub struct MutNodeListOf<'a, TNode: IntoNode> {
    pub(crate) items: &'a mut Vec<TNode>,
}

pub trait NodeListOfTrait<TNode: IntoNode>: Index<usize> {
    /// Returns a reference to the inner items.
    fn items(&self) -> &Vec<TNode>;
    /// Returns the length of the list.
    fn len(&self) -> usize {
        self.items().len()
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    fn item(&self, index: usize) -> Option<&TNode> {
        self.items().get(index)
    }
    fn iter(&self) -> std::slice::Iter<'_, TNode> {
        self.items().iter()
    }
}
impl<TNode: IntoNode> Index<usize> for NodeListOf<'_, TNode> {
    type Output = TNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<'a, TNode: IntoNode> NodeListOfTrait<TNode> for NodeListOf<'a, TNode> {
    fn items(&self) -> &Vec<TNode> {
        self.items
    }
}
impl<TNode: IntoNode> Index<usize> for MutNodeListOf<'_, TNode> {
    type Output = TNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<'a, TNode: IntoNode> NodeListOfTrait<TNode> for MutNodeListOf<'a, TNode> {
    fn items(&self) -> &Vec<TNode> {
        self.items
    }
}
impl<'a, TNode: IntoNode> MutNodeListOf<'a, TNode> {
    pub fn item_mut(&mut self, index: usize) -> Option<&mut TNode> {
        self.items.get_mut(index)
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, TNode> {
        self.items.iter_mut()
    }
}

mod helpers {
    use crate::{ChildNode, IntoNode, Node, NodeBase, WeakNodeRef};

    pub fn get_mut_node_at_index<'a>(
        parentref: &WeakNodeRef,
        index: usize,
    ) -> Option<&'a mut ChildNode> {
        match parentref.inner.upgrade() {
            Some(parent_node_ref) => unsafe { &mut *parent_node_ref.as_ptr() }
                .children
                .get_mut(index),
            None => None,
        }
    }
    /// Return the child node at a particular index, if it exists.
    pub fn get_node_at_index<'a>(parentref: &WeakNodeRef, index: usize) -> Option<&'a ChildNode> {
        match parentref.inner.upgrade() {
            Some(parent_node_ref) => unsafe { &*parent_node_ref.as_ptr() }.children.get(index),
            None => None,
        }
    }

    /// Create a copy of a node still attached to the parent node.
    pub fn clone_node<T: IntoNode>(noderef: &T, deep: bool) -> Node {
        let inner_node = IntoNode::cast(noderef).inner.borrow();
        if deep {
            Node {
                inner: std::rc::Rc::new(std::cell::RefCell::new(NodeBase {
                    node_type: inner_node.node_type.clone(),
                    event_target: inner_node.event_target.clone(),
                    owner_document: inner_node.owner_document.clone(),
                    parent: inner_node.parent.clone(),
                    children: inner_node
                        .children
                        .as_slice()
                        .iter()
                        .map(|noderef| ChildNode {
                            inner: clone_node(noderef, deep),
                        })
                        .collect(),
                })),
            }
        } else {
            Node {
                inner: std::rc::Rc::new(std::cell::RefCell::new(inner_node.clone())),
            }
        }
    }

    /// Get the number of children a node has.
    pub fn get_children_length<T: IntoNode>(parent: &T) -> usize {
        IntoNode::cast(parent).inner.borrow().children.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntoNode, Node, NodeBase, NodeType};

    #[test]
    fn node_size() {
        let node = NodeBase::new();
        println!("{:?}", std::mem::size_of_val(&node));
    }

    #[test]
    fn parent_child_node_check() {
        let mut parent = Node::new();
        let mut child = Node::new();
        let mut grandchild = Node::new();
        parent.append_child(&mut child);
        parent
            .first_child_mut()
            .unwrap()
            .append_child(&mut grandchild);

        assert_eq!(parent.first_child().unwrap(), &child);
        assert_eq!(parent.last_child().unwrap(), &child);
        assert_eq!(child.parent_node().as_ref().unwrap(), &parent);

        assert_eq!(child.first_child().unwrap(), &grandchild);

        assert!(parent.contains(&grandchild));
    }

    #[test]
    fn sibling_node_check() {
        let mut parent = Node::new();
        let mut child1 = Node::new();
        let mut child2 = Node::new();
        let mut child3 = Node::new();
        parent.append_child(&mut child1);
        parent.append_child(&mut child2);
        parent.append_child(&mut child3);

        assert_eq!(child1.next_sibling().unwrap(), &child2);
        assert_eq!(child2.next_sibling().unwrap(), &child3);
        assert_eq!(child2.previous_sibling().unwrap(), &child1);
        assert_eq!(child3.previous_sibling().unwrap(), &child2);
    }

    #[test]
    fn equality_node_check() {
        let node1 = Node::new();
        let node2 = Node::new();
        let node1clone = node1.clone();

        assert!(node1.is_equal_node(&node2));
        assert!(node1.is_same_node(&node1clone));
        assert!(!node1.is_same_node(&node2));

        node1.inner.borrow_mut().node_type = NodeType::Element;

        assert!(!node1.is_equal_node(&node2));
    }
}
