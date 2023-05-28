use super::EventTargetRef;

#[derive(Eq, Hash, PartialEq)]
pub enum EventType {
    Click,
    MouseOver,
    MouseOut,
}

/// An event which takes place in the DOM.
pub type Event = dyn IntoEvent;
pub type DOMHighResTimeStamp = usize;

#[allow(deprecated)]
pub trait IntoEvent: event_internal::AsEventInner {
    /// Returns true or false depending on how event was initialized. True if event goes through its target's ancestors in reverse tree order, and false otherwise.
    fn bubbles(&self) -> bool {
        self.__as_event().bubbles
    }
    #[deprecated]
    fn cancel_bubble(&self) -> bool {
        self.__as_event().cancel_bubble
    }
    #[deprecated]
    fn set_cancel_bubble(&mut self, value: bool) {
        self.__as_event_mut().cancel_bubble = value;
    }
    /// Returns true or false depending on how event was initialized. Its return value does not always carry meaning, but true can indicate that part of the operation during which event was dispatched, can be canceled by invoking the preventDefault() method.
    fn cancelable(&self) -> bool {
        self.__as_event().cancelable
    }
    /// Returns true or false depending on how event was initialized. True if event invokes listeners past a ShadowRoot node that is the root of its target, and false otherwise.
    fn composed(&self) -> bool {
        self.__as_event().composed
    }
    /// Returns the object whose event listener's callback is currently being invoked.
    fn current_target(&self) -> Option<&EventTargetRef> {
        self.__as_event().current_target.as_ref()
    }
    /// Returns mutable reference to the  object whose event listener's callback is currently being invoked.
    fn current_target_mut(&mut self) -> Option<&mut EventTargetRef> {
        self.__as_event_mut().current_target.as_mut()
    }
    /// Returns true if preventDefault() was invoked successfully to indicate cancelation, and false otherwise.
    fn default_prevented(&self) -> bool {
        self.__as_event().default_prevented
    }
    /// Returns the event's phase, which is one of NONE, CAPTURING_PHASE, AT_TARGET, and BUBBLING_PHASE.
    fn event_phase(&self) -> &EventPhase {
        &self.__as_event().event_phase
    }
    /// Returns true if event was dispatched by the user agent, and false otherwise.
    fn is_trusted(&self) -> bool {
        self.__as_event().is_trusted
    }
    #[deprecated]
    fn return_value(&self) -> bool {
        self.__as_event().return_value
    }
    #[deprecated]
    fn set_return_value(&mut self, value: bool) {
        self.__as_event_mut().return_value = value;
    }
    #[deprecated]
    fn src_element(&self) -> Option<&EventTargetRef> {
        self.__as_event().src_element.as_ref()
    }
    #[deprecated]
    fn src_element_mut(&mut self) -> Option<&mut EventTargetRef> {
        self.__as_event_mut().src_element.as_mut()
    }
    /// Returns the object to which event is dispatched (its target).
    fn target(&self) -> Option<&EventTargetRef> {
        self.__as_event().target.as_ref()
    }
    /// Returns mutable reference to the object to which event is dispatched (its target).
    fn target_mut(&mut self) -> Option<&mut EventTargetRef> {
        self.__as_event_mut().target.as_mut()
    }
    /// Returns the event's timestamp as the number of milliseconds measured relative to the time origin.
    fn time_stamp(&self) -> &DOMHighResTimeStamp {
        &self.__as_event().time_stamp
    }
    /// Returns the type of event, e.g. "click", "hashchange", or "submit".
    fn type_(&self) -> &EventType {
        &self.__as_event().type_
    }
    /// Returns the invocation target objects of event's path (objects on which listeners will be invoked), except for any nodes in shadow trees of which the shadow root's mode is "closed" that are not reachable from event's currentTarget.
    fn composed_path(&self) -> Vec<&EventTargetRef> {
        self.__as_event().__composed_path()
    }
    #[deprecated]
    fn init_event(&mut self, r#type: EventType, bubbles: bool, cancelable: bool) {
        self.__as_event_mut()
            .__init_event(r#type, bubbles, cancelable)
    }
    /// If invoked when the cancelable attribute value is true, and while executing a listener for the event with passive set to false, signals to the operation that caused event to be dispatched that it needs to be canceled.
    fn prevent_default(&mut self) {
        self.__as_event_mut().__prevent_default()
    }
    /// Invoking this method prevents event from reaching any registered event listeners after the current one finishes running and, when dispatched in a tree, also prevents event from reaching any other objects.
    fn stop_immediate_propagation(&mut self) {
        self.__as_event_mut().__stop_immediate_propagation()
    }
    /// When dispatched in a tree, invoking this method prevents event from reaching any objects other than the current object.
    fn stop_propagation(&mut self) {
        self.__as_event_mut().__stop_propagation()
    }
}

#[derive(PartialEq)]
pub enum EventPhase {
    None = 0,
    CapturingPhase = 1,
    AtTarget = 2,
    BubblingPhase = 3,
}

pub(crate) mod event_internal {
    use super::{DOMHighResTimeStamp, EventPhase, EventType};
    use crate::EventTargetRef;
    pub struct EventInner {
        pub type_: EventType,
        pub bubbles: bool,
        #[deprecated]
        pub cancel_bubble: bool,
        pub cancelable: bool,
        pub composed: bool,
        pub current_target: Option<EventTargetRef>,
        pub default_prevented: bool,
        pub event_phase: EventPhase,
        pub is_trusted: bool,
        pub return_value: bool,
        #[deprecated]
        pub src_element: Option<EventTargetRef>,
        pub target: Option<EventTargetRef>,
        pub time_stamp: DOMHighResTimeStamp,
    }
    impl EventInner {
        /// Returns the invocation target objects of event's path (objects on which listeners will be invoked), except for any nodes in shadow trees of which the shadow root's mode is "closed" that are not reachable from event's currentTarget.
        pub fn __composed_path(&self) -> Vec<&EventTargetRef> {
            todo!()
        }
        #[deprecated]
        pub fn __init_event(&mut self, r#type: EventType, bubbles: bool, cancelable: bool) {
            todo!()
        }
        /// If invoked when the cancelable attribute value is true, and while executing a listener for the event with passive set to false, signals to the operation that caused event to be dispatched that it needs to be canceled.
        pub fn __prevent_default(&mut self) {
            todo!()
        }
        /// Invoking this method prevents event from reaching any registered event listeners after the current one finishes running and, when dispatched in a tree, also prevents event from reaching any other objects.
        pub fn __stop_immediate_propagation(&mut self) {
            todo!()
        }
        /// When dispatched in a tree, invoking this method prevents event from reaching any objects other than the current object.
        pub fn __stop_propagation(&mut self) {
            todo!()
        }
    }
    pub trait AsEventInner {
        /// Convert to a reference to event.
        fn __as_event(&self) -> &EventInner;
        /// Convert to a mutable reference to event.
        fn __as_event_mut(&mut self) -> &mut EventInner;
    }
}
