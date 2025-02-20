
#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


# [repr (C)] # [derive (Debug , Default , Copy , Clone , serde :: Serialize)] # [serde (rename_all = "UPPERCASE")] pub struct color { pub red : :: std :: os :: raw :: c_int , pub green : :: std :: os :: raw :: c_int , pub blue : :: std :: os :: raw :: c_int , }