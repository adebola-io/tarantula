// use std::ops::Index;

use crate::{AsElement, AsNode, Attr, DOMException, Element};

pub(crate) enum ListType {
    ClassList,
    RelList,
    HtmlFor,
    SandBox,
    Part,
}

impl ListType {
    fn as_str(&self) -> &str {
        match self {
            ListType::ClassList => "class",
            ListType::RelList => "rel",
            ListType::HtmlFor => todo!(),
            ListType::SandBox => todo!(),
            ListType::Part => "part",
        }
    }
}

/// A set of space-separated tokens. Such a set is returned by Element.classList, HTMLLinkElement.relList, HTMLAnchorElement.relList, HTMLAreaElement.relList, HTMLIframeElement.sandbox, or HTMLOutputElement.htmlFor. It is indexed beginning with 0 as with JavaScript Array objects. DOMTokenList is always case-sensitive.
///
/// [MDN Reference](https://developer.mozilla.org/docs/Web/API/DOMTokenList)
pub struct MutDOMTokenList<'a> {
    owner_element: &'a mut Element,
    list_type: ListType,
}

/// A set of space-separated tokens. Such a set is returned by Element.classList, HTMLLinkElement.relList, HTMLAnchorElement.relList, HTMLAreaElement.relList, HTMLIframeElement.sandbox, or HTMLOutputElement.htmlFor. It is indexed beginning with 0 as with JavaScript Array objects. DOMTokenList is always case-sensitive.
///
/// [MDN Reference](https://developer.mozilla.org/docs/Web/API/DOMTokenList)
pub struct DOMTokenList<'a> {
    owner_element: &'a Element,
    list_type: ListType,
}

