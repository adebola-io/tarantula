use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    DOMTokenList, Event, EventDriven, EventListener, EventListenerOptions, EventName, HtmlNode,
    HtmlTag, NamedNodeMap, Query,
};

/// Base element struct.
struct Element<'a> {
    tag_name: HtmlTag<'a>,
    parent: Option<ElementRef<'a>>,
    attributes: NamedNodeMap<'a>,
    child_nodes: Vec<HtmlNode<'a>>,
    location: Option<[u32; 4]>,
    event_listeners: HashMap<
        EventName,
        Vec<(
            EventListener<'a, ElementRef<'a>>,
            Option<EventListenerOptions>,
        )>,
    >,
}
impl<'a> Element<'a> {
    fn new(tag_name: HtmlTag<'a>) -> Self {
        Self {
            tag_name,
            parent: None,
            attributes: NamedNodeMap::new(),
            child_nodes: vec![],
            location: None,
            event_listeners: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct ElementRef<'a> {
    element: Rc<RefCell<Element<'a>>>,
}

// Exclude references from being displayed.
impl std::fmt::Debug for ElementRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let element = &*self.element.borrow();
        let element_name = element.tag_name.to_string();
        let mut name_chars = element_name.chars();
        let formatted_name = format!(
            "{}{}Element",
            name_chars.next().unwrap().to_ascii_uppercase(),
            name_chars.collect::<String>()
        );
        f.debug_struct(formatted_name.as_str())
            .field("attributes", &element.attributes)
            .field("location", &element.location)
            .field("children", &element.child_nodes)
            .finish()
    }
}

impl<'a> PartialEq for ElementRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.element.as_ptr() == other.element.as_ptr()
    }
}

impl<'a> ElementRef<'a> {
    pub fn new(tag_name: HtmlTag<'a>) -> Self {
        Self {
            element: Rc::new(RefCell::new(Element::new(tag_name))),
        }
    }
}

impl<'a> From<&'a str> for ElementRef<'a> {
    fn from(tag: &'a str) -> Self {
        Self {
            element: Rc::new(RefCell::new(Element::new(HtmlTag::from(tag)))),
        }
    }
}

impl<'a> ElementRef<'a> {
    fn inner(&self) -> &mut Element<'a> {
        unsafe { &mut *(self.element.as_ptr()) }
    }
}

impl<'a> ElementRef<'a> {
    /// Returns the tag name of the element.
    fn tag_name(&self) -> &HtmlTag {
        &self.inner().tag_name
    }

    /// Return the attributes of the element.
    pub fn attributes(&self) -> &NamedNodeMap<'a> {
        &self.inner().attributes
    }

    /// Return the location of the element in the original source document, if it exists.
    pub fn location(&self) -> Option<&[u32; 4]> {
        self.inner().location.as_ref()
    }

    /// Returns the id of the element.
    pub fn id(&self) -> Option<&str> {
        self.element
            .borrow()
            .attributes
            .get_named_item("id")
            .map(|attr| attr.value)
    }

    /// Returns a reference to the list of class names applied.
    pub fn class_list(&self) -> &DOMTokenList<'a> {
        &self.inner().attributes.class_list
    }

    /// Returns a mutable reference to the list of class names.
    pub fn class_list_mut(&mut self) -> &mut DOMTokenList<'a> {
        &mut self.inner().attributes.class_list
    }

    /// Checks if the element has child elements.
    pub fn has_children(&'a self) -> bool {
        self.children().nth(0).is_some()
    }

    /// Checks if an element matches a selector.
    pub fn matches(&'a self, selector: &str) -> bool {
        todo!()
    }
}

impl<'a> ElementRef<'a> {
    /// Returns the parent of the element.
    pub fn parent(&'a self) -> Option<&'a ElementRef<'a>> {
        self.inner().parent.as_ref()
    }

    /// Returns the children elements of the element.
    pub fn children(&self) -> impl Iterator<Item = &ElementRef<'a>> {
        self.inner()
            .child_nodes
            .as_slice()
            .iter()
            .filter(|node| node.is_element())
            .map(|node| match node {
                HtmlNode::Element(element_ref) => element_ref,
                _ => unreachable!(),
            })
    }

    /// Add an element after the last child node in the element.
    pub fn append(&mut self, child: &mut ElementRef<'a>) {
        child.inner().parent = Some(self.clone());
        self.inner()
            .child_nodes
            .push(HtmlNode::Element(child.clone()))
    }

    /// Add an element before the first child node in the element.
    pub fn prepend(&mut self, child: &mut ElementRef<'a>) {
        child.inner().parent = Some(self.clone());
        self.inner()
            .child_nodes
            .insert(0, HtmlNode::Element(child.clone()))
    }

    /// Returns true if other is an inclusive descendant of node, and false otherwise.
    pub fn contains(&self, child: &ElementRef<'a>) -> bool {
        for _child in self.children() {
            if _child == child {
                return true;
            }
            if _child.contains(child) {
                return true;
            }
        }
        false
    }

    /// Disconnect a child element.
    pub fn remove_child(&mut self, child: &mut ElementRef<'a>) {
        let index = self
            .inner()
            .child_nodes
            .as_slice()
            .iter()
            .enumerate()
            .find(|tuple| match tuple.1 {
                HtmlNode::Element(element) => child == element,
                _ => false,
            })
            .map(|tuple| tuple.0);
        if let Some(index) = index {
            self.inner().child_nodes.remove(index);
        }
        child.inner().parent = None;
    }

    /// Disconnect the element from its parent.
    pub fn remove(&mut self) {
        let parent_option = &mut self.inner().parent;
        if let Some(parent) = parent_option {
            parent.inner().child_nodes.retain(|node| match node {
                HtmlNode::Element(element) => element != self,
                _ => true,
            })
        }
        self.inner().parent = None;
    }
}

