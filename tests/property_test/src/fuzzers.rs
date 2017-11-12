use quickcheck::{Arbitrary, Gen, StdGen};
use std::fmt;
use rand::thread_rng;

#[derive(PartialEq, Debug, Clone)]
pub struct BaseTypeC {
    pub def: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TypeQualifierC {
    pub def: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct PointerLevelC {
    pub def: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArrayDimensionC {
    pub def: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct BasicTypeDeclarationC {
    pub type_name: String,
    pub type_qualifier: String,
    pub pointer_level: String,
    pub ident_id: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct StructDeclarationC {
    pub fields: String,
    pub ident_id: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct UnionDeclarationC {
    pub fields: String,
    pub ident_id: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionPointerDeclarationC {
    pub type_qualifier: String,
    pub type_name: String,
    pub pointer_level: String,
    pub params: String,
    pub ident_id: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionPrototypeC {
    pub type_qualifier: String,
    pub type_name: String,
    pub pointer_level: String,
    pub params: String,
    pub ident_id: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParameterC {
    pub type_qualifier: String,
    pub type_name: String,
    pub pointer_level: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ParameterListC {
    def: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct HeaderC {
    pub def: String,
}

#[derive(PartialEq, Debug, Clone)]
enum DeclarationC {
    FunctionDecl(FunctionPrototypeC),
    FunctionPtrDecl(FunctionPointerDeclarationC),
    StructDecl(StructDeclarationC),
    UnionDecl(UnionDeclarationC),
    VariableDecl(BasicTypeDeclarationC),
}

trait MakeUnique {
    fn make_unique(&mut self, stamp: usize);
}

impl MakeUnique for DeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        match self {
            &mut DeclarationC::FunctionDecl(ref mut d) => d.make_unique(stamp),
            &mut DeclarationC::FunctionPtrDecl(ref mut d) => d.make_unique(stamp),
            &mut DeclarationC::StructDecl(ref mut d) => d.make_unique(stamp),
            &mut DeclarationC::UnionDecl(ref mut d) => d.make_unique(stamp),
            &mut DeclarationC::VariableDecl(ref mut d) => d.make_unique(stamp),
        }
    }
}

impl Arbitrary for DeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> DeclarationC {
        match usize::arbitrary(g) % 5 {
            0 => DeclarationC::FunctionDecl(FunctionPrototypeC::arbitrary(g)),
            1 => DeclarationC::FunctionPtrDecl(FunctionPointerDeclarationC::arbitrary(g)),
            2 => DeclarationC::StructDecl(StructDeclarationC::arbitrary(g)),
            3 => DeclarationC::UnionDecl(UnionDeclarationC::arbitrary(g)),
            _ => DeclarationC::VariableDecl(BasicTypeDeclarationC::arbitrary(g)),
        }
    }
}

impl fmt::Display for DeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DeclarationC::FunctionPtrDecl(ref d) => write!(f, "{}", d),
            &DeclarationC::StructDecl(ref d) => write!(f, "{}", d),
            &DeclarationC::UnionDecl(ref d) => write!(f, "{}", d),
            &DeclarationC::VariableDecl(ref d) => write!(f, "{}", d),
            &DeclarationC::FunctionDecl(ref d) => write!(f, "{}", d),
        }
    }
}

impl Arbitrary for BaseTypeC {
    fn arbitrary<G: Gen>(g: &mut G) -> BaseTypeC {
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
            "long double",
            "void*",
            "whitelistable",
            "blacklistable",
        ];
        match base_type.iter().nth(usize::arbitrary(g) % base_type.len()) {
            Some(s) => BaseTypeC {
                def: String::from(*s),
            },
            None => BaseTypeC {
                def: String::from("int"),
            },
        }
    }
}

impl fmt::Display for BaseTypeC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

impl Arbitrary for TypeQualifierC {
    fn arbitrary<G: Gen>(g: &mut G) -> TypeQualifierC {
        let qualifier = vec!["const", ""];
        match qualifier.iter().nth(usize::arbitrary(g) % qualifier.len()) {
            Some(s) => TypeQualifierC {
                def: String::from(*s),
            },
            None => TypeQualifierC {
                def: String::from(""),
            },
        }
    }
}

impl fmt::Display for TypeQualifierC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

impl Arbitrary for PointerLevelC {
    fn arbitrary<G: Gen>(g: &mut G) -> PointerLevelC {
        PointerLevelC {
            def: (0..usize::arbitrary(g)).map(|_| "*").collect::<String>(),
        }
    }
}

impl fmt::Display for PointerLevelC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

impl Arbitrary for ArrayDimensionC {
    fn arbitrary<G: Gen>(g: &mut G) -> ArrayDimensionC {
        // keep these small, they clang complains when they get too big
        let dimensions = usize::arbitrary(g) % 5;
        let mut def = String::new();
        // don't allow size 0 dimension until #684 and #1153 are closed
        for _ in 1..dimensions {
            def += &format!("[{}]", (usize::arbitrary(g) % 15) + 1);
        }
        ArrayDimensionC { def: def }
    }
}

impl fmt::Display for ArrayDimensionC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

impl MakeUnique for BasicTypeDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

impl Arbitrary for BasicTypeDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> BasicTypeDeclarationC {
        BasicTypeDeclarationC {
            type_qualifier: TypeQualifierC::arbitrary(g).def,
            type_name: BaseTypeC::arbitrary(g).def,
            pointer_level: PointerLevelC::arbitrary(g).def,
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

impl fmt::Display for BasicTypeDeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} ident_{};",
            self.type_qualifier,
            self.type_name,
            self.pointer_level,
            self.ident_id
        )
    }
}

impl MakeUnique for StructDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

impl Arbitrary for StructDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> StructDeclarationC {
        let mut fields_string = String::new();
        // reduce generator size as a method of putting a bound on recursion.
        // when size < 1 the empty list is generated.
        let reduced_size: usize = (g.size() / 2) as usize + 1;
        let mut decls: Vec<DeclarationC> =
            Arbitrary::arbitrary(&mut StdGen::new(thread_rng(), reduced_size));

        for (i, decl) in decls.iter_mut().enumerate() {
            match decl {
                &mut DeclarationC::FunctionDecl(_) => {}
                decl => {
                    decl.make_unique(i);
                    fields_string += &format!("{}", decl);
                }
            }
        }

        StructDeclarationC {
            fields: fields_string,
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

impl fmt::Display for StructDeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "struct {{ {} }} struct_{};", self.fields, self.ident_id)
    }
}

impl MakeUnique for UnionDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

impl Arbitrary for UnionDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> UnionDeclarationC {
        let mut fields_string = String::new();
        // reduce generator size as a method of putting a bound on recursion.
        // when size < 1 the empty list is generated.
        let reduced_size: usize = (g.size() / 2) as usize + 1;
        let mut decls: Vec<DeclarationC> =
            Arbitrary::arbitrary(&mut StdGen::new(thread_rng(), reduced_size));

