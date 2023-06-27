use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    AsElement, AsNode, Attr, Element, HTMLElement, LiveCollection, LiveCollectionType, Node,
    NodeBase, Tag,
};

pub(crate) struct DocumentBase {
    pub url: String,
    node_to_element_map: RefCell<HashMap<*mut NodeBase, Element>>,
    live_collections: Vec<Rc<RefCell<LiveCollection<Element>>>>,
}

impl DocumentBase {
    /// Refresh the document.
    pub(crate) fn refresh(&mut self, target: &Element) {
        println!("Refreshing...");
        self.refresh_collections(target);
    }

    fn refresh_collections(&mut self, target: &Element) {
        self.live_collections
            .iter()
            .filter(|live_collection| live_collection.borrow().target.contains(target))
            .for_each(|collection| collection.borrow_mut().update());
    }
}

impl std::fmt::Debug for DocumentBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DocumentBase")
            .field("url", &self.url)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub(crate) inner: Rc<RefCell<DocumentBase>>,
}

impl PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakDocumentRef {
    pub(crate) inner: Weak<RefCell<DocumentBase>>,
}

impl PartialEq for WeakDocumentRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner.ptr_eq(&other.inner)
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(DocumentBase {
                url: String::new(),
                node_to_element_map: RefCell::new(HashMap::new()),
                live_collections: vec![],
            })),
        }
    }
    /// Create an HTML attribute with the specified `local_name`.
    pub fn create_attribute(&self, local_name: &str) -> Attr {
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&self.inner),
        };
        Attr::in_document(local_name, weak_ref)
    }
    /// Create an HTML element with the specified `tagname`.
    pub fn create_element(&self, tagname: &str) -> HTMLElement {
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&self.inner),
        };
        let element = HTMLElement::in_document(tagname, weak_ref);
        self.associate_node_with_element(
            AsNode::cast(&element).get_base_ptr(),
            AsElement::cast(&element).clone_ref(),
        );
        element
    }

    pub(crate) fn is_html_document(&self) -> bool {
        true
    }
    pub(crate) fn associate_node_with_element(&self, node_base: *mut NodeBase, element: Element) {
        self.inner()
            .node_to_element_map
            .borrow_mut()
            .insert(node_base, element);
    }

    pub(crate) fn lookup_node(&self, node_base: *mut NodeBase) -> Element {
        self.inner()
            .node_to_element_map
            .borrow_mut()
            .get(&node_base)
            .expect("DOMLookupError: Attempted to retrieve node that does not exist in document.")
            .clone_ref()
    }

    /// Find a live collection in the document with the parameters given.
    pub(crate) fn lookup_class_collection(
        &self,
        target: &Element,
        class_names: &str,
    ) -> Option<Rc<RefCell<LiveCollection<Element>>>> {
        self.inner()
            .live_collections
            .iter()
            .find(|collection| {
                matches!(
                    &collection.borrow().collection_type,
                    LiveCollectionType::Class(class_name)
                ) && collection.borrow().target.is_same_node(target)
            })
            .map(|collection_ref| collection_ref.clone())
    }

    pub(crate) fn add_live_class_collection(
        &mut self,
        target: &Element,
        class_names: &str,
    ) -> Rc<RefCell<LiveCollection<Element>>> {
        let new_collection = LiveCollection {
            collection_type: LiveCollectionType::Class(class_names.to_owned()),
            target: target.clone_ref(),
            items: target.class_search(class_names),
        };
        self.inner()
            .live_collections
            .push(Rc::new(RefCell::new(new_collection)));
        self.inner().live_collections.last().unwrap().clone()
    }

    fn inner(&self) -> &mut DocumentBase {
        unsafe { &mut *self.inner.as_ptr() }
    }
}
