use quickcheck::{Arbitrary, Gen, StdGen};
use std::fmt;
use rand::thread_rng;

/// BaseTypeC is used in generation of C headers to represent the C language's
/// primitive types as well as `void*`.
#[derive(Debug, Clone)]
pub struct BaseTypeC {
    /// String representation of C type.
    pub def: String,
}

/// TypeQualifierC is used in generation of C headers to represent qualifiers
/// such as `const`.
#[derive(Debug, Clone)]
pub struct TypeQualifierC {
    /// String representation of C type qualifier.
    pub def: String,
}

/// PointerLevelC is used in generation of C headers to represent number of
/// `*` for pointer types.
#[derive(Debug, Clone)]
pub struct PointerLevelC {
    /// String representation of C declaration's pointer level.
    pub def: String,
}

/// ArrayDimensionC is used in generation of C headers to represent number of
/// `[]` used to define array types.
#[derive(Debug, Clone)]
pub struct ArrayDimensionC {
    /// String representation of C declaration's array dimension.
    pub def: String,
}

/// BasicTypeDeclarationC is used in generation of C headers to represent
/// declarations outside of function pointers that take the form
/// `BaseTypeC` + `TypeQualifierC` + `PointerLevelC` + `ident_id`.
#[derive(Debug, Clone)]
pub struct BasicTypeDeclarationC {
    /// The declaration's base type, i.e. `int`.
    pub type_name: BaseTypeC,
    /// The declaration's type qualifier, i.e. `const`.
    pub type_qualifier: TypeQualifierC,
    /// The declaration's pointer level, i.e. `***`.
    pub pointer_level: PointerLevelC,
    /// The declaration's array dimension, i.e. [][][].
    pub array_dimension: ArrayDimensionC,
    /// The declaration's identifier, i.e. ident_N.
    pub ident_id: String,
}

/// StructDeclarationC is used in generation of C headers to represent the
/// definition of a struct type.
#[derive(Debug, Clone)]
pub struct StructDeclarationC {
    /// The declaration's fields.
    pub fields: DeclarationListC,
    /// The declaration's array dimension, i.e. [][][].
    pub array_dimension: ArrayDimensionC,
    /// The declaration's identifier, i.e. struct_N.
    pub ident_id: String,
}

/// UnionDeclarationC is used in generation of C headers to represent the
/// definition of a union type.
#[derive(Debug, Clone)]
pub struct UnionDeclarationC {
    /// The declaration's fields.
    pub fields: DeclarationListC,
    /// The declaration's array dimension, i.e. [][][].
    pub array_dimension: ArrayDimensionC,
    /// The declaration's identifier, i.e. union_N.
    pub ident_id: String,
}

/// FunctionPointerDeclarationC is used in generation of C headers to represent
/// the definition of a function pointer type.
#[derive(Debug, Clone)]
pub struct FunctionPointerDeclarationC {
    /// The function's type qualifier, i.e. `const`.
    pub type_qualifier: TypeQualifierC,
    /// The function's return type, i.e. `int`.
    pub type_name: BaseTypeC,
    /// The function's pointer level, i.e. `***`.
    pub pointer_level: PointerLevelC,
    /// The function's parameters.
    pub params: ParameterListC,
    /// The declaration's identifier, i.e. func_ptr_N.
    pub ident_id: String,
}

/// FunctionPrototypeC is used in generation of C headers to represent the
/// definition of a function prototype.
#[derive(Debug, Clone)]
pub struct FunctionPrototypeC {
    /// The function's type qualifier, i.e. `const`.
    pub type_qualifier: TypeQualifierC,
    /// The function's return type, i.e. `int`.
    pub type_name: BaseTypeC,
    /// The function's pointer level, i.e. `***`.
    pub pointer_level: PointerLevelC,
    /// The function's parameters.
    pub params: ParameterListC,
    /// The prototype's identifier, i.e. `func_N`.
    pub ident_id: String,
}

/// ParameterC is used in generation of C headers to represent the
/// definition function parameters.
#[derive(Debug, Clone)]
pub struct ParameterC {
    /// The parameter's type qualifier, i.e. `const`.
    pub type_qualifier: TypeQualifierC,
    /// The parameter's base type, i.e. `int`.
    pub type_name: BaseTypeC,
    /// The parameter's pointer level, i.e. `***`.
    pub pointer_level: PointerLevelC,
}

/// ParameterListC is used in generation of C headers to represent a list of
/// definitions of function parameters.
#[derive(Debug, Clone)]
pub struct ParameterListC {
    /// Parameters that define a C function signature.
    pub params: Vec<ParameterC>,
}