        for (i, decl) in decls.iter_mut().enumerate() {
            match decl {
                &mut DeclarationC::FunctionDecl(_) => {}
                decl => {
                    decl.make_unique(i);
                    fields_string += &format!("{}", decl);
                }
            }
        }

        UnionDeclarationC {
            fields: fields_string,
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

impl fmt::Display for UnionDeclarationC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "union {{ {} }} union_{};", self.fields, self.ident_id)
    }
}

impl MakeUnique for FunctionPointerDeclarationC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

impl Arbitrary for FunctionPointerDeclarationC {
    fn arbitrary<G: Gen>(g: &mut G) -> FunctionPointerDeclarationC {
        FunctionPointerDeclarationC {
            type_qualifier: format!("{}", TypeQualifierC::arbitrary(g)),
            type_name: format!("{}", BaseTypeC::arbitrary(g)),
            pointer_level: format!("{}", PointerLevelC::arbitrary(g)),
            params: format!("{}", ParameterListC::arbitrary(g)),
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

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

impl MakeUnique for FunctionPrototypeC {
    fn make_unique(&mut self, stamp: usize) {
        self.ident_id += &format!("_{}", stamp);
    }
}

impl Arbitrary for FunctionPrototypeC {
    fn arbitrary<G: Gen>(g: &mut G) -> FunctionPrototypeC {
        FunctionPrototypeC {
            type_qualifier: format!("{}", TypeQualifierC::arbitrary(g)),
            type_name: format!("{}", BaseTypeC::arbitrary(g)),
            pointer_level: format!("{}", PointerLevelC::arbitrary(g)),
            params: format!("{}", ParameterListC::arbitrary(g)),
            ident_id: format!("{}", usize::arbitrary(g)),
        }
    }
}

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

impl Arbitrary for ParameterC {
    fn arbitrary<G: Gen>(g: &mut G) -> ParameterC {
        ParameterC {
            type_qualifier: format!("{}", TypeQualifierC::arbitrary(g)),
            type_name: format!("{}", BaseTypeC::arbitrary(g)),
            pointer_level: format!("{}", PointerLevelC::arbitrary(g)),
        }
    }
}

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

impl Arbitrary for ParameterListC {
    fn arbitrary<G: Gen>(g: &mut G) -> ParameterListC {
        let mut params_string = String::new();
        let params: Vec<ParameterC> = Arbitrary::arbitrary(g);
        for (i, p) in params.iter().enumerate() {
            match i {
                0 => params_string += &format!("{}", p),
                _ => params_string += &format!(",{}", p),
            }
        }

        ParameterListC { def: params_string }
    }
}

impl fmt::Display for ParameterListC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}

impl Arbitrary for HeaderC {
    fn arbitrary<G: Gen>(g: &mut G) -> HeaderC {
        let known_types = "typedef struct { short s; } whitelistable; \
                           typedef struct { float f;} blacklistable;";
        let mut header_c = String::from(known_types);
        let mut decls: Vec<DeclarationC> = Arbitrary::arbitrary(g);

        for (i, decl) in decls.iter_mut().enumerate() {
            decl.make_unique(i);
            header_c += &format!("{}", decl);
        }

        HeaderC { def: header_c }
    }
}

impl fmt::Display for HeaderC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.def)
    }
}
