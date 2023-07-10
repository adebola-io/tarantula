use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{
    element::ElementBase,
    html_collection::{LiveCollection, LiveCollectionType},
    html_element::HTMLElementBase,
    node::NodeBase,
    tag::Tag,
    AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode, Attr, Element,
    HTMLAnchorElement, HTMLCollection, HTMLCollectionOf, HTMLElement, HTMLOrSVGScriptElement, Node,
};

pub struct HTMLAllCollection;

pub(crate) struct DocumentBase {
    document_node: Option<Node>,
    url: String,
    map_html: HashMap<*mut NodeBase, HTMLElement>,
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

    fn node_name(&self) -> String {
        String::from("#document")
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
            map_html: HashMap::new(),
            live_collections: vec![],
        };
        let document = Self {
            inner: Rc::new(RefCell::new(base)),
        };
        let weak_ref = WeakDocumentRef {
            inner: Rc::downgrade(&document.inner),
        };
        document.inner.borrow_mut().document_node = Some(Node::in_document(9, weak_ref));
        document
    }
    pub(crate) fn is_html_document(&self) -> bool {
        true
    }
    pub(crate) fn associate_node_with_element(
        &self,
        node_base: *mut NodeBase,
        html_element: HTMLElement,
    ) {
        self.inner().map_html.insert(node_base, html_element);
    }

    /// Find an element with a node base.
    pub(crate) fn lookup_node(&self, node_base: *mut NodeBase) -> Option<Element> {
        Some(
            self.inner()
                .map_html
                .get(&node_base)?
                .inner
                .borrow()
                .element
                .clone_ref(),
        )
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
        self.inner().map_html.remove(&base_ptr);
    }
}

impl Document {
    /// Returns the URL for the document.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/Document/URL)
    pub fn url(&self) -> &str {
        &self.inner().url
    }
    /// Returns the color of active links in the document.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/Document/alinkColor)
    #[deprecated]
    pub fn a_link_color(&self) -> &str {
        todo!()
    }
    /// Sets the color of active links in the document.
    ///
    /// [MDN Reference](https://developer.mozilla.org/docs/Web/API/Document/alinkColor)
    #[deprecated]
    pub fn set_a_link_color(&mut self, value: &str) {
        todo!()
    }
    pub fn all(&self) -> HTMLAllCollection {
        todo!()
    }
    #[deprecated]
    pub fn anchors(&self) -> HTMLCollectionOf<HTMLAnchorElement> {
        todo!()
    }
    pub fn applets(&self) -> HTMLCollection {
        todo!()
    }
    pub fn bg_color(&self) -> &str {
        todo!()
    }
    pub fn set_bg_color(&mut self, value: &str) {
        todo!()
    }
    pub fn body(&self) -> HTMLElement {
        todo!()
    }
    pub fn set_body(&mut self, value: HTMLElement) {
        todo!()
    }
    pub fn character_set(&self) -> &str {
        todo!()
    }
    pub fn charset(&self) -> &str {
        todo!()
    }
    pub fn compat_mode(&self) -> &str {
        todo!()
    }
    pub fn content_type(&self) -> &str {
        todo!()
    }
    pub fn cookie(&self) -> &str {
        todo!()
    }
    pub fn set_cookie(&mut self, value: &str) {
        todo!()
    }
    pub fn current_script(&self) -> HTMLOrSVGScriptElement {
        todo!()
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
        let html_element = HTMLElement::in_document(tagname, weak_ref);
        self.associate_node_with_element(
            AsNode::cast(&html_element).get_base_ptr(),
            html_element.clone_ref(),
        );
        html_element
    }
}
