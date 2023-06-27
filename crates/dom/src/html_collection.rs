// use std::{cell::RefCell, ops::Index, rc::Rc};

// use crate::{Element, ElementRef, AsElement};

use std::{cell::RefCell, rc::Rc};

use crate::{AsElement, Element, Tag};

pub(crate) enum LiveCollectionType {
    Tag(Tag),
    Class(String),
}

pub(crate) struct LiveCollection<T: AsElement> {
    pub collection_type: LiveCollectionType,
    pub target: Element,
    pub items: Vec<T>,
}
impl LiveCollection<Element> {
    pub fn update(&mut self) {
        match &self.collection_type {
            LiveCollectionType::Tag(tag) => self.items = self.target.tag_search(tag),
            LiveCollectionType::Class(class_names) => {
                self.items = self.target.class_search(class_names)
            }
        }
    }
}

/// A generic collection (array-like object similar to arguments) of elements (in document order) and offers methods and properties for selecting from the list.
pub struct HTMLCollection {
    pub(crate) items: Vec<Element>,
}

impl HTMLCollection {
    /// Retrieves the number of objects in a collection.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Retrieves an object from various collections.
    pub fn item(&self, index: usize) -> Option<&Element> {
        self.items.get(index)
    }
    /// Retrieves an object from various collections
    pub fn item_mut(&mut self, index: usize) -> Option<&mut Element> {
        self.items.get_mut(index)
    }
    /// Retrieves a select object or an object from an options collection.
    pub fn named_item(&self, name: &str) -> Option<&Element> {
        todo!()
    }
    /// Retrieves a select object or an object from an options collection, mutably.
    pub fn named_item_mut(&mut self, name: &str) -> Option<&mut Element> {
        todo!()
    }
}
// impl Index<usize> for HtmlCollection {
//     type Output = Element;

//     fn index(&self, index: usize) -> &Self::Output {
//         unsafe { &*(self.items[index].as_ptr()) }
//     }
// }

pub struct HTMLCollectionOf<T: AsElement> {
    pub(crate) collection: Rc<RefCell<LiveCollection<T>>>,
}

impl<T: AsElement> HTMLCollectionOf<T> {
    fn items(&self) -> &mut Vec<T> {
        &mut unsafe { &mut *self.collection.as_ptr() }.items
    }
    /// Retrieves the number of objects in a collection.
    pub fn len(&self) -> usize {
        self.items().len()
    }
    /// Retrieves an object from various collections.
    pub fn item(&self, index: usize) -> Option<&T> {
        self.items().get(index)
    }
}

// impl<'a, T> Index<usize> for HtmlCollectionOf<'a, T>
// where
//     T: AsElement + ?Sized,
// {
//     type Output = T;

//     fn index(&self, index: usize) -> &Self::Output {
//         todo!()
//         // unsafe { &*(self.items[index].as_ptr()) }
//     }
// }