/// DeclarationC is used in generation of C headers to represent all supported
/// C type declarations allowed in the generated header.
#[derive(Debug, Clone)]
pub enum DeclarationC {
    /// Function prototype declaration kind.
    FunctionDecl(FunctionPrototypeC),
    /// Function pointer declaration kind.
    FunctionPtrDecl(FunctionPointerDeclarationC),
    /// Struct declaration kind.
    StructDecl(StructDeclarationC),
    /// Union declaration kind.
    UnionDecl(UnionDeclarationC),
    /// Basic type declaration kind.
    VariableDecl(BasicTypeDeclarationC),
}

/// DeclarationListC is used in generation of C headers to represent a list of
/// declarations.
#[derive(Debug, Clone)]
pub struct DeclarationListC {
    /// Grouping of C declarations.
    pub decls: Vec<DeclarationC>,
}

/// HeaderC is used in generation of C headers to represent a collection of
/// declarations.
#[derive(Clone)]
pub struct HeaderC {
    /// The header's declarations.
    pub def: DeclarationListC,
}

/// MakeUnique is used in generation of C headers to make declaration
/// identifiers unique by incorporating the `stamp` parameter into it's name.
trait MakeUnique {
    fn make_unique(&mut self, stamp: usize);
}

/// MakeUnique is used in generation of C headers to make DeclarationC
/// identifiers unique.
impl MakeUnique for DeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        match *self {
            DeclarationC::FunctionDecl(ref mut d) => d.make_unique(stamp),
            DeclarationC::FunctionPtrDecl(ref mut d) => d.make_unique(stamp),
            DeclarationC::StructDecl(ref mut d) => d.make_unique(stamp),
            DeclarationC::UnionDecl(ref mut d) => d.make_unique(stamp),
            DeclarationC::VariableDecl(ref mut d) => d.make_unique(stamp),
        }
    }
}

/// A qucickcheck trait for describing how DeclarationC types can be
/// randomly generated and shrunk.
impl Arbitrary for DeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> DeclarationC {
        match g.gen_range(0, 5) {
            0 => DeclarationC::FunctionDecl(FunctionPrototypeC::arbitrary(g)),
            1 => DeclarationC::FunctionPtrDecl(FunctionPointerDeclarationC::arbitrary(g)),
            2 => DeclarationC::StructDecl(StructDeclarationC::arbitrary(g)),
            3 => DeclarationC::UnionDecl(UnionDeclarationC::arbitrary(g)),
            4 => DeclarationC::VariableDecl(BasicTypeDeclarationC::arbitrary(g)),
            _ => unreachable!(),
        }
    }
}

/// Enables to string and format for DeclarationC types.
impl fmt::Display for DeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeclarationC::FunctionPtrDecl(ref d) => write!(f, "{}", d),
            DeclarationC::StructDecl(ref d) => write!(f, "{}", d),
            DeclarationC::UnionDecl(ref d) => write!(f, "{}", d),
            DeclarationC::VariableDecl(ref d) => write!(f, "{}", d),
            DeclarationC::FunctionDecl(ref d) => write!(f, "{}", d),
        }
    }
}

/// A qucickcheck trait for describing how DeclarationListC types can be
/// randomly generated and shrunk.
impl Arbitrary for DeclarationListC {
    fn arbitrary<G: Gen>(g: &mut G) -> DeclarationListC {
        DeclarationListC {
            decls: Arbitrary::arbitrary(g),
        }
    }
}

/// Enables to string and format for DeclarationListC types.
impl fmt::Display for DeclarationListC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();
        for decl in &self.decls {
            display += &format!("{}", decl);
        }
        write!(f, "{}", display)
    }
}

/// A qucickcheck trait for describing how BaseTypeC types can be
/// randomly generated and shrunk.
impl Arbitrary for BaseTypeC {
    fn arbitrary<G: Gen>(g: &mut G) -> BaseTypeC {
        // Special case `long double` until issue #550 is resolved.
        let base_type = vec![
            "char",
            "signed char",
            "unsigned char",
            "short",
            "short int",
            "signed short",
            "signed short int",
            "unsigned short",
            "unsigned short int",
            "int",
            "signed",
            "signed int",
            "unsigned",
            "unsigned int",
            "long",
            "long int",
            "signed long",
            "signed long int",
            "unsigned long",
            "unsigned long int",
            "long long",
            "long long int",
            "signed long long",
            "signed long long int",
            "unsigned long long",
            "unsigned long long int",
            "float",
            "double",
            #[cfg(feature = "long-doubles")]
            "long double",
            "void*",
        ];
        BaseTypeC {
            def: String::from(*g.choose(&base_type).unwrap()),
        }
    }
}

/// Enables to string and format for BaseTypeC types,
impl fmt::Display for BaseTypeC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

