use crate::{
    document::WeakDocumentRef, AsElement, AsEventTarget, DOMException, Document, EventTarget,
    HTMLCollection, HTMLElement, MutNodeListOf, NodeListOf,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct GetRootNodeOptions;

#[derive(PartialEq)]
pub(crate) struct NodeBase {
    pub node_type: u8,
    /// The inner event target.
    pub event_target: EventTarget,
    pub owner_document: Option<WeakDocumentRef>,
    /// Tuple containing the parent node and the index of this node in the parent's child list.
    pub parent: Option<(WeakNodeRef, usize)>,
    pub children: Vec<ChildNode>,
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
impl<T: AsNode> From<&T> for WeakNodeRef {
    fn from(node: &T) -> Self {
        WeakNodeRef {
            inner: Rc::downgrade(&AsNode::cast(node).inner),
        }
    }
}

/// Node is an interface from which a number of DOM API object types inherit. It allows those types to be treated similarly; for example, inheriting the same set of methods, or being tested in the same way.
#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) inner: Rc<RefCell<NodeBase>>,
}

impl<T: AsNode> PartialEq<T> for Node {
    fn eq(&self, other: &T) -> bool {
        Rc::ptr_eq(&self.inner, &AsNode::cast(other).inner)
    }
}

impl Node {
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

    fn is_child_of(&self, parent: &impl AsNode) -> bool {
        for child in parent.child_nodes() {
            if child.is_same_node(self) {
                return true;
            }
        }
        false
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
    /// Create a node inside a document.
    pub(crate) fn in_document(node_type: u8, weak_ref: WeakDocumentRef) -> Self {
        Self {
            inner: Rc::new(RefCell::new(NodeBase {
                node_type,
                event_target: EventTarget::new(),
                owner_document: Some(weak_ref),
                parent: None,
                children: vec![],
            })),
        }
    }
    /// Returns a raw pointer to the underlying node base.
    pub(crate) fn get_base_ptr(&self) -> *mut NodeBase {
        self.inner.as_ptr()
    }

    /// Inner implementation of `remove()`.
    fn __remove(&mut self) {
        if let Some(mut parent) = self.parent_node() {
            AsNode::cast_mut(&mut parent).__remove_child(self).unwrap();
        }
    }

    /// Inner implementation of `insert_before()`.
    fn __insert_before<'a, T: AsNode>(
        &mut self,
        new_child: &'a mut T,
        reference_node: Option<&mut impl AsNode>,
    ) -> Result<&'a T, DOMException> {
        let mut index = match reference_node {
            Some(reference_node) => {
                if AsNode::cast(reference_node).is_child_of(self) {
                    AsNode::cast(reference_node).index().unwrap()
                } else {
                    return Err(DOMException::HierarchyRequestError(
                        "Reference node is not a child of this node",
                    ));
                }
            }
            None => 0,
        };
        helpers::validate_hierarchy(self, new_child)?;
        if new_child.node_type() == Self::DOCUMENT_FRAGMENT_NODE {
            let children = new_child.child_nodes_mut();
            while !children.items.is_empty() {
                let reference_node = unsafe { &mut *(self.inner.as_ptr()) }
                    .children
                    .get_mut(index);
                self.__insert_before(&mut children.items.remove(0), reference_node)?;
                index += 1;
            }
            return Ok(new_child);
        }

        let mut child = ChildNode::from(&*new_child);
        // Disconnect from former parent.
        AsNode::cast_mut(&mut child).__remove();

        AsNode::cast_mut(&mut child).set_parent(Some((WeakNodeRef::from(&*self), index)));
        let children = self.child_nodes_mut().items;
        children.insert(index, child);

        // Shift all following indexes.
        loop {
            index += 1;
            AsNode::cast_mut(&mut children[index]).set_index(index);
            if index + 1 == children.len() {
                break;
            }
        }
        Ok(new_child)
    }

    /// Inner implementation of `remove_child()`.
    fn __remove_child<'a, T: AsNode>(
        &mut self,
        node: &'a mut T,
    ) -> Result<&'a mut T, DOMException> {
        if !AsNode::cast(node).is_child_of(self) {
            return Err(DOMException::HierarchyRequestError(
                "Node to remove is not a child of this node.",
            ));
        }
        let children = self.child_nodes_mut().items;
        let node_ref = AsNode::cast_mut(node);
        let mut index = node_ref.index().unwrap();
        children.remove(index);
        // Shift indexes.
        while index < children.len() {
            AsNode::cast_mut(&mut children[index]).set_index(index);
            index += 1;
        }
        // Remove parent pointer.
        node_ref.set_parent(None);
        Ok(node)
    }

    /// Inner implementation of `replace_child()`.
    fn __replace_child<'a, T: AsNode>(
        &mut self,
        new_child: &mut impl AsNode,
        old_child: &'a mut T,
    ) -> Result<&'a mut T, DOMException> {
        let old_child_as_node = AsNode::cast_mut(old_child);
        if !old_child_as_node.is_child_of(self) {
            return Err(DOMException::HierarchyRequestError(
                "Node to be replaced is not a child of this node.",
            ));
        }

        helpers::validate_hierarchy(self, new_child)?;

        if new_child.node_type() == Self::DOCUMENT_FRAGMENT_NODE {
            let children = new_child.child_nodes_mut();
            let mut i = 0;
            while i < children.items.len() {
                AsNode::cast_mut(self)
                    .__insert_before(&mut children.items.remove(i), Some(old_child))?;
                i += 1;
            }
            self.remove_child(old_child).unwrap();
            return Ok(old_child);
        }

        let index = old_child_as_node.index().unwrap();
        let mut new_child = ChildNode::from(&*new_child);
        // Disconnect from old parent.
        AsNode::cast_mut(&mut new_child).__remove();

        AsNode::cast_mut(&mut new_child)
            .set_parent(old_child_as_node.inner.borrow_mut().parent.take());
        self.child_nodes_mut().items[index] = new_child;

        Ok(old_child)
    }

    /// Refresh the DOM.
    fn update_document(&self) {
        // Refresh DOM.
        if let Some(document) = self.owner_document() {
            document
                .inner
                .borrow_mut()
                .refresh(&document.lookup_node(self.get_base_ptr()))
        }
    }

    /// Inner implementation of `after()`.
    fn __append_child<'a, T: AsNode>(&mut self, child: &'a mut T) -> Result<&'a T, DOMException> {
        helpers::validate_hierarchy(self, child)?;

        if child.node_type() == Self::DOCUMENT_FRAGMENT_NODE {
            for subchild in child.child_nodes_mut() {
                self.__append_child(subchild)?;
            }
            child.child_nodes_mut().items.clear();
        } else {
            let mut childnode = ChildNode::from(&*child);
            // Disconnect from former parent.
            AsNode::cast_mut(&mut childnode).__remove();
            let weak_reference = WeakNodeRef::from(&*self);
            let index = helpers::get_children_length(self);
            childnode.inner.set_parent(Some((weak_reference, index)));
            self.child_nodes_mut().items.push(childnode);
        }
        Ok(child)
    }
}

