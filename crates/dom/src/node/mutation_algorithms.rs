use super::trees::{
    following_nodes, is_host_including_inclusive_ancestor, preceeding_nodes, root_of,
};
use crate::{
    assign_slot, domitem::DOMItem, is_shadow_host_and, is_slottable, AsChildNode, AsNode,
    AsParentNode, ChildNode, DOMException, Node, ParentNode,
};

mod append_utils {
    use crate::{domitem::DOMItem, node::WeakNodeRef, AsNode, ChildNode, ParentNode};

    /// Append a node to the end of another node.
    pub fn push_into(parent: &mut impl AsNode, node: ChildNode) {
        let weak_ref = WeakNodeRef::from(&*parent);
        let children = parent.child_nodes_mut().items;
        node.inner.base().parent = Some((weak_ref, children.len()));
        children.push(node);
    }

    /// Insert child into node at an index.
    pub fn insert_at(index: usize, parent: &mut impl AsNode, node: ChildNode) {
        let weak_ref = WeakNodeRef::from(&*parent);
        let children = parent.child_nodes_mut().items;
        node.inner.base().parent = Some((weak_ref, index));
        // Shift indexes.
        for sibling in children[(index + 1)..].iter_mut() {
            let index = AsNode::cast(sibling).index().unwrap();
            AsNode::cast_mut(sibling).set_index(index + 1);
        }
    }
}

/// Insert a node before a reference node.
///
/// [Reference](https://dom.spec.whatwg.org/#concept-node-ensure-pre-insertion-validity)
pub fn ensure_pre_insertion_validity(
    node: &impl AsNode,
    child: Option<&impl AsNode>,
    parent: &ParentNode,
) -> Result<(), DOMException> {
    let hierarchy_request_error: fn(&str) -> Result<(), DOMException> = hierarchy_request_error;
    let parent_type = parent.node_type();
    let node_type = node.node_type();
    // 1. If parent is not a Document, DocumentFragment, or Element node, then throw a "HierarchyRequestError" DOMException.
    if ![
        Node::DOCUMENT_NODE,
        Node::DOCUMENT_FRAGMENT_NODE,
        Node::ELEMENT_NODE,
    ]
    .contains(&parent_type)
    {
        return hierarchy_request_error(
            "Parent is neither a Document, DocumentFragment or Element node.",
        );
    }
    // 2. If node is a host-including inclusive ancestor of parent, then throw a "HierarchyRequestError" DOMException.
    if is_host_including_inclusive_ancestor(node, parent) {
        return hierarchy_request_error(
            "Node to pre-insert is a host-including inclusive ancestor of parent.",
        );
    }
    // 3.If child is non-null and its parent is not parent, then throw a "NotFoundError" DOMException.
    if child
        .as_ref()
        .is_some_and(|child| !AsNode::cast(*child).is_child_of(parent))
    {
        return Err(DOMException::NotFoundError(
            "Child does not exist in parent node.".to_owned(),
        ));
    }
    // 4. If node is not a DocumentFragment, DocumentType, Element, or CharacterData node, then throw a "HierarchyRequestError" DOMException.
    if [
        Node::DOCUMENT_FRAGMENT_NODE,
        Node::DOCUMENT_TYPE_NODE,
        Node::ELEMENT_NODE,
        // CharacterData
        Node::TEXT_NODE,
        Node::PROCESSING_INSTRUCTION_NODE,
        Node::COMMENT_NODE,
    ]
    .contains(&node_type)
    {
        return hierarchy_request_error("Node to insert is neither a DocumentFragment, DocumentType, Element or CharacterData Node");
    }
    // 5. If either node is a Text node and parent is a document, or node is a doctype and parent is not a document, then throw a "HierarchyRequestError" DOMException.
    if (node_type == Node::TEXT_NODE && parent_type == Node::DOCUMENT_NODE)
        || (node_type == Node::DOCUMENT_TYPE_NODE && parent_type != Node::DOCUMENT_NODE)
    {
        return hierarchy_request_error("Node hierarchy violated. Text Nodes cannot be inserted directly into Documents, and Doctype nodes cannot inserted anywhere other than a Document.");
    }
    // 6. If parent is a document, and any of the statements below, switched on the interface node implements, are true, then throw a "HierarchyRequestError" DOMException.
    // -> DocumentFragment
    if parent_type == Node::DOCUMENT_NODE {
        // If node has more than one element child or has a Text node child.
        // Otherwise, if node has one element child and either parent has an element child, child is a doctype, or child is non-null and a doctype is following child.
        if node_type == Node::DOCUMENT_FRAGMENT_NODE {
            let mut element_found = false;
            for subnode in node.child_nodes() {
                let subnode_type = subnode.node_type();
                if subnode_type == Node::ELEMENT_NODE {
                    if element_found {
                        return hierarchy_request_error("Cannot insert DocumentFragment with multiple Element nodes into Document.");
                    } else {
                        element_found = true;
                    }
                } else if subnode_type == Node::TEXT_NODE {
                    return hierarchy_request_error(
                        "Cannot insert DocumentFragment with Text node into Document.",
                    );
                }
            }
            if element_found
                && (parent.child_element_count() > 0
                    || child.is_some_and(|child| child.node_type() == Node::DOCUMENT_TYPE_NODE)
                    || child.is_some_and(|child| {
                        following_nodes(child)
                            .iter()
                            .any(|node| node.node_type() == Node::DOCUMENT_TYPE_NODE)
                    }))
            {
                return hierarchy_request_error(
                    "Cannot insert invalid DocumentFragment into Document",
                );
            }
            // -> Element
        } else if node_type == Node::ELEMENT_NODE {
            // parent has an element child, child is a doctype, or child is non-null and a doctype is following child.
            if (parent_type == Node::ELEMENT_NODE
                && child.is_some_and(|child| child.node_type() == Node::DOCUMENT_TYPE_NODE))
                || (child.is_some_and(|child| {
                    following_nodes(child)
                        .iter()
                        .any(|node| node.node_type() == Node::DOCUMENT_TYPE_NODE)
                }))
            {
                return hierarchy_request_error(
                    "Pre-insert failed because nodes violate node hierarchy.",
                );
            }
            // -> DocumentType
        } else if node_type == Node::DOCUMENT_TYPE_NODE {
            // parent has a doctype child, child is non-null and an element is preceding child, or child is null and parent has an element child.
            if (parent
                .child_nodes()
                .iter()
                .any(|child| child.node_type() == Node::DOCUMENT_TYPE_NODE)
                && child.is_some_and(|child| {
                    preceeding_nodes(child)
                        .iter()
                        .any(|node| node.node_type() == Node::ELEMENT_NODE)
                }))
                || (child.is_none() && parent.child_element_count() > 0)
            {
                return hierarchy_request_error(
                    "Pre-insert failed because nodes violate node hierarchy.",
                );
            }
        }
    }
    Ok(())
}