impl<'a> Query<'a> for ElementRef<'a> {
    fn query_selector(&'a self, selector: &str) -> Option<&ElementRef> {
        for reference in self.children() {
            if reference.matches(selector) {
                return Some(reference);
            }
            let selector_match = reference.query_selector(selector);
            if selector_match.is_some() {
                return selector_match;
            }
        }
        None
    }

    fn query_selector_all(&'a self, selector: &str) -> Vec<&ElementRef> {
        let mut matches = vec![];
        for reference in self.children() {
            if reference.matches(selector) {
                matches.push(reference);
            }
            if reference.has_children() {
                matches.append(&mut reference.query_selector_all(selector));
            }
        }
        matches
    }

    fn get_elements_by_class_name(&'a self, class_name: &str) -> Vec<&ElementRef> {
        let mut matches = vec![];
        for child in self.children() {
            for child_class_list_item in &child.class_list().list {
                if child_class_list_item == &class_name {
                    matches.push(child);
                    break;
                }
            }

            if child.has_children() {
                matches.append(&mut child.get_elements_by_class_name(class_name));
            }
        }
        matches
    }

    fn get_element_by_id(&'a self, id: &str) -> Option<&ElementRef> {
        for reference in self.children() {
            if let Some(_id) = reference.id() {
                if _id == id {
                    return Some(reference);
                }
            }
            let id_match = reference.get_element_by_id(id);
            if id_match.is_some() {
                return id_match;
            }
        }
        None
    }

    fn get_elements_by_tag_name(&'a self, tag: &HtmlTag) -> Vec<&ElementRef> {
        let mut matches = vec![];
        for child in self.children() {
            if child.tag_name() == tag {
                matches.push(child);
            }
            if child.has_children() {
                matches.append(&mut child.get_elements_by_tag_name(tag));
            }
        }
        matches
    }

    fn find(&'a self, predicate: fn(element: &ElementRef) -> bool) -> Option<&ElementRef> {
        if !self.has_children() {
            return None;
        }
        for child in self.children() {
            if predicate(child) {
                return Some(child);
            }
            let element_match = child.find(predicate);
            if element_match.is_some() {
                return element_match;
            }
        }
        None
    }
}

impl<'a> EventDriven for ElementRef<'a> {
    fn add_event_listener<T>(
        &mut self,
        _type: T,
        listener: EventListener<Self>,
        options: Option<EventListenerOptions>,
    ) where
        T: Into<EventName>,
    {
        let event_name = _type.into();
        if let Some(listeners) = self.inner().event_listeners.get_mut(&event_name) {
            listeners.push((listener, options))
        } else {
            self.inner()
                .event_listeners
                .insert(event_name, vec![(listener, options)]);
        }
    }

    fn dispatch_event(event: Event<Self>) -> bool {
        todo!()
    }
}

impl<'a> ElementRef<'a> {
    fn bubble(&mut self, event: Event<Self>) {}

    /// Trigger a click event.
    pub fn click(&mut self) {
        self.trigger_event("click");
    }

    fn trigger_event(&mut self, event_type: &str) {
        let listener_option = self.inner().event_listeners.get(&EventName::Click);
        let mut event = Event::new(event_type, self.clone());
        if let Some(listeners) = listener_option {
            listeners.as_slice().iter().for_each(|listener| {
                let handler = listener.0;
                handler(self.clone(), &mut event)
            });
        }
        if let Some(parent) = &mut self.inner().parent {
            parent.bubble(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dom::ElementRef;

    #[test]
    fn element_appending() {
        let mut div = ElementRef::from("div");
        let mut span = ElementRef::from("span");
        div.append(&mut span);

        assert!(div.contains(&span));
    }

    #[test]
    fn element_removal() {
        let mut ul = ElementRef::from("ul");
        let mut li = ElementRef::from("li");
        ul.append(&mut li);

        assert_eq!(ul.children().next(), Some(&li));

        li.remove();

        assert!(!ul.contains(&li));
    }

    #[test]
    fn element_nesting() {
        let mut ul = ElementRef::from("ul");
        let mut li = ElementRef::from("li");
        let mut a = ElementRef::from("a");
        let mut span = ElementRef::from("span");

        ul.append(&mut li);
        li.append(&mut a);
        a.append(&mut span);

        assert!(ul.contains(&span));
    }

    #[test]
    fn tests_clicking() {
        let mut ul = ElementRef::from("ul");
    }
}
