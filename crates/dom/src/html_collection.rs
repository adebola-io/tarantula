use std::{cell::RefCell, ops::Index, rc::Rc};

use crate::{Element, ElementRef, IntoElement};

// /// A generic collection (array-like object similar to arguments) of elements (in document order) and offers methods and properties for selecting from the list.
// pub trait HtmlCollectionBase {}

pub struct HtmlCollection {
    pub(crate) items: Vec<ElementRef>,
}
impl HtmlCollection {
    /// Retrieves the number of objects in a collection.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Retrieves an object from various collections.
    pub fn item(&self, index: usize) -> Option<&Element> {
        unsafe { self.items.get(index).map(|x| &*x.as_ptr()) }
    }
    /// Retrieves an object from various collections
    pub fn item_mut(&mut self, index: usize) -> Option<&mut Element> {
        unsafe { self.items.get(index).map(|x| &mut *x.as_ptr()) }
    }
    /// Retrieves a select object or an object from an options collection.
    pub fn named_item(&self, name: &str) -> Option<&ElementRef> {
        todo!()
    }
    /// Retrieves a select object or an object from an options collection, mutably.
    pub fn named_item_mut(&mut self, name: &str) -> Option<&mut ElementRef> {
        todo!()
    }
}
impl Index<usize> for HtmlCollection {
    type Output = Element;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*(self.items[index].as_ptr()) }
    }
}

pub struct HtmlCollectionOf<T>
where
    T: IntoElement + ?Sized,
{
    items: Vec<Rc<RefCell<T>>>,
}

impl<T> HtmlCollectionOf<T>
where
    T: IntoElement + ?Sized,
{
    /// Retrieves the number of objects in a collection.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Retrieves an object from various collections.
    pub fn item(&self, index: usize) -> Option<&T> {
        unsafe { self.items.get(index).map(|x| &*x.as_ptr()) }
    }
    pub fn item_mut(&mut self, index: usize) -> Option<&mut T> {
        unsafe { self.items.get(index).map(|x| &mut *x.as_ptr()) }
    }
}
impl<T> Index<usize> for HtmlCollectionOf<T>
where
    T: IntoElement + ?Sized,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*(self.items[index].as_ptr()) }
    }
}