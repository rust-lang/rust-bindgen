//! A type that represents the union of a set of regular expressions.

use regex::RegexSet as RxSet;
use std::cell::Cell;

/// A dynamic set of regular expressions.
#[derive(Debug)]
pub struct RegexSet {
    items: Vec<String>,
    matched: Vec<Cell<bool>>,
    set: Option<RxSet>,
}

impl RegexSet {
    /// Is this set empty?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Insert a new regex into this set.
    pub fn insert<S>(&mut self, string: S)
    where
        S: AsRef<str>,
    {
        self.items.push(string.as_ref().to_owned());
        self.matched.push(Cell::new(false));
        self.set = None;
    }

    /// Returns slice of String from its field 'items'
    pub fn get_items(&self) -> &[String] {
        &self.items[..]
    }

    /// Returns regexes in the set which didn't match any strings yet
    pub fn unmatched_items(&self) -> Vec<String> {
        let mut items = vec![];
        for (i, item) in self.items.iter().enumerate() {
            if !self.matched[i].get() {
                items.push(item.clone());
            }
        }
        items
    }

    /// Construct a RegexSet from the set of entries we've accumulated.
    ///
    /// Must be called before calling `matches()`, or it will always return
    /// false.
    pub fn build(&mut self) {
        let items = self.items.iter().map(|item| format!("^{}$", item));
        self.set = match RxSet::new(items) {
            Ok(x) => Some(x),
            Err(e) => {
                error!("Invalid regex in {:?}: {:?}", self.items, e);
                None
            }
        }
    }

    /// Does the given `string` match any of the regexes in this set?
    pub fn matches<S>(&self, string: S) -> bool
    where
        S: AsRef<str>,
    {
        let s = string.as_ref();
        if let Some(set) = self.set.as_ref() {
            let matches = set.matches(s);
            if matches.matched_any() {
                for i in matches.iter() {
                    self.matched[i].set(true);
                }
                return true;
            }
        }
        false
    }
}

impl Default for RegexSet {
    fn default() -> Self {
        RegexSet {
            items: vec![],
            matched: vec![],
            set: None,
        }
    }
}
