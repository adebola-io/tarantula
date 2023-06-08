use std::ops::{Index, IndexMut};

use crate::{AsNode, Attr};

/// A collection of Attr objects. Objects inside a NamedNodeMap are not in any particular order, unlike NodeList, although they may be accessed by an index as in an array.
pub struct NamedNodeMap {
    pub(crate) items: Vec<(String, Attr)>,
    /// Index of the class attribute.
    class_index: Option<usize>,
    class_attr: Attr,
}

impl IntoIterator for NamedNodeMap {
    type Item = Attr;

    type IntoIter = std::iter::Map<std::vec::IntoIter<(String, Attr)>, fn((String, Attr)) -> Attr>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().map(|tuple| tuple.1)
    }
}

impl NamedNodeMap {
    fn index_of(&self, qualified_name: &str) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .find(|tuple| tuple.1 .0 == qualified_name)
            .map(|tuple| tuple.0)
    }
    /// Returns the number of attributes in the node map.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Retrieve the (somewhat) hidden class attribute.
    pub(crate) fn get_class(&self) -> &Attr {
        &self.class_attr
    }
    /// Retrieve the (somewhat) hidden class attribute mutably.
    pub(crate) fn get_class_mut(&mut self) -> &mut Attr {
        &mut self.class_attr
    }
    /// Returns an item from the node map using its qualified name, or None if there is no item with the name.
    /// # Example
    /// ```
    /// // Add example.
    /// ```
    pub fn get_named_item(&self, qualified_name: &str) -> Option<&Attr> {
        if qualified_name == "class" {
            if self.class_index.is_some() {
                return Some(self.get_class());
            } else {
                return None;
            }
        }
        self.items
            .iter()
            .find(|tuple| tuple.0 == qualified_name)
            .map(|tuple| &tuple.1)
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
        if qualified_name == "class" {
            if self.class_index.is_some() {
                return Some(self.get_class_mut());
            } else {
                return None;
            }
        }
        self.items
            .iter_mut()
            .find(|tuple| tuple.0 == qualified_name)
            .map(|tuple| &mut tuple.1)
    }
    pub fn get_named_item_ns_mut(
        &mut self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<&mut Attr> {
        todo!()
    }
    pub fn item(&self, index: usize) -> Option<&Attr> {
        let mut index = index;
        if let Some(i) = self.class_index {
            if i == index {
                return Some(self.get_class());
            } else if i < index {
                index -= 1;
            }
        };
        self.items.get(index).map(|tuple| &tuple.1)
    }
    pub fn item_mut(&mut self, index: usize) -> Option<&mut Attr> {
        let mut index = index;
        if let Some(i) = self.class_index {
            if i == index {
                return Some(self.get_class_mut());
            } else if i < index {
                index -= 1;
            }
        };
        self.items.get_mut(index).map(|tuple| &mut tuple.1)
    }
    pub fn remove_named_item(&mut self, qualified_name: &str) -> Option<Attr> {
        if qualified_name == "class" {
            self.class_index = None;
            let clone = self.class_attr.clone_node(true);
            self.class_attr.value.clear();
            return Some(clone);
        } else {
            match self.index_of(qualified_name) {
                Some(index) => Some(self.items.remove(index).1),
                None => None,
            }
        }
    }
    pub fn remove_named_item_ns(&mut self, namespace: Option<&str>, local_name: &str) -> Attr {
        todo!()
    }
    pub fn set_named_item(&mut self, attr: Attr) -> Option<&mut Attr> {
        // if attr.name == "class" {
        //     let len = self.items.len();
        //     self.class_index = Some(len);
        //     self.class_attr = attr;
        //     return &mut self.class_attr
        // }
        todo!()
    }
    pub fn set_named_item_ns(&mut self, attr: Attr) -> Option<&mut Attr> {
        todo!()
    }
}

impl Index<usize> for NamedNodeMap {
    type Output = Attr;

    fn index(&self, index: usize) -> &Self::Output {
        &(self.items[index].1)
    }
}
impl IndexMut<usize> for NamedNodeMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.items[index].1)
    }
}
