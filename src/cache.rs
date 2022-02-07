use scraper::Selector;
use std::{collections::HashMap, fmt, ops::Index};

pub struct SelectorCache(HashMap<&'static str, Selector>);

#[derive(Debug)]
pub struct ParseSelectorError(&'static str);

impl fmt::Display for ParseSelectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid selector: {}", self.0)
    }
}

impl SelectorCache {
    pub fn new() -> SelectorCache {
        SelectorCache(HashMap::new())
    }

    pub fn add(
        &mut self,
        name: &'static str,
        selector: &'static str,
    ) -> Result<(), ParseSelectorError> {
        match Selector::parse(selector) {
            Ok(s) => {
                self.0.insert(name, s);
                Ok(())
            }
            Err(_) => Err(ParseSelectorError(selector)),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Selector> {
        self.0.get(name)
    }
}

impl Default for SelectorCache {
    fn default() -> SelectorCache {
        SelectorCache::new()
    }
}

impl Index<&str> for SelectorCache {
    type Output = Selector;

    fn index(&self, name: &str) -> &Selector {
        self.get(name).unwrap()
    }
}
