#![feature(test)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
extern crate test;


use std::collections::HashMap;

//mod charstring;

mod giwrap;
use giwrap::*;

mod charit;
use charit::*;

struct SymCache {
	cache: HashMap<String, ginac_ex>
}
impl SymCache {
	fn new() -> SymCache {
		SymCache{cache: HashMap::new()}
	}
	fn get_or_create(&mut self, name: &str) -> ginac_ex {
		match self.cache.get(name) {
			Some(v) => ginac_ex::from_copy(v),
			None => {
				let sym = ginac_ex::from_symbol_str(name);
				self.cache.insert(String::from(name), ginac_ex::from_copy(&sym));
				sym
			}
		}
	}
}
/// reads either a single letter symbol or a slosh symbol. Ignores underscores
pub fn read_symbol<'a>(it: &mut Charit<'a>) -> &'a str {
	let lpos = it.lpos;
	let c = it.next().unwrap();
	if c == '\\' {
		while let Some(v) = it.next() {
			if !v.is_ascii_alphabetic() {
				break;
			}
		}
		it.back();
	} else {
		assert!(c.is_ascii_alphabetic());
	}
	let name = &it.slice[lpos..it.lpos];
	assert!(name != "\\");
	name
}
fn read_tex_argument_parsed<'a>(it: &mut Charit<'a>, symcache: &mut SymCache) -> ginac_ex {
	it.eat_whitespace();
	let c = it.peek().unwrap();
	if c == '{' {
		it.next();
		itparse(it, symcache, EndConds{eof: false, plus_minus: false, round_brace: false, curly_brace: true}).unwrap()
	} else {
		let name = read_symbol(it);
		if let Some(n) = it.peek() {
			if !n.is_ascii_whitespace() && !['(', '{', '\\'].contains(&n) {
				panic!("bad latex style");
			}
		}
		symcache.get_or_create(&name)
	}
}
fn read_tex_argument_literal<'a>(it: &mut Charit<'a>) -> &'a str {
	let lpos = it.lpos;
	read_tex_argument_parsed(it, &mut SymCache::new());
	&it.slice[lpos..it.lpos]
}
#[derive(Debug)]
struct EndConds {
	eof: bool,
	plus_minus: bool,
	round_brace: bool,
	curly_brace: bool,
}
fn itparse(it: &mut Charit, symcache: &mut SymCache, endconds: EndConds) -> Option<ginac_ex> {
	dbg!(&it, &endconds);
	let mut totex: Option<ginac_ex> = None;
	while let Some(c) = it.next() {
		if c.is_ascii_whitespace() {
		} else if ['+', '-'].contains(&c) {
			if !endconds.plus_minus {
				let mut innerval = itparse(it, symcache, EndConds{eof: true, plus_minus: true, round_brace: endconds.round_brace, curly_brace: endconds.curly_brace}).unwrap();
				if c == '-' {
					innerval *= ginac_ex::from_integer(-1);
				}
				match totex {
					Some(ref mut v) => *v += innerval, // for parsing "expr1 + expr2"
					None => totex = Some(innerval), // for parsing " + expr2 "
				}
				it.back();
			}
			else {
				return totex;
			}
		} else if c == '(' {
			let innerval = itparse(it, symcache, EndConds{eof: false, plus_minus: false, round_brace: true, curly_brace: false});
			match totex {
				Some(ref mut v) => *v *= innerval.unwrap(), // for parsing "expr1 (expr2)"
				None => totex = innerval, // for parsing " (expr2) " or " expr1 + (expr2) "
			}
		} else if c == ')' {
			assert!(endconds.round_brace);
			if endconds.plus_minus {
				return totex;
			}
			let totex = totex.unwrap();
			it.eat_whitespace();
			if it.peek() == Some('^') {
				it.next();
				let exponent = read_tex_argument_parsed(it, symcache);
				dbg!(&it, &endconds);
				return Some(totex.powex(&exponent));
			}
			return Some(totex);
		} else if c.is_ascii_alphabetic() || c == '\\' {
			it.back();
			let name = read_symbol(it);

			let mut exponent = None;
			let mut subscript = None;
			for _i in 0..2 {
				it.eat_whitespace();
				if it.peek() == Some('_') {
					assert!(subscript == None);
					it.next();
					subscript = Some(read_tex_argument_literal(it));
				} else if it.peek() == Some('^') {
					assert!(exponent == None);
					it.next();
					exponent = Some(read_tex_argument_parsed(it, symcache));
				}
			}

			let var = match subscript {
				None => symcache.get_or_create(&name),
				Some(sub) => symcache.get_or_create(&(name.to_owned() + "_" + sub)),
			};

			let ex = match exponent {
				None => var,
				Some(expon) => var.powex(&expon),
			};
			match totex {
				Some(ref mut v) => *v *= ex,
				None => totex = Some(ex),
			}
		} else if c == '}' {
			assert!(endconds.curly_brace);
			return totex;
		} else {
			panic!("Unknown symbol"); //todo : - * numbers
		}
	}
	assert!(endconds.eof);
	totex
}
// fn itparse2(it: &mut std::str::Chars, symlist: &mut Vec<(String, ginac_ex)>, endconds: EndConds) -> Option<ginac_ex> {
// 	let mut totex: Option<ginac_ex> = None;
// 	//pos_str = it.as_str();
// 	//pos_it = it.clone();
// 	while true {
// 		let c = match it.next() {
// 			Some(v) => v,
// 			None => break,
// 		};
// 		if c == '3' {
// 			itparse2()
// 		}
// 		dbg!(c);
// 	}
// 	assert!(endconds.eof);
// 	totex
// }

