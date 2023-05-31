#[derive(Debug)]
pub enum DOMException {
    HierarchyRequestError(&'static str),
}

impl DOMException {
    /// Returns `true` if the domexception is [`HierarchyRequestError`].
    ///
    /// [`HierarchyRequestError`]: DOMException::HierarchyRequestError
    #[must_use]
    pub fn is_hierarchy_request_error(&self) -> bool {
        matches!(self, Self::HierarchyRequestError(..))
    }
}