/// A qucickcheck trait for describing how TypeQualifierC types can be
/// randomly generated and shrunk.
impl Arbitrary for TypeQualifierC {
    fn arbitrary<G: Gen>(g: &mut G) -> TypeQualifierC {
        let qualifier = vec!["const", ""];
        TypeQualifierC {
            def: String::from(*g.choose(&qualifier).unwrap()),
        }
    }
}

/// Enables to string and format for TypeQualifierC types.
impl fmt::Display for TypeQualifierC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

/// A qucickcheck trait for describing how PointerLevelC types can be
/// randomly generated and shrunk.
impl Arbitrary for PointerLevelC {
    fn arbitrary<G: Gen>(g: &mut G) -> PointerLevelC {
        PointerLevelC {
            // 16 is an arbitrary "not too big" number for capping pointer level.
            def: (0..g.gen_range(0, 16)).map(|_| "*").collect::<String>(),
        }
    }
}

/// Enables to string and format for PointerLevelC types.
impl fmt::Display for PointerLevelC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

/// A qucickcheck trait for describing how ArrayDimensionC types can be
/// randomly generated and shrunk.
impl Arbitrary for ArrayDimensionC {
    fn arbitrary<G: Gen>(g: &mut G) -> ArrayDimensionC {
        // Keep these small, clang complains when they get too big.
        let dimensions = g.gen_range(0, 5);
        let mut def = String::new();

        let lower_bound;
        if cfg!(feature = "zero-sized-arrays") {
            lower_bound = 0;
        } else {
            lower_bound = 1;
        }

        for _ in 1..dimensions {
            // 16 is an arbitrary "not too big" number for capping array size.
            def += &format!("[{}]", g.gen_range(lower_bound, 16));
        }
        ArrayDimensionC { def }
    }
}

/// Enables to string and format for ArrayDimensionC types.
impl fmt::Display for ArrayDimensionC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

/// MakeUnique is used in generation of C headers to make BasicTypeDeclarationC
/// identifiers unique.
impl MakeUnique for BasicTypeDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

/// A qucickcheck trait for describing how BasicTypeDeclarationC types can be
/// randomly generated and shrunk.
impl Arbitrary for BasicTypeDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> BasicTypeDeclarationC {
        BasicTypeDeclarationC {
            type_qualifier: Arbitrary::arbitrary(g),
            type_name: Arbitrary::arbitrary(g),
            pointer_level: Arbitrary::arbitrary(g),
            array_dimension: Arbitrary::arbitrary(g),
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

/// Enables to string and format for BasicTypeDeclarationC types.
impl fmt::Display for BasicTypeDeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} ident_{}{};",
            self.type_qualifier,
            self.type_name,
            self.pointer_level,
            self.ident_id,
            self.array_dimension
        )
    }
}

/// MakeUnique is used in generation of C headers to make StructDeclarationC
/// identifiers unique.
impl MakeUnique for StructDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

/// A qucickcheck trait for describing how StructDeclarationC types can be
/// randomly generated and shrunk.
impl Arbitrary for StructDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> StructDeclarationC {
        // Reduce generator size as a method of putting a bound on recursion.
        // When size < 1 the empty list is generated.
        let reduced_size: usize = (g.size() / 2) as usize + 1;
        let mut decl_list: DeclarationListC =
            Arbitrary::arbitrary(&mut StdGen::new(thread_rng(), reduced_size));
        let mut fields: DeclarationListC = DeclarationListC { decls: vec![] };

        for (i, decl) in decl_list.decls.iter_mut().enumerate() {
            match *decl {
                DeclarationC::FunctionDecl(_) => {}
                ref mut decl => {
                    decl.make_unique(i);
                    fields.decls.push(decl.clone());
                }
            }
        }

        StructDeclarationC {
            fields,
            ident_id: format!("{}", usize::arbitrary(g)),
            array_dimension: Arbitrary::arbitrary(g),
        }
    }
}

/// Enables to string and format for StructDeclarationC types.
impl fmt::Display for StructDeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "struct {{ {} }} struct_{}{};",
            self.fields,
            self.ident_id,
            self.array_dimension
        )
    }
}

/// MakeUnique is used in generation of C headers to make UnionDeclarationC
/// identifiers unique.
impl MakeUnique for UnionDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

/// A qucickcheck trait for describing how UnionDeclarationC types can be
/// randomly generated and shrunk.
impl Arbitrary for UnionDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> UnionDeclarationC {
        // Reduce generator size as a method of putting a bound on recursion.
        // When size < 1 the empty list is generated.
        let reduced_size: usize = (g.size() / 2) as usize + 1;
        let mut decl_list: DeclarationListC =
            Arbitrary::arbitrary(&mut StdGen::new(thread_rng(), reduced_size));
        let mut fields: DeclarationListC = DeclarationListC { decls: vec![] };

