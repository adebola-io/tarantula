use super::{HTMLElementRef, IntoEventTarget};
use crate::{
    DOMException, DocumentRef, EventTarget, HTMLCollection, MutHTMLCollection, WeakDocumentRef,
};
use std::{
    cell::RefCell,
    ops::{Index, IndexMut},
    rc::{Rc, Weak},
};

pub struct GetRootNodeOptions;

#[derive(Clone, PartialEq)]
pub(crate) struct NodeBase {
    pub node_type: u8,
    /// The inner event target.
    pub event_target: EventTarget,
    pub owner_document: Option<WeakDocumentRef>,
    /// Tuple containing the parent node and the index of this node in the parent's child list.
    pub parent: Option<(WeakNodeRef, usize)>,
    pub children: Vec<ChildNode>,
}
impl NodeBase {
    pub fn new() -> Self {
        Self {
            node_type: MISC_NODE,
            event_target: EventTarget::new(),
            parent: None,
            owner_document: None,
            children: vec![],
        }
    }
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

const MISC_NODE: u8 = 0;

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
impl PartialEq<ParentNode> for Node {
    fn eq(&self, other: &ParentNode) -> bool {
        self == &(*other).inner
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(NodeBase::new())),
        }
    }
    /// Returns the position of self in its parent, if it has one.
    fn index(&self) -> Option<usize> {
        self.inner.borrow().parent.as_ref().map(|tuple| tuple.1)
    }

    fn set_index(&mut self, index: usize) {
        self.inner
            .borrow_mut()
            .parent
            .as_mut()
            .map(|tuple| tuple.1 = index);
    }

    fn is_child_of<T: IntoNode>(&self, parent: &T) -> bool {
        IntoNode::cast(parent)
            .child_nodes()
            .iter()
            .find(|node| node.is_same_node(self))
            .is_some()
    }

    fn set_parent(&mut self, parent: Option<(WeakNodeRef, usize)>) {
        self.inner.borrow_mut().parent = parent;
    }

    fn is_appendable(&self) -> bool {
        let node_type = self.node_type();
        node_type == MISC_NODE
            || node_type == Self::ELEMENT_NODE
            || node_type == Self::DOCUMENT_NODE
            || node_type == Self::DOCUMENT_FRAGMENT_NODE
    }
}

