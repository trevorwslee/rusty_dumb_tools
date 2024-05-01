use std::collections::HashMap;

use crate::json::{DumbJsonProcessor, JsonEntry, JsonEntryHandler};

#[test]
pub fn test_json_simple() {
    let json_segment = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    _test_json_standard(json_segment, check_map);

    let json_segment = r#"{"int":123,"float":9.87,"str":"abc","null":null}"#;
    let check_map = HashMap::from([
        ("int", "123"),
        ("float", "9.87"),
        ("str", "abc"),
        ("null", "null"),
    ]);
    _test_json_standard(json_segment, check_map);
}

fn _test_json_standard(json_segment: &str, check_map: HashMap<&str, &str>) {
    let mut handler = TestJsonEntryHandler::new();
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    let res = json_processor.push_json_segment(json_segment);
    assert!(res.is_some() && res.unwrap().is_empty());
    let res_map = handler.entry_map;
    assert!(res_map.len() == check_map.len());
    for (k, v) in res_map.iter() {
        assert!(check_map.contains_key(k.as_str()));
        assert!(check_map.get(k.as_str()).unwrap() == v);
    }
}

struct TestJsonEntryHandler {
    pub entry_map: HashMap<String, String>,
}
impl TestJsonEntryHandler {
    fn new() -> Self {
        TestJsonEntryHandler {
            entry_map: HashMap::new(),
        }
    }
}
impl JsonEntryHandler for TestJsonEntryHandler {
    fn handle_json_entry(&mut self, json_entry: &JsonEntry) {
        println!(
            "* JSON entry: \"{}\" => \"{}\"",
            json_entry.field_name, json_entry.field_value
        );
        self.entry_map.insert(
            json_entry.field_name.clone(),
            json_entry.field_value.clone(),
        );
    }
}
