use crate::DOMException;

pub trait InnerHtml {
    /// Returns the value of the HTML or SVG markup contained.
    fn inner_html(&self) -> String;
    /// Sets the value of the HTML or SVG markup contained.
    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException>;
}
