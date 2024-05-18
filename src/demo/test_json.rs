#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::json::{self, DumbJsonProcessor, InPlaceJsonEntryHandler, JsonEntry, JsonEntryHandler};

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
    _test_json_simple(true, false);
}
#[test]
pub fn test_json_simple_by_bytes() {
    _test_json_simple(true, true);
}
#[test]
pub fn test_json_simple_chunked() {
    _test_json_simple(false, false);
}
#[test]
pub fn test_json_simple_chunked_by_bytes() {
    _test_json_simple(false, true);
}
fn _test_json_simple(one_piece: bool, by_bytes: bool) {
    let json = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    _test_json(json, &check_map, one_piece, by_bytes);

    let json = r#"{"hello":"wo
rld"}"#;
    let check_map = HashMap::from([(
        "hello", "wo
rld",
    )]);
    _test_json(json, &check_map, one_piece, by_bytes);

    let json = r#"{"hello":"world","hello2":"world2"}"#;
    let check_map = HashMap::from([("hello", "world"), ("hello2", "world2")]);
    _test_json(json, &check_map, one_piece, by_bytes);

    let json = r#"{"hello":" w:\"o{r}l\"[d], "}"#;
    let check_map = HashMap::from([("hello", " w:\"o{r}l\"[d], ")]);
    _test_json(json, &check_map, one_piece, by_bytes);

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
    _test_json(json, &check_map, one_piece, by_bytes);
}