/// Pre-insert a node into a parent before a child.
///
/// [Reference](https://dom.spec.whatwg.org/#concept-node-pre-insert)
pub fn pre_insert<'node, T: AsNode>(
    node: &'node mut T,
    child: Option<&impl AsNode>,
    parent: &mut ParentNode,
) -> Result<&'node mut T, DOMException> {
    // 1. Ensure pre-insertion validity of node into parent before child.
    ensure_pre_insertion_validity(node, child, parent)?;
    // 2. Let referenceChild be child.
    let mut reference_child = child.map(|child| ChildNode::from(child));
    // 3. If referenceChild is node, then set referenceChild to node’s next sibling.
    if let Some(reference) = &reference_child {
        if reference.is_same_node(node) {
            reference_child = node.next_sibling_mut().map(|sibling| sibling.clone_ref())
        }
    }
    insert(node, parent, reference_child.as_mut(), None)?;
    Ok(node)
}

/// [Reference](https://dom.spec.whatwg.org/#concept-node-insert)
pub fn insert<T: AsNode>(
    node: &mut T,
    parent: &mut impl AsParentNode,
    child: Option<&mut impl AsNode>,
    suppress_observers: Option<bool>,
) -> Result<(), DOMException> {
    let is_fragment = node.node_type() == Node::DOCUMENT_FRAGMENT_NODE;
    if is_fragment && node.child_nodes().len() == 0 {
        return Ok(());
    }
    let mut nodes = vec![];
    if is_fragment {
        for child in node.child_nodes_mut() {
            nodes.push(remove(child, suppress_observers))
        }
        let added_nodes: Vec<ChildNode> = vec![];
        let previous_sibling: Option<&Node> = None;
        let next_sibling: Option<&Node> = None;
        queue_tree_mutation_record(
            node,
            added_nodes.as_slice(),
            nodes.as_slice(),
            previous_sibling,
            next_sibling,
        );
    } else {
        nodes.push(ChildNode::from(&*node))
    }

    let count = nodes.len();

    // 5. If child is non-null, then:
    // - For each live range whose start node is parent and start offset is greater than child’s index, increase its start offset by count.
    // - For each live range whose end node is parent and end offset is greater than child’s index, increase its end offset by count.
    if let Some(ref child) = child {
        let child_index = AsNode::cast(*child).index().unwrap();
        let mut document = node.owner_document().unwrap();
        let live_ranges = document.live_ranges_mut();
        live_ranges
            .filter(|range| {
                range.start.node.is_same_node(parent) && range.start.offset > child_index
            })
            .for_each(|range| range.start.offset += count);
        let live_ranges = document.live_ranges_mut();
        live_ranges
            .filter(|range| range.end.node.is_same_node(parent) && range.end.offset > child_index)
            .for_each(|range| range.end.offset += count);
    }
    // 6. Let previousSibling be child’s previous sibling or parent’s last child if child is null.
    let mut previous_sibling = match child {
        Some(ref child) => child.previous_sibling(),
        None => parent.last_child(),
    }
    .map(|node_ref| node_ref.clone_ref());

    // 7. For each node in nodes, in tree order:
    for mut node in nodes.iter_mut() {
        //  1. Adopt node into parent’s node document.
        adopt_into_document(parent.owner_document().unwrap(), node);

        //  2. If child is null, then append node to parent’s children.
        //  3. Otherwise, insert node into parent’s children before child’s index.
        match child {
            None => append_utils::push_into(parent, node.clone_ref()),
            Some(ref child) => {
                let index = AsNode::cast(*child).index().unwrap();
                append_utils::insert_at(index, parent, node.clone_ref());
            }
        }
        //  4. If parent is a shadow host whose shadow root’s slot assignment is "named" and node is a slottable, then assign a slot for node.
        if is_shadow_host_and(parent, |shadow_root| shadow_root.slot_assignment.is_named())
            && is_slottable(node)
        {
            assign_slot(node)
        }
        //  5. If parent’s root is a shadow root, and parent is a slot whose assigned nodes is the empty list, then run signal a slot change for parent.
        if is_shadow_root(root_of(parent)) && is_slot(parent) && assigned_nodes(parent).len() == 0 {
            signal_slot_change(parent)
        }
        //  6. Run assign slottables for a tree with node’s root.
        assign_slottables(&mut root_of(node));

        //  7. For each shadow-including inclusive descendant inclusiveDescendant of node, in shadow-including tree order:
        for inclusive_descendant in shadow_including_inclusive_descendants(&mut node) {
            // - Run the insertion steps with inclusiveDescendant.
            insertion_steps(inclusive_descendant);
            //      2. If inclusiveDescendant is connected, then:
            //          1. If inclusiveDescendant is custom, then enqueue a custom element callback reaction with inclusiveDescendant, callback name "connectedCallback", and an empty argument list.
            //          2. Otherwise, try to upgrade inclusiveDescendant.
            if inclusive_descendant.is_connected() {
                if is_custom(inclusive_descendant) {
                    enqueue_custom_element_callback_reaction(
                        inclusive_descendant,
                        CustomElementCallBack::ConnectedCallBack,
                        &[],
                    );
                } else {
                    try_upgrade(inclusive_descendant)
                }
            }
        }
    }
    // 8. If suppress observers flag is unset, then queue a tree mutation record for parent with nodes, « », previousSibling, and child.
    if suppress_observers.is_none() || !suppress_observers.unwrap() {
        let removed_nodes: &[ChildNode] = &[];
        queue_tree_mutation_record(
            parent,
            &nodes,
            removed_nodes,
            previous_sibling.as_ref(),
            match child {
                Some(ref child) => Some(&**child),
                None => None,
            },
        )
    }

    // 9. Run the children changed steps for parent.
    children_changed_steps(parent);

    Ok(())
}

