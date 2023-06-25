use std::ops::Index;

use crate::{AsElement, AsNode, Attr, DOMException, Element};

pub enum ListType {
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

/**
 * A set of space-separated tokens. Such a set is returned by Element.classList, HTMLLinkElement.relList, HTMLAnchorElement.relList, HTMLAreaElement.relList, HTMLIframeElement.sandbox, or HTMLOutputElement.htmlFor. It is indexed beginning with 0 as with JavaScript Array objects. DOMTokenList is always case-sensitive.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/DOMTokenList)
 */
pub struct MutDOMTokenList<'a> {
    owner_element: &'a mut Element,
    list_type: ListType,
}

/**
 * A set of space-separated tokens. Such a set is returned by Element.classList, HTMLLinkElement.relList, HTMLAnchorElement.relList, HTMLAreaElement.relList, HTMLIframeElement.sandbox, or HTMLOutputElement.htmlFor. It is indexed beginning with 0 as with JavaScript Array objects. DOMTokenList is always case-sensitive.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/DOMTokenList)
 */
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
    /// Returns the number of tokens.
    pub fn len(&self) -> usize {
        self.owner_attribute()
            .map(|attribute| attribute.value.split(' ').count())
            .unwrap_or(0)
    }
    /// Returns true if the list contains a specified token.
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn contains(&self, token: &str) -> bool {
        self.owner_attribute()
            .map(|attribute| attribute.value.split(' ').any(|_token| _token == token))
            .unwrap_or(false)
    }
    /// Returns the token with index `index`
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn item(&self, index: usize) -> Option<&str> {
        self.owner_attribute()
            .map(|attribute| attribute.value.split(' ').nth(index))
            .flatten()
    }
    /// Adds all arguments passed, except those already present.
    /// # Errors
    /// - Returns a "SyntaxError" DOMException if  the argument is the empty string.
    /// - Returns an "InvalidCharacterError" DOMException if the argument contains any ASCII whitespace.
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn add(&mut self, token: &str) -> Result<(), DOMException> {
        validate_token(token)?;
        if self.owner_attribute().is_none() {
            let attr = self
                .owner_element
                .owner_document()
                .unwrap()
                .create_attribute(self.list_type.as_str());
            self.owner_element.attributes_mut().set_named_item(attr);
        }
        if !self.contains(token) {
            self.owner_attribute_mut().unwrap().value.push(' ');
            self.owner_attribute_mut().unwrap().value.push_str(token);
        }
        Ok(())
    }
    /// Removes the arguments passed, if they are present.
    /// # Errors
    /// - Returns a "SyntaxError" DOMException if the argument is the empty string.
    /// - Returns an "InvalidCharacterError" DOMException if the argument contains any ASCII whitespace.
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn remove(&mut self, token: &str) -> Result<(), DOMException> {
        validate_token(token)?;
        if let Some(attr) = self.owner_attribute_mut() {
            attr.value = attr.value.split(' ').filter(|t| *t != token).collect();
        }
        Ok(())
    }
    /// Replaces a token with another token.
    ///
    /// It returns true if the token was replaced, and false otherwise (the token to replace does not exist in the list).
    ///
    /// # Errors
    /// - Returns a "SyntaxError" DOMException if one of the arguments is the empty string.
    /// - Returns an "InvalidCharacterError" DOMException if one of the arguments contains any ASCII whitespace.
    /// # Example
    /// ```
    /// // Add an example
    /// ```
    pub fn replace(&mut self, token: &str, new_token: &'a str) -> Result<bool, DOMException> {
        validate_token(token)?;
        validate_token(new_token)?;
        let mut is_changed = false;
        if let Some(attr) = self.owner_attribute_mut() {
            attr.value = attr
                .value
                .split(' ')
                .map(|old_token| {
                    if old_token != token {
                        is_changed = true;
                        new_token
                    } else {
                        old_token
                    }
                })
                .collect();
        }

        Ok(is_changed)
    }
    /// Returns true if the token is in the associated attribute's supported tokens.
    /// # Errors
    /// Returns a `TypeError` if the associated attribute has no supported tokens defined.
    pub fn supports(_token: &str) -> Result<bool, DOMException> {
        todo!()
    }
    /// Toggles a token, removing if it is present and adding it otherwise.
    /// # Errors
    /// - Returns a "SyntaxError" DOMException if one of the arguments is the empty string.
    /// - Returns an "InvalidCharacterError" DOMException if one of the arguments contains any ASCII whitespace.
    /// # Example
    /// ```
    /// // Add an example
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

impl<'a> Index<usize> for MutDOMTokenList<'a> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        match self.owner_attribute() {
            Some(attr) => match attr.value.split(' ').nth(index) {
                Some(value) => value,
                None => panic!(
                    "Index out of bounds for DOMTokenList. No value at index {}",
                    index
                ),
            },
            None => panic!(
                "Index out of bounds for DOMTokenList. No value at index {}",
                index,
            ),
        }
    }
}

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
    /// Returns the number of tokens
    pub fn len(&self) -> usize {
        self.owner_attribute()
            .map(|attribute| attribute.value.split(' ').count())
            .unwrap_or(0)
    }
    /// Returns true if the list contains a specified token.
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn contains(&self, token: &str) -> bool {
        self.owner_attribute()
            .map(|attribute| attribute.value.split(' ').any(|_token| _token == token))
            .unwrap_or(false)
    }
    /// Returns the token with index `index`
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn item(&self, index: usize) -> Option<&str> {
        self.owner_attribute()
            .map(|attribute| attribute.value.split(' ').nth(index))
            .flatten()
    }

    /// Returns true if the token is in the associated attribute's supported tokens.
    /// # Errors
    /// Returns a `TypeError` if the associated attribute has no supported tokens defined.
    pub fn supports(token: &str) -> bool {
        todo!()
    }
}

impl<'a> Index<usize> for DOMTokenList<'a> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        match self.owner_attribute() {
            Some(attr) => match attr.value.split(' ').nth(index) {
                Some(value) => value,
                None => panic!(
                    "Index out of bounds for DOMTokenList. No value at index {}",
                    index
                ),
            },
            None => panic!(
                "Index out of bounds for DOMTokenList. No value at index {}",
                index,
            ),
        }
    }
}
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