/*

a+
a++b


*/

//todo: * -

pub struct SymbolMap {
	pub hm: HashMap<String, ginac_ex>,
}
impl Fn<(&str,)> for SymbolMap {
	extern "rust-call" fn call(&self, args: (&str,)) -> ginac_ex {
		ginac_ex::from_copy(self.hm.get(args.0).unwrap())
	}
}
impl FnMut<(&str,)> for SymbolMap {
	extern "rust-call" fn call_mut(&mut self, args: (&str,)) -> ginac_ex {
		self.call(args)
	}
}
impl FnOnce<(&str,)> for SymbolMap {
	type Output = ginac_ex;
	extern "rust-call" fn call_once(self, args: (&str,)) -> ginac_ex {
		self.call(args)
	}
}

//include!("../modbind.rs");
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn unittests() {
		let mut vec: Vec<(&str,Box<dyn Fn(&SymbolMap) -> ginac_ex>)> = Vec::new();
		macro_rules! add {
				($vec:ident, $tex:expr, $ex:expr) => {
					$vec.push(($tex, Box::new($ex)));
				};
		}
		add!(vec, "a", |s:&SymbolMap| s("a"));
		add!(vec, "a+b", |s:&SymbolMap| s("a")+s("b"));
		add!(vec, "(a+b)", |s:&SymbolMap| s("a")+s("b"));
		add!(vec, "ab+de+gh", |s:&SymbolMap| s("a")*s("b")+s("d")*s("e")+s("g")*s("h"));
		add!(vec, "(d(b+c)+a)e\\alpha", |s:&SymbolMap| (s("d")*(s("b")+s("c"))+s("a"))*s("e")*s("\\alpha"));
		add!(vec, "\\alpha\\beta", |s:&SymbolMap| s("\\alpha")*s("\\beta"));
		add!(vec, "a_b(c+d)", |s:&SymbolMap| s("a_b")*(s("c")+s("d")));
		//add!(vec, "a^2_b", |s:&SymbolMap| s("a_b")*s("a_b"));
		add!(vec, "(a+b)^c", |s:&SymbolMap| (s("a")+s("b")).powex(&s("c")));
		add!(vec, "a-a", |_s:&SymbolMap| ginac_ex::from_integer(0));
		add!(vec, "a+b-a", |s:&SymbolMap| s("b"));
		//add!(vec, "-a", |s:&SymbolMap| -s("a")); todo
		for pair in vec.iter() {
			let mut symcache = SymCache::new();
			let totex = itparse(&mut Charit::new(pair.0), &mut symcache, EndConds{eof: true, plus_minus: false, round_brace: false, curly_brace: false}).unwrap();
			let symmap = SymbolMap{hm: symcache.cache}; // TODO: this is a bit hacky
			if totex != pair.1(&symmap) {
				dbg!(pair.0);
				panic!("not equal");
			}
		}
	}
	macro_rules! negative_test {
		($name:ident, $tex:expr) => {
			#[test]
			#[should_panic]
			fn $name() {
				itparse(&mut Charit::new($tex), &mut SymCache::new(), EndConds{eof: true, plus_minus: false, round_brace: false, curly_brace: false});
			}
		};
	}
	negative_test!(empty_addition, "a+  ");
	negative_test!(double_addition, "a++b");
	negative_test!(unmatched_bracket_open, "(a  ");
	negative_test!(unmatched_bracket_close, "a)  ");
	negative_test!(slosh_alone, "\\  ");
	negative_test!(superscript_alone_1, "a^");
	negative_test!(superscript_alone_2, "a^ ");
	negative_test!(double_subscrict_1, "a_b_c");
	negative_test!(double_subscrict_2, "a_b _c");
	negative_test!(double_superscript, "(a+b)^c ^d");

    #[test]
	fn curwork() {
		//let slice = "ab+de+gh";
		//let slice = "(d(b+c)+a)e\\alpha";
		//let slice = "(d(b+c)+a)e\\alpha+\\beta((c\\alpha)+d(a+e))";
		let slice = "-a_b";
		//let poslist = PosList::new(slice);
		//let tex = CharString::new(&slice, &poslist);

		let mut symcache = SymCache::new();
		//let totex = parse_tex(tex, &mut symlist);
		let _totex = itparse(&mut Charit::new(slice), &mut symcache, EndConds{eof: true, plus_minus: false, round_brace: false, curly_brace: false});

		// match _totex {
		// 	Some(v) => {v.print(); v.print_tree();},
		// 	None => {println!("totex is None");},
		// }
	}
	// use test::Bencher;
	// use charit::*;
	// #[bench]
	// fn bench_mixed(b: &mut Bencher) {
	// 	let slice = "(d(b+c)+a)e\\alpha+\\beta((c\\alpha)+d(a+e))";
	// 	b.iter(||
	// 		   {
	// 			   let mut symlist = Vec::new();
	// 			   itparse(&mut Charit::new(slice), &mut symlist, EndConds{eof: true, plus_minus: false, round_brace: false})
	// 		   }
	// 	);
	// }
}




// macro_rules! foo {
//     ($e:expr) => {let poslist = generate_poslist($e); let tex = CharString::new(& $e, &poslist);};
// }

