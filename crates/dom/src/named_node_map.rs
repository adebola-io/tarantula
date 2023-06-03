use std::ops::{Index, IndexMut};

use crate::Attr;

/// A collection of Attr objects. Objects inside a NamedNodeMap are not in any particular order, unlike NodeList, although they may be accessed by an index as in an array.
pub struct NamedNodeMap {
    items: Vec<(String, Attr)>,
}

impl NamedNodeMap {
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn get_named_item(qualified_name: &str) -> Option<&Attr> {
        todo!()
    }
    pub fn get_named_item_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<&Attr> {
        todo!()
    }
    pub fn item(&self, index: usize) -> Option<&Attr> {
        todo!()
    }
    pub fn remove_named_item(&mut self, qualified_name: &str) -> Attr {
        todo!()
    }
    pub fn remove_named_item_ns(&mut self, namespace: Option<&str>, local_name: &str) -> Attr {
        todo!()
    }
    pub fn set_named_item(&mut self, attr: Attr) -> Option<&mut Attr> {
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
