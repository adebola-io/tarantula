use std::{cell::RefCell, mem::ManuallyDrop, rc::Rc};

use crate::{
    dom_token_list::ListType, AsChildNode, AsEventTarget, AsNode, AsParentNode, Attr, DOMException,
    DOMTokenList, HTMLCollectionOf, InnerHtml, MutDOMTokenList, NamedNodeMap, Node, Tag,
};

pub struct ShadowRoot;
pub struct ShadowRootInit;
pub struct CheckVisibilityOptions;
pub struct DOMRect;
pub struct DOMRectList;
pub struct InsertPosition;
pub struct FullscreenOptions;
pub struct ScrollToOptions;
pub enum NameSpaceUri {
    SVG,
    XHTML,
}

pub(crate) struct ElementBase {
    attributes: Option<NamedNodeMap>,
    node: Node,
    tagname: Tag,
    is_html: bool,
}
/// Element is the most general base class from which all objects in a Document inherit. It only has methods and properties common to all kinds of elements. More specific classes inherit from Element.
pub struct Element {
    pub(crate) inner_ref: Rc<RefCell<ElementBase>>,
}

impl Element {
    fn inner(&self) -> &mut ElementBase {
        unsafe { &mut *self.inner_ref.as_ptr() }
    }
    /// Create a new Element that references the same base.
    fn clone_ref(&self) -> Self {
        Self {
            inner_ref: self.inner_ref.clone(),
        }
    }

    pub(crate) fn in_document(
        tagname: &str,
        is_html: bool,
        weak_ref: crate::WeakDocumentRef,
    ) -> Element {
        let element = Self {
            inner_ref: Rc::new(RefCell::new(ElementBase {
                attributes: None,
                is_html,
                node: Node::in_document(1, weak_ref),
                tagname: Tag::from(tagname),
            })),
        };
        element.inner().attributes = Some(NamedNodeMap {
            owner_element: Rc::downgrade(&element.inner_ref),
            items: vec![],
        });
        element
    }

