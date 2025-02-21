use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use bindgen::{
    callbacks::{DiscoveredItem, DiscoveredItemId, ParseCallbacks},
    Builder,
};

#[derive(Debug)]
struct UnusedTemplateParamCallback(
    Rc<RefCell<HashSet<DiscoveredItemId>>>,
    Rc<RefCell<HashMap<DiscoveredItemId, String>>>,
);

impl ParseCallbacks for UnusedTemplateParamCallback {
    fn denote_discards_template_param(&self, id: DiscoveredItemId) {
        self.0.borrow_mut().insert(id);
    }

    fn new_item_found(
        &self,
        id: DiscoveredItemId,
        item: DiscoveredItem,
        _source_location: Option<&bindgen::callbacks::SourceLocation>,
        _parent: Option<DiscoveredItemId>,
    ) {
        match item {
            DiscoveredItem::Struct {
                final_name: name, ..
            } |
            DiscoveredItem::Alias {
                alias_name: name, ..
            } => {
                self.1.borrow_mut().insert(id, name);
            }
            _ => {}
        }
    }
}

/// This test could be combined with the `item_discovery_callback`
/// test, but as this is a separate callback we'll keep the tests
/// separate too.
#[test]
fn test_unused_template_param() {
    let discovered = Rc::new(RefCell::new(HashSet::new()));
    let names = Rc::new(RefCell::new(HashMap::new()));
    let header_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/parse_callbacks/unused_template_param.hpp"
    );
    Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(UnusedTemplateParamCallback(
            Rc::clone(&discovered),
            Rc::clone(&names),
        )))
        .clang_arg("--std=c++11")
        .generate()
        .expect("TODO: panic message");

    let reported_unused_template_parameter_item_names: HashSet<_> = discovered
        .borrow()
        .iter()
        .map(|id| names.borrow().get(id).expect("Name not reported").clone())
        .collect();

    assert_eq!(
        reported_unused_template_parameter_item_names,
        HashSet::from([
            "IgnoresTemplateParam".to_string(),
            "TypedefIgnoresTemplateParam2".to_string()
        ]),
        "Different items than expected reported unused template params"
    );
}
