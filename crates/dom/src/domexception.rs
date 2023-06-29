#[derive(Debug)]
pub enum DOMException {
    HierarchyRequestError(&'static str),
    SyntaxError(&'static str),
    InvalidCharacterError(&'static str),
    TypeError(&'static str),
}

impl DOMException {
    /// Returns `true` if the domexception is [`HierarchyRequestError`].
    ///
    /// [`HierarchyRequestError`]: DOMException::HierarchyRequestError
    #[must_use]
    pub fn is_hierarchy_request_error(&self) -> bool {
        matches!(self, Self::HierarchyRequestError(..))
    }

    fn message(&self) -> &'static str {
        match self {
            DOMException::HierarchyRequestError(message)
            | DOMException::SyntaxError(message)
            | DOMException::InvalidCharacterError(message)
            | DOMException::TypeError(message) => message,
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::HierarchyRequestError(_) => "HierarchyRequestError",
            DOMException::SyntaxError(_) => "SyntaxError",
            DOMException::InvalidCharacterError(_) => "InvalidCharacterError",
            DOMException::TypeError(_) => "TypeError",
        }
    }
}

impl std::fmt::Display for DOMException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DOMException::{}: {}\n", self.name(), self.message())
    }
}