        for (i, decl) in decl_list.decls.iter_mut().enumerate() {
            match *decl {
                DeclarationC::FunctionDecl(_) => {}
                ref mut decl => {
                    decl.make_unique(i);
                    fields.decls.push(decl.clone());
                }
            }
        }

        UnionDeclarationC {
            fields,
            ident_id: format!("{}", usize::arbitrary(g)),
            array_dimension: Arbitrary::arbitrary(g),
        }
    }
}

/// Enables to string and format for UnionDeclarationC types.
impl fmt::Display for UnionDeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "union {{ {} }} union_{}{};",
            self.fields,
            self.ident_id,
            self.array_dimension
        )
    }
}

/// MakeUnique is used in generation of C headers to make
/// FunctionPointerDeclarationC identifiers unique.
impl MakeUnique for FunctionPointerDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

/// A qucickcheck trait for describing how FunctionPointerDeclarationC types can
/// be randomly generated and shrunk.
impl Arbitrary for FunctionPointerDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> FunctionPointerDeclarationC {
        FunctionPointerDeclarationC {
            type_qualifier: Arbitrary::arbitrary(g),
            type_name: Arbitrary::arbitrary(g),
            pointer_level: Arbitrary::arbitrary(g),
            params: Arbitrary::arbitrary(g),
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

/// Enables to string and format for FunctionPointerDeclarationC types.
impl fmt::Display for FunctionPointerDeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} (*func_ptr_{})({});",
            self.type_qualifier,
            self.type_name,
            self.pointer_level,
            self.ident_id,
            self.params
        )
    }
}

/// MakeUnique is used in generation of C headers to make FunctionPrototypeC
/// identifiers unique.
impl MakeUnique for FunctionPrototypeC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

/// A qucickcheck trait for describing how FunctionPrototypeC types can be
/// randomly generated and shrunk.
impl Arbitrary for FunctionPrototypeC {
    fn arbitrary<G: Gen>(g: &mut G) -> FunctionPrototypeC {
        FunctionPrototypeC {
            type_qualifier: Arbitrary::arbitrary(g),
            type_name: Arbitrary::arbitrary(g),
            pointer_level: Arbitrary::arbitrary(g),
            params: Arbitrary::arbitrary(g),
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

/// Enables to string and format for FunctionPrototypeC types.
impl fmt::Display for FunctionPrototypeC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} func_{}({});",
            self.type_qualifier,
            self.type_name,
            self.pointer_level,
            self.ident_id,
            self.params
        )
    }
}

/// A qucickcheck trait for describing how ParameterC types can be
/// randomly generated and shrunk.
impl Arbitrary for ParameterC {
    fn arbitrary<G: Gen>(g: &mut G) -> ParameterC {
        ParameterC {
            type_qualifier: Arbitrary::arbitrary(g),
            type_name: Arbitrary::arbitrary(g),
            pointer_level: Arbitrary::arbitrary(g),
        }
    }
}

/// Enables to string and format for ParameterC types.
impl fmt::Display for ParameterC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.type_qualifier,
            self.type_name,
            self.pointer_level
        )
    }
}

/// A qucickcheck trait for describing how ParameterListC types can be
/// randomly generated and shrunk.
impl Arbitrary for ParameterListC {
    fn arbitrary<G: Gen>(g: &mut G) -> ParameterListC {
        ParameterListC {
            params: Arbitrary::arbitrary(g),
        }
    }
}

/// Enables to string and format for ParameterListC types.
impl fmt::Display for ParameterListC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();
        for (i, p) in self.params.iter().enumerate() {
            match i {
                0 => display += &format!("{}", p),
                _ => display += &format!(",{}", p),
            }
        }
        write!(f, "{}", display)
    }
}

/// A qucickcheck trait for describing how HeaderC types can be
/// randomly generated and shrunk.
impl Arbitrary for HeaderC {
    fn arbitrary<G: Gen>(g: &mut G) -> HeaderC {
        let mut decl_list: DeclarationListC = Arbitrary::arbitrary(g);
        for (i, decl) in decl_list.decls.iter_mut().enumerate() {
            decl.make_unique(i);
        }
        HeaderC { def: decl_list }
    }
}

/// Enables to string and format for HeaderC types.
impl fmt::Display for HeaderC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();
        for decl in &self.def.decls {
            display += &format!("{}", decl);
        }
        write!(f, "{}", display)
    }
}

/// Use Display trait for Debug so that any failing property tests report
/// generated C code rather than the data structures that contain it.
impl fmt::Debug for HeaderC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
