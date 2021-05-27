/// Charit is basically some kind of char iterator
#[derive(Debug)]
pub struct Charit<'a> {
	pub slice: &'a str,
	pub lpos: usize,
	pub hpos: usize
}
impl<'a> Charit<'a> {
	pub fn new(slice: &'a str) -> Charit<'a> {
		let mut hpos = 1;
		while hpos < slice.len() && !slice.is_char_boundary(hpos) {
			hpos += 1;
		}
		Charit{slice, lpos: 0, hpos}
	}
	pub fn peek(&self) -> Option<char> {
		if self.lpos < self.slice.len(){
			self.slice[self.lpos..self.hpos].chars().next()
		} else {
			None
		}
	}
	pub fn next(&mut self) -> Option<char> {
		let ret = self.peek();
		self.lpos = self.hpos;
		self.hpos += 1;
		while self.hpos < self.slice.len() && !self.slice.is_char_boundary(self.hpos) {
			self.hpos += 1;
		}
		// Possible improvement: We could replace the while loop above with something using std::String.char_indices()
		ret
	}
	pub fn back(&mut self) {
		self.hpos = self.lpos;
		self.lpos = self.lpos.checked_sub(1).unwrap();
		while !self.slice.is_char_boundary(self.lpos) {
			self.lpos = self.lpos.checked_sub(1).unwrap();
		}
		// Possible improvement: We could replace the while loop above with something using std::String.char_indices()
	}
	pub fn eat_whitespace(&mut self) {
		while let Some(c) = self.peek() {
			if !c.is_ascii_whitespace() {
				break;
			}
			self.next();
		}
	}
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_charit() {
		let wrap = |x: Option<&char>| {
			match x {
				Some(c) => Some(*c),
				None => None,
			}
		};

		let slice = "abcdef";

		let mut charit = Charit::new(slice);
		let mut cit = slice.chars().peekable();
		for _i in 1..10 {
			assert!(charit.peek() == wrap(cit.peek()));
			assert!(charit.peek() == wrap(cit.peek()));
			assert!(charit.next() == cit.next());
		}

		let mut charit = Charit::new(slice);
		let mut cit = slice.chars().peekable();
		for _i in 1..10 {
			assert!(charit.peek() == wrap(cit.peek()));
			assert!(charit.peek() == wrap(cit.peek()));
			assert!(charit.next() == cit.next());
			charit.back();
			charit.next();
		}
	}
}