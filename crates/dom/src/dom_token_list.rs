use std::ops::Index;

use crate::{Attr, DOMException};

pub struct MutDOMTokenList<'a> {
    owner_attribute: &'a mut Attr,
    len: usize,
}

pub struct DOMTokenList<'a> {
    owner_attribute: &'a Attr,
    len: usize,
}

impl<'a> DOMTokenList<'a> {
    /// Create a token list from an attribute.
    pub fn from_attribute(owner_attribute: &'a Attr) -> Self {
        Self {
            owner_attribute,
            len: owner_attribute.value.split(' ').count(),
        }
    }
}

impl<'a> MutDOMTokenList<'a> {
    /// Create a token list from an attribute.
    pub fn from_attribute(owner_attribute: &'a mut Attr) -> Self {
        let len = owner_attribute.value.split(' ').count();
        Self {
            owner_attribute,
            len,
        }
    }
}

impl<'a> MutDOMTokenList<'a> {
    /// Returns the number of tokens.
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns true if the list contains a specified token.
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn contains(&self, token: &str) -> bool {
        self.owner_attribute
            .value
            .split(' ')
            .any(|_token| _token == token)
    }
    /// Returns the token with index `index`
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn item(&self, index: usize) -> Option<&str> {
        self.owner_attribute.value.split(' ').nth(index)
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
        if !self.contains(token) {
            self.owner_attribute.value.push(' ');
            self.owner_attribute.value.push_str(token);
            self.len += 1;
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
        self.owner_attribute.value = self
            .owner_attribute
            .value
            .split(' ')
            .filter(|t| *t != token)
            .collect();
        self.len -= 1;
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
        self.owner_attribute.value = self
            .owner_attribute
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
        Ok(is_changed)
    }
    /// Returns true if the token is in the associated attribute's supported tokens.
    /// # Errors
    /// Returns a `TypeError` if the associated attribute has no supported tokens defined.
    pub fn supports(token: &str) -> bool {
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

impl Index<usize> for MutDOMTokenList<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        match self.owner_attribute.value.split(' ').nth(index) {
            Some(value) => value,
            None => panic!(
                "Index out of bounds for DOMTokenList. No value at index {}",
                index
            ),
        }
    }
}

impl<'a> DOMTokenList<'a> {
    /// Returns the number of tokens
    pub fn len(&self) -> usize {
        self.len
    }
    /// Returns true if the list contains a specified token.
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn contains(&self, token: &str) -> bool {
        self.owner_attribute
            .value
            .split(' ')
            .any(|_token| _token == token)
    }
    /// Returns the token with index `index`
    /// # Example
    /// ```
    /// // Add an example.
    /// ```
    pub fn item(&self, index: usize) -> Option<&str> {
        self.owner_attribute.value.split(' ').nth(index)
    }

    /// Returns true if the token is in the associated attribute's supported tokens.
    /// # Errors
    /// Returns a `TypeError` if the associated attribute has no supported tokens defined.
    pub fn supports(token: &str) -> bool {
        todo!()
    }
}

impl Index<usize> for DOMTokenList<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        match self.owner_attribute.value.split(' ').nth(index) {
            Some(value) => value,
            None => panic!(
                "Index out of bounds for DOMTokenList. No value at index {}",
                index
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