impl IntoNode for Node {
    #[inline(always)]
    fn cast(&self) -> &Node {
        self
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut Node {
        self
    }
}

impl IntoEventTarget for Node {
    #[inline(always)]
    fn cast(&self) -> &EventTarget {
        unsafe { &(*self.inner.as_ptr()).event_target }
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut EventTarget {
        unsafe { &mut (*self.inner.as_ptr()).event_target }
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
        self.child_nodes().items.get(0)
    }
    /// Returns a mutable reference to the first child.
    fn first_child_mut(&mut self) -> Option<&mut ChildNode> {
        self.child_nodes_mut().items.get_mut(0)
    }
    /// Returns true if node is connected and false otherwise.
    fn is_connected(&self) -> bool {
        todo!()
    }
    /// Returns the last child.
    fn last_child(&self) -> Option<&ChildNode> {
        self.child_nodes().items.last()
    }
    /// Returns the last child mutably.
    fn last_child_mut(&mut self) -> Option<&mut ChildNode> {
        self.child_nodes_mut().items.last_mut()
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
    fn node_type(&self) -> u8 {
        IntoNode::cast(self).inner.borrow_mut().node_type
    }
    fn node_value(&self) -> Option<&str> {
        todo!()
    }
    fn set_node_value(&mut self, value: &str) {
        if !self.node_value().is_none() {
            todo!()
        }
    }
    /// Returns the node document. Returns None for documents.
    fn owner_document(&self) -> Option<DocumentRef> {
        if self.node_type() == Self::DOCUMENT_NODE {
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
        if let Some(tuple) = &IntoNode::cast(self).inner.borrow().parent {
            if tuple.1 == 0 {
                None
            } else {
                helpers::get_node_at_index(&tuple.0, tuple.1 - 1)
            }
        } else {
            None
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
    /// Adds a node to the end of the list of the child nodes of this node.
    /// It returns a reference to the node appended, or an error if the appending was unsuccessful.
    /// # Errors
    /// - Returns a [`HierarchyRequestError`] DOMException if the constraints of the node tree are violated.
    /// # Example
    /// ```rust
    /// use dom::{Node, IntoNode};
    ///
    /// let mut node = Node::new();
    /// let mut child = Node::new();
    /// node.append_child(&mut child).unwrap();
    ///
    /// assert_eq!(node.first_child().unwrap(), &child)
    ///
    /// ```
    fn append_child<'a, T: IntoNode>(&mut self, child: &'a mut T) -> Result<&'a T, DOMException> {
        helpers::validate_hierarchy(self, child)?;

        if child.node_type() == Self::DOCUMENT_FRAGMENT_NODE {
            for subchild in child.child_nodes_mut() {
                self.append_child(subchild)?;
            }
        } else {
            let mut childnode = ChildNode::from(&*child);
            // Disconnect from former parent.
            childnode.remove();
            let weak_reference = WeakNodeRef::from(&*self);
            let index = helpers::get_children_length(self);
            childnode.inner.set_parent(Some((weak_reference, index)));
            self.child_nodes_mut().items.push(childnode);
        }
        Ok(child)
    }
    /// Returns a duplicate of node.
    /// If deep is true, the node's descendants are also cloned.
    /// Event listeners are not cloned.
    /// # Example
    /// ```
    /// use dom::{Node, IntoNode};
    ///
    /// let mut node = Node::new();
    /// assert!(!node.is_same_node(&node.clone_node(false)));
    /// ```
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
    /// # Example
    /// ```
    /// use dom::{Node, IntoNode};
    ///
    /// let mut node1 = Node::new();
    /// let mut node2 = Node::new();
    /// node1.append_child(&mut node2);
    ///
    /// assert!(node1.contains(&node2));
    /// assert!(node1.contains(&node1)); // Nodes can contain themselves.
    /// ```
    fn contains<T: IntoNode>(&self, other: &T) -> bool {
        if self.is_same_node(other) {
            return true;
        }
        for child in self.child_nodes() {
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
    /// Returns boolean value indicating whether node has children.
    /// ```
    /// use dom::{Node, IntoNode};
    ///
    /// let mut node = Node::new();
    /// assert!(!node.has_child_nodes());
    /// let mut child = Node::new();
    /// node.append_child(&mut child);
    /// assert!(node.has_child_nodes());
    /// ```
    fn has_child_nodes(&self) -> bool {
        self.child_nodes().len() > 0
    }
    fn insert_before<'a, T: IntoNode, U: IntoNode>(
        &mut self,
        node: &'a mut T,
        child: Option<&mut U>,
    ) -> Result<&'a T, DOMException> {
        todo!()
    }
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        todo!()
    }
    /// Returns whether node and otherNode have the same properties.
    fn is_equal_node<T: IntoNode>(&self, other_node: &T) -> bool {
        let inner_node = IntoNode::cast(self).inner.borrow();
        let other_inner_node = IntoNode::cast(other_node).inner.borrow();
        *inner_node == *other_inner_node
    }
    fn is_same_node<T: IntoNode>(&self, other_node: &T) -> bool {
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
    /// Removes a child node and returns the removed node. It does nothing if the node to remove is not a child of this node.
    fn remove_child<'a, T: IntoNode>(&mut self, node: &'a mut T) -> &'a mut T {
        if let Some(parent) = node.parent_node() {
            if parent.is_same_node(IntoNode::cast(self)) {
                let children = self.child_nodes_mut().items;
                let mut index = IntoNode::cast(node).index().unwrap();
                children.remove(index);
                // Shift indexes.
                while index < children.len() {
                    IntoNode::cast_mut(&mut children[index]).set_index(index - 1);
                    index += 1;
                }
                // Remove parent pointer.
                IntoNode::cast_mut(node).inner.borrow_mut().parent = None;
            }
        };
        node
    }
    fn replace_child<T: IntoNode, U: IntoNode>(&mut self, node: &mut U, child: &mut T) -> &T {
        todo!()
    }
    const ELEMENT_NODE: u8 = 1;
    const ATTRIBUTE_NODE: u8 = 2;
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
    const DOCUMENT_POSITION_FOLLOWING: u8 = 0x04;
    /// Set when other is an ancestor of node.
    const DOCUMENT_POSITION_CONTAINS: u8 = 0x08;
    /// Set when other is a descendant of node.
    const DOCUMENT_POSITION_CONTAINED_BY: u8 = 0x10;
    const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u8 = 0x20;
}

// PARENT NODE.

#[derive(Debug, Clone)]
pub struct ParentNode {
    inner: Node,
}
impl<T: IntoNode> PartialEq<T> for ParentNode {
    fn eq(&self, other: &T) -> bool {
        &self.inner == IntoNode::cast(other)
    }
}
impl IntoEventTarget for ParentNode {
    #[inline(always)]
    fn cast(&self) -> &EventTarget {
        IntoEventTarget::cast(&self.inner)
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut EventTarget {
        IntoEventTarget::cast_mut(&mut self.inner)
    }
}
impl IntoNode for ParentNode {
    #[inline(always)]
    fn cast(&self) -> &Node {
        &self.inner
    }
    #[inline(always)]
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
    fn append<'a, T: 'a + IntoNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        self.append_child(node.into())?;
        Ok(())
    }
    /// Inserts nodes before the first child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn prepend<'a, T: 'a + IntoNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        let node = IntoNode::cast_mut(node.into());
        helpers::validate_hierarchy(self, node)?;
        if node.node_type() == Self::DOCUMENT_FRAGMENT_NODE {
            for child in node.child_nodes_mut().iter_mut().rev() {
                self.prepend(child)?;
            }
            Ok(())
        } else {
            let mut self_node = IntoNode::cast(self).inner.borrow_mut();
            let child_node = ChildNode {
                inner: node.clone(),
            };
            let weak_reference = WeakNodeRef::from(&*self);
            node.set_parent(Some((weak_reference, 0)));
            // TODO: Shift all indexes and insert node with one pass.
            // Shift all indexes.
            self_node
                .children
                .iter_mut()
                .enumerate()
                .for_each(|(index, child)| child.inner.set_index(index + 1));
            self_node.children.insert(0, child_node);
            Ok(())
        }
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
    fn replace_children<T: IntoNode>(&mut self, nodes: Vec<T>) {
        todo!()
    }
}

// CHILD NODE.

#[derive(Debug, Clone)]
pub struct ChildNode {
    inner: Node,
}
impl<T: IntoNode> PartialEq<T> for ChildNode {
    fn eq(&self, other: &T) -> bool {
        &self.inner == IntoNode::cast(other)
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
impl<T: IntoNode> From<&T> for ChildNode {
    fn from(node: &T) -> Self {
        ChildNode {
            inner: IntoNode::cast(node).clone(),
        }
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
    fn after<'a, T: 'a + IntoNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        let node = IntoNode::cast_mut(node.into());
        helpers::validate_hierarchy(self, node)?;
        if let Some(mut parent) = self.parent_node() {
            let mut index = IntoNode::cast(self).index().unwrap() + 1;
            node.set_parent(Some((WeakNodeRef::from(&parent), index)));
            let child_nodes = parent.child_nodes_mut().items;
            // Insert node.
            child_nodes.insert(index, ChildNode::from(&*node));
            // Shift succeding indexes.
            loop {
                index += 1;
                child_nodes[index].inner.set_index(index + 1);
                if index + 1 == child_nodes.len() {
                    break;
                }
            }
        }
        Ok(())
    }
    /// Inserts nodes just before node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn before(&mut self, node: &mut Node) -> Result<(), DOMException> {
        if let Some(previous) = self.previous_sibling_mut() {
            previous.after(node)
        } else {
            if let Some(mut parent) = self.parent_node() {
                parent.prepend(node)
            } else {
                Ok(())
            }
        }
    }
    /// Removes node.
    fn remove(&mut self) {
        if let Some(mut parent) = self.parent_node() {
            parent.remove_child(self);
        }
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

// NODE LIST.

pub struct NodeListOf<'a, TNode: IntoNode> {
    items: &'a Vec<TNode>,
}

pub struct MutNodeListOf<'a, TNode: IntoNode> {
    items: &'a mut Vec<TNode>,
}

impl<TNode: IntoNode> Index<usize> for NodeListOf<'_, TNode> {
    type Output = TNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<'a, TNode: IntoNode> IntoIterator for NodeListOf<'a, TNode> {
    type Item = &'a TNode;

    type IntoIter = std::slice::Iter<'a, TNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
impl<'a, TNode: IntoNode> NodeListOf<'a, TNode> {
    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    pub fn item(&self, index: usize) -> Option<&TNode> {
        self.items.get(index)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, TNode> {
        self.items.iter()
    }
}
impl<TNode: IntoNode> Index<usize> for MutNodeListOf<'_, TNode> {
    type Output = TNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<TNode: IntoNode> IndexMut<usize> for MutNodeListOf<'_, TNode> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
impl<'a, TNode: IntoNode> MutNodeListOf<'a, TNode> {
    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    pub fn item(&self, index: usize) -> Option<&TNode> {
        self.items.get(index)
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    pub fn item_mut(&mut self, index: usize) -> Option<&mut TNode> {
        self.items.get_mut(index)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, TNode> {
        self.items.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, TNode> {
        self.items.iter_mut()
    }
}
impl<'a, TNode: IntoNode> IntoIterator for MutNodeListOf<'a, TNode> {
    type Item = &'a mut TNode;

    type IntoIter = std::slice::IterMut<'a, TNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

mod helpers {
    use crate::{ChildNode, DOMException, IntoNode, Node, NodeBase, WeakNodeRef};

    pub fn validate_hierarchy<T: IntoNode, U: IntoNode>(
        parent: &T,
        child: &U,
    ) -> Result<(), DOMException> {
        let parent_is_document = parent.node_type() == Node::DOCUMENT_NODE;
        if !IntoNode::cast(parent).is_appendable() {
            Err(DOMException::HierarchyRequestError(
                "Self is not a Document, DocumentFragment or Element.",
            ))
        } else if child.contains(parent) {
            Err(DOMException::HierarchyRequestError(
                "Appending child will lead to DOM cycle.",
            ))
        } else if (child.node_type() == Node::TEXT_NODE) && parent_is_document {
            Err(DOMException::HierarchyRequestError(
                "Nodes of type '#text' may not be inserted inside nodes of type '#document'.",
            ))
        } else if (child.node_type() == Node::DOCUMENT_TYPE_NODE) && !parent_is_document {
            Err(DOMException::HierarchyRequestError(
                "DocumentType must always be direct descendant of Document",
            ))
        } else if child.node_type() == Node::DOCUMENT_FRAGMENT_NODE {
            let mut element_count = 0;
            for subchild in child.child_nodes() {
                if subchild.node_type() == Node::ELEMENT_NODE {
                    element_count += 1;
                    if element_count > 1 && parent_is_document {
                        return Err(DOMException::HierarchyRequestError("Nodes of type '#document-fragment' may not be inserted inside nodes of type '#document'"));
                    }
                } else if (subchild.node_type() == Node::TEXT_NODE) && parent_is_document {
                    return Err(DOMException::HierarchyRequestError("Nodes of type '#document-fragment' may not be inserted inside nodes of type '#document'"));
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }

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
                    event_target: crate::EventTarget::new(),
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
    use crate::{IntoNode, Node, NodeBase};

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
        parent.append_child(&mut child).unwrap();
        parent
            .first_child_mut()
            .unwrap()
            .append_child(&mut grandchild)
            .unwrap();

        assert_eq!(parent.first_child().unwrap(), &child);
        assert_eq!(parent.last_child().unwrap(), &child);
        assert_eq!(child.parent_node().as_ref().unwrap(), &parent);

        assert_eq!(child.first_child().unwrap(), &grandchild);

        assert!(parent.contains(&grandchild));

        parent.remove_child(&mut child);
        assert_eq!(child.parent_node(), None);
        assert_eq!(parent.child_nodes().len(), 0);
    }

    #[test]
    fn sibling_node_check() {
        let mut parent = Node::new();
        let mut child1 = Node::new();
        let mut child2 = Node::new();
        let mut child3 = Node::new();
        parent.append_child(&mut child1).unwrap();
        parent.append_child(&mut child2).unwrap();
        parent.append_child(&mut child3).unwrap();

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

        node1.inner.borrow_mut().node_type = 1;

        assert!(!node1.is_equal_node(&node2));
    }
}
