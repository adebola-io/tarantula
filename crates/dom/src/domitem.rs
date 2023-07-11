pub trait DOMItem<T> {
    /// Create another item referencing the same base.
    fn clone_ref(&self) -> Self;
    /// Return a mutable reference to the item's base.
    fn base(&self) -> &mut T;
}