impl AsNode for Node {
    #[inline(always)]
    fn cast(&self) -> &Node {
        self
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut Node {
        self
    }
    fn clone_node(&self, deep: bool) -> Self {
        let noderef = helpers::clone_node(AsNode::cast(self), deep);
        noderef.inner.borrow_mut().parent = None;
        noderef
    }

    fn node_name(&self) -> String {
        let node_type = self.node_type();
        String::from(if node_type == Self::DOCUMENT_NODE {
            "#document"
        } else if node_type == Self::CDATA_SECTION_NODE {
            "#cdata-section"
        } else if node_type == Self::COMMENT_NODE {
            "#comment"
        } else if node_type == Self::DOCUMENT_FRAGMENT_NODE {
            "#document-fragment"
        } else if node_type == Self::TEXT_NODE {
            "#text"
        } else {
            unimplemented!("Node name should be implemented by Node-inherited structs.")
        })
    }
}

impl AsEventTarget for Node {
    #[inline(always)]
    fn cast(&self) -> &EventTarget {
        unsafe { &(*self.inner.as_ptr()).event_target }
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut EventTarget {
        unsafe { &mut (*self.inner.as_ptr()).event_target }
    }
}

pub trait AsNode: AsEventTarget {
    #[doc(hidden)]
    /// Convert to a reference to [`Node`].
    fn cast(&self) -> &Node;
    #[doc(hidden)]
    /// Convert to a mutable reference to [`Node`].
    fn cast_mut(&mut self) -> &mut Node;
    /// Returns a string slice representing the base URL of the document containing this node.
    ///
    /// MDN Reference: [`Node.baseURI`](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)
    fn base_uri(&self) -> &str {
        todo!()
    }
    /// Returns a live [`NodeList`] containing all the children of this node (including elements, text and comments).
    /// NodeList being live means that if the children of the Node change, the NodeList object is automatically updated.
    ///
    /// MDN Reference: [`Node.childNodes`](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)
    ///
    /// [`NodeList`]: crate::NodeList
    fn child_nodes(&self) -> NodeListOf<'_, ChildNode> {
        NodeListOf {
            items: unsafe { &(*AsNode::cast(self).inner.as_ptr()).children },
        }
    }
    /// Returns a [`MutNodeList`] containing all the children of this node (including elements, text and comments).
    /// NodeList being live means that if the children of the Node change, the NodeList object is automatically updated.
    ///
    /// MDN Reference: [`Node.childNodes`](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)
    ///
    /// [`MutNodeList`]: crate::MutNodeList
    fn child_nodes_mut(&mut self) -> MutNodeListOf<'_, ChildNode> {
        MutNodeListOf {
            items: unsafe { &mut (*AsNode::cast(self).inner.as_ptr()).children },
        }
    }
    /// Returns a reference to a [`ChildNode`] representing the first direct child node of the node, or None if the node has no child.
    ///
    /// MDN Reference: [`Node.firstChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)
    fn first_child(&self) -> Option<&ChildNode> {
        self.child_nodes().items.get(0)
    }
    /// Returns a mutable reference to a [`ChildNode`] representing the first direct child node of the node, or None if the node has no child.
    ///
    /// MDN Reference: [`Node.firstChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)
    fn first_child_mut(&mut self) -> Option<&mut ChildNode> {
        self.child_nodes_mut().items.get_mut(0)
    }
    /// Returns a boolean indicating whether or not the Node is connected (directly or indirectly) to the context object, e.g. the [`Document`] object in the case of the normal DOM, or the ShadowRoot in the case of a shadow DOM.
    ///
    /// MDN Reference: [`Node.isConnected`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected)
    fn is_connected(&self) -> bool {
        todo!()
    }
    /// Returns a reference to a [`ChildNode`] representing the last direct child node of the node, or None if the node has no child.
    ///
    /// MDN Reference: [`Node.lastChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/lasthild)
    fn last_child(&self) -> Option<&ChildNode> {
        self.child_nodes().items.last()
    }
    /// Returns a mutable reference to a [`ChildNode`] representing the last direct child node of the node, or None if the node has no child.
    ///
    /// MDN Reference: [`Node.lastChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/lasthild)
    fn last_child_mut(&mut self) -> Option<&mut ChildNode> {
        self.child_nodes_mut().items.last_mut()
    }
    /// Returns a reference to a [`ChildNode`] representing the next node in the tree, or None if there isn't such node.
    ///
    /// MDN Reference: [`Node.nextSibing`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)
    fn next_sibling(&self) -> Option<&ChildNode> {
        match &AsNode::cast(self).inner.borrow().parent {
            Some(tuple) => helpers::get_node_at_index(&tuple.0, tuple.1 + 1),
            _ => None,
        }
    }
    /// Returns a mutable reference to a [`ChildNode`] representing the next node in the tree, or None if there isn't such node.
    ///
    /// MDN Reference: [`Node.nextSibing`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)
    fn next_sibling_mut(&mut self) -> Option<&mut ChildNode> {
        match &AsNode::cast(self).inner.borrow().parent {
            Some(tuple) => helpers::get_mut_node_at_index(&tuple.0, tuple.1 + 1),
            _ => None,
        }
    }
    /// Returns a string containing the name of the Node.
    ///
    /// The structure of the name will differ with the node type. E.g. An [`HTMLElement`] will contain the name of the corresponding tag, like `"audio"` for an [`HTMLAudioElement`], a [`Text`] node will have the `"#text"` string, or a [`Document`] node will have the `"#document"` string.
    ///
    /// MDN Reference: [`Node.nodeName`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    fn node_name(&self) -> String;
    /// Returns an `u8` representing the type of the node. Possible values are:
    ///
    /// | Name | Value |
    /// |------|-----|
    /// | ELEMENT_NODE | 1 |
    /// | ATTRIBUTE_NODE | 2 |
    /// | TEXT_NODE | 3 |
    /// | CDATA_SECTION_NODE | 4 |
    /// | PROCESSING_INSTRUCTION_NODE | 7 |
    /// | COMMENT_NODE | 8 |
    /// | DOCUMENT_NODE | 9 |
    /// | DOCUMENT_TYPE_NODE | 10 |
    /// | DOCUMENT_FRAGMENT_NODE | 11 |
    ///
    /// MDN Reference: [`Node.nodeType`](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType).
    fn node_type(&self) -> u8 {
        AsNode::cast(self).inner.borrow_mut().node_type
    }
    /// Returns the value of the current node.
    ///
    /// MDN Reference: [Node.nodeValue]{https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    fn node_value(&self) -> Option<&str> {
        let node_type = self.node_type();
        if node_type == Self::DOCUMENT_NODE
            || node_type == Self::DOCUMENT_FRAGMENT_NODE
            || node_type == Self::DOCUMENT_TYPE_NODE
            || node_type == Self::ELEMENT_NODE
        {
            None
        } else {
            unimplemented!("node value should be implemented by subtraits.")
        }
    }
    /// Sets the value of the current node.
    ///
    /// MDN Reference: [Node.nodeValue]{https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)
    fn set_node_value(&mut self, value: &str) {
        if !self.node_value().is_none() {
            todo!()
        }
    }
    /// Returns the node document. Returns None for documents.
    fn owner_document(&self) -> Option<Document> {
        if self.node_type() == Self::DOCUMENT_NODE {
            return None;
        }
        if let Some(weak_document_ref) = &AsNode::cast(self).inner.borrow().owner_document {
            weak_document_ref
                .inner
                .upgrade()
                .map(|inner| Document { inner })
        } else {
            None
        }
    }
    /// Returns the parent element.
    fn parent_element(&self) -> Option<&HTMLElement> {
        todo!()
    }
    /// Returns the parent element mutably.
    fn parent_element_mut(&mut self) -> Option<&mut HTMLElement> {
        todo!()
    }
    /// Returns the parent.
    fn parent_node(&self) -> Option<ParentNode> {
        match &AsNode::cast(self).inner.borrow().parent {
            Some(tuple) => tuple.0.inner.upgrade().map(|inner| ParentNode {
                inner: Node { inner },
            }),
            _ => None,
        }
    }
    /// Returns the previous sibling.
    fn previous_sibling(&self) -> Option<&ChildNode> {
        if let Some(tuple) = &AsNode::cast(self).inner.borrow().parent {
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
        match &AsNode::cast(self).inner.borrow().parent {
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
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let mut node = document.create_element("div");
    /// let mut child = document.create_element("span");
    /// node.append_child(&mut child).unwrap();
    ///
    /// assert_eq!(node.first_child().unwrap(), &child)
    ///
    /// ```
    fn append_child<'a, T: AsNode>(&mut self, child: &'a mut T) -> Result<&'a T, DOMException> {
        let this = AsNode::cast_mut(self);
        let result = this.__append_child(child)?;
        this.update_document();
        Ok(result)
    }
    /// Returns a duplicate of node.
    /// If deep is true, the node's descendants are also cloned.
    /// Event listeners are not cloned.
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let node = document.create_element("div");
    /// assert!(!node.is_same_node(&node.clone_node(false)));
    /// ```
    fn clone_node(&self, deep: bool) -> Self;
    /// Returns a bitmask indicating the position of other relative to node.
    fn compare_document_position(&self) -> u8 {
        todo!()
    }
    /// Returns true if other is an inclusive descendant of node, and false otherwise.
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let mut node1 = document.create_element("div");
    /// let mut node2 = document.create_element("div");
    /// node1.append_child(&mut node2);
    ///
    /// assert!(node1.contains(&node2));
    /// assert!(node1.contains(&node1)); // Nodes can contain themselves.
    /// ```
    fn contains(&self, other: &impl AsNode) -> bool {
        if self.is_same_node(other) {
            return true;
        }
        for child in self.child_nodes() {
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
    ///
    /// MDN Reference: [`Node.hasChildNodes()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let mut node = document.create_element("div");
    /// assert!(!node.has_child_nodes());
    /// let mut child = document.create_element("div");
    /// node.append_child(&mut child);
    /// assert!(node.has_child_nodes());
    /// ```
    fn has_child_nodes(&self) -> bool {
        self.child_nodes().len() > 0
    }
    /// Inserts a Node before the reference node as a child of a specified parent node.
    ///
    /// MDN Reference: [`Node.insertBefore()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)
    /// # Errors
    /// - Returns a [`HierarchyRequestError`] DOMException if the constraints of the node tree are violated.
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let mut parent = document.create_element("div");
    /// let mut reference_child = document.create_element("div");
    /// parent.append_child(&mut reference_child).unwrap();
    ///
    /// let mut new_node = document.create_element("div");
    /// parent.insert_before(&mut new_node, Some(&mut reference_child)).unwrap();
    ///
    /// assert_eq!(parent.first_child().unwrap(), &new_node);
    /// assert_eq!(parent.last_child().unwrap(), &reference_child);
    /// assert_eq!(new_node.next_sibling().unwrap(), &reference_child);
    /// assert_eq!(reference_child.previous_sibling().unwrap(), &new_node);
    /// ```
    fn insert_before<'a, T: AsNode>(
        &mut self,
        new_child: &'a mut T,
        reference_node: Option<&mut impl AsNode>,
    ) -> Result<&'a T, DOMException> {
        let this = AsNode::cast_mut(self);
        let result = this.__insert_before(new_child, reference_node)?;
        this.update_document();
        Ok(result)
    }
    /// Accepts a namespace URI as an argument and returns a boolean value with a value of true if the namespace is the default namespace on the given node or false if not.
    ///
    /// MDN Reference: [`Node.isDefaultNamespace()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)
    fn is_default_namespace(&self, namespace: Option<&str>) -> bool {
        todo!()
    }
    /// Returns a boolean value which indicates whether or not two nodes are of the same type and all their defining data points match.
    ///
    /// MDN Reference: [`Node.isEqualNode()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let node1 = document.create_element("div");
    /// let node2 = document.create_element("div");
    ///
    /// assert!(node1.is_equal_node(&node2));
    /// ```
    fn is_equal_node(&self, other_node: &impl AsNode) -> bool {
        let inner_node = AsNode::cast(self).inner.borrow();
        let other_inner_node = AsNode::cast(other_node).inner.borrow();
        *inner_node == *other_inner_node
    }
    /// Returns a boolean value indicating whether or not the two nodes are the same (that is, they reference the same object).
    ///
    /// MDN Reference: [`Node.isSameNode()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let mut parent = document.create_element("div");
    /// let mut child = document.create_element("div");
    /// parent.append_child(&mut child);
    ///
    /// assert!(parent.first_child().unwrap().is_same_node(&child));
    /// ```
    fn is_same_node(&self, other_node: &impl AsNode) -> bool {
        AsNode::cast(self) == AsNode::cast(other_node)
    }
    /// Accepts a prefix and returns the namespace URI associated with it on the given node if found (and None if not). Supplying None for the prefix will return the default namespace.
    ///
    /// MDN Reference: [`Node.lookupNamespaceURI()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)
    fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<&str> {
        todo!()
    }
    /// Returns a string containing the prefix for a given namespace URI, if present, and None if not. When multiple prefixes are possible, the result is implementation-dependent.
    ///
    /// MDN Reference: [`Node.lookupPrefix()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)
    fn lookup_prefix(&self, namespace: Option<&str>) -> Option<&str> {
        todo!()
    }
    /// Removes empty exclusive Text nodes and concatenates the data of remaining contiguous exclusive Text nodes As the first of their nodes.
    fn normalize(&mut self) {
        todo!()
    }
    /// Removes a child node and returns the removed node.
    ///
    /// MDN Reference: [`Node.removeChild`](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)
    /// # Errors
    /// Returns an error if the node to remove is not a child of this node.
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let mut parent = document.create_element("div");
    /// let mut child = document.create_element("div");
    /// parent.append_child(&mut child).unwrap();
    ///
    /// assert!(parent.first_child().unwrap().is_same_node(&child));
    ///
    /// parent.remove_child(&mut child).unwrap();
    ///
    /// assert!(!parent.has_child_nodes());
    /// assert!(child.parent_node().is_none());
    /// ```
    fn remove_child<'a, T: AsNode>(&mut self, node: &'a mut T) -> Result<&'a mut T, DOMException> {
        let this = AsNode::cast_mut(self);
        let result = this.__remove_child(node)?;
        this.update_document();
        Ok(result)
    }
    /// Replaces one child Node of the current one with the second one given in parameter.
    ///
    /// MDN Reference: [`Node.replaceChild()`](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)
    /// # Errors
    /// Returns an error if the node to replace is not a child of this node, or the addtion of the new node violates the constraints of the node tree.
    /// # Example
    /// ```
    /// use dom::{Document, AsNode};
    ///
    /// let document = Document::new();
    /// let mut parent = document.create_element("div");
    /// let mut child1 = document.create_element("div");
    /// parent.append_child(&mut child1).unwrap();
    ///
    /// let mut child2 = document.create_element("div");
    /// parent.replace_child(&mut child2, &mut child1).unwrap();
    ///
    /// assert!(child1.parent_node().is_none());
    /// assert!(child2.parent_node().unwrap().is_same_node(&parent));
    /// ```
    fn replace_child<'a, T: AsNode>(
        &mut self,
        new_child: &mut impl AsNode,
        old_child: &'a mut T,
    ) -> Result<&'a mut T, DOMException> {
        let this = AsNode::cast_mut(self);
        let result = this.__replace_child(new_child, old_child)?;
        this.update_document();
        Ok(result)
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
impl<T: AsNode> PartialEq<T> for ParentNode {
    fn eq(&self, other: &T) -> bool {
        &self.inner == AsNode::cast(other)
    }
}
impl AsEventTarget for ParentNode {
    #[inline(always)]
    fn cast(&self) -> &EventTarget {
        AsEventTarget::cast(&self.inner)
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut EventTarget {
        AsEventTarget::cast_mut(&mut self.inner)
    }
}
impl AsNode for ParentNode {
    #[inline(always)]
    fn cast(&self) -> &Node {
        &self.inner
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut Node {
        &mut self.inner
    }
    fn clone_node(&self, deep: bool) -> Self {
        ParentNode {
            inner: self.inner.clone_node(deep),
        }
    }

    fn node_name(&self) -> String {
        AsNode::cast(self).node_name()
    }
}
impl AsParentNode for ParentNode {}

pub trait AsParentNode: AsNode {
    fn child_element_count(&self) -> usize {
        todo!()
    }
    fn children(&self) -> HTMLCollection {
        HTMLCollection {
            items: unsafe { &*AsNode::cast(self).inner.as_ptr() }
                .children
                .iter()
                .map(|child_node| {
                    let node_base = child_node.inner.inner.as_ptr();
                    self.owner_document().unwrap().lookup_node(node_base)
                })
                .collect(),
        }
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
    fn append<'a, T: 'a + AsNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        let node = AsNode::cast_mut(node.into());
        self.append_child(node)?;
        // DOM updates are triggered already.|
        Ok(())
    }
    /// Inserts nodes before the first child of node, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn prepend<'a, T: 'a + AsNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        let node = AsNode::cast_mut(node.into());
        let none: Option<&mut Node> = None;
        AsNode::cast_mut(self).insert_before(node, none)?;
        // DOM updates are triggered already.|
        Ok(())
    }
    /// Traverse tree and find the first element that matches a selector, if it exists.
    fn query_selector<T: AsElement>(&self, selector: &str) -> Option<T> {
        unimplemented!()
    }
    /// Traverse tree and find all the elements that matches a selector.
    fn query_selector_all<T: AsElement>(&self, selector: &str) -> Option<T> {
        unimplemented!()
    }
    // /// Replace all children of node with nodes, while replacing strings in nodes with equivalent Text nodes.
    // /// # Panics
    // /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_children(&mut self, nodes: Vec<impl AsNode>) {
        todo!()
    }
}

