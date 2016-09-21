use std::borrow::Borrow;
use regex::Regex;

// Yeah, I'm aware this is sorta crappy, should be cheaper to compile a regex
// ORing all the patterns, I guess...
#[derive(Debug)]
pub struct RegexSet {
    items: Vec<Regex>
}

impl RegexSet {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn extend<I>(&mut self, iter: I)
        where I: IntoIterator<Item=String>
    {
        for s in iter.into_iter() {
            self.insert(&s)
        }
    }

    pub fn insert<S>(&mut self, string: &S)
        where S: Borrow<str>
    {
        let s = string.borrow();
        match Regex::new(&format!("^{}$", s)) {
            Ok(r) => {
                self.items.push(r);
            }
            Err(err) => {
                error!("Invalid pattern provided: {}, {:?}", s, err);
            }
        }
    }

    pub fn matches<S>(&self, string: &S) -> bool
        where S: Borrow<str>
    {
        let s = string.borrow();
        for r in &self.items {
            if r.is_match(s) {
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
        }
    }
}
