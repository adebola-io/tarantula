pub type EventListenerOptions = dyn IntoEventListenerOptions;
pub trait IntoEventListenerOptions {
    fn capture(&self) -> Option<bool>;
    fn set_capture(&mut self, value: Option<bool>);
}

pub struct AddEventListenerOptions {
    pub once: Option<bool>,
    pub passive: Option<bool>,
    pub signal: Option<AbortSignal>,
    pub capture: Option<bool>,
}
impl IntoEventListenerOptions for AddEventListenerOptions {
    fn capture(&self) -> Option<bool> {
        self.capture
    }
    fn set_capture(&mut self, value: Option<bool>) {
        self.capture = value
    }
}
pub enum AddEventListenerOptionsOrBoolean {
    EventListenerOptions(AddEventListenerOptions),
    Bool(bool),
}

pub struct AbortSignal;
