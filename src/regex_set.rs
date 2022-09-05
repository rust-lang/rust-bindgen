//! A type that represents the union of a set of regular expressions.

use regex::RegexSet as RxSet;
use std::cell::Cell;

/// A static list of regular expressions.
#[derive(Debug, Default, Clone)]
pub struct RegexItems {
    inner: Vec<String>,
}

impl RegexItems {
    /// Returns slice of String from its field 'items'
    pub fn get_items(&self) -> &[String] {
        &self.inner
    }

    /// Insert a new regex into this list.
    pub fn insert(&mut self, item: impl AsRef<str>) {
        self.inner.push(item.as_ref().to_owned());
    }
}

/// A dynamic set of regular expressions.
#[derive(Debug)]
pub struct RegexSet {
    items: Vec<String>,
    /// Whether any of the items in the set was ever matched. The length of this
    /// vector is exactly the length of `items`.
    matched: Vec<Cell<bool>>,
    set: Option<RxSet>,
    /// Whether we should record matching items in the `matched` vector or not.
    record_matches: bool,
}

impl RegexSet {
    /// Construct a RegexSet from the set of entries we've accumulated.
    pub fn new(items: RegexItems, record_matches: bool) -> Self {
        let set = match RxSet::new(
            items.inner.iter().map(|item| format!("^{}$", item)),
        ) {
            Ok(x) => Some(x),
            Err(e) => {
                warn!("Invalid regex in {:?}: {:?}", items.inner, e);
                None
            }
        };

        Self {
            matched: vec![Cell::new(false); items.inner.len()],
            items: items.inner,
            set,
            record_matches,
        }
    }

    /// Is this set empty?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns an iterator over regexes in the set which didn't match any
    /// strings yet.
    pub fn unmatched_items(&self) -> impl Iterator<Item = &String> {
        self.items.iter().enumerate().filter_map(move |(i, item)| {
            if !self.record_matches || self.matched[i].get() {
                return None;
            }

            Some(item)
        })
    }

    /// Does the given `string` match any of the regexes in this set?
    pub fn matches<S>(&self, string: S) -> bool
    where
        S: AsRef<str>,
    {
        let s = string.as_ref();
        let set = match self.set {
            Some(ref set) => set,
            None => return false,
        };

        if !self.record_matches {
            return set.is_match(s);
        }

        let matches = set.matches(s);
        if !matches.matched_any() {
            return false;
        }
        for i in matches.iter() {
            self.matched[i].set(true);
        }

        true
    }
}
