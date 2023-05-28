use std::{cell::RefCell, rc::Weak};

pub type DocumentRef = Weak<RefCell<Document>>;

pub struct Document {
    pub URL: String,
}
