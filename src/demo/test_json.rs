use std::collections::HashMap;

use crate::json::{DumbJsonProcessor, InPlaceJsonEntryHandler, JsonEntry, JsonEntryHandler};

use super::ProcessJsonProgress;

#[test]
pub fn test_json_in_place() {
    let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "In-Place JSON entry: `{}` => `{}`",
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
    _test_json_simple(true);
}
#[test]
pub fn test_json_simple_chunked() {
    _test_json_simple(false);
}
fn _test_json_simple(one_piece: bool) {
    let json = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    _test_json(json, &check_map, one_piece);

    let json = r#"{"hello":"world","hello2":"world2"}"#;
    let check_map = HashMap::from([("hello", "world"), ("hello2", "world2")]);
    _test_json(json, &check_map, one_piece);

    let json = r#"{"hello":" w:\"o{r}l\"[d], "}"#;
    let check_map = HashMap::from([("hello", " w:\"o{r}l\"[d], ")]);
    _test_json(json, &check_map, one_piece);

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
    _test_json(json, &check_map, one_piece);
}

#[test]
pub fn test_json_array() {
    _test_json_array(true);
}
#[test]
pub fn test_json_array_chunked() {
    _test_json_array(false);
}
fn _test_json_array(one_piece: bool) {
    let json = r#"{"str_arr":["item0","item1"]}"#;
    let check_map = HashMap::from([("str_arr.0", "item0"), ("str_arr.1", "item1")]);
    _test_json(json, &check_map, one_piece);

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
    _test_json(json, &check_map, one_piece);
}

#[test]
pub fn test_json_obj_array() {
    _test_json_obj_array(true);
}
#[test]
pub fn test_json_obj_array_chunked() {
    _test_json_obj_array(false);
}
fn _test_json_obj_array(one_piece: bool) {
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
    _test_json(json, &check_map, one_piece);
}

#[test]
pub fn test_json_emojis() {
    _test_json_emojis(true);
}
#[test]
pub fn test_json_emojis_chunked() {
    _test_json_emojis(false);
}
fn _test_json_emojis(one_piece: bool) {
    let json = r#"{"str":"😀"}"#;
    let check_map = HashMap::from([("str", "😀")]);
    _test_json(json, &check_map, one_piece);

    let json = r#"{
        "str" : "😀😁😂😃😄😅😆😇😈😉😊😋😌😍😎😏"
    }"#;
    let check_map = HashMap::from([("str", "😀😁😂😃😄😅😆😇😈😉😊😋😌😍😎😏")]);
    _test_json(json, &check_map, one_piece);
}


// fn _test_json(json: &str, check_map: &HashMap<&str, &str>) {
//     _test_json_ex(json, check_map, true); // TODO: enable this line
//     _test_json_ex(json, check_map, false);
// }
fn _test_json(json: &str, check_map: &HashMap<&str, &str>, one_piece: bool) {
    let mut handler = TestJsonEntryHandler::new();
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    if one_piece {
        let res = json_processor.push_json(json);
        if res.is_err() {
            panic!("res is err [{}]", res.unwrap_err());
        }
        let res = res.unwrap();
        if !res.is_empty() {
            panic!("res is not empty");
        }
    } else {
        let json_chars: Vec<char> = json.chars().collect();
        //let json_piece = json.trim();
        let len = json_chars.len();
        let mut start = 0;
        let mut end = 0;
        let mut progress = ProcessJsonProgress::new();
        while end < len {
            end = start + 5;
            if end > len {
                end = len;
            }
            let json_piece_chars = &json_chars[start..end];
            let json_piece: String = json_piece_chars.iter().collect();
            let result = json_processor.push_json_piece(json_piece.as_str(), &mut progress);
            if result.is_err() {
                panic!("result is err [{}]", result.unwrap_err());
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
