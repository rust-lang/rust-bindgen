/// The only usecase of this class is for constructing a CharString.
pub struct PosList {
	poslist: Vec<usize>
}
impl PosList {
	/// Construct from string slice.
	pub fn new(slice: &str) -> PosList {
		let mut vec = Vec::new();
		vec.push(0);

		let mut pos = 0;
		while pos < slice.len() {
			pos += 1;
			while pos < slice.len() && !slice.is_char_boundary(pos) {
				pos += 1;
			}
			vec.push(pos);
		}
		PosList{poslist: vec}
	}
}
/// CharString is similar to a &str.
pub struct CharString<'a> {
	slice: &'a str,
	poslist: &'a Vec<usize>,
	pub low: usize,
	pub high: usize
}
impl<'a> CharString<'a> {
	/// Constructor. Example usuage:
	/// ```
	/// let slice = "abc";
	/// let poslist = PosList::new(slice);
	/// let cs = CharString::new(&slice, &poslist);
	/// ```
	pub fn new(slice: &'a str, poslist: &'a PosList) -> CharString<'a> {
		CharString{slice: slice, poslist: &poslist.poslist, low: 0, high: poslist.poslist.len()-1}
	}
	/// Creates a (cheap) copy of CharString where sat(i1, i2) would panic if i1 < low or i2 > high.
	pub fn copy_restrict(&self, low: usize, high: usize) -> CharString<'a> {
		CharString{slice: self.slice, poslist: self.poslist, low: low, high: high}
	}
	// pub fn len(&self) -> usize {
	// 	self.poslist.len()-1
	// }
	/// Gets the slice at position pos1 to pos2.
	pub fn sat(&self, pos1: usize, pos2: usize) -> &str {
		assert!(pos2 >= pos1);
		assert!(pos1 >= self.low && pos2 <= self.high);
		&self.slice[self.poslist[pos1]..self.poslist[pos2]]
	}
	/// Similar to sat(pos, pos+1), but returns a char instead of a &str.
	pub fn cat(&self, pos: usize) -> char {
		assert!(pos >= self.low && pos < self.high);
		let s = &self.slice[self.poslist[pos]..self.poslist[pos+1]];
		s.chars().nth(0).unwrap()
	}
}
impl<'a> std::fmt::Debug for CharString<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}{}",
			   &self.slice[self.poslist[0]..self.poslist[self.low]],
			   ansi_term::Style::new().bold().underline().paint(&self.slice[self.poslist[self.low]..self.poslist[self.high]]),
			   &self.slice[self.poslist[self.high]..self.poslist[self.poslist.len()-1]]
		)
		// f.debug_struct("CharString")
		// 	.field("slice", &self.slice)
		// 	.field("low", &self.low)
		// 	.field("high", &self.high)
		// 	.finish()
	}
}


// struct Outer<'a> {
// 	v: Vec<usize>,
// 	m: MyStruct<'a>,
// }
// struct Inner {}
// struct MyStruct<'a> {
// 	r: &'a Inner,
// }
// fn construct_inner() -> Inner {
// 	let inner = Inner{};
// 	inner
// }
// fn construct_struct<'a>(inner: &'a Inner) -> MyStruct<'a> {
// 	MyStruct{r: inner}
// }
// fn main() {
// 	let inner = construct_inner();
// 	let myStruct = construct_struct(&inner);
// }
// fn construct_both<'a, 'b:'a>() -> (Inner<'b>, MyStruct<'a>) {
// 	let inner = Inner{};
// 	(inner, MyStruct{r: &inner})
// }
// fn caller() {
// 	//let (inner, myStruct) = construct_both();
// }

