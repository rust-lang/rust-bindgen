#[cfg(test)]
mod tests {
    use bindgen::callbacks::{DeriveInfo, ParseCallbacks};
    use bindgen::{Bindings, Builder};
    use std::path::{Path, PathBuf};

    #[derive(Debug)]
    struct AddDerivesCallback(Vec<String>);

    impl AddDerivesCallback {
        fn new(derives: &[&str]) -> Self {
            Self(derives.iter().map(|s| (*s).to_string()).collect())
        }
    }

    impl ParseCallbacks for AddDerivesCallback {
        fn add_derives(&self, _info: &DeriveInfo<'_>) -> Vec<String> {
            self.0.clone()
        }
    }

    struct WriteAdapter<'a>(&'a mut Vec<u8>);

    impl std::io::Write for WriteAdapter<'_> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn write_bindings_to_string(bindings: &Bindings) -> String {
        let mut output = Vec::<u8>::new();
        bindings
            .write(Box::new(WriteAdapter(&mut output)))
            .unwrap_or_else(|e| {
                panic!("Failed to write generated bindings: {e}")
            });
        String::from_utf8(output).unwrap_or_else(|e| {
            panic!("Failed to convert generated bindings to string: {e}")
        })
    }

    fn make_builder(header_path: &Path, add_derives: &[&str]) -> Builder {
        Builder::default()
            .header(header_path.display().to_string())
            .derive_debug(true)
            .derive_copy(false)
            .derive_default(false)
            .derive_partialeq(false)
            .derive_eq(false)
            .derive_partialord(false)
            .derive_ord(false)
            .derive_hash(false)
            .parse_callbacks(Box::new(AddDerivesCallback::new(add_derives)))
    }

    /// Tests that adding a derive trait that's already derived automatically
    /// does not result in a duplicate derive trait (which would not compile).
    #[test]
    fn test_add_derives_callback_dedupe() {
        let crate_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let header_path = crate_dir.join(
            "tests/parse_callbacks/add_derives_callback/header_add_derives.h",
        );

        let builder = make_builder(&header_path, &["Debug"]);
        let bindings = builder
            .generate()
            .unwrap_or_else(|e| panic!("Failed to generate bindings: {e}"));
        let output = write_bindings_to_string(&bindings);
        let output_without_spaces = output.replace(' ', "");
        assert!(
            output_without_spaces.contains("#[derive(Debug)]") &&
                !output_without_spaces.contains("#[derive(Debug,Debug)]"),
            "Unexpected bindgen output:\n{}",
            output.as_str()
        );
    }

    /// Tests that adding a derive trait that's not already derived automatically
    /// adds it to the end of the derive list.
    #[test]
    fn test_add_derives_callback() {
        let crate_dir =
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let header_path = crate_dir.join(
            "tests/parse_callbacks/add_derives_callback/header_add_derives.h",
        );

        let builder = make_builder(&header_path, &["Default"]);
        let bindings = builder
            .generate()
            .unwrap_or_else(|e| panic!("Failed to generate bindings: {e}"));
        let output = write_bindings_to_string(&bindings);
        let output_without_spaces = output.replace(' ', "");
        assert!(
            output_without_spaces.contains("#[derive(Debug,Default)]"),
            "Unexpected bindgen output:\n{}",
            output.as_str()
        );
    }
}
