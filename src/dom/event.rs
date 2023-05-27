pub enum EventType {}

pub struct DOMHighResTimeStamp;

pub struct NoTarget;

impl EventTarget for () {}

pub trait Event<T: EventTarget = ()> {
    /// Returns true or false depending on how event was initialized. True if event goes through its target's ancestors in reverse tree order, and false otherwise.
    fn bubbles(&self) -> bool;
    #[deprecated]
    fn cancel_bubble(&self) -> bool;
    /// Returns true or false depending on how event was initialized. Its return value does not always carry meaning, but true can indicate that part of the operation during which event was dispatched, can be canceled by invoking the preventDefault() method.
    fn cancelable(&self) -> bool;
    /// Returns true or false depending on how event was initialized. True if event invokes listeners past a ShadowRoot node that is the root of its target, and false otherwise.
    fn composed(&self) -> bool;
    /// Returns the object whose event listener's callback is currently being invoked.
    fn current_target(&self) -> Option<&T>;
    /// Returns true if preventDefault() was invoked successfully to indicate cancelation, and false otherwise.
    fn default_prevented(&self) -> bool;
    /// Returns the event's phase, which is one of NONE, CAPTURING_PHASE, AT_TARGET, and BUBBLING_PHASE.
    fn event_phase(&self) -> u8;
    /// Returns true if event was dispatched by the user agent, and false otherwise.
    fn is_trusted(&self) -> bool;
    #[deprecated]
    fn return_value(&self) -> bool;
    #[deprecated]
    fn src_element(&self) -> Option<&T>;
    /// Returns the object to which event is dispatched (its target).
    fn target(&self) -> Option<&T>;
    /// Returns the event's timestamp as the number of milliseconds measured relative to the time origin.
    fn time_stamp(&self) -> DOMHighResTimeStamp;
    /// Returns the type of event, e.g. "click", "hashchange", or "submit".
    fn type_(&self) -> &EventType;
    /// Returns the invocation target objects of event's path (objects on which listeners will be invoked), except for any nodes in shadow trees of which the shadow root's mode is "closed" that are not reachable from event's currentTarget.
    fn composed_path(&self) -> [&T];
    #[deprecated]
    fn init_event(&mut self, r#type: impl Into<EventType>, bubbles: bool, cancelable: bool);
    /// If invoked when the cancelable attribute value is true, and while executing a listener for the event with passive set to false, signals to the operation that caused event to be dispatched that it needs to be canceled.
    fn prevent_default(&mut self);
    /// Invoking this method prevents event from reaching any registered event listeners after the current one finishes running and, when dispatched in a tree, also prevents event from reaching any other objects.
    fn stop_immediate_propagation(&mut self);
    /// When dispatched in a tree, invoking this method prevents event from reaching any objects other than the current object.
    fn stop_propagation(&mut self);

    const NONE: u8 = 0;
    const CAPTURING_PHASE: u8 = 1;
    const AT_TARGET: u8 = 2;
    const BUBBLING_PHASE: u8 = 3;
}

pub struct EventListenerObject;
impl EventListenerObject {
    pub fn handle_event<T: Event>(&self, _object: T) {
        todo!()
    }
}

pub enum EventListenerOrEventListenerObject<T: Event> {
    EventListener(fn(T) -> ()),
    EventListenerObject(EventListenerObject),
}

pub enum AddEventListenerOptionsOrBoolean {
    EventListenerOptions { capture: Option<bool> },
    Bool(bool),
}

/// EventTarget is a DOM interface implemented by objects that can receive events and may have listeners for them.
pub trait EventTarget {
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
    fn add_event_listener<T: Event>(
        &mut self,
        type_: impl Into<EventType>,
        callback: Option<EventListenerOrEventListenerObject<T>>,
        options: Option<AddEventListenerOptionsOrBoolean>,
    ) {
    }
    /// Dispatches a synthetic event event to target and returns true if either event's cancelable attribute value is false or its preventDefault() method was not invoked, and false otherwise.
    fn dispatch_event<T: Event>(&mut self, event: T) -> bool {
        false
    }
    /// Removes the event listener in target's event listener list with the same type, callback, and options.
    fn remove_event_listener<T: Event>(
        &mut self,
        type_: impl Into<EventType>,
        callback: Option<EventListenerOrEventListenerObject<T>>,
        options: Option<AddEventListenerOptionsOrBoolean>,
    ) {
    }
}