#[test]
pub fn test_json_escaped() {
    _test_json_escaped(true, false);
}
#[test]
pub fn test_json_escaped_by_bytes() {
    _test_json_escaped(true, true);
}
#[test]
pub fn test_json_escaped_chunked() {
    _test_json_escaped(false, false);
}
#[test]
pub fn test_json_escaped_chunked_by_bytes() {
    _test_json_escaped(false, true);
}
fn _test_json_escaped(one_piece: bool, by_bytes: bool) {
    let json = r#"{"hello":"\"\\\""}"#;
    let check_map = HashMap::from([("hello", "\"\\\"")]);
    _test_json(json, &check_map, one_piece, by_bytes);

    let json = r#"{"hello":"\\n\\r\\t\\b\\f"}"#;
    let check_map = HashMap::from([("hello", r#"\n\r\t\b\f"#)]);
    _test_json(json, &check_map, one_piece, by_bytes);
}

#[test]
pub fn test_json_array() {
    _test_json_array(true, false);
}
#[test]
pub fn test_json_array_by_bytes() {
    _test_json_array(true, true);
}
#[test]
pub fn test_json_array_chunked() {
    _test_json_array(false, false);
}
#[test]
pub fn test_json_array_chunked_by_bytes() {
    _test_json_array(false, true);
}
fn _test_json_array(one_piece: bool, by_bytes: bool) {
    let json = r#"{"str_arr":["item0","item1"]}"#;
    let check_map = HashMap::from([("str_arr.0", "item0"), ("str_arr.1", "item1")]);
    _test_json(json, &check_map, one_piece, by_bytes);

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
    _test_json(json, &check_map, one_piece, by_bytes);
}

#[test]
pub fn test_json_obj_array() {
    _test_json_obj_array(true, false);
}
#[test]
pub fn test_json_obj_array_by_bytes() {
    _test_json_obj_array(true, true);
}
#[test]
pub fn test_json_obj_array_chunked() {
    _test_json_obj_array(false, false);
}
#[test]
pub fn test_json_obj_array_chunked_by_bytes() {
    _test_json_obj_array(false, true);
}
fn _test_json_obj_array(one_piece: bool, by_bytes: bool) {
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
    _test_json(json, &check_map, one_piece, by_bytes);
}

#[test]
pub fn test_json_emojis() {
    _test_json_emojis(true, false);
}
#[test]
pub fn test_json_emojis_by_bytes() {
    _test_json_emojis(true, true);
}
#[test]
pub fn test_json_emojis_chunked() {
    _test_json_emojis(false, false);
}
#[test]
pub fn test_json_emojis_chunked_by_bytes() {
    _test_json_emojis(false, true);
}
fn _test_json_emojis(one_piece: bool, by_bytes: bool) {
    let json = r#"{"str":"😀"}"#;
    let check_map = HashMap::from([("str", "😀")]);
    _test_json(json, &check_map, one_piece, by_bytes);

    let json = r#"{
        "str" : "😀😁😂😃😄😅😆😇😈😉😊😋😌😍😎😏"
    }"#;
    let check_map = HashMap::from([("str", "😀😁😂😃😄😅😆😇😈😉😊😋😌😍😎😏")]);
    _test_json(json, &check_map, one_piece, by_bytes);

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
    _test_json(json, &check_map, one_piece, by_bytes);
}

#[test]
pub fn test_json_emojis_bug() {
    let one_piece = false;
    let by_bytes = true;

    let json = r#"{"str":"😀"}"#;
    let check_map = HashMap::from([("str", "😀")]);
    _test_json(json, &check_map, one_piece, by_bytes);

    let json = r#"{"str":"👨‍👩‍👧‍👦"}"#;
    let check_map = HashMap::from([("str", "👨‍👩‍👧‍👦")]);
    _test_json(json, &check_map, one_piece, by_bytes);
}

#[test]
pub fn test_multiple_jsons() {
    _test_multiple_jsons(true, false);
}
#[test]
pub fn test_multiple_jsons_by_bytes() {
    _test_multiple_jsons(true, true);
}
#[test]
pub fn test_multiple_jsons_chunked() {
    _test_multiple_jsons(false, false);
}
#[test]
pub fn test_multiple_jsons_chunked_by_bytes() {
    _test_multiple_jsons(false, true);
}
fn _test_multiple_jsons(one_piece: bool, by_bytes: bool) {
    let json = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    let mut progress = ProcessJsonProgress::new();
    _test_json_ex(json, &check_map, one_piece, by_bytes, &mut progress);
    _test_json_ex(json, &check_map, one_piece, by_bytes, &mut progress);

    let json1 = r#"{"hello":"world"}"#;
    let json2 = r#"{"hello":"world"}"#;
    let check_map = HashMap::from([("hello", "world")]);
    let mut progress = ProcessJsonProgress::new();
    _test_json_ex(json1, &check_map, one_piece, by_bytes, &mut progress);
    _test_json_ex(json2, &check_map, one_piece, by_bytes, &mut progress);
}

#[test]
pub fn test_multiple_jsons_wrapped() {
    _test_multiple_jsons_2(true);
}
#[test]
pub fn test_multiple_jsons_looped() {
    //_test_multiple_jsons_2(false);  // FIXME: it hangs ... debug
}
fn _test_multiple_jsons_2(wrapped: bool) {
    let jsons = r#"[{"country": "Hong Kong", "web_pages": ["https://www.chuhai.edu.hk/"], "alpha_two_code": "HK", "domains": ["chuhai.edu.hk"], "state-province": null, "name": "Hong Kong Chu Hai College"}, {"country": "Hong Kong", "web_pages": ["https://www.cityu.edu.hk/"], "alpha_two_code": "HK", "domains": ["cityu.edu.hk", "um.cityu.edu.hk", "my.cityu.edu.hk"], "state-province": null, "name": "City University of Hong Kong"}, {"country": "Hong Kong", "web_pages": ["https://www.cuhk.edu.hk/"], "alpha_two_code": "HK", "domains": ["cuhk.edu.hk", "link.cuhk.edu.hk"], "state-province": null, "name": "The Chinese University of Hong Kong"}, {"country": "Hong Kong", "web_pages": ["https://www.hkapa.edu/"], "alpha_two_code": "HK", "domains": ["hkapa.edu"], "state-province": null, "name": "The Hong Kong Academy for Performing Arts"}, {"country": "Hong Kong", "web_pages": ["https://www.hkbu.edu.hk/"], "alpha_two_code": "HK", "domains": ["hkbu.edu.hk", "life.hkbu.edu.hk", "associate.hkbu.edu.hk"], "state-province": null, "name": "Hong Kong Baptist University"}, {"country": "Hong Kong", "web_pages": ["https://www.hksyu.edu/"], "alpha_two_code": "HK", "domains": ["hksyu.edu"], "state-province": null, "name": "Hong Kong Shue Yan University"}, {"country": "Hong Kong", "web_pages": ["https://www.hku.hk/"], "alpha_two_code": "HK", "domains": ["hku.hk"], "state-province": null, "name": "The University of Hong Kong"}, {"country": "Hong Kong", "web_pages": ["https://www.ln.edu.hk/"], "alpha_two_code": "HK", "domains": ["ln.edu.hk", "ln.hk"], "state-province": null, "name": "Lingnan University"}, {"country": "Hong Kong", "web_pages": ["https://www.hkmu.edu.hk/"], "alpha_two_code": "HK", "domains": ["hkmu.edu.hk", "ouhk.edu.hk"], "state-province": null, "name": "Hong Kong Metropolitan University"}, {"country": "Hong Kong", "web_pages": ["https://www.polyu.edu.hk/"], "alpha_two_code": "HK", "domains": ["polyu.edu.hk", "connect.polyu.hk"], "state-province": null, "name": "The Hong Kong Polytechnic University"}, {"country": "Hong Kong", "web_pages": ["https://hkust.edu.hk/"], "alpha_two_code": "HK", "domains": ["ust.hk", "connect.ust.hk"], "state-province": null, "name": "The Hong Kong University of Science and Technology"}, {"country": "Hong Kong", "web_pages": ["https://www.eduhk.hk"], "alpha_two_code": "HK", "domains": ["s.eduhk.hk", "eduhk.hk"], "state-province": null, "name": "The Education University of Hong Kong"}, {"country": "Hong Kong", "web_pages": ["http://www.hsu.edu.hk/"], "alpha_two_code": "HK", "domains": ["hsu.edu.hk"], "state-province": null, "name": "The Hang Seng University of Hong Kong"}, {"country": "Hong Kong", "web_pages": ["https://cdnis.edu.hk"], "alpha_two_code": "HK", "domains": ["cdnis.edu.hk"], "state-province": null, "name": "Canadian International School of Hong Kong"}]"#;
    let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "* `{}` => `{}`",
            json_entry.field_name, json_entry.field_value
        );
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    if wrapped {
        let mut input = String::new();
        input.push_str("{\"universities\":");
        input.push_str(&jsons);
        input.push_str("}");
        //println!("{}", input);
        json_processor.push_json(&input);
    } else {
        let mut progress = ProcessJsonProgress::new();
        let mut input = jsons.to_string();
        loop {
            json_processor.push_json_piece(&input, &mut progress);
            input = progress.get_remaining();
            //println!("*** {}", input);
            //break;
            if input.is_empty() {
                break;
            }
        }
    }
}

#[test]
pub fn test_multiple_json_pieces() {
    let json_pieces = vec![r#"{"hello1":"world1"}"#, r#"{"hello2":"world2"}"#];
    let check_map = HashMap::from([("hello1", "world1"), ("hello2", "world2")]);
    _test_multiple_json_pieces(json_pieces, check_map);

    let json_pieces = vec![r#"{"hello1""#, r#":"world1"}{"hello2""#, r#":"world2"}"#];
    let check_map = HashMap::from([("hello1", "world1"), ("hello2", "world2")]);
    _test_multiple_json_pieces(json_pieces, check_map);
}

// #[test]
// pub fn test_multiple_json_pieces() {
//     let mut handler = TestJsonEntryHandler::new();
//     let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
//     for json_piece in json_pieces {
//         let result = json_processor.push_json_piece(json_piece, &mut progress);
//         if result.is_err() {
//             panic!("chunk result is err [{}]", result.unwrap_err());
//         }
//     }
// }

// fn _test_json(json: &str, check_map: &HashMap<&str, &str>) {
//     _test_json_ex(json, check_map, true);
//     _test_json_ex(json, check_map, false);
// }
fn _test_json(json: &str, check_map: &HashMap<&str, &str>, one_piece: bool, by_bytes: bool) {
    let mut progress = ProcessJsonProgress::new();
    _test_json_ex(json, check_map, one_piece, by_bytes, &mut progress);
    if one_piece {
        assert!(progress.get_remaining().is_empty());
    }
}
fn _test_json_ex(
    json: &str,
    check_map: &HashMap<&str, &str>,
    one_piece: bool,
    by_bytes: bool,
    progress: &mut ProcessJsonProgress,
) {
    let mut handler = TestJsonEntryHandler::new();
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    if one_piece {
        if by_bytes {
            let json_bytes = json.as_bytes();
            let result = json_processor.push_json_bytes(json_bytes, progress);
            if result.is_err() {
                panic!("one-piece result is err [{}]", result.unwrap_err());
            }
        } else {
            let result = json_processor.push_json_piece(json, progress);
            if result.is_err() {
                panic!("one-piece result is err [{}]", result.unwrap_err());
            }
        }
    } else {
        if by_bytes {
            let json_bytes = json.as_bytes();
            //let json_chars: Vec<char> = json.chars().collect();
            let len = json_bytes.len();
            let mut start = 0;
            let mut end = 0;
            while end < len {
                end = start + 5;
                if end > len {
                    end = len;
                }
                let json_piece_bytes = &json_bytes[start..end];
                //let json_piece: String = json_piece_chars.iter().collect();
                let result = json_processor.push_json_bytes(json_piece_bytes, progress);
                if result.is_err() {
                    panic!("chunk result is err [{}]", result.unwrap_err());
                }
                start = end;
            }
            if !progress.is_done() {
                panic!("progress is done");
            }
            let remaining = progress.get_remaining();
            if !remaining.is_empty() {
                panic!("remaining is not empty -- [{}]", remaining);
            }
        } else {
            let json_chars: Vec<char> = json.chars().collect();
            let len = json_chars.len();
            let mut start = 0;
            let mut end = 0;
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
                start = end;
            }
            if !progress.is_done() {
                panic!("progress is done");
            }
            let remaining = progress.get_remaining();
            if !remaining.is_empty() {
                panic!("remaining is not empty -- [{}]", remaining);
            }
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

fn _test_multiple_json_pieces(json_pieces: Vec<&str>, check_map: HashMap<&str, &str>) {
    let mut progress = ProcessJsonProgress::new();
    let mut handler = TestJsonEntryHandler::new();
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));

    for json_piece in json_pieces {
        let result = json_processor.push_json_piece(json_piece, &mut progress);
        if result.is_err() {
            panic!("chunk result is err [{}]", result.unwrap_err());
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