    pub(crate) fn is_html(&self) -> bool {
        self.inner().is_html
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner_ref, &other.inner_ref)
    }
}
impl AsEventTarget for Element {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.inner().node)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.inner().node)
    }
}
impl AsNode for Element {
    fn cast(&self) -> &Node {
        &self.inner().node
    }
    fn cast_mut(&mut self) -> &mut Node {
        &mut self.inner().node
    }
    fn node_name(&self) -> String {
        self.tag_name()
    }
    fn clone_node(&self, deep: bool) -> Self {
        let mut element = Element {
            inner_ref: Rc::new(RefCell::new(ElementBase {
                node: self.inner().node.clone_node(deep),
                is_html: self.inner().is_html,
                attributes: None,
                tagname: self.inner().tagname.clone(),
            })),
        };
        element.inner().attributes = Some(NamedNodeMap {
            owner_element: Rc::downgrade(&element.inner_ref),
            items: vec![],
        });
        for attr in self.attributes().iter() {
            element
                .attributes_mut()
                .set_named_item(attr.clone_node(deep));
        }
        element
    }
}
impl AsChildNode for Element {}
impl AsParentNode for Element {}
impl InnerHtml for Element {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsElement for Element {
    fn cast(&self) -> &Element {
        self
    }
    fn cast_mut(&mut self) -> &mut Element {
        self
    }
}

pub trait AsElement: AsNode + AsChildNode + AsParentNode + InnerHtml {
    /// Downcasts struct to a reference to an element.
    fn cast(&self) -> &Element;
    /// Downcasts struct to a mutable reference to an element.
    fn cast_mut(&mut self) -> &mut Element;
    /// Returns a reference to a [`NamedNodeMap`] containing the assigned attributes of the corresponding HTML element.
    ///
    /// MDN Reference: [`Element.attributes`](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes).
    fn attributes(&self) -> &NamedNodeMap {
        AsElement::cast(self).inner().attributes.as_ref().unwrap()
    }
    /// Returns a mutable reference to a [`NamedNodeMap`] containing the assigned attributes of the corresponding HTML element.
    ///
    /// MDN Reference: [`Element.attributes`](https://developer.mozilla.org/en-US/docs/Web/API/Element/attributes).
    fn attributes_mut(&mut self) -> &mut NamedNodeMap {
        AsElement::cast_mut(self)
            .inner()
            .attributes
            .as_mut()
            .unwrap()
    }
    /// Returns a [`DOMTokenList`] containing the list of class attributes.
    ///
    /// MDN Reference: [`Element.classList`](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList).
    fn class_list(&self) -> DOMTokenList {
        DOMTokenList::from_element(AsElement::cast(self), ListType::ClassList)
    }
    /// Returns a [`MutDOMTokenList`] containing the list of class attributes.
    ///
    /// MDN Reference: [`Element.classList`](https://developer.mozilla.org/en-US/docs/Web/API/Element/classList).
    fn class_list_mut(&mut self) -> MutDOMTokenList {
        MutDOMTokenList::from_element(AsElement::cast_mut(self), ListType::ClassList)
    }
    /// Returns a string slice representing the class of the element.
    ///
    /// MDN Reference: [`Element.className`](https://developer.mozilla.org/en-US/docs/Web/API/Element/className).
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut element = document.create_element("p");
    /// assert_eq!(element.class_name(), ""); // class is initially empty.
    ///
    /// element.set_attribute("class", "paragraph");
    /// assert_eq!(element.class_name(), "paragraph");
    /// ```
    fn class_name(&self) -> &str {
        self.attributes()
            .get_named_item("class")
            .map(|attr| attr.value())
            .unwrap_or("")
    }
    /// Sets the value of the element's class attribute.
    ///
    /// MDN Reference: [`Element.className`](https://developer.mozilla.org/en-US/docs/Web/API/Element/className).
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut element = document.create_element("p");
    /// assert_eq!(element.class_name(), ""); // class is initially empty.
    ///
    /// element.set_class_name("paragraph");
    /// assert_eq!(element.class_name(), "paragraph");
    /// ```
    fn set_class_name(&mut self, value: &str) {
        match self.attributes_mut().get_named_item_mut("class") {
            Some(attr) => attr.set_value(value.to_owned()),
            None => {
                let mut class_attr = self.owner_document().unwrap().create_attribute("class");
                class_attr.set_value(value.to_owned());
                self.attributes_mut().set_named_item(class_attr);
            }
        }
    }
    /// Returns a number representing the inner height of the element.
    ///
    /// MDN Reference: [`Element.clientHeight`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientHeight)
    fn client_height(&self) -> usize {
        todo!()
    }
    /// Returns a number representing the width of the left border of the element.
    ///
    /// MDN Reference: [`Element.clientLeft`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientLeft).
    fn client_left(&self) -> usize {
        todo!()
    }
    /// Returns a number representing the width of the top border of the element.
    ///
    /// MDN Reference: [`Element.clientTop`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientTop).
    fn client_top(&self) -> usize {
        todo!()
    }
    /// Returns a number representing the inner width of the element.
    ///
    /// MDN Reference: [`Element.clientWidth`](https://developer.mozilla.org/en-US/docs/Web/API/Element/clientWidth).
    fn client_width(&self) -> usize {
        todo!()
    }
    /// Returns a string slice representing the id of the element. It returns an empty slice if there is no id specified.
    ///
    /// MDN Reference: [`Element.id`](https://developer.mozilla.org/en-US/docs/Web/API/Element/id).
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut element = document.create_element("p");
    /// assert_eq!(element.id(), ""); // id is initially empty.
    ///
    /// element.set_attribute("id", "paragraph-1");
    /// assert_eq!(element.id(), "paragraph-1");
    /// ```
    fn id(&self) -> &str {
        self.attributes()
            .get_named_item("id")
            .map(|attr| attr.value())
            .unwrap_or("")
    }
    /// Sets the value of the id attribute on the element.
    ///
    /// MDN Reference: [`Element.id`](https://developer.mozilla.org/en-US/docs/Web/API/Element/id).
    ///
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut element = document.create_element("p");
    /// assert_eq!(element.id(), ""); // id is initially empty.
    ///
    /// element.set_id("paragraph-1");
    /// assert_eq!(element.id(), "paragraph-1");
    /// ```
    fn set_id(&mut self, value: &str) {
        match self.attributes_mut().get_named_item_mut("id") {
            Some(attr) => attr.set_value(value.to_string()),
            None => {
                let mut attr = self.owner_document().unwrap().create_attribute("id");
                attr.set_value(value.to_string());
                self.attributes_mut().set_named_item(attr);
            }
        }
    }
    /// Returns a string slice representing the local part of the qualified name of the element.
    ///
    /// MDN Reference: [`Element.localName`](https://developer.mozilla.org/en-US/docs/Web/API/Element/localName).
    fn local_name(&self) -> &str {
        todo!()
    }
    /// The namespace URI of the element, or [`None`] if it is no namespace.
    ///
    /// MDN Reference: [`Element.namespaceURI`](https://developer.mozilla.org/en-US/docs/Web/API/Element/namespaceURI).
    fn namespace_uri(&self) -> Option<&str> {
        todo!()
    }
    /// Returns a string slice containing an HTML serialization of the element and its descendants.
    ///
    /// MDN Reference: [`Element.outerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML).
    fn outer_html(&self) -> &str {
        todo!()
    }
    /// Sets the outer html of the element.
    ///
    /// MDN Reference: [`Element.outerHTML`](https://developer.mozilla.org/en-US/docs/Web/API/Element/outerHTML).
    fn set_outer_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
    /// Represents the part identifier(s) of the element (i.e. set using the part attribute), returned as a DOMTokenList.
    ///
    /// MDN Reference: [`Element.part`](https://developer.mozilla.org/en-US/docs/Web/API/Element/part).
    fn part(&self) -> DOMTokenList {
        DOMTokenList::from_element(AsElement::cast(self), ListType::Part)
    }
    /// Represents the part identifier(s) of the element (i.e. set using the part attribute), returned as a DOMTokenList.
    ///
    /// MDN Reference: [`Element.part`](https://developer.mozilla.org/en-US/docs/Web/API/Element/part).
    fn part_mut(&mut self) -> MutDOMTokenList {
        MutDOMTokenList::from_element(AsElement::cast_mut(self), ListType::Part)
    }
    /// Returns the namespace prefix of the element.
    ///
    /// MDN Reference: [`Element.prefix`](https://developer.mozilla.org/en-US/docs/Web/API/Element/prefix).
    fn prefix(&self) -> Option<&str> {
        todo!()
    }
    /// Returns a number representing the scroll view height of an element.
    ///
    /// MDN Reference: [`Element.scrollHeight`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollHeight).
    fn scroll_height(&self) -> usize {
        todo!()
    }
    /// Returns a number representing the left scroll offset of the element.
    ///
    /// MDN Reference: [`Element.scrollLeft`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft).
    fn scroll_left(&self) -> usize {
        todo!()
    }
    /// Sets the left scroll offset of the element.
    ///
    /// MDN Reference: [`Element.scrollLeft`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollLeft).
    fn set_scroll_left(&mut self, value: usize) {
        todo!()
    }
    /// Returns a number representing number of pixels the top of the element is scrolled vertically.
    ///
    /// MDN Reference: [`Element.scrollTop`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop).
    fn scroll_top(&self) -> usize {
        todo!()
    }
    /// Sets the number of pixels the top of the element is scrolled vertically.
    ///
    /// MDN Reference: [`Element.scrollTop`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollTop).
    fn set_scroll_top(&mut self, value: usize) {
        todo!()
    }
    /// Returns a number representing the scroll view width of an element.
    ///
    /// MDN Reference: [`Element.scrollWidth`](https://developer.mozilla.org/en-US/docs/Web/API/Element/scrollWidth).
    fn scroll_width(&self) -> usize {
        todo!()
    }
    /// Returns reference to the open shadow root that is hosted by the element, or [`None`] if no open shadow root is present.
    ///
    /// MDN Reference: [`Element.shadowRoot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot).
    fn shadow_root(&self) -> Option<&ShadowRoot> {
        todo!()
    }
    /// Returns a mutable reference to the open shadow root that is hosted by the element, or [`None`] if no open shadow root is present.
    ///
    /// MDN Reference: [`Element.shadowRoot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/shadowRoot).
    fn shadow_root_mut(&mut self) -> Option<&mut ShadowRoot> {
        todo!()
    }
    /// Returns the name of the shadow DOM slot the element is inserted in, or an empty string.
    ///
    /// MDN Reference: [`Element.slot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot).
    fn slot(&self) -> &str {
        todo!()
    }
    /// Sets the name of the shadow DOM slot the element is inserted in.
    ///
    /// MDN Reference: [`Element.slot`](https://developer.mozilla.org/en-US/docs/Web/API/Element/slot).
    fn set_slot(&self, value: &str) {
        todo!()
    }
    /// Returns a string with the name of the tag for the given element. For example, if the element is an `<img>`, its tag name is "IMG" (for HTML documents;
    /// it may be cased differently for XML/XHTML documents).
    ///
    /// MDN Reference: [`Element.tagName`](https://developer.mozilla.org/en-US/docs/Web/API/Element/tagName).
    fn tag_name(&self) -> String {
        let element = AsElement::cast(self);
        if element.is_html() {
            element.inner().tagname.to_uppercase()
        } else {
            todo!()
        }
    }
    // METHODS
    /// Attaches a shadow DOM tree to the specified element and returns a mutable reference to its [`ShadowRoot`].
    ///
    /// MDN Reference: [`Element.attachShadow()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/attachShadow).
    fn attach_shadow(&mut self, init: ShadowRootInit) -> &mut ShadowRoot {
        todo!()
    }
    fn check_visibility(&self, options: Option<CheckVisibilityOptions>) -> bool {
        todo!()
    }
    /// Traverses the element and its parents (heading toward the document root) until it finds a node that matches the specified CSS selector.
    ///
    /// MDN Reference: [`Element.closest()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/closest).
    fn closest(&self, selector: &str) -> Option<Element> {
        todo!()
    }
    /// Returns the value of a specified attribute on the element.
    ///
    /// MDN Reference: [`Element.getAttribute()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttribute).
    fn get_attribute(&self, qualified_name: &str) -> Option<&str> {
        if matches!(AsElement::cast(self).inner().tagname, Tag::Script) && qualified_name == "nonce"
        {
            return Some("");
        }
        let mut qualified_name = qualified_name;
        let mut lowercased_opt = None;

        if AsElement::cast(self).is_html()
            && qualified_name.chars().any(|char| !char.is_lowercase())
        {
            lowercased_opt = Some(qualified_name.to_lowercase());
        }

        self.attributes()
            .get_named_item(match &lowercased_opt {
                Some(lowercased) => lowercased,
                None => qualified_name,
            })
            .map(|attr| attr.value())
    }
    /// Returns the string value of the attribute with the specified namespace and name.
    ///
    /// MDN Reference: [`Element.getAttributeNS()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNS).
    fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<&str> {
        self.attributes()
            .get_named_item_ns(namespace, local_name)
            .map(|attr| attr.__name.as_str())
    }
    /// Returns a vector containing the attribute names of the element.
    ///
    /// MDN Reference: [`Element.getAttributeNames()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNames).
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut a = document.create_element("a");
    ///
    /// a.set_id("link-1");
    /// a.set_class_name("user-link");
    /// a.set_attribute("href", "http://example.com");
    /// a.set_attribute("referrerpolicy", "no-referrer");
    ///
    /// assert_eq!(a.get_attribute_names(), vec!["id", "class", "href", "referrerpolicy"]);
    /// ```
    fn get_attribute_names(&self) -> Vec<&str> {
        self.attributes()
            .iter()
            .map(|attr| attr.__name.as_str())
            .collect()
    }
    /// Returns a reference to the specified attribute of the specified element, as an [`Attr`] node.
    ///
    /// MDN Reference: [`Element.getAttributeNode()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)
    fn get_attribute_node(&self, qualified_name: &str) -> Option<&Attr> {
        self.attributes().get_named_item(qualified_name)
    }
    /// Returns a mutable reference to the specified attribute of the specified element, as an [`Attr`] node.
    ///
    /// MDN Reference: [`Element.getAttributeNode()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNode)
    fn get_attribute_node_mut(&mut self, qualified_name: &str) -> Option<&mut Attr> {
        self.attributes_mut().get_named_item_mut(qualified_name)
    }
    /// Returns a reference to the specified attribute of the specified element, as an [`Attr`] node.
    ///
    /// MDN Reference: [`Element.getAttributeNodeNS()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)
    fn get_attribute_node_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<&Attr> {
        self.attributes().get_named_item_ns(namespace, local_name)
    }
    /// Returns a mutable reference to the specified attribute of the specified element, as an [`Attr`] node.
    ///
    /// MDN Reference: [`Element.getAttributeNodeNS()`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getAttributeNodeNS)
    fn get_attribute_node_ns_mut(
        &mut self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> Option<&mut Attr> {
        self.attributes_mut()
            .get_named_item_ns_mut(namespace, local_name)
    }
    fn get_bounding_client_rect(&self) -> DOMRect {
        todo!()
    }
    fn get_client_rects(&self) -> DOMRectList {
        todo!()
    }
    /// Returns a collection of elements that have the given class names, from within this element.
    ///
    /// MDN Reference: [`Element.getElementsByClassName()`](https://developer.mozilla.org/docs/Web/API/Element/getElementsByClassName)
    fn get_elements_by_class_name(&self, class_names: &str) -> HTMLCollectionOf<Element> {
        let mut items = vec![];
        for child in self.children().items {
            if class_names
                .split(' ')
                .all(|classname| child.class_list().contains(classname))
            {
                items.push(child.clone_ref())
            }
            items.append(&mut child.get_elements_by_class_name(class_names).items)
        }
        HTMLCollectionOf { items }
    }
    /// Returns a collection of elements that have the given tag, from within this element.
    ///
    /// MDN Reference: [`Element.getElementsByTagName()`](https://developer.mozilla.org/docs/Web/API/Element/getElementsByTagName)
    fn get_elements_by_tag_name(&self, qualified_name: &str) -> HTMLCollectionOf<Element> {
        todo!()
    }
    fn get_elements_by_tag_name_ns(
        &self,
        namespace: Option<&str>,
        local_name: &str,
    ) -> HTMLCollectionOf<Element> {
        todo!()
    }
    fn has_attribute(&self, qualified_name: &str) -> bool {
        self.attributes().get_named_item(qualified_name).is_some()
    }
    fn has_attribute_ns(&self, namespace: Option<&str>, qualified_name: &str) -> bool {
        todo!()
    }
    fn has_attributes(&self) -> bool {
        self.attributes().len() > 0
    }
    fn has_pointer_capture(&self, pointer_id: usize) -> bool {
        todo!()
    }
    fn insert_adjacent_element<T: AsElement>(
        &mut self,
        _where: InsertPosition,
        element: &mut T,
    ) -> Option<&T> {
        todo!()
    }
    fn insert_adjacent_html(&mut self, position: InsertPosition, text: &str) {
        todo!()
    }
    fn insert_adjacent_text(&mut self, data: &str) {
        todo!()
    }
    fn matches(&self, selectors: &str) -> bool {
        todo!()
    }
    fn release_pointer_capture(&mut self, pointer_id: usize) {
        todo!()
    }
    fn remove_attribute(&mut self, qualified_name: &str) {
        todo!()
    }
    fn remove_attribute_ns(&mut self, namespace: Option<&str>, local_name: &str) {
        todo!()
    }
    fn remove_attribute_node(&mut self, attr: &mut Attr) -> &Attr {
        todo!()
    }
    fn request_fullscreen(&self, options: Option<FullscreenOptions>) {
        todo!()
    }
    fn request_pointer_lock(&self) {
        todo!()
    }
    fn scroll(&mut self, options: Option<ScrollToOptions>) {
        todo!()
    }
    fn scroll_xy(&mut self, x: usize, y: usize) {
        todo!()
    }
    fn scroll_by(&mut self, options: Option<ScrollToOptions>) {
        todo!()
    }
    fn scroll_by_xy(&mut self, x: usize, y: usize) {
        todo!()
    }
    fn scroll_into_view(&mut self, arg: ScrollIntoView) {
        todo!()
    }
    fn scroll_to(&mut self, options: Option<ScrollToOptions>) {
        todo!()
    }
    fn scroll_to_xy(&mut self, x: usize, y: usize) {
        todo!()
    }
    fn set_attribute(&mut self, qualified_name: &str, value: &str) {
        let mut attr = self
            .owner_document()
            .unwrap()
            .create_attribute(qualified_name);
        attr.set_value(value.to_owned());
        self.attributes_mut().set_named_item(attr);
    }
    fn set_attribute_ns(&mut self, namespace: Option<&str>, qualified_name: &str, value: &str) {
        todo!()
    }
    fn set_attribute_node(&mut self, mut attr: Attr) -> Option<&Attr> {
        self.attributes_mut()
            .set_named_item(attr)
            .map(|attr| &*attr)
    }
    fn set_attribute_node_ns(&mut self, mut attr: Attr) -> Option<&Attr> {
        self.attributes_mut()
            .set_named_item_ns(attr)
            .map(|attr| &*attr)
    }
    fn set_pointer_capture(&mut self, pointer_id: usize) {
        todo!()
    }
    fn toggle_attribute(&mut self, qualified_name: &str, force: bool) -> bool {
        todo!()
    }
    #[deprecated]
    fn webkit_matches_selector(&mut self, selectors: &str) -> bool {
        todo!()
    }
}

pub struct ScrollIntoView;
