use crate::{domitem::DOMItem, AsNode, Node, ParentNode};

/// Returns all the nodes that precede a node in [tree order](https://dom.spec.whatwg.org/#concept-tree-order).
pub fn preceeding_nodes(reference: &impl AsNode) -> Vec<Node> {
    let mut prior_nodes = vec![];
    match reference.parent_node() {
        Some(parent) => {
            let reference_index = AsNode::cast(reference).index().unwrap();
            for child_node in parent.child_nodes().items[0..reference_index].iter() {
                prior_nodes.push(child_node.inner.clone_ref())
            }
            prior_nodes.insert(0, parent.inner.clone_ref());
            let mut preceeding_nodes_of_parent = preceeding_nodes(&parent);
            preceeding_nodes_of_parent.append(&mut prior_nodes);
            preceeding_nodes_of_parent
        }
        None => prior_nodes,
    }
}

/// Returns all the nodes that follow a node in [tree order](https://dom.spec.whatwg.org/#concept-tree-order).
pub fn following_nodes(reference: &impl AsNode) -> Vec<&Node> {
    let mut successive_nodes = vec![];
    // Add children.
    for descendant in descendant_nodes(reference) {
        successive_nodes.push(descendant)
    }
    // Add next nodes.
    match AsNode::cast(reference).base().parent {
        Some((ref weak_parent_ref, index)) => {
            if let Some(parent) = weak_parent_ref.inner.upgrade() {
                for sibling in (unsafe { &*parent.as_ptr() }).children[(index + 1)..].iter() {
                    successive_nodes.push(&sibling.inner);
                    successive_nodes.append(&mut descendant_nodes(sibling))
                }
            }
            successive_nodes
        }
        None => successive_nodes,
    }
}
/// Returns all the nodes that are [descendants](https://dom.spec.whatwg.org/#concept-tree-descendant) of a node, in tree order.
pub fn descendant_nodes(reference: &impl AsNode) -> Vec<&Node> {
    let mut descendants = vec![];
    for child in reference.child_nodes() {
        descendants.push(&child.inner);
        descendants.append(&mut descendant_nodes(child))
    }
    descendants
}

pub fn host_of(b: &impl AsNode) -> Option<Node> {
    None
}

/// The root of an object is itself, if its parent is null, or else it is the root of its parent. The root of a tree is any object participating in that tree whose parent is null.
pub fn root_of(object: &impl AsNode) -> ParentNode {
    let mut parent = object.parent_node();
    match parent {
        Some(parent) => root_of(&parent),
        None => ParentNode {
            inner: AsNode::cast(object).clone_ref(),
        },
    }
}

/// Returns true if `a` is a host-including inclusive ancestor of `b`.
pub fn is_host_including_inclusive_ancestor(a: &impl AsNode, b: &impl AsNode) -> bool {
    a.contains(b)
        || host_of(&root_of(b))
            .is_some_and(|root_host| is_host_including_inclusive_ancestor(a, &root_host))
}
