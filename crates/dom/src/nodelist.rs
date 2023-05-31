// NODE LIST.

use std::ops::{Index, IndexMut};

use crate::AsNode;

pub struct NodeListOf<'a, TNode: AsNode> {
    pub(crate) items: &'a Vec<TNode>,
}

pub struct MutNodeListOf<'a, TNode: AsNode> {
    pub(crate) items: &'a mut Vec<TNode>,
}

impl<TNode: AsNode> Index<usize> for NodeListOf<'_, TNode> {
    type Output = TNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<'a, TNode: AsNode> IntoIterator for NodeListOf<'a, TNode> {
    type Item = &'a TNode;

    type IntoIter = std::slice::Iter<'a, TNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
impl<'a, TNode: AsNode> NodeListOf<'a, TNode> {
    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    pub fn item(&self, index: usize) -> Option<&TNode> {
        self.items.get(index)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, TNode> {
        self.items.iter()
    }
}
impl<TNode: AsNode> Index<usize> for MutNodeListOf<'_, TNode> {
    type Output = TNode;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
impl<TNode: AsNode> IndexMut<usize> for MutNodeListOf<'_, TNode> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
impl<'a, TNode: AsNode> MutNodeListOf<'a, TNode> {
    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    pub fn item(&self, index: usize) -> Option<&TNode> {
        self.items.get(index)
    }
    /// Returns the node with index index from the collection. The nodes are sorted in tree order.
    pub fn item_mut(&mut self, index: usize) -> Option<&mut TNode> {
        self.items.get_mut(index)
    }
    pub fn iter(&self) -> std::slice::Iter<'_, TNode> {
        self.items.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, TNode> {
        self.items.iter_mut()
    }
}
impl<'a, TNode: AsNode> IntoIterator for MutNodeListOf<'a, TNode> {
    type Item = &'a mut TNode;

    type IntoIter = std::slice::IterMut<'a, TNode>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
