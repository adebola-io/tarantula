#[derive(Eq, Hash, PartialEq)]
pub enum EventName {
    Abort,
    Click,
    Close,
    MouseOver,
    MouseIn,
    Unknown,
}

impl From<&str> for EventName {
    fn from(event_name: &str) -> Self {
        match event_name {
            "abort" => Self::Abort,
            "close" => Self::Close,
            "click" => Self::Click,
            _ => EventName::Unknown,
        }
    }
}

pub trait EventDriven: Clone + Sized {
    /// Appends an event listener for events whose type attribute value is type.
    /// The callback argument sets the callback that will be invoked when the event is dispatched.
    fn add_event_listener<T: Into<EventName>>(
        &mut self,
        _type: T,
        listener: EventListener<Self>,
        options: Option<EventListenerOptions>,
    );
    /// Dispatches a synthetic event event to target and returns true if either event's cancelable attribute value is false or its preventDefault() method was not invoked, and false otherwise.
    fn dispatch_event(event: Event<Self>) -> bool;
}

pub type EventListener<'a, T> = fn(T, &mut Event<T>) -> ();

pub enum EventPhase {
    AtTarget,
    BubblingPhase,
    CapturingPhase,
}

pub struct Event<TargetType: EventDriven> {
    pub name: EventName,
    pub phase: EventPhase,
    pub bubbles: bool,
    pub target: TargetType,
}

impl<TargetType> Event<TargetType>
where
    TargetType: EventDriven,
{
    pub(crate) fn new(event_type: &str, target: TargetType) -> Self {
        Event {
            name: event_type.into(),
            phase: EventPhase::AtTarget,
            bubbles: true,
            target,
        }
    }
}

pub struct EventListenerOptions {
    pub capture: Option<bool>,
    pub once: Option<bool>,
    pub passive: Option<bool>,
    pub signal: Option<AbortSignal>,
}

pub struct AbortSignal;
