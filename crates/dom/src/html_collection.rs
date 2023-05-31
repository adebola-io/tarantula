// use std::{cell::RefCell, ops::Index, rc::Rc};

// use crate::{Element, ElementRef, AsElement};

// // /// A generic collection (array-like object similar to arguments) of elements (in document order) and offers methods and properties for selecting from the list.
// // pub trait HtmlCollectionBase {}

pub struct HTMLCollection<'a> {
    pub(crate) items: &'a Vec<()>,
}
pub struct MutHTMLCollection<'a> {
    pub(crate) items: &'a mut Vec<()>,
}
// impl HtmlCollection {
//     /// Retrieves the number of objects in a collection.
//     pub fn len(&self) -> usize {
//         self.items.len()
//     }
//     /// Retrieves an object from various collections.
//     pub fn item(&self, index: usize) -> Option<&Element> {
//         // unsafe { self.items.get(index).map(|x| &*x.as_ptr()) }
//         todo!()
//     }
//     /// Retrieves an object from various collections
//     pub fn item_mut(&mut self, index: usize) -> Option<&mut Element> {
//         // unsafe { self.items.get(index).map(|x| &mut *x.as_ptr()) }
//         todo!()
//     }
//     /// Retrieves a select object or an object from an options collection.
//     pub fn named_item(&self, name: &str) -> Option<&ElementRef> {
//         todo!()
//     }
//     /// Retrieves a select object or an object from an options collection, mutably.
//     pub fn named_item_mut(&mut self, name: &str) -> Option<&mut ElementRef> {
//         todo!()
//     }
// }
// impl Index<usize> for HtmlCollection {
//     type Output = Element;

//     fn index(&self, index: usize) -> &Self::Output {
//         unsafe { &*(self.items[index].as_ptr()) }
//     }
// }

// pub struct HtmlCollectionOf<'a, T>
// where
//     T: 'a + AsElement + ?Sized,
// {
//     items: Vec<&'a mut T>,
// }

// impl<'a, T> HtmlCollectionOf<'a, T>
// where
//     T: AsElement + ?Sized,
// {
//     /// Retrieves the number of objects in a collection.
//     pub fn len(&self) -> usize {
//         self.items.len()
//     }
//     /// Retrieves an object from various collections.
//     pub fn item(&self, index: usize) -> Option<&T> {
//         // unsafe { self.items.get(index).map(|x| &*x.as_ptr()) }
//         todo!()
//     }
//     pub fn item_mut(&mut self, index: usize) -> Option<&mut T> {
//         // unsafe { self.items.get(index).map(|x| &mut *x.as_ptr()) }
//         todo!()
//     }
// }
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
