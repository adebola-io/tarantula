// use std::{cell::RefCell, ops::Index, rc::Rc};

// use crate::{Element, ElementRef, AsElement};

use std::{cell::RefCell, ops::Index, rc::Rc};

use crate::{tag::Tag, AsElement, AsNode, ChildNode, Element, Node};

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
pub struct HTMLCollection<'a> {
    pub(crate) items: &'a Vec<ChildNode>,
}

impl<'a> HTMLCollection<'a> {
    /// Retrieves the number of objects in a collection.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Retrieves an object from various collections.
    pub fn item(&self, index: usize) -> Option<Element> {
        let node = self.items.get(index)?;
        node.owner_document()?
            .lookup_html_element(AsNode::cast(node).get_base_ptr())
    }
    /// Retrieves a select object or an object from an options collection.
    pub fn named_item(&self, name: &str) -> Option<Element> {
        todo!()
    }
}
impl<'a> IntoIterator for HTMLCollection<'a> {
    type Item = Element;

    type IntoIter = std::iter::Map<std::slice::Iter<'a, ChildNode>, fn(&'a ChildNode) -> Element>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter().map(|node| {
            node.owner_document()
                .unwrap()
                .lookup_html_element(AsNode::cast(node).get_base_ptr())
                .unwrap()
        })
    }
}

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
    /// Returns an iterator over the elements in the collection.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items().iter()
    }
    /// Returns a mutable iterator over the elements in the collection.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items().iter_mut()
    }
}

impl<T: AsElement> Index<usize> for HTMLCollectionOf<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items()[index]
    }
}