impl<'a> MutDOMTokenList<'a> {
    /// Create a token list from an attribute.
    pub(crate) fn from_element(owner_element: &'a mut Element, list_type: ListType) -> Self {
        Self {
            owner_element,
            list_type,
        }
    }
    fn owner_attribute(&self) -> Option<&Attr> {
        self.owner_element
            .get_attribute_node(self.list_type.as_str())
    }
    fn owner_attribute_mut(&mut self) -> Option<&mut Attr> {
        self.owner_element
            .get_attribute_node_mut(self.list_type.as_str())
    }
    /// Returns the number of tokens in the list.
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut div = document.create_element("div");
    ///
    /// assert_eq!(div.class_list_mut().len(), 0);
    ///
    /// div.class_list_mut().add("box").unwrap();
    /// assert_eq!(div.class_list_mut().len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.owner_attribute()
            .map(|attribute| attribute.value().split(' ').count())
            .unwrap_or(0)
    }
    /// Returns true if the list contains a specified token.
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut div = document.create_element("div");
    ///
    /// div.class_list_mut().add("font-bold").unwrap();
    /// div.class_list_mut().add("font-product-sans").unwrap();
    /// div.class_list_mut().add("text-red-400").unwrap();
    ///
    /// assert!(div.class_list_mut().contains("font-bold"));
    /// ```
    pub fn contains(&self, token: &str) -> bool {
        self.owner_attribute()
            .map(|attribute| attribute.value().split(' ').any(|_token| _token == token))
            .unwrap_or(false)
    }
    /// Returns the token with index `index`
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut div = document.create_element("div");
    ///
    /// div.set_class_name("card first-card emphasis card-depth");
    /// assert_eq!(div.class_list_mut().item(0).unwrap(), "card");
    /// assert_eq!(div.class_list_mut().item(1).unwrap(), "first-card");
    /// ```
    pub fn item(&self, index: usize) -> Option<&str> {
        self.owner_attribute()
            .map(|attribute| attribute.value().split(' ').nth(index))
            .flatten()
    }
    /// Adds all arguments passed, except those already present.
    /// # Errors
    /// - Returns a `SyntaxError` DOMException if  the argument is the empty string.
    /// - Returns an `InvalidCharacterError` DOMException if the argument contains any ASCII whitespace.
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    ///
    /// let mut p = document.create_element("p");
    /// let mut class_list = p.class_list_mut();
    /// class_list.add("text-red").unwrap();
    ///
    /// assert!(p.has_attribute("class"));
    /// assert_eq!(p.get_attribute("class").unwrap(), "text-red")
    /// ```
    pub fn add(&mut self, token: &str) -> Result<(), DOMException> {
        validate_token(token)?;
        if self.owner_attribute().is_none() {
            let mut attr = self
                .owner_element
                .owner_document()
                .unwrap()
                .create_attribute(self.list_type.as_str());
            attr.set_value(String::new());
            self.owner_element.attributes_mut().set_named_item(attr);
        }
        if !self.contains(token) {
            let attr = self.owner_attribute_mut().unwrap();
            if !attr.value().is_empty() {
                attr.__value.as_mut().unwrap().push(' ');
            }
            attr.__value.as_mut().unwrap().push_str(token);
        }
        Ok(())
    }
    /// Removes the arguments passed, if they are present.
    /// # Errors
    /// - Returns a `SyntaxError` DOMException if the argument is the empty string.
    /// - Returns an `InvalidCharacterError` DOMException if the argument contains any ASCII whitespace.
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut div = document.create_element("div");
    /// div.set_attribute("class", "bg-blue-300 text-black");
    ///
    /// div.class_list_mut().remove("bg-blue-300").unwrap();
    ///
    /// assert_eq!(div.get_attribute("class").unwrap(), "text-black");
    /// ```
    pub fn remove(&mut self, token: &str) -> Result<(), DOMException> {
        validate_token(token)?;
        let mut string = String::new();
        if let Some(attr) = self.owner_attribute_mut() {
            for value in attr.value().split(' ') {
                if value != token {
                    if !string.is_empty() {
                        string.push(' ')
                    }
                    string.push_str(value)
                }
            }
            attr.set_value(string);
        }
        Ok(())
    }
    /// Replaces a token with another token.
    ///
    /// It returns true if the token was replaced, and false otherwise (the token to replace does not exist in the list).
    ///
    /// # Errors
    /// - Returns a `SyntaxError` [`DOMException`] if one of the arguments is the empty string.
    /// - Returns an `InvalidCharacterError` [`DOMException`] if one of the arguments contains any ASCII whitespace.
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    ///
    /// let mut div = document.create_element("div");
    /// div.set_class_name("bg-black txt-blue");
    /// let replaced = div.class_list_mut().replace("txt-blue", "txt-green").unwrap();
    ///
    /// assert!(replaced);
    /// assert_eq!(div.class_name(), "bg-black txt-green");
    /// ```
    pub fn replace(&mut self, token: &str, new_token: &'a str) -> Result<bool, DOMException> {
        validate_token(token)?;
        validate_token(new_token)?;
        let mut is_changed = false;
        if let Some(attr) = self.owner_attribute_mut() {
            attr.set_value(
                attr.value()
                    .split(' ')
                    .map(|old_token| {
                        if old_token == token {
                            is_changed = true;
                            new_token
                        } else {
                            old_token
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join(" "),
            );
        }

        Ok(is_changed)
    }
    /// Returns true if the token is in the associated attribute's supported tokens.
    /// # Errors
    /// Returns a `TypeError` if the associated attribute has no supported tokens defined.
    pub fn supports(_token: &str) -> Result<bool, DOMException> {
        unimplemented!("This feature of the DOM is yet to be implemented.")
    }
    /// Toggles a token, removing if it is present and adding it otherwise.
    ///
    /// It returns a boolean representing whether the token is now in the list or not.
    /// # Errors
    /// - Returns a `SyntaxError` [`DOMException`] if one of the arguments is the empty string.
    /// - Returns an `InvalidCharacterError` [`DOMException`] if one of the arguments contains any ASCII whitespace.
    /// # Examples
    /// Toggling without the force flag.
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut a = document.create_element("a");
    ///
    /// // Toggling once.
    /// let added = a.class_list_mut().toggle("underline", None);
    /// assert!(added.unwrap());
    /// assert!(a.class_list().contains("underline"));
    ///
    /// // Toggling twice.
    /// let added = a.class_list_mut().toggle("underline", None);
    /// assert!(!added.unwrap());
    /// assert!(!a.class_list().contains("underline"));
    /// ```
    ///
    /// Toggling with the flag.<br>
    ///
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut body = document.create_element("body");
    ///
    /// // Toggling once.
    /// body.class_list_mut().toggle("base", Some(true)).unwrap();
    /// assert!(body.class_list().contains("base"));
    /// // Toggling twice.
    /// body.class_list_mut().toggle("base", Some(true)).unwrap();
    /// assert!(body.class_list().contains("base"));
    ///
    /// // With flag set to false
    /// // Toggling once
    /// body.class_list_mut().toggle("base", Some(false)).unwrap();
    /// assert!(!body.class_list().contains("base"));
    /// // Toggling twice.
    /// body.class_list_mut().toggle("base", Some(false)).unwrap();
    /// assert!(!body.class_list().contains("base"));
    /// ```
    pub fn toggle(&mut self, token: &str, force: Option<bool>) -> Result<bool, DOMException> {
        match force {
            Some(true) => {
                self.add(token)?;
                Ok(true)
            }
            Some(false) => {
                self.remove(token)?;
                Ok(false)
            }
            None => Ok(if self.contains(token) {
                self.remove(token)?;
                false
            } else {
                self.add(token)?;
                true
            }),
        }
    }
}

// impl<'a> Index<usize> for MutDOMTokenList<'a> {
//     type Output = &'a str;
//     fn index(&self, index: usize) -> &Self::Output {
//         match self.owner_attribute() {
//             Some(attr) => {
//                 if let Some(value) = self.item(index) {
//                     &value
//                 } else {
//                     panic!(
//                         "Index out of bounds for DOMTokenList. No value at index {}",
//                         index
//                     )
//                 }
//             }
//             None => panic!(
//                 "Index out of bounds for DOMTokenList. No value at index {}",
//                 index,
//             ),
//         }
//     }
// }

impl<'a> DOMTokenList<'a> {
    /// Create a token list from an attribute.
    pub(crate) fn from_element(owner_element: &'a Element, list_type: ListType) -> Self {
        Self {
            owner_element,
            list_type,
        }
    }
    /// Returns the attribute of the list if it exists.
    fn owner_attribute(&self) -> Option<&Attr> {
        self.owner_element
            .get_attribute_node(self.list_type.as_str())
    }
    /// Returns the number of tokens in the list.
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut div = document.create_element("div");
    ///
    /// assert_eq!(div.class_list_mut().len(), 0);
    ///
    /// div.class_list_mut().add("box").unwrap();
    /// assert_eq!(div.class_list().len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.owner_attribute()
            .map(|attribute| attribute.value().split(' ').count())
            .unwrap_or(0)
    }
    /// Returns true if the list contains a specified token.
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut div = document.create_element("div");
    ///
    /// div.class_list_mut().add("font-bold").unwrap();
    /// div.class_list_mut().add("font-product-sans").unwrap();
    /// div.class_list_mut().add("text-red-400").unwrap();
    ///
    /// assert!(div.class_list().contains("font-bold"));
    /// ```
    pub fn contains(&self, token: &str) -> bool {
        self.owner_attribute()
            .map(|attribute| attribute.value().split(' ').any(|_token| _token == token))
            .unwrap_or(false)
    }
    /// Returns the token with index `index`
    /// # Example
    /// ```
    /// use dom::{Document, AsElement};
    ///
    /// let document = Document::new();
    /// let mut div = document.create_element("div");
    ///
    /// div.set_class_name("card first-card emphasis card-depth");
    /// assert_eq!(div.class_list().item(0).unwrap(), "card");
    /// assert_eq!(div.class_list().item(1).unwrap(), "first-card");
    /// ```
    pub fn item(&self, index: usize) -> Option<&str> {
        self.owner_attribute()
            .map(|attribute| attribute.value().split(' ').nth(index))
            .flatten()
    }
    /// Returns true if the token is in the associated attribute's supported tokens.
    /// # Errors
    /// Returns a `TypeError` if the associated attribute has no supported tokens defined.
    pub fn supports(token: &str) -> bool {
        unimplemented!("This feature of the DOM is yet to be implemented.")
    }
}

// impl<'a> Index<usize> for DOMTokenList<'a> {
//     type Output = str;
//     fn index(&self, index: usize) -> &Self::Output {
//         match self.owner_attribute() {
//             Some(attr) => match attr.value().split(' ').nth(index) {
//                 Some(value) => value,
//                 None => panic!(
//                     "Index out of bounds for DOMTokenList. No value at index {}",
//                     index
//                 ),
//             },
//             None => panic!(
//                 "Index out of bounds for DOMTokenList. No value at index {}",
//                 index,
//             ),
//         }
//     }
// }

fn validate_token(token: &str) -> Result<(), DOMException> {
    if token.len() == 0 {
        return Err(DOMException::SyntaxError(
            "Empty string added to DOMTokenList.",
        ));
    }
    if token.find(|ch: char| ch.is_ascii_whitespace()).is_some() {
        return Err(DOMException::InvalidCharacterError(
            "Token with whitespace added to DOMTokenList.",
        ));
    }
    Ok(())
}
