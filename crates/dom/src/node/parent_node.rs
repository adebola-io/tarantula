// PARENT NODE.

pub trait AsParentNode: AsNode {
    /// Returns the number of children of this node that are elements.
    fn child_element_count(&self) -> usize {
        self.child_nodes()
            .iter()
            .filter(|node| node.node_type() == Self::ELEMENT_NODE)
            .count()
    }
    /// Returns the children elements of this node.
    fn children(&self) -> HTMLCollection {
        HTMLCollection {
            items: self.child_nodes().items,
        }
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
    /// # Errors
    /// - Returns a `HierarchyRequestError` DOMException if the constraints of the node tree are violated.
    /// # Example
    /// ```
    /// use dom::{traits::*, Document};
    ///
    /// let document = Document::new();
    /// let mut element = document.create_element("div");
    /// let mut child = document.create_element("div");
    /// element.append(&mut child);
    ///
    /// assert_eq!(child.parent_node().unwrap(), element);
    /// assert_eq!(element.first_child().unwrap(), child);
    ///
    /// ```
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
    /// # Errors
    /// - Returns a `HierarchyRequestError` DOMException if the constraints of the node tree are violated.
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
    // /// # Errors
    // /// - Returns a `HierarchyRequestError` DOMException if the constraints of the node tree are violated.
    fn replace_children(&mut self, nodes: Vec<impl AsNode>) {
        todo!()
    }
}

use crate::{
    domitem::DOMItem, AsElement, AsEventTarget, AsNode, DOMException, EventTarget, HTMLCollection,
    Node,
};

#[derive(Debug)]
pub struct ParentNode {
    pub(super) inner: Node,
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
}
impl AsParentNode for ParentNode {}
