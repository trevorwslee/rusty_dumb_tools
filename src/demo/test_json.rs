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
        assert!(json_entry.field_name == "greeting");
        assert!(
            json_entry.field_value.to_string()
                == "Hi❗ How are u/U/ü/Ü/ú/Ù/ü/Û/姑❓  😉👨‍👩‍👧‍👦👍🏽 / 🍔🧑"
        );
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    let json = r#"{ "greeting" : "Hi❗ How are u/U/ü/Ü/ú/Ù/ü/Û/姑❓  😉👨‍👩‍👧‍👦👍🏽 / 🍔🧑" }"#;
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

    let json = r#"{"hello":"wo
rld"}"#;
    let check_map = HashMap::from([(
        "hello", "wo
rld",
    )]);
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
pub fn test_json_escaped() {
    _test_json_escaped(true);
}
#[test]
pub fn test_json_escaped_chunked() {
    _test_json_escaped(false);
}
fn _test_json_escaped(one_piece: bool) {
    let json = r#"{"hello":"\"\\\""}"#;
    let check_map = HashMap::from([("hello", "\"\\\"")]);
    _test_json(json, &check_map, one_piece);

    let json = r#"{"hello":"\\n\\r\\t\\b\\f"}"#;
    let check_map = HashMap::from([("hello", r#"\n\r\t\b\f"#)]);
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

    // Emojis that are composed of multiple Unicode characters cannot be represented by a single char in Rust. These include emojis with skin tone modifiers, gender modifiers, or those that represent complex symbols like country flags.
    // Here are a few examples:
    // 1. *Emojis with skin tone modifiers*: These emojis use a base emoji followed by a skin tone modifier. For example, the emoji 👍🏽 (thumbs up with a medium skin tone) is composed of two Unicode characters: 👍 (thumbs up) and 🏽 (medium skin tone modifier).
    // 2. *Emojis with gender modifiers*: These emojis use a base emoji followed by a gender modifier. For example, the emoji 👩‍⚕ (woman health worker) is composed of three Unicode characters: 👩 (woman), ‍ (Zero Width Joiner), and ⚕ (medical symbol).
    // 3. *Country flag emojis*: These emojis are composed of two regional indicator symbols. For example, the emoji 🇺🇸 (flag of the United States) is composed of two Unicode characters: 🇺 (regional indicator symbol letter U) and 🇸 (regional indicator symbol letter S).
    // 4. *Emojis with Zero Width Joiner (ZWJ) sequences*: These emojis are composed of multiple emojis joined by a Zero Width Joiner. For example, the emoji 👨‍👩‍👧‍👦 (family: man, woman, girl, boy) is composed of seven Unicode characters: 👨 (man), ‍ (ZWJ), 👩 (woman), ‍ (ZWJ), 👧 (girl), ‍ (ZWJ), and 👦 (boy).

    let json = r#"{
        "str" : "👍🏽👩‍⚕ 🇺🇸🇭🇰👨‍👩‍👧‍👦"
    }"#;
    let check_map = HashMap::from([("str", "👍🏽👩‍⚕ 🇺🇸🇭🇰👨‍👩‍👧‍👦")]);
    _test_json(json, &check_map, one_piece);
}

#[test]
pub fn test_multiple_jsons() {
    _test_multiple_jsons(true);
}
#[test]
pub fn test_multiple_jsons_chunked() {
    _test_multiple_jsons(false);
}
fn _test_multiple_jsons(one_piece: bool) {
    let json = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    let mut progress = ProcessJsonProgress::new();
    _test_json_ex(json, &check_map, one_piece, &mut progress);
    _test_json_ex(json, &check_map, one_piece, &mut progress);
}

// fn _test_json(json: &str, check_map: &HashMap<&str, &str>) {
//     _test_json_ex(json, check_map, true); // TODO: enable this line
//     _test_json_ex(json, check_map, false);
// }
fn _test_json(json: &str, check_map: &HashMap<&str, &str>, one_piece: bool) {
    let mut progress = ProcessJsonProgress::new();
    _test_json_ex(json, check_map, one_piece, &mut progress);
    if one_piece {
        assert!(progress.get_remaining().is_empty());
    }
}
fn _test_json_ex(
    json: &str,
    check_map: &HashMap<&str, &str>,
    one_piece: bool,
    progress: &mut ProcessJsonProgress,
) {
    let mut handler = TestJsonEntryHandler::new();
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    if one_piece {
        let result = json_processor.push_json_piece(json, progress);
        if result.is_err() {
            panic!("one-piece result is err [{}]", result.unwrap_err());
        }
    // let res = json_processor.push_json(json);
    //     if res.is_err() {
    //         panic!("res is err [{}]", res.unwrap_err());
    //     }
    //     let res = res.unwrap();
    //     if !res.is_empty() {
    //         panic!("res is not empty");
    //     }
    } else {
        let json_chars: Vec<char> = json.chars().collect();
        //let json_piece = json.trim();
        let len = json_chars.len();
        let mut start = 0;
        let mut end = 0;
        //let mut progress = ProcessJsonProgress::new();
        while end < len {
            end = start + 5;
            if end > len {
                end = len;
            }
            let json_piece_chars = &json_chars[start..end];
            let json_piece: String = json_piece_chars.iter().collect();
            let result = json_processor.push_json_piece(json_piece.as_str(), progress);
            if result.is_err() {
                panic!("chunk result is err [{}]", result.unwrap_err());
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
