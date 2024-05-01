use std::collections::HashMap;

use crate::json::{DumbJsonProcessor, InPlaceJsonEntryHandler, JsonEntry, JsonEntryHandler};

#[test]
pub fn test_json_in_place() {
    let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "In-Place JSON entry: {} => {}",
            json_entry.field_name, json_entry.field_value
        );
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    let json_segment = r#"{"hello":"world"}"#;
    let res = json_processor.push_json_segment(json_segment);
    assert!(res.is_some() && res.unwrap().is_empty());
    print!("~~~")
}

#[test]
pub fn test_json_simple() {
    let json_segment = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    _test_json(json_segment, check_map);

    let json_segment = r#"{"hello":" w:\"o{r}l\"[d], "}"#;
    let check_map = HashMap::from([("hello", " w:\"o{r}l\"[d], ")]);
    _test_json(json_segment, check_map);

    let json_segment = r#"{
          "int" : 123 ,
          "float" : 9.87 ,
          "str" : "this is abc" ,
          "null" : null
        }"#;
    let check_map = HashMap::from([
        ("int", "123"),
        ("float", "9.87"),
        ("str", "this is abc"),
        ("null", "null"),
    ]);
    _test_json(json_segment, check_map);
}

#[test]
pub fn test_json_array() {
    let json_segment = r#"
    {
        "str": "this is abc",
        "str_arr" : [ "item0" , "item1" ],
        "int" : 123,
        "int_arr" : [ 0 , 1 ],
    }"#;
    let check_map = HashMap::from([
        ("str", "this is abc"),
        ("str_arr.0", "item0"),
        ("str_arr.1", "item1"),
        ("int", "123"),
        ("int_arr.0", "0"),
        ("int_arr.1", "1"),
    ]);
    _test_json(json_segment, check_map);
}

#[test]
pub fn test_json_obj_array() {
    let json_segment = r#"
    {
        "items": [ 
            {
              "str" :  "this is abc" ,
              "str_arr" : [ "item0" , "item1" ] ,
              "int" : 123 ,
              "int_arr" : [ 0 , 1 ]
            }, {
                "str" :  "str2" ,
                "float" : 1.234
            }
         ]
    }"#;
    let check_map = HashMap::from([
        ("items.0.str", "this is abc"),
        ("items.0.str_arr.0", "item0"),
        ("items.0.str_arr.1", "item1"),
        ("items.0.int", "123"),
        ("items.0.int_arr.0", "0"),
        ("items.0.int_arr.1", "1"),
        ("items.1.str", "str2"),
        ("items.1.float", "1.234"),
    ]);
    _test_json(json_segment, check_map);
}

fn _test_json(json_segment: &str, check_map: HashMap<&str, &str>) {
    let mut handler = TestJsonEntryHandler::new();
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    let res = json_processor.push_json_segment(json_segment);
    if res.is_none() {
        panic!("res is none");
    }
    let res = res.unwrap();
    if !res.is_empty() {
        panic!("res is not empty");
    }
    //assert!(res.is_some() && res.unwrap().is_empty());
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
            json_entry.field_name,
            json_entry.field_value.to_string()
        );
        self.entry_map.insert(
            json_entry.field_name.clone(),
            json_entry.field_value.to_string().clone(),
        );
    }
}
