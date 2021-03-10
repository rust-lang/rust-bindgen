use bindgen::callbacks::ParseCallbacks;

pub fn lookup(cb: &str) -> Box<dyn ParseCallbacks> {
    match cb {
        _ => panic!("Couldn't find name ParseCallbacks: {}", cb),
    }
}
