use crate::{AsNode, Node};

pub enum SlotAssignment {
    Named,
    Manual,
}

impl SlotAssignment {
    /// Returns `true` if the slot assignment is [`Named`].
    ///
    /// [`Named`]: SlotAssignment::Named
    #[must_use]
    pub fn is_named(&self) -> bool {
        matches!(self, Self::Named)
    }
}

pub struct ShadowRoot {
    pub(crate) slot_assignment: SlotAssignment,
}
pub struct ShadowRootInit;

pub(crate) fn is_slottable(node: &impl AsNode) -> bool {
    node.node_type() == Node::ELEMENT_NODE || node.node_type() == Node::TEXT_NODE
}

pub(crate) fn is_shadow_host_and(
    node: &impl AsNode,
    root_predicate: fn(ShadowRoot) -> bool,
) -> bool {
    todo!()
}

pub(crate) fn assign_slot(node: &mut impl AsNode) {
    todo!()
}