/// [Reference](https://dom.spec.whatwg.org/#concept-node-append)
pub fn append<'node, T: AsNode>(
    node: &'node mut T,
    parent: &mut ParentNode,
) -> Result<&'node mut T, DOMException> {
    pre_insert(node, None::<&ChildNode>, parent)
}

/// [Reference](https://dom.spec.whatwg.org/#concept-node-replace)
pub fn replace<'child, T: AsNode>(
    child: &'child mut T,
    node: &mut impl AsNode,
    parent: &mut impl AsParentNode,
) -> Result<&'child mut T, DOMException> {
    let hierarchy_request_error: fn(&str) -> Result<&mut T, DOMException> = hierarchy_request_error;
    let parent_type = parent.node_type();
    let node_type = node.node_type();
    // 1. If parent is not a Document, DocumentFragment, or Element node, then throw a "HierarchyRequestError" DOMException.
    if ![
        Node::DOCUMENT_NODE,
        Node::DOCUMENT_FRAGMENT_NODE,
        Node::ELEMENT_NODE,
    ]
    .contains(&parent_type)
    {
        return hierarchy_request_error("Parent is not a Document, DocumentFragment or Element");
    }
    // 2. If node is a host-including inclusive ancestor of parent, then throw a "HierarchyRequestError" DOMException.
    if is_host_including_inclusive_ancestor(node, parent) {
        return hierarchy_request_error("Node is a host inclusing inclusive ancestor of Parent");
    }

    // 3. If child’s parent is not parent, then throw a "NotFoundError" DOMException.
    if !AsNode::cast(child).is_child_of(parent) {
        return Err(DOMException::HierarchyRequestError(format!(
            "Child to replace is not a child of Parent"
        )));
    }

    // 4. If node is not a DocumentFragment, DocumentType, Element, or CharacterData node, then throw a "HierarchyRequestError" DOMException.
    if ![
        Node::DOCUMENT_FRAGMENT_NODE,
        Node::DOCUMENT_TYPE_NODE,
        Node::ELEMENT_NODE,
        // CharacterData
        Node::TEXT_NODE,
        Node::PROCESSING_INSTRUCTION_NODE,
        Node::COMMENT_NODE,
    ]
    .contains(&node_type)
    {
        return hierarchy_request_error(
            "Node is not a DocumentFragment, Doctype, Element or CharacterData Node",
        );
    }

    // 5. If either node is a Text node and parent is a document, or node is a doctype and parent is not a document, then throw a "HierarchyRequestError" DOMException.
    if (node_type == Node::TEXT_NODE && parent_type == Node::DOCUMENT_NODE)
        || (node_type == Node::DOCUMENT_TYPE_NODE && parent_type != Node::DOCUMENT_NODE)
    {
        return hierarchy_request_error(
            "Cannot append Text node to Document, or Doctype to non-Document Node",
        );
    }

    // 6. If parent is a document, and any of the statements below, switched on the interface node implements, are true, then throw a "HierarchyRequestError" DOMException.
    if parent_type == Node::DOCUMENT_NODE {
        // -> DocumentFragment
        // If node has more than one element child or has a Text node child.
        // Otherwise, if node has one element child and either parent has an element child that is not child or a doctype is following child.
        if node_type == Node::DOCUMENT_FRAGMENT_NODE {
            let mut element_found = false;
            for subnode in node.child_nodes() {
                let subnode_type = subnode.node_type();
                if subnode_type == Node::ELEMENT_NODE {
                    if element_found {
                        return hierarchy_request_error("Cannot insert DocumentFragment with multiple Element nodes into Document.");
                    } else {
                        element_found = true;
                    }
                } else if subnode_type == Node::TEXT_NODE {
                    return hierarchy_request_error(
                        "Cannot insert DocumentFragment with Text node into Document.",
                    );
                }
            }
            if element_found
                && (parent.child_nodes().iter().any(|sibling| {
                    sibling.node_type() == Node::ELEMENT_NODE && !sibling.is_same_node(child)
                }) || following_nodes(child)
                    .iter()
                    .any(|following_node| following_node.node_type() == Node::DOCUMENT_TYPE_NODE))
            {
                return hierarchy_request_error(
                    "Cannot insert invalid DocumentFragment into Document",
                );
            }
        } else if node_type == Node::ELEMENT_NODE {
            // -> Element
            // parent has an element child that is not child or a doctype is following child.
            if parent.child_nodes().iter().any(|subnode| {
                subnode.node_type() == Node::ELEMENT_NODE && !subnode.is_same_node(child)
            }) || following_nodes(child)
                .iter()
                .any(|sibling| sibling.node_type() == Node::DOCUMENT_TYPE_NODE)
            {
                return hierarchy_request_error(
                    "Replace failed because nodes violate node hierarchy.",
                );
            }
        } else if node_type == Node::DOCUMENT_TYPE_NODE {
            // -> DocumentType
            // parent has a doctype child that is not child, or an element is preceding child.
            if parent.child_nodes().iter().any(|subnode| {
                subnode.node_type() == Node::DOCUMENT_TYPE_NODE && !subnode.is_same_node(child)
            }) || preceeding_nodes(child)
                .iter()
                .any(|node| node.node_type() == Node::ELEMENT_NODE)
            {
                return hierarchy_request_error(
                    "Replace failed because nodes violate node hierarchy.",
                );
            }
        }
    }

    // 7. Let referenceChild be child’s next sibling.
    let mut reference_child = child.next_sibling().map(|next| next.clone_ref());

    // 8. If referenceChild is node, then set referenceChild to node’s next sibling.
    if let Some(ref reference) = reference_child {
        if reference.is_same_node(node) {
            reference_child = node.next_sibling().map(|next| next.clone_ref());
        }
    }

    // 9. Let previousSibling be child’s previous sibling.
    let previous_sibling = child.previous_sibling().map(|next| next.clone_ref());

    // 10. Let removedNodes be the empty set.
    let mut removed_nodes = vec![];

    // 11. If child’s parent is non-null, then:
    // - Set removedNodes to « child ».
    // - Remove child with the suppress observers flag set.
    if !child.is_same_node(node) && child.parent_node().is_some() {
        removed_nodes.push(AsNode::cast(child).clone_ref());
        remove(child, Some(true));
    }

    // 12. Let nodes be node’s children if node is a DocumentFragment node; otherwise « node ».
    let nodes = if node_type == Node::DOCUMENT_FRAGMENT_NODE {
        node.child_nodes()
            .iter()
            .map(|child| child.inner.clone_ref())
            .collect()
    } else {
        vec![AsNode::cast(child).clone_ref()]
    };

    // 13. Insert node into parent before referenceChild with the suppress observers flag set.
    insert(node, parent, reference_child.as_mut(), Some(true))?;

    // 14. Queue a tree mutation record for parent with nodes, removedNodes, previousSibling, and referenceChild.
    queue_tree_mutation_record(
        parent,
        &nodes,
        &removed_nodes,
        previous_sibling.as_ref(),
        reference_child.as_ref(),
    );

    // 15. Return child.
    Ok(child)
}

