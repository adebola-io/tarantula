use crate::dom::DOMTokenList;

#[derive(Debug)]
pub struct Attr<'a> {
    pub name: &'a str,
    pub namespace_uri: Option<&'a str>,
    pub prefix: Option<&'a str>,
    pub value: &'a str,
}

impl<'a> Attr<'a> {
    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }

    pub fn local_name(&self) -> &'a str {
        self.name
    }

    /// Generate a new attribute.
    fn new(key: &'a str, value: &'a str) -> Self {
        Attr {
            name: key,
            namespace_uri: None,
            prefix: None,
            value,
        }
    }
}

#[derive(Debug)]
pub struct NamedNodeMap<'a> {
    len: usize,
    pub(crate) list: Vec<Attr<'a>>,
    pub(crate) class_list: DOMTokenList<'a>,
}

impl<'a> NamedNodeMap<'a> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn new() -> Self {
        Self {
            len: 0,
            list: vec![],
            class_list: DOMTokenList { list: vec![] },
        }
    }
}

impl<'a> NamedNodeMap<'a> {
    /// Find an attribute using its key.
    pub fn get_named_item(&self, qualified_name: &str) -> Option<&Attr<'a>> {
        self.list
            .as_slice()
            .iter()
            .find(|attr| attr.name == qualified_name)
    }

    pub fn remove_named_item(&mut self, qualified_name: &str) -> Option<Attr<'a>> {
        self.list
            .as_slice()
            .iter()
            .enumerate()
            .find(|tuple| tuple.1.name == qualified_name)
            .map(|tuple| tuple.0)
            .map(|index| self.list.remove(index))
    }

    /// Checks if an attribute exists.
    pub fn has(&self, key: &str) -> bool {
        self.list
            .as_slice()
            .iter()
            .find(|attr| attr.name == key)
            .is_some()
    }

    /// Set an attribute in the attributes list.
    pub fn set(&mut self, key: &'a str, value: &'a str) {
        let attr_option = self
            .list
            .as_mut_slice()
            .iter_mut()
            .find(|attr| attr.name == key);
        match attr_option {
            Some(attr) => attr.set_name(value),
            None => self.list.push(Attr::new(key, value)),
        }
    }
}
