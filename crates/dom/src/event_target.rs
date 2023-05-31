use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::AddEventListenerOptionsOrBoolean;

use super::{Event, EventType};

#[derive(Debug, Clone, PartialEq)]
pub struct EventListenerObject;
impl EventListenerObject {
    pub fn handle_event(&self, _object: &mut Event) {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventListenerOrEventListenerObject {
    EventListener(()),
    EventListenerObject(EventListenerObject),
}

pub type EventTargetRef = Rc<RefCell<EventTarget>>;

#[derive(Debug, Clone, PartialEq)]
/// EventTarget is a DOM interface implemented by objects that can receive events and may have listeners for them.
pub struct EventTarget {
    pub listeners: HashMap<
        EventType,
        Vec<(
            Option<EventListenerOrEventListenerObject>,
            Option<AddEventListenerOptionsOrBoolean>,
        )>,
    >,
}

impl EventTarget {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
}

impl AsEventTarget for EventTarget {
    #[inline(always)]
    fn cast(&self) -> &EventTarget {
        self
    }
    #[inline(always)]
    fn cast_mut(&mut self) -> &mut EventTarget {
        self
    }
}

pub trait AsEventTarget: Sized {
    /// Convert to a reference to event target.
    fn cast(&self) -> &EventTarget;
    /// Convert to a mutable reference to event target.
    fn cast_mut(&mut self) -> &mut EventTarget;
    /// Appends an event listener for events whose type attribute value is type. The callback argument sets the callback that will be invoked when the event is dispatched.
    ///
    /// The options argument sets listener-specific options. For compatibility this can be a boolean, in which case the method behaves exactly as if the value was specified as options's capture.
    ///
    /// When set to true, options's capture prevents callback from being invoked when the event's eventPhase attribute value is BUBBLING_PHASE. When false (or not present), callback will not be invoked when event's eventPhase attribute value is CAPTURING_PHASE. Either way, callback will be invoked if event's eventPhase attribute value is AT_TARGET.
    ///  
    /// When set to true, options's passive indicates that the callback will not cancel the event by invoking preventDefault(). This is used to enable performance optimizations described in § 2.8 Observing event listeners.
    ///  
    /// When set to true, options's once indicates that the callback will only be invoked once after which the event listener will be removed.
    ///  
    /// If an AbortSignal is passed for options's signal, then the event listener will be removed when signal is aborted.
    ///  
    /// The event listener is appended to target's event listener list and is not appended if it has the same type, callback, and capture.
    fn add_event_listener(
        &mut self,
        type_: impl Into<EventType>,
        callback: Option<EventListenerOrEventListenerObject>,
        options: Option<AddEventListenerOptionsOrBoolean>,
    ) {
        let target = self.cast_mut();
        let type_ = type_.into();

        match target.listeners.get_mut(&type_) {
            Some(listeners) => {
                listeners.push((callback, options));
            }
            None => {
                target.listeners.insert(type_, vec![(callback, options)]);
            }
        };
    }
    /// Dispatches a synthetic event event to target and returns true if either event's cancelable attribute value is false or its preventDefault() method was not invoked, and false otherwise.
    fn dispatch_event(&mut self, event: &mut Event) -> bool {
        false
    }
    /// Removes the event listener in target's event listener list with the same type, callback, and options.
    fn remove_event_listener(
        &mut self,
        type_: impl Into<EventType>,
        callback: Option<EventListenerOrEventListenerObject>,
        options: Option<AddEventListenerOptionsOrBoolean>,
    ) {
        todo!()
    }
}
