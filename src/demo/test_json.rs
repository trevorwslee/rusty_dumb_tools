use std::collections::HashMap;

use crate::json::{DumbJsonProcessor, InPlaceJsonEntryHandler, JsonEntry, JsonEntryHandler};

use super::ProcessJsonProgress;

#[test]
pub fn test_json_in_place() {
    let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "In-Place JSON entry: {} => {}",
            json_entry.field_name, json_entry.field_value
        );
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    let json = r#"{"hello":"world"}"#;
    let res = json_processor.push_json(json);
    assert!(res.is_ok() && res.unwrap().is_empty());
    print!("~~~")
}

#[test]
pub fn test_json_simple() {
    let json = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    _test_json(json, &check_map);

    let json = r#"{"hello":" w:\"o{r}l\"[d], "}"#;
    let check_map = HashMap::from([("hello", " w:\"o{r}l\"[d], ")]);
    _test_json(json, &check_map);

    let json = r#"{
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
    _test_json(json, &check_map);
}

#[test]
pub fn test_json_array() {
    let json = r#"
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
    _test_json(json, &check_map);
}

#[test]
pub fn test_json_obj_array() {
    let json = r#"
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
    _test_json(json, &check_map);
}

fn _test_json(json: &str, check_map: &HashMap<&str, &str>) {
    _test_json_ex(json, check_map, true);
    _test_json_ex(json, check_map, true);
}
fn _test_json_ex(json: &str, check_map: &HashMap<&str, &str>, one_piece: bool) {
    let mut handler = TestJsonEntryHandler::new();
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    if one_piece {
        let res = json_processor.push_json(json);
        if res.is_err() {
            panic!("res is err");
        }
        let res = res.unwrap();
        if !res.is_empty() {
            panic!("res is not empty");
        }
    } else {
        let json_piece = json.trim();
        let len = json_piece.len();
        let mut start = 0;
        let mut end = 0;
        let mut progress = ProcessJsonProgress::new();
        while end < len {
            //let mut rng = rand::thread_rng();
            //end = rng.gen_range(start + 1, len + 1);
            end = start + 5;
            if end > len {
                end = len;
            }
            let json_piece = &json_piece[start..end];
            let result = json_processor.push_json_piece(json_piece, &mut progress);
            if result.is_err() {
                panic!("result is err");
            }
            // let res = res.unwrap();
            // if !res.is_empty() {
            //     panic!("res is not empty");
            // }
            start = end;
        }

        if !progress.is_done() {
            panic!("progress is done");
        }
        let remaining = progress.get_remaining();
        if !remaining.is_empty() {
            panic!("remaining is not empty");
        }
    }
    let res_map = handler.entry_map;
    if res_map.len() != check_map.len() {
        println!("res_map: {:?}", res_map);
        println!("check_map: {:?}", check_map);
        panic!("res_map.len() != check_map.len()");
    }
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
