use std::{
    cell::RefCell,
    collections::{btree_map::IterMut, HashMap},
    rc::{Rc, Weak},
};

use crate::{
    domitem::DOMItem,
    element::ElementBase,
    html_collection::{LiveCollection, LiveCollectionType},
    node::{NodeBase, NodeType},
    tag::Tag,
    AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode, Attr, Element,
    HTMLAnchorElement, HTMLCollection, HTMLCollectionOf, HTMLElement, HTMLElementBase,
    HTMLOrSVGScriptElement, Node, Range,
};

pub struct HTMLAllCollection;

pub(crate) struct DocumentBase {
    document_node: Option<Node>,
    pub url: String,
    html_elements: HashMap<*mut NodeBase, Weak<RefCell<HTMLElementBase>>>,
    ranges: Vec<Range>,
    live_collections: Vec<Weak<RefCell<LiveCollection<Element>>>>,
}

impl DocumentBase {
    /// Refresh the document.
    pub(crate) fn refresh(&mut self, target: &Element) {
        self.refresh_collections(target);
    }

    fn refresh_collections(&mut self, target: &Element) {
        self.live_collections
            .iter()
            .filter_map(|live_collection| live_collection.upgrade())
            .for_each(|collection| collection.borrow_mut().update());
    }

    pub(crate) fn base_url(&self) -> &str {
        "Hello"
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

impl AsDocument for Document {
    fn cast(&self) -> &Document {
        self
    }

    fn cast_mut(&mut self) -> &mut Document {
        self
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

impl<T: AsNode> PartialEq<T> for Document {
    fn eq(&self, other: &T) -> bool {
        self.inner().document_node.as_ref().unwrap() == other
    }
}

impl AsParentNode for Document {}
impl AsNode for Document {
    fn cast(&self) -> &Node {
        self.inner().document_node.as_ref().unwrap()
    }

    fn cast_mut(&mut self) -> &mut Node {
        self.inner().document_node.as_mut().unwrap()
    }

    fn clone_node(&self, deep: bool) -> Self {
        todo!()
    }
}
impl AsEventTarget for Document {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(AsNode::cast(self))
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(AsNode::cast_mut(self))
    }
}

impl Document {
    pub fn new() -> Self {
        let mut base = DocumentBase {
            document_node: None,
            url: String::new(),
            html_elements: HashMap::new(),
            live_collections: vec![],
            ranges: vec![],
        };
        let document = Self {
            inner: Rc::new(RefCell::new(base)),
        };
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&document.inner),
        };
        document.inner.borrow_mut().document_node =
            Some(Node::in_document(NodeType::DocumentNode, weak_ref));
        document
    }
    pub(crate) fn is_html_document(&self) -> bool {
        true
    }
    pub(crate) fn associate_node_with_element(
        &self,
        node_base: *mut NodeBase,
        html_element: Weak<RefCell<HTMLElementBase>>,
    ) {
        self.inner().html_elements.insert(node_base, html_element);
    }

    /// Find an element with a node base.
    pub(crate) fn lookup_html_element(&self, node_base: *mut NodeBase) -> Option<Element> {
        let node = self.inner().html_elements.get(&node_base)?.upgrade()?;
        let element = node.borrow().element().clone_ref();
        Some(element)
    }
    /// Find a live collection in the document with the parameters given.
    pub(crate) fn lookup_class_collection(
        &self,
        target: &Element,
        class_names: &str,
    ) -> Option<Rc<RefCell<LiveCollection<Element>>>> {
        self.inner().live_collections.iter().find_map(|collection| {
            collection
                .upgrade()
                .map(|collection_ref| {
                    if matches!(
                        &collection_ref.borrow().collection_type,
                        LiveCollectionType::Class(class_name)
                    ) && collection_ref.borrow().target.is_same_node(target)
                    {
                        Some(collection_ref)
                    } else {
                        None
                    }
                })
                .flatten()
        })
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
        let new_collection_ref = Rc::new(RefCell::new(new_collection));
        self.inner()
            .live_collections
            .push(Rc::downgrade(&new_collection_ref));
        new_collection_ref
    }

    fn inner(&self) -> &mut DocumentBase {
        unsafe { &mut *self.inner.as_ptr() }
    }
    pub(crate) fn lookup_tag_collection(
        &self,
        target: &Element,
        tag: &Tag,
    ) -> Option<Rc<RefCell<LiveCollection<Element>>>> {
        self.inner().live_collections.iter().find_map(|collection| {
            collection
                .upgrade()
                .map(|collection_ref| {
                    if matches!(
                        &collection_ref.borrow().collection_type,
                        LiveCollectionType::Tag(tag)
                    ) && collection_ref.borrow().target.is_same_node(target)
                    {
                        Some(collection_ref)
                    } else {
                        None
                    }
                })
                .flatten()
        })
    }

    pub(crate) fn add_live_tag_collection(
        &mut self,
        target: &Element,
        tag: &Tag,
    ) -> Rc<RefCell<LiveCollection<Element>>> {
        let new_collection = LiveCollection {
            collection_type: LiveCollectionType::Tag(tag.to_owned()),
            target: target.clone_ref(),
            items: target.tag_search(tag),
        };
        let new_collection_ref = Rc::new(RefCell::new(new_collection));
        self.inner()
            .live_collections
            .push(Rc::downgrade(&new_collection_ref));
        new_collection_ref
    }
    pub(crate) fn drop_node(&mut self, base_ptr: *mut NodeBase) {
        self.inner().html_elements.remove(&base_ptr);
    }

    pub(crate) fn document_base_url(&self) -> &str {
        todo!()
    }

    pub(crate) fn live_ranges(&self) -> impl Iterator<Item = &Range> {
        self.inner().ranges.iter().filter(|range| range.is_live)
    }
    pub(crate) fn live_ranges_mut(&mut self) -> impl Iterator<Item = &mut Range> {
        self.inner().ranges.iter_mut().filter(|range| range.is_live)
    }
}

pub trait AsDocument {
    fn cast(&self) -> &Document;
    fn cast_mut(&mut self) -> &mut Document;
    /// Returns the URL for the document.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/Document/URL)
    fn url(&self) -> &str {
        &AsDocument::cast(self).inner().url
    }
    /// Returns the color of active links in the document.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/Document/alinkColor)
    #[deprecated]
    fn a_link_color(&self) -> &str {
        todo!()
    }
    /// Sets the color of active links in the document.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/Document/alinkColor)
    #[deprecated]
    fn set_a_link_color(&mut self, value: &str) {
        todo!()
    }
    fn all(&self) -> HTMLAllCollection {
        todo!()
    }
    #[deprecated]
    fn anchors(&self) -> HTMLCollectionOf<HTMLAnchorElement> {
        todo!()
    }
    fn applets(&self) -> HTMLCollection {
        todo!()
    }
    fn bg_color(&self) -> &str {
        todo!()
    }
    fn set_bg_color(&mut self, value: &str) {
        todo!()
    }
    fn body(&self) -> HTMLElement {
        todo!()
    }
    fn set_body(&mut self, value: HTMLElement) {
        todo!()
    }
    fn character_set(&self) -> &str {
        todo!()
    }
    fn charset(&self) -> &str {
        todo!()
    }
    fn compat_mode(&self) -> &str {
        todo!()
    }
    fn content_type(&self) -> &str {
        todo!()
    }
    fn cookie(&self) -> &str {
        todo!()
    }
    fn set_cookie(&mut self, value: &str) {
        todo!()
    }
    fn current_script(&self) -> HTMLOrSVGScriptElement {
        todo!()
    }
    /// Create an HTML attribute with the specified `local_name`.
    fn create_attribute(&self, local_name: &str) -> Attr {
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&AsDocument::cast(self).inner),
        };
        Attr::in_document(local_name, weak_ref)
    }
    /// Create an HTML element with the specified `tagname`.
    fn create_element(&self, tagname: &str) -> HTMLElement {
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&AsDocument::cast(self).inner),
        };
        let html_element = HTMLElement::in_document(tagname, weak_ref);
        AsDocument::cast(self).associate_node_with_element(
            AsNode::cast(&html_element).get_base_ptr(),
            Rc::downgrade(&html_element.base),
        );
        html_element
    }
}
