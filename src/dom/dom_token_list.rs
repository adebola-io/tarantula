use std::ops::Index;

#[derive(Debug)]
/// Set of space-separated case sensitive tokens returned by [`ElementRef::class_list`].
pub struct DOMTokenList<'a> {
    pub(crate) list: Vec<&'a str>,
}

impl<'a> DOMTokenList<'a> {
    pub fn from(list: &'a str) -> Self {
        Self {
            list: list.split(' ').collect(),
        }
    }
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}

impl<'a> DOMTokenList<'a> {
    /// Returns the number of tokens.
    pub fn len(&self) -> usize {
        self.list.len()
    }
    /// Change the value of the underlying string.
    pub fn set_value(&mut self, value: &'a str) {
        self.list.clear();
        self.list = value.split(' ').collect();
    }
}

impl<'a> DOMTokenList<'a> {
    ///  Adds an argument to the list, if it is not already present.
    ///  # Errors
    ///  - If the argument is an empty string.
    ///  - If the argument contains any ASCII whitespace.
    pub fn add(&mut self, token: &'a str) {
        validate(token);
        if !self.contains(token) {
            self.inner_add(token);
        }
    }

    #[inline(always)]
    fn inner_add(&mut self, token: &'a str) {
        self.list.push(token);
    }

    /// Returns true if token is present, and false otherwise.
    pub fn contains(&self, token: &str) -> bool {
        self.list.as_slice().contains(&token)
    }

    /// Returns the token with index `index`.
    pub fn item(&self, index: usize) -> Option<&&str> {
        self.list.as_slice().get(index)
    }

    /// Remove tokens from the list.
    pub fn remove(&mut self, token: &str) {
        validate(token);
        self.list.retain(|slice| slice != &token);
    }

    /// Read token list as a string slice.
    pub fn as_str(&self) -> &str {
        todo!()
    }

    /// Replaces token with newToken.
    ///
    /// Returns true if token was replaced with newToken, and false otherwise.
    /// # Errors
    /// - if one of the arguments is the empty string.
    /// - if one of the arguments contains any ASCII whitespace.
    pub fn replace(&mut self, token: &str, new_token: &'a str) -> bool {
        validate(token);
        validate(new_token);
        let token_index_option = self
            .list
            .as_slice()
            .iter()
            .enumerate()
            .find(|slice| slice.1 == &token)
            .map(|tuple| tuple.0);
        match token_index_option {
            Some(index) => {
                self.list[index] = new_token;
                true
            }
            None => false,
        }
    }

    /// Toggles a token in the list.
    /// Returns true if token is now present, and false otherwise.
    /// # Errors
    /// - If token is empty.
    /// - If token contains any ASCII whitespaces.
    pub fn toggle(&mut self, token: &'a str) -> bool {
        if self.contains(token) {
            self.remove(token);
            false
        } else {
            validate(token);
            self.inner_add(token);
            true
        }
    }
}

fn validate(token: &str) {
    if token.len() == 0 {
        panic!("Empty String passed to DOM Token")
    } else if token.contains(|ch: char| ch.is_ascii_whitespace()) {
        panic!("Whitespace in DOM Token")
    }
}

impl<'a> ToString for DOMTokenList<'a> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.list.as_slice().iter().enumerate().for_each(|slice| {
            s.push_str(slice.1);
            if slice.0 + 1 < self.list.len() {
                s.push(' ');
            }
        });
        s
    }
}

impl Index<usize> for DOMTokenList<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        match self.list.as_slice().get(index) {
            Some(s) => s,
            None => panic!("Index out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::DOMTokenList;

    #[test]
    fn it_creates_list() {
        let time = std::time::Instant::now();
        let mut list = DOMTokenList::from("bg-blue text-white font-sans");
        list.add("box");

        list.set_value("mama mia");

        assert!(list.contains("mama"));
        println!("{:?}", time.elapsed());
        println!("{}", list.to_string());
    }

    #[test]
    fn it_replaces_tokens_in_list() {
        let mut list = DOMTokenList::new();
        list.add("card");
        list.add("color");
        list.replace("card", "product");
        assert_eq!(list.to_string(), "product color");
    }

    #[test]
    fn slice_test() {
        let list = DOMTokenList::from("hello sefunmi where are you now");
    }
}
