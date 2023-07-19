// CHILD NODE.

use crate::{domitem::DOMItem, AsEventTarget, AsNode, DOMException, EventTarget, Node};

use super::mutation_algorithms;

#[derive(Debug)]
pub struct ChildNode {
    pub(super) inner: Node,
}
impl ChildNode {
    pub(super) fn clone_ref(&self) -> Self {
        ChildNode {
            inner: self.inner.clone_ref(),
        }
    }
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
            inner: Node {
                base: AsNode::cast(node).base.clone(),
            },
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
            mutation_algorithms::pre_insert(node, Some(self), &mut parent)?;
        }
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
    /// # Errors
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
