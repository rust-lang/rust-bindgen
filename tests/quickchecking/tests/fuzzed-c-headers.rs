
extern crate quickcheck;
extern crate quickchecking;
extern crate rand;

use quickchecking::fuzzers::{ArrayDimensionC, BaseTypeC, BasicTypeDeclarationC, DeclarationC,
                             DeclarationListC, FunctionPointerDeclarationC, FunctionPrototypeC,
                             HeaderC, ParameterC, ParameterListC, PointerLevelC,
                             StructDeclarationC, TypeQualifierC, UnionDeclarationC};
use quickcheck::{Arbitrary, StdGen};
use rand::thread_rng;

#[test]
fn test_declaraion_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: DeclarationC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_declaraion_list_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: DeclarationListC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_base_type_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: BaseTypeC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_type_qualifier_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: TypeQualifierC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_pointer_level_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: PointerLevelC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_array_dimension_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: ArrayDimensionC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_basic_type_declaration_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: BasicTypeDeclarationC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_struct_declaration_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: StructDeclarationC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_union_declaration_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: UnionDeclarationC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_function_pointer_declaration_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: FunctionPointerDeclarationC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_function_prototype_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: FunctionPrototypeC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_parameter_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: ParameterC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_parameter_list_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: ParameterListC = Arbitrary::arbitrary(gen);
}

#[test]
fn test_header_c_does_not_panic() {
    let ref mut gen = StdGen::new(thread_rng(), 50);
    let _: HeaderC = Arbitrary::arbitrary(gen);
}