// CHILD NODE.

#[derive(Debug)]
pub struct ChildNode {
    inner: Node,
}
impl<T: AsNode> PartialEq<T> for ChildNode {
    fn eq(&self, other: &T) -> bool {
        &self.inner == AsNode::cast(other)
    }
}
impl AsEventTarget for ChildNode {
    fn cast(&self) -> &EventTarget {
        AsEventTarget::cast(&self.inner)
    }

    fn cast_mut(&mut self) -> &mut EventTarget {
        AsEventTarget::cast_mut(&mut self.inner)
    }
}
impl<T: AsNode> From<&T> for ChildNode {
    fn from(node: &T) -> Self {
        ChildNode {
            inner: AsNode::cast(node).clone(),
        }
    }
}
impl AsNode for ChildNode {
    fn cast(&self) -> &Node {
        &self.inner
    }
    fn cast_mut(&mut self) -> &mut Node {
        &mut self.inner
    }
    fn clone_node(&self, deep: bool) -> Self {
        ChildNode {
            inner: self.inner.clone_node(deep),
        }
    }
    fn node_name(&self) -> String {
        AsNode::cast(self).node_name()
    }
}
impl AsChildNode for ChildNode {}

pub trait AsChildNode: AsNode {
    /// Inserts nodes just after node, while replacing strings in nodes with equivalent Text nodes.
    /// # Errors
    /// - Errors with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn after<'a, T: 'a + AsNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        let node = AsNode::cast_mut(node.into());
        if let Some(mut parent) = self.parent_node() {
            match self.next_sibling_mut() {
                Some(next) => parent.insert_before(node, Some(next))?,
                None => parent.append_child(node)?,
            };
        }
        // DOM updates are triggered already.
        Ok(())
    }
    /// Inserts nodes just before node, while replacing strings in nodes with equivalent Text nodes.
    /// # Errors
    /// - Returns a "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn before<'a, T: 'a + AsNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        let node = AsNode::cast_mut(node.into());
        if let Some(mut parent) = self.parent_node() {
            parent.insert_before(node, Some(self))?;
        }
        // DOM updates are triggered already.
        Ok(())
    }
    /// Removes node from its parent. If the node has no parent then nothing happens.
    /// MDN Reference
    fn remove(&mut self) {
        let node = AsNode::cast_mut(self);
        let former_parent = node.parent_node();
        node.__remove();
        if let Some(parent) = former_parent {
            AsNode::cast(&parent).update_document();
        }
    }
    /// Replaces node with nodes, while replacing strings in nodes with equivalent Text nodes.
    /// # Panics
    /// - Panics with "HierarchyRequestError" DOMException if the constraints of the node tree are violated.
    fn replace_with<'a, T: 'a + AsNode>(
        &mut self,
        node: impl Into<&'a mut T>,
    ) -> Result<(), DOMException> {
        let node = AsNode::cast_mut(node.into());
        if let Some(mut parent) = node.parent_node() {
            parent.replace_child(AsNode::cast_mut(self), node)?;
        }
        // DOM updates are triggered already.
        Ok(())
    }
}

