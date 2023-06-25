use std::{
    cell::RefCell,
    ops::{Index, IndexMut},
    rc::Weak,
};

use crate::{Attr, ElementBase};

/// A collection of Attr objects. Objects inside a NamedNodeMap are not in any particular order, unlike NodeList, although they may be accessed by an index as in an array.
pub struct NamedNodeMap {
    pub(crate) owner_element: Weak<RefCell<ElementBase>>,
    pub(crate) items: Vec<Attr>,
}

impl NamedNodeMap {
    fn index_of(&self, qualified_name: &str) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .find(|tuple| tuple.1.name == qualified_name)
            .map(|tuple| tuple.0)
    }
    /// Returns the number of attributes in the node map.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn iter(&self) -> std::slice::Iter<Attr> {
        self.items.iter()
    }
    /// Returns an item from the node map using its qualified name, or None if there is no item with the name.
    /// # Example
    /// ```
    /// // Add example.
    /// ```
    pub fn get_named_item(&self, qualified_name: &str) -> Option<&Attr> {
        self.items.iter().find(|attr| attr.name == qualified_name)
    }
    pub fn get_named_item_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<&Attr> {
        todo!()
    }
    /// Returns a mutable reference to an item from the node map using its qualified name, or None if there is no item with the name.
    /// # Example
    /// ```
    /// // Add example.
    /// ```
    pub fn get_named_item_mut(&mut self, qualified_name: &str) -> Option<&mut Attr> {
        self.items
            .iter_mut()
            .find(|attr| attr.name == qualified_name)
    }
    pub fn get_named_item_ns_mut(
        &mut self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<&mut Attr> {
        todo!()
    }
    pub fn item(&self, index: usize) -> Option<&Attr> {
        self.items.get(index)
    }
    pub fn item_mut(&mut self, index: usize) -> Option<&mut Attr> {
        self.items.get_mut(index)
    }
    pub fn remove_named_item(&mut self, qualified_name: &str) -> Option<Attr> {
        self.index_of(qualified_name)
            .map(|index| self.items.remove(index))
    }
    pub fn remove_named_item_ns(&mut self, namespace: Option<&str>, local_name: &str) -> Attr {
        todo!()
    }
    pub fn set_named_item(&mut self, mut attr: Attr) -> Option<&mut Attr> {
        attr.set_owner_element(self.owner_element.clone());
        self.items.push(attr);
        let index = self.items.len() - 1;
        self.items.get_mut(index)
    }
    pub fn set_named_item_ns(&mut self, attr: Attr) -> Option<&mut Attr> {
        todo!()
    }
}

impl Index<usize> for NamedNodeMap {
    type Output = Attr;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.items[index])
    }
}
impl IndexMut<usize> for NamedNodeMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.items[index])
    }
}