fn hierarchy_request_error<T>(message: &str) -> Result<T, DOMException> {
    Err(DOMException::HierarchyRequestError(message.to_owned()))
}

fn children_changed_steps(parent: &impl AsParentNode) {
    todo!()
}

fn try_upgrade(inclusive_descendant: &mut ChildNode) {
    todo!()
}

fn is_custom(inclusive_descendant: &mut ChildNode) -> bool {
    todo!()
}

fn enqueue_custom_element_callback_reaction(
    inclusive_descendant: &mut ChildNode,
    callback_name: CustomElementCallBack,
    argument_list: &[()],
) {
    todo!()
}

fn insertion_steps(inclusive_descendant: &mut ChildNode) {
    todo!()
}

fn shadow_including_inclusive_descendants<'a>(node: &'a mut ChildNode) -> Vec<&'a mut ChildNode> {
    todo!()
}

fn signal_slot_change(parent: &mut impl AsParentNode) {
    todo!()
}

fn assign_slottables(node: &mut impl AsParentNode) {
    todo!()
}

fn assigned_nodes(parent: &impl AsParentNode) -> &Vec<ChildNode> {
    todo!()
}

fn is_slot(parent: &impl AsNode) -> bool {
    todo!()
}

fn is_shadow_root(parent: ParentNode) -> bool {
    todo!()
}

fn adopt_into_document(document: crate::Document, node: &mut ChildNode) {
    todo!()
}

fn queue_tree_mutation_record(
    target: &impl AsNode,
    added_nodes: &[impl AsNode],
    removed_nodes: &[impl AsNode],
    previous_sibling: Option<&impl AsNode>,
    next_sibling: Option<&impl AsNode>,
) {
    todo!()
}

// [Reference](https://dom.spec.whatwg.org/#concept-node-remove)
fn remove(child: &mut impl AsNode, suppress_observers: Option<bool>) -> ChildNode {
    todo!()
}
pub enum CustomElementCallBack {
    ConnectedCallBack,
}
