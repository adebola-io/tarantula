// use std::{cell::RefCell, rc::Rc};

// use crate::{Event, HtmlCollectionOf, Tag};

// use super::{ChildNode, AsNode, AsParentNode};

// pub type Element = dyn AsElement;
// pub type ElementRef = Rc<RefCell<Element>>;

// pub struct NamedNodeMap;
// impl NamedNodeMap {
//     fn set(&mut self, arg: &str, value: String) {
//         todo!()
//     }

//     fn get(&self, arg: &str) -> Option<&str> {
//         todo!()
//     }

//     fn len(&self) -> usize {
//         todo!()
//     }
// }
// pub struct DOMTokenList;
// pub struct ShadowRootInit;
// pub struct ShadowRootRef;
// pub struct CheckVisibilityOptions;
// pub struct Attr {
//     pub name: String,
// }
// pub struct DOMRect;
// pub struct DOMRectList;

// pub trait AsElement: AsNode + ChildNode + AsParentNode + internal::AsElementInner {
//     fn as_element(&self) -> &Element;
//     fn as_element_mut(&mut self) -> &mut Element;
//     fn attributes(&self) -> &NamedNodeMap {
//         &self.z_as_element().attributes
//     }
//     fn attributes_mut(&mut self) -> &mut NamedNodeMap {
//         &mut self.z_as_element_mut().attributes
//     }
//     fn class_list(&self) -> &DOMTokenList {
//         &self.z_as_element().class_list
//     }
//     fn class_list_mut(&mut self) -> &mut DOMTokenList {
//         &mut self.z_as_element_mut().class_list
//     }
//     /// Returns the value of element's class content attribute
//     fn class_name(&self) -> &str {
//         &self.z_as_element().class_name
//     }
//     /// Sets the value of the element's class content attribute.
//     fn set_class_name(&mut self, value: &str) {
//         self.z_as_element_mut().class_name = value.to_owned();
//     }
//     fn client_height(&self) -> usize {
//         self.z_as_element().client_height
//     }
//     fn client_left(&self) -> usize {
//         self.z_as_element().client_left
//     }
//     fn client_top(&self) -> usize {
//         self.z_as_element().client_top
//     }
//     fn client_width(&self) -> usize {
//         self.z_as_element().client_width
//     }
//     /// Returns the value of element's id content attribute.
//     fn id(&self) -> &str {
//         self.z_as_element().attributes.get("id").unwrap_or("")
//     }
//     /// Sets the value of element's id content attribute.
//     fn set_id(&mut self, value: &str) {
//         self.z_as_element_mut()
//             .attributes
//             .set("id", value.to_owned());
//     }
//     /// Returns the local name.
//     fn local_name(&self) -> &str {
//         &self.z_as_element().local_name
//     }
//     /// Returns the namespace.
//     fn namespace_uri(&self) -> Option<&str> {
//         self.z_as_element()
//             .namespace_uri
//             .as_ref()
//             .map(|x| x.as_str())
//     }
//     fn outer_html(&self) -> &str {
//         todo!()
//     }
//     fn set_outer_html(&mut self, value: &str) {
//         todo!()
//     }
//     fn owner_document(&self) -> Option<&crate::Document> {
//         AsNode::owner_document(self)
//     }
//     fn owner_document_mut(&mut self) -> Option<&mut crate::Document> {
//         AsNode::owner_document_mut(self)
//     }
//     fn part(&self) -> &DOMTokenList {
//         &self.z_as_element().part
//     }
//     fn part_mut(&mut self) -> &mut DOMTokenList {
//         &mut self.z_as_element_mut().part
//     }
//     /// Returns the namespace prefix.
//     fn prefix(&self) -> Option<&str> {
//         self.z_as_element().prefix.as_ref().map(|x| x.as_str())
//     }
//     fn scroll_height(&self) -> usize {
//         self.z_as_element().scroll_height
//     }
//     fn scroll_left(&self) -> usize {
//         self.z_as_element().scroll_left
//     }
//     fn set_scroll_left(&mut self, value: usize) {
//         self.z_as_element_mut().scroll_left = value
//     }
//     fn scroll_top(&self) -> usize {
//         self.z_as_element().scroll_top
//     }
//     fn set_scroll_top(&mut self, value: usize) {
//         self.z_as_element_mut().scroll_top = value;
//     }
//     fn scroll_width(&self) -> usize {
//         self.z_as_element().scroll_width
//     }
//     /// Returns element's shadow root, if any, and if shadow root's mode is "open", and null otherwise.
//     fn shadow_root(&self) -> Option<&ShadowRootRef> {
//         self.z_as_element().shadow_root.as_ref()
//     }
//     /// Returns element's shadow root mutably, if any, and if shadow root's mode is "open", and null otherwise.
//     fn shadow_root_mut(&mut self) -> Option<&mut ShadowRootRef> {
//         self.z_as_element_mut().shadow_root.as_mut()
//     }
//     /// Returns the value of element's slot content attribute.
//     fn slot(&self) -> &str {
//         self.attributes().get("slot").unwrap_or("")
//     }
//     /// Sets the value of element's slot content attribute.
//     fn set_slot(&mut self, value: &str) {
//         self.attributes_mut().set("slot", value.to_owned())
//     }
//     /// Returns the HTML-uppercased qualified name.
//     fn tag_name(&self) -> &str {
//         self.z_as_element().tag.as_str()
//     }
//     /// Creates a shadow root for element and returns it.
//     fn attach_shadow(&mut self, init: ShadowRootInit) -> &ShadowRootRef {
//         todo!()
//     }
//     fn check_visibility(&self, options: Option<CheckVisibilityOptions>) -> bool {
//         todo!()
//     }
//     /// Returns the first (starting at element) inclusive ancestor that matches selectors, and None otherwise.
//     fn closest(&self, selector: &str) -> Option<&ElementRef> {
//         todo!()
//     }
//     /// Returns the first (starting at element) inclusive ancestor that matches selectors mutably, and `None` otherwise.
//     fn closest_mut(&mut self, selector: &str) -> Option<&mut ElementRef> {
//         todo!()
//     }
//     /// Returns element's first attribute whose qualified name is `qualified_name`, and `None` if there is no such attribute otherwise.
//     fn get_attribute(&self, qualified_name: &str) -> Option<&str> {
//         todo!()
//     }
//     /// Returns element's attribute whose namespace is `namespace` and local name is `local_name`, and `None` if there is no such attribute otherwise.
//     fn get_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<&str> {
//         todo!()
//     }
//     /// Returns the qualified names of all element's attributes. Can contain duplicates.
//     fn get_attribute_names(&self) -> &[&str] {
//         todo!()
//     }
//     fn get_attribute_node(&self) -> Option<&Attr> {
//         todo!()
//     }
//     fn get_attribute_node_mut(&mut self) -> Option<&mut Attr> {
//         todo!()
//     }
//     fn get_attribute_node_ns(&self, namespace: Option<&str>, local_name: &str) -> Option<&Attr> {
//         todo!()
//     }
//     fn get_bounding_client_rect(&self) -> &DOMRect {
//         todo!()
//     }
//     fn get_bounding_client_rect_mut(&mut self) -> &mut DOMRect {
//         todo!()
//     }
//     fn get_client_rects(&self) -> &DOMRectList {
//         todo!()
//     }
//     fn get_client_rects_mut(&mut self) -> &mut DOMRectList {
//         todo!()
//     }
//     /// Returns a HTMLCollection of the elements in the object on which the method was invoked (a document or an element) that have all the classes given by classNames. The classNames argument is interpreted as a space-separated list of classes.
//     fn get_elements_by_class_name(&self, class_names: &str) -> HtmlCollectionOf<Element> {
//         todo!()
//     }
//     fn get_elements_by_tag_name(&self, qualified_name: Tag) -> HtmlCollectionOf<Element> {
//         todo!()
//     }
//     fn get_elements_by_tag_name_ns(&self, namespace_uri: &str) -> HtmlCollectionOf<Element> {
//         todo!()
//     }
//     /// Returns true if element has an attribute whose qualified name is `qualified_name`, and false otherwise.
//     fn has_attribute(&self, qualified_name: &str) -> bool {
//         self.z_as_element().attributes.get(qualified_name).is_some()
//     }
//     /// Returns true if element has an attribute whose namespace is namespace and local name is `local_name`.
//     fn has_attribute_ns(&self, namespace: Option<&str>, local_name: &str) -> bool {
//         todo!()
//     }
//     /// Returns true if element has attributes, and false otherwise.
//     fn has_attributes(&self) -> bool {
//         self.attributes().len() > 0
//     }
//     fn has_pointer_capture(&self, pointer_id: usize) -> bool {
//         todo!()
//     }
// }

// #[doc(hidden)]
// pub(crate) mod internal {
//     use crate::{DOMTokenList, Document, NamedNodeMap, ShadowRootRef, Tag};

//     pub struct ElementInner {
//         pub attributes: NamedNodeMap,
//         pub class_list: DOMTokenList,
//         pub class_name: String,
//         pub client_height: usize,
//         pub client_left: usize,
//         pub client_top: usize,
//         pub client_width: usize,
//         pub local_name: String,
//         pub namespace_uri: Option<String>,
//         pub owner_document: *mut Document,
//         pub part: DOMTokenList,
//         pub prefix: Option<String>,
//         pub scroll_height: usize,
//         pub scroll_left: usize,
//         pub scroll_top: usize,
//         pub scroll_width: usize,
//         pub shadow_root: Option<ShadowRootRef>,
//         pub tag: Tag,
//     }

//     pub trait AsElementInner {
//         fn z_as_element(&self) -> &ElementInner;
//         fn z_as_element_mut(&mut self) -> &mut ElementInner;
//     }
// }