mod helpers {
    use std::{cell::RefCell, rc::Rc};

    use crate::{node::NodeBase, node::WeakNodeRef, AsNode, ChildNode, DOMException, Node};

    pub fn validate_hierarchy<T: AsNode, U: AsNode>(
        parent: &T,
        child: &U,
    ) -> Result<(), DOMException> {
        let parent_is_document = parent.node_type() == Node::DOCUMENT_NODE;
        if !AsNode::cast(parent).is_appendable() {
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
            let parent_has_element_already = parent
                .child_nodes()
                .iter()
                .find(|child| child.node_type() == Node::ELEMENT_NODE)
                .is_some();
            let mut element_count = 0;
            for subchild in child.child_nodes() {
                if subchild.node_type() == Node::ELEMENT_NODE {
                    if parent_is_document && parent_has_element_already {
                        return Err(DOMException::HierarchyRequestError(
                            "Only onde document allowed at root.",
                        ));
                    }
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
    pub fn clone_node<T: AsNode>(noderef: &T, deep: bool) -> Node {
        let inner_node = AsNode::cast(noderef).inner.borrow();
        if deep {
            Node {
                inner: Rc::new(RefCell::new(NodeBase {
                    node_type: inner_node.node_type,
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
                inner: Rc::new(RefCell::new(NodeBase {
                    node_type: inner_node.node_type,
                    event_target: crate::EventTarget::new(),
                    owner_document: inner_node.owner_document.clone(),
                    parent: inner_node.parent.clone(),
                    children: inner_node
                        .children
                        .as_slice()
                        .iter()
                        .map(|noderef| ChildNode {
                            inner: Node {
                                inner: noderef.inner.inner.clone(),
                            },
                        })
                        .collect(),
                })),
            }
        }
    }

    /// Get the number of children a node has.
    pub fn get_children_length<T: AsNode>(parent: &T) -> usize {
        AsNode::cast(parent).inner.borrow().children.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{node::NodeBase, AsNode, Document, HTMLElement, Node};
    #[test]
    fn parent_child_node_check() {
        let document = Document::new();
        let mut parent: HTMLElement = document.create_element("div");
        let mut child = document.create_element("span");
        let mut grandchild = document.create_element("p");
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

        parent.remove_child(&mut child).unwrap();
        assert_eq!(child.parent_node(), None);
        assert_eq!(parent.child_nodes().len(), 0);
    }

    #[test]
    fn sibling_node_check() {
        let document = Document::new();
        let mut parent = document.create_element("div");
        let mut child1 = document.create_element("div");
        let mut child2 = document.create_element("div");
        let mut child3 = document.create_element("div");
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
        let document = Document::new();
        let node1 = document.create_element("div");
        let node2 = document.create_element("div");
        let node1clone = node1.clone_ref();

        assert!(node1.is_equal_node(&node2));
        assert!(node1.is_same_node(&node1clone));
        assert!(!node1.is_same_node(&node2));
    }
}
