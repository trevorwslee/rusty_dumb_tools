//! A simple JSON processor / stream parser -- [`crate::json::DumbJsonProcessor`]

#![deny(warnings)]
#![allow(unused)]

use std::fmt;

//use unicode_segmentation::UnicodeSegmentation;

use crate::prelude::DumbError;

const DEBUG_ON: bool = false;

// fn main() {
//     let s = "Hello, world! 👋🌍".to_string();
//     let graphemes = UnicodeSegmentation::graphemes(s.as_str(), true).collect::<Vec<&str>>();
//     println!("{:?}", graphemes);
// }

// #[test]
// fn test_json_processor() {
//     let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
//         println!(
//             "In PlaceJson item: {} => {}",
//             json_entry.field_name, json_entry.field_value
//         );
//     });
//     let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
//     let json_piece = r#"{"hello":"world"}"#;
//     let res = json_processor.push_json_piece(json_piece);
//     assert!(res.is_some() && res.unwrap().is_empty());
//     print!("~~~")
// }

// struct MyStruct {
//   field: Box<dyn MyTrait>,
// }
// impl MyStruct {
//   fn new(trait_impl: Box<dyn MyTrait>) -> MyStruct {
//       MyStruct { field: trait_impl }
//   }
//   fn test(&self) {
//       self.field.called();
//   }
// }
// trait MyTrait {
//   fn called(&self);
// }
// struct TestStruct;
// impl MyTrait for TestStruct {
//     fn called(&self) {
//         // Implementation of the called method for TestStruct
//         println!("Called method from TestStruct");
//     }
// }
// fn test_fn() {
//     // Instantiate TestStruct
//     let test_instance = TestStruct;
//     // Create an instance of MyStruct passing in the TestStruct instance
//     let my_struct_instance = MyStruct::new(Box::new(test_instance));
//     // Call the test method of MyStruct
//     my_struct_instance.test();
// }

/// A simple JSON processor / stream parser, that processes input JSON (possibly streamed piece by piece).
/// As soon as JSON entries are recognized, the configured callback is called for those recognized JSON entries
///
/// for example:
/// ```
/// use rusty_dumb_tools::json::{DumbJsonProcessor, InPlaceJsonEntryHandler, JsonEntry, JsonEntryHandler};
/// let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
///     println!(
///         "In-Place JSON entry: `{}` => `{}`",
///         json_entry.field_name, json_entry.field_value
///     );
///     assert!(json_entry.field_name == "greeting");
///     assert!(json_entry.field_value.to_string() == "Hi❗ How are youüúüUÜÙÛ❓  👩‍⚕👨‍👩‍👧‍👦🇭🇰👍🏽😆");
/// });
/// let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
/// let json = r#"{ "greeting" : "Hi❗ How are youüúüUÜÙÛ❓  👩‍⚕👨‍👩‍👧‍👦🇭🇰👍🏽😆" }"#;
/// let res = json_processor.push_json(json);
/// assert!(res.is_ok() && res.unwrap().is_empty());
/// print!("~~~")
/// ```
/// Note that [`InPlaceJsonEntryHandler`] is simply a helper that implements the [`JsonEntryHandler`] trait,
/// which acts as a callback to handle [`JsonEntry`] as soon as it comes:
/// * [`JsonEntryHandler::handle_json_entry`] is called when a JSON entry comes to be handled
/// * [`JsonEntry`] is passed as argument when [`JsonEntryHandler::handle_json_entry`] is called
/// * [`JsonEntry::field_name`] tells the "path" of the JSON entry; see the example below
/// * [`JsonEntry::field_value`] is the value ([`JsonFieldValue`]) of the JSON entry:
///   - [`JsonFieldValue::String`] for string value
///   - [`JsonFieldValue::Whole`] for integer value
///   - [`JsonFieldValue::Decimal`] for float value
///   - [`JsonFieldValue::Boolean`] for boolean value
///   - [`JsonFieldValue::Null`] for null value
///
/// For example, for the following JSON:
/// ```json
/// {
///   "simple_key": "simple_value,
///   "nested": {
///     "nested_key": "nested_value"
///   },
///   "array": [ "item0", "item1" ],
///   "obj_array": [
///     { "obj_key": "obj0" },
///     { "obj_key": "obj1" }
///   ]
/// }
/// ```
/// the field-name/field-value pairs are:
/// - "simple_key" => "simple_value"
/// - "nested.nested_key" => "nested_value"
/// - "array.0" => "item0"
/// - "array.1" => "item1"
/// - "obj_array.0.obj_key" => "obj0"
/// - "obj_array.1.obj_key" => "obj1"
pub struct DumbJsonProcessor<'a> {
    json_entry_handler: Box<&'a mut dyn JsonEntryHandler>,
    //for_array: bool,
    unescape_escaped: bool, // TODO: seems this must be true; hence, remove it later
    //nested_parser: Option<Box<DumbJsonProcessor>>,
    // state: &'static str,
    // buffer: String,
    // skipping: String,
    // finalized:bool,
    // field_name: Option<String>,
    // field_value: Option<String>,
    // count: i16,
    //first_stage: ProcessorStage,
    nc_remaining: String,
    nc_remaining_bytes: Vec<u8>,
}

impl<'a> DumbJsonProcessor<'a> {
    pub fn new(json_entry_handler: Box<&mut dyn JsonEntryHandler>) -> DumbJsonProcessor {
        DumbJsonProcessor {
            json_entry_handler,
            //for_array: false,
            unescape_escaped: true,
            //nested_parser: None,
            // state: "",
            // buffer: String::new(),
            // skipping: String::new(),
            // finalized: false,
            // field_name: None,
            // field_value: None,
            // count: 0,
            //first_stage: ProcessorStage::new(String::new(), false),
            nc_remaining: String::new(),
            nc_remaining_bytes: Vec::new(),
        }
    }
    /// push a JSON piece to the processor; note that the JSON piece can be a complete JSON, or part of a JSON;
    /// as soon as JSON entries are recognized, callback is called for those recognized JSON entries
    ///
    /// it requires an input [`ProcessJsonProgress`] to keep track of the progress
    /// - the initial [`ProcessJsonProgress`] can be created by [`ProcessJsonProgress::new`]
    /// - after each push, the progress will be updated;
    ///   and you can use [`ProcessJsonProgress::is_done`] to check if the input ended up a complete JSON, or needing additional JSON pieces
    /// - if it is done, you can use [`ProcessJsonProgress::get_remaining`] to get the remaining of the input outside of the JSON
    ///   e.g. an empty string if “}” is the last character of the last input JSON piece
    pub fn push_json_piece<'b>(
        &mut self,
        json_piece: &str,
        progress: &'b mut ProcessJsonProgress,
    ) -> Result<&'b mut ProcessJsonProgress, DumbError> {
        let in_json_piece = if progress.is_done() {
            progress._reset()
        } else {
            String::new()
        } + json_piece;
        let in_str = self.nc_remaining.clone() + in_json_piece.as_str();
        let in_chars: Vec<char> = in_str.chars().collect();
        let in_char_count = in_chars.len();
        let mut i_end = 0;
        for i in (0..in_char_count).rev() {
            let in_char = in_chars[i];
            if in_char <= '~' {
                i_end = i + 1;
                break;
            }
        }
        self.nc_remaining = in_chars[i_end..].iter().collect();
        let in_chars: Vec<char> = in_chars[0..i_end].to_vec();
        //let buffer = in_chars
        //    .iter()
        //    .map(|&x| x.to_string())
        //    .collect::<Vec<String>>();
        // let graphemes =
        //     UnicodeSegmentation::graphemes(in_str.as_str(), true).collect::<Vec<&str>>();
        // let consumed_len = graphemes.join("").len();
        // self.nc_remaining = in_json_piece[consumed_len..].to_string();
        // let buffer = graphemes
        //     .iter()
        //     .map(|&x| x.to_string())
        //     .collect::<Vec<String>>();
        let mut stage = progress.stages.last_mut().unwrap();
        let mut result = self._push_json_piece(in_chars, &mut stage)?;
        if result.is_some() {
            let remaining = self.nc_remaining.clone() + &result.unwrap();
            self.nc_remaining = String::new();
            result = Some(remaining)
        }
        progress.result = result;
        return Ok(progress);
    }
    /// like [`DumbJsonProcessor::push_json_piece`] but for a complete JSON
    ///
    /// It returns the remaining after processing the complete JSON; e.g. an empty string if "}" is the last character of the last input JSON
    pub fn push_json(&mut self, json: &str) -> Result<String, DumbError> {
        let mut progress = ProcessJsonProgress::new();
        self.push_json_piece(json, &mut progress)?;
        if progress.is_done() {
            return Ok(progress.get_remaining());
        } else {
            return Err(DumbError::from("JSON is not complete"));
        }
    }
    /// like [`DumbJsonProcessor::push_json_piece`] but accepts `[u8]` bytes
    pub fn push_json_bytes<'b>(
        &mut self,
        bytes: &[u8],
        progress: &'b mut ProcessJsonProgress,
    ) -> Result<&'b mut ProcessJsonProgress, DumbError> {
        let mut in_bytes: Vec<u8> = Vec::new();
        if !self.nc_remaining_bytes.is_empty() {
            let nc_remaining_bytes = self.nc_remaining_bytes.clone();
            self.nc_remaining_bytes = Vec::new();
            in_bytes.extend(nc_remaining_bytes);
        }
        in_bytes.extend(bytes.to_vec());
        let mut json_piece = String::new();
        let mut invalid_bytes: Vec<u8> = Vec::new();
        loop {
            let lossy = String::from_utf8_lossy(&in_bytes);
            json_piece = lossy.to_string();
            if !json_piece.ends_with("\u{FFFD}") {
                break;
            }
            let lb = in_bytes.pop(); // try with one less byte
            match lb {
                Some(lb) => invalid_bytes.push(lb),
                None => break,
            }
        }
        self.nc_remaining_bytes.extend(invalid_bytes);
        self.push_json_piece(json_piece.as_str(), progress)
    }
    fn _push_json_piece(
        &mut self,
        buffer: Vec<char>,
        stage: &mut ProcessorStage,
    ) -> Result<Option<String>, DumbError> {
        if DEBUG_ON {
            println!("INPUT json_piece: {}", buffer.iter().collect::<String>());
        }
        //let stage = stages.last_mut().unwrap();
        //let graphemes = UnicodeSegmentation::graphemes(json_piece, true).collect::<Vec<&str>>();
        //let consumed_len = graphemes.join("").len();
        //stage.remaining = json_piece[consumed_len..].to_string();
        if stage.buffer.is_empty() {
            stage.buffer = buffer;
        } else {
            stage.buffer.extend(buffer);
        }
        loop {
            let stream_parse_res = self._stream_parse(stage)?;
            if stream_parse_res.need_more_data {
                return Ok(None);
            }
            if stream_parse_res.done {
                stage.finalized = true;
                return Ok(Some(stage.buffer.iter().collect::<String>()));
            } else {
                if stage.buffer.is_empty() {
                    break;
                }
            }
        }
        return Ok(None);
        // let key = "key";
        // let value = "value";
        // let json_entry = JsonEntry { key, value };
        // self.json_entry_handler.handle_json_entry(&json_entry);
    }
    fn _scan_to(
        &mut self,
        stage: &mut ProcessorStage,
        what: char,
        allow_escape: bool,
    ) -> Option<i32> {
        let ori_buffer = stage.buffer.clone(); // TODO: make clone only when stage.buffer changed
        let buf_len = stage.buffer.len();
        //let buf_chars: Vec<char> = self.buffer.chars().collect();
        let mut escaping = false;
        let mut i = 0;
        let mut max_i = buf_len;
        while i < max_i {
            let c = stage.buffer[i];
            if escaping {
                escaping = false;
                if self.unescape_escaped {
                    let prefix = stage.buffer[0..i - 1].to_vec();
                    let suffix = stage.buffer[i..].to_vec();
                    stage.buffer = prefix;
                    stage.buffer.extend(suffix);
                    i -= 1;
                    max_i -= 1;
                }
            } else {
                if allow_escape && c == '\\' {
                    escaping = true
                } else if c == what {
                    return Some(i as i32);
                }
            }
            i += 1
        }
        if escaping {
            stage.buffer = ori_buffer;
            return None;
        } else {
            return Some(-1);
        }
    }
    // fn _scan_to(
    //     &mut self,
    //     stage: &mut ProcessorStage,
    //     what: char,
    //     allow_escape: bool,
    // ) -> Option<i32> {
    //     let ori_buffer = stage.buffer.clone();
    //     let buf_len = stage.buffer.len();
    //     //let buf_chars: Vec<char> = self.buffer.chars().collect();
    //     let mut escaping = false;
    //     let mut i = 0;
    //     let mut max_i = buf_len;
    //     while i < max_i {
    //         let c = stage.buffer.chars().nth(i).unwrap();
    //         if escaping {
    //             escaping = false;
    //             if self.unescape_escaped {
    //                 stage.buffer = stage.buffer[0..i - 1].to_string() + &stage.buffer[i..];
    //                 i -= 1;
    //                 max_i -= 1;
    //             }
    //         } else {
    //             if allow_escape && c == '\\' {
    //                 escaping = true
    //             } else if c == what {
    //                 return Some(i as i32);
    //             }
    //         }
    //         i += 1
    //     }
    //     if escaping {
    //         stage.buffer = ori_buffer;
    //         return None;
    //     } else {
    //         return Some(-1);
    //     }
    // }
    // fn __skip(
    //     &mut self,
    //     stage: &mut ProcessorStage,
    //     what: char,
    //     inclusive: bool,
    //     allow_escape: bool,
    // ) -> Option<String> {
    //     match self._scan_to(stage, what, allow_escape) {
    //         Some(i) => {
    //             if i == -1 {
    //                 stage.skipping.push_str(stage.buffer.as_str());
    //                 stage.buffer.clear();
    //                 return Some(String::new());
    //             } else {
    //                 let i = i as usize;
    //                 let skipped = if inclusive {
    //                     let skipped = stage.skipping.clone() + &stage.buffer[0..i + 1];
    //                     //skipped.push_str(&self.buffer[0..i+1]);
    //                     stage.buffer = stage.buffer[i + 1..].to_string();
    //                     skipped
    //                 } else {
    //                     let skipped = stage.skipping.clone() + &stage.buffer[0..i];
    //                     //skipped.push_str(&self.buffer[0..i]);
    //                     stage.buffer = stage.buffer[i..].to_string();
    //                     skipped
    //                 };
    //                 stage.skipping.clear();
    //                 return Some(skipped);
    //             }
    //         }
    //         None => return None,
    //     }
    // }
    fn __skip(
        &mut self,
        stage: &mut ProcessorStage,
        what: char,
        inclusive: bool,
        allow_escape: bool,
    ) -> Option<Vec<char>> {
        match self._scan_to(stage, what, allow_escape) {
            Some(i) => {
                if i == -1 {
                    stage.skipping.extend(stage.buffer.to_vec());
                    stage.buffer.clear();
                    return Some(Vec::new());
                } else {
                    let i = i as usize;
                    let skipped = if inclusive {
                        let mut skipped = stage.skipping.clone();
                        skipped.extend(stage.buffer[0..i + 1].to_vec());
                        //skipped.push_str(&self.buffer[0..i+1]);
                        stage.buffer = stage.buffer[i + 1..].to_vec();
                        skipped
                    } else {
                        let mut skipped = stage.skipping.clone();
                        skipped.extend(stage.buffer[0..i].to_vec());
                        //skipped.push_str(&self.buffer[0..i]);
                        stage.buffer = stage.buffer[i..].to_vec();
                        skipped
                    };
                    stage.skipping.clear();
                    return Some(skipped);
                }
            }
            None => return None,
        }
    }
    // fn _skip_to(
    //     &mut self,
    //     stage: &mut ProcessorStage,
    //     what: char,
    //     allow_escape: bool,
    // ) -> Option<String> {
    //     return self.__skip(stage, what, true, allow_escape);
    // }
    fn _skip_to(
        &mut self,
        stage: &mut ProcessorStage,
        what: char,
        allow_escape: bool,
    ) -> Option<Vec<char>> {
        return self.__skip(stage, what, true, allow_escape);
    }
    // fn _skip_ws(&mut self, stage: &mut ProcessorStage) -> bool {
    //     let buf_chars = stage.buffer.chars();
    //     for (i, c) in buf_chars.enumerate() {
    //         if !c.is_whitespace() {
    //             stage.buffer = stage.buffer[i..].to_string();
    //             return true;
    //         }
    //     }
    //     stage.buffer.clear();
    //     return false;
    // }
    fn _skip_ws(&mut self, stage: &mut ProcessorStage) -> bool {
        // let buf_chars = stage.buffer.chars();
        let buffer = &stage.buffer;
        let buffer_len = buffer.len();
        for i in 0..buffer_len {
            let c = &buffer[i];
            if !c.is_whitespace() {
                stage.buffer = stage.buffer[i..].to_vec();
                return true;
            }
        }
        stage.buffer.clear();
        return false;
    }
    fn _stream_parse(&mut self, stage: &mut ProcessorStage) -> Result<StreamParseRes, DumbError> {
        //let stage = stages.last_mut().unwrap();
        if stage.state.is_empty() {
            let skip_what = if stage.for_array { '[' } else { '{' };
            if self._skip_to(stage, skip_what, false).is_none() {
                return Ok(StreamParseRes::to_be_continued());
            }
            stage.state = "{"
        }
        if stage.state == "{" {
            if stage.for_array {
                stage.field_name = Some(stage.count.to_string());
                stage.state = ":"
            } else {
                let idx = self._scan_to(stage, '"', false);
                let close_idx = self._scan_to(stage, '}', false);
                if idx.is_none() || close_idx.is_none() {
                    return Ok(StreamParseRes::need_more_data());
                }
                let idx = idx.unwrap();
                let close_idx = close_idx.unwrap();
                if close_idx != -1 && (idx == -1 || close_idx < idx) {
                    stage.state = "$"
                } else {
                    let skipped_to = self._skip_to(stage, '"', false);
                    if skipped_to.is_none() {
                        return Ok(StreamParseRes::need_more_data());
                    }
                    let skipped_to = skipped_to.unwrap();
                    if skipped_to.is_empty() {
                        return Ok(StreamParseRes::to_be_continued());
                    }
                    stage.state = "{>"
                }
            }
        }
        if stage.state == "{>" {
            let skipped_to = self._skip_to(stage, '"', false);
            if skipped_to.is_none() {
                return Ok(StreamParseRes::need_more_data());
            }
            let skipped_to = skipped_to.unwrap();
            if skipped_to.is_empty() {
                return Ok(StreamParseRes::to_be_continued());
            }
            stage.field_name = Some(
                skipped_to[..skipped_to.len() - 1]
                    .iter()
                    .collect::<String>(),
            );
            stage.state = ">:"
        }
        if stage.state == ">:" {
            let skipped_to = self._skip_to(stage, ':', false);
            if skipped_to.is_none() {
                return Ok(StreamParseRes::need_more_data());
            }
            let skipped_to = skipped_to.unwrap();
            if skipped_to.is_empty() {
                return Ok(StreamParseRes::to_be_continued());
            }
            stage.state = ":"
        }
        if stage.state == ":" {
            if !self._skip_ws(stage) {
                return Ok(StreamParseRes::to_be_continued());
            }
            stage.state = "^"
        }
        if stage.state == "^" {
            let c = stage.buffer[0];
            if c == '{' || c == '[' || c == '"' {
                stage.buffer = stage.buffer[1..].to_vec();
                if c == '{' {
                    stage.state = "^>{"
                } else if c == '[' {
                    stage.state = "^>["
                } else {
                    assert!(c == '"');
                    stage.state = "^>\""
                }
            } else {
                stage.state = "^>"
            }
        }
        if stage.state == "^>{" || stage.state == "^>[" {
            let parsing_array = stage.state == "^>[";
            //let mut json_data = stage.buffer.clone();
            // if self.nested_parser.is_none() {
            //     //let nested_parser = DumbJsonProcessor::new(Box::new(NestedJsonEntryHandler::new(self)));
            //     //self.nested_parser = Some(Box::new(nested_parser));
            //     json_data = (if parsing_array { '[' } else { '{' }).to_string() + &json_data
            // }
            // let json_piece =
            //     (if parsing_array { '[' } else { '{' }).to_string() + stage.buffer.join("").as_str();
            let mut child_buffer = if stage.child_stage.is_none() {
                let new_stage = ProcessorStage::new(stage.get_field_name(), parsing_array);
                stage.child_stage = Some(Box::new(new_stage));
                vec![if parsing_array { '[' } else { '{' }]
            } else {
                Vec::new()
            };
            child_buffer.extend(stage.buffer.clone());
            stage.buffer.clear();
            let child_stage = stage.child_stage.as_mut().unwrap().as_mut();
            // let child_stage = match stage.child_stage {
            //     Some(ref mut child_stage) => {
            //         let cs = child_stage.as_mut();
            //         cs
            //     },
            //     None => {
            //         let mut new_stage = ProcessorStage::new(stage.get_field_name(), parsing_array);
            //         stage.child_stage = Some(Box::new(new_stage));
            //         stage.child_stage.as_mut().unwrap()
            //     }
            // };
            //stages.push(stage);
            //let mut nested_stage = ProcessorStage::new(stage.get_field_name(), parsing_array);
            let rest = self._push_json_piece(child_buffer, child_stage)?;
            // let rest = match self.nested_parser {
            //     Some(ref mut nested_parser) => {
            //         nested_parser._push_json_segment(stage, json_data.as_str())
            //     }
            //     None => {
            //         assert!(false);
            //         None
            //     }
            // };
            if rest.is_none() {
                return Ok(StreamParseRes::need_more_data());
            }
            let rest = rest.unwrap();
            //self.nested_parser = None;
            stage.child_stage = None;
            stage.buffer = rest.chars().collect();
            // let rest_graphemes =
            //     UnicodeSegmentation::graphemes(rest.as_str(), true).collect::<Vec<&str>>();
            // stage.buffer = rest_graphemes
            //     .iter()
            //     .map(|&x| x.to_string())
            //     .collect::<Vec<String>>();
            stage.state = "$";
            stage.count += 1
        }
        if stage.state == "^>\"" {
            // if true {
            //     //let mut startedoff_escaped = false;
            //     if !stage.skipping.is_empty() {
            //         let skipping_n = &stage.skipping[stage.skipping.len() - 1];
            //         if skipping_n.len() == 1 {
            //             let skipping_n = skipping_n.chars().next().unwrap();
            //             if skipping_n == '\\' {
            //                 //startedoff_escaped = true;
            //                 let ori_buffer = stage.buffer.clone();
            //                 stage.buffer = vec!["\\".to_string()];
            //                 stage.buffer.extend(ori_buffer);
            //                 stage.skipping.pop();
            //             }
            //         }
            //     }
            // }
            let skipped_to = self._skip_to(stage, '"', true);
            if skipped_to.is_none() {
                return Ok(StreamParseRes::need_more_data());
            }
            let skipped_to = skipped_to.unwrap();
            if skipped_to.is_empty() {
                return Ok(StreamParseRes::to_be_continued());
            }
            stage.field_value = Some(JsonFieldValue::new_str(
                skipped_to[..skipped_to.len() - 1]
                    .iter()
                    .collect::<String>(),
            ));
            self._submit(stage);
            stage.count += 1;
            stage.state = "$";
        }
        if stage.state == "^>" || stage.state == "$" {
            let close_token = if stage.for_array { ']' } else { '}' };
            let sep_idx = self._scan_to(stage, ',', false);
            let close_idx = self._scan_to(stage, close_token, false);
            if sep_idx.is_none() || close_idx.is_none() {
                return Ok(StreamParseRes::need_more_data());
            }
            let sep_idx = sep_idx.unwrap();
            let close_idx = close_idx.unwrap();
            let (skipped_to, done) = if sep_idx != -1 && (close_idx == -1 || sep_idx < close_idx) {
                let skipped_to = self._skip_to(stage, ',', false);
                (skipped_to, false)
            } else {
                let skipped_to = self._skip_to(stage, close_token, false);
                (skipped_to, true)
            };
            if skipped_to.is_none() {
                return Ok(StreamParseRes::need_more_data());
            }
            let skipped_to = skipped_to.unwrap();
            if skipped_to.is_empty() {
                return Ok(StreamParseRes::to_be_continued());
            }
            if stage.state == "^>" {
                let field_value = skipped_to[..skipped_to.len() - 1]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .to_string(); //skipped.substring(0, skipped.length - 1).trim()
                let field_value_is_empty = field_value.is_empty();
                stage.field_value = Some(JsonFieldValue::new_none_str(field_value)?);
                if !field_value_is_empty {
                    self._submit(stage);
                }
                stage.count += 1
            }
            stage.state = "{";
            return Ok(if done {
                StreamParseRes::done()
            } else {
                StreamParseRes::to_be_continued()
            });
        }
        return Ok(StreamParseRes::to_be_continued());
    }
    fn _submit(&mut self, stage: &mut ProcessorStage) {
        let field_name = stage.get_field_name(); //field_name.clone().unwrap();
        let field_value = stage.get_field_value(); //field_value.clone().unwrap();
        let json_entry = JsonEntry {
            field_name: field_name,
            field_value: field_value,
        };
        self.json_entry_handler.handle_json_entry(&json_entry);
    }
    // pub fn __handle_child_json_entry<'a>(&self, child_json_entry: &'a JsonEntry) {
    //     // let parent_field_name = self.field_name.clone().unwrap();
    //     // let field_name = parent_field_name + "." + &child_json_entry.field_name;
    //     // let field_value = child_json_entry.field_value.clone();
    //     // let json_entry = JsonEntry { field_name: field_name, field_value: field_value };
    //     // self.json_entry_handler.handle_json_entry(&json_entry);
    // }
}

#[derive(Debug, Clone)]
pub struct ProcessJsonProgress {
    stages: Vec<ProcessorStage>,
    result: Option<String>,
}
impl ProcessJsonProgress {
    pub fn new() -> ProcessJsonProgress {
        ProcessJsonProgress {
            stages: vec![ProcessorStage::new(String::new(), false)],
            result: None,
        }
    }
    pub fn is_done(&self) -> bool {
        return self.result.is_some();
    }
    pub fn get_remaining(&self) -> String {
        match &self.result {
            Some(v) => v.clone(),
            None => panic!("Not done yet"),
        }
    }
    fn _reset(&mut self) -> String {
        let result = self.result.clone();
        self.stages = vec![ProcessorStage::new(String::new(), false)];
        self.result = None;
        if result.is_some() {
            return result.unwrap();
        } else {
            return String::new();
        }
    }
}

#[derive(Debug, Clone)]
struct ProcessorStage {
    parent_field_name: String,
    for_array: bool,
    state: &'static str,
    buffer: Vec<char>,
    skipping: Vec<char>,
    finalized: bool,
    field_name: Option<String>,
    field_value: Option<JsonFieldValue>,
    count: i16,
    child_stage: Option<Box<ProcessorStage>>,
}
impl ProcessorStage {
    pub fn new(parent_field_name: String, for_array: bool) -> ProcessorStage {
        ProcessorStage {
            parent_field_name: parent_field_name,
            for_array: for_array,
            state: "",
            buffer: Vec::new(),
            skipping: Vec::new(),
            finalized: false,
            field_name: None,
            field_value: None,
            count: 0,
            child_stage: None,
        }
    }
    pub fn get_field_name(&self) -> String {
        let field_name = if self.parent_field_name.is_empty() {
            self.field_name.clone().unwrap()
        } else {
            self.parent_field_name.clone() + "." + &self.field_name.clone().unwrap()
        };
        return field_name;
    }
    pub fn get_field_value(&self) -> JsonFieldValue {
        return self.field_value.clone().unwrap();
    }
}

struct StreamParseRes {
    done: bool,
    need_more_data: bool,
}
impl StreamParseRes {
    fn done() -> StreamParseRes {
        StreamParseRes {
            done: true,
            need_more_data: false,
        }
    }
    fn need_more_data() -> StreamParseRes {
        StreamParseRes {
            done: false,
            need_more_data: true,
        }
    }
    fn to_be_continued() -> StreamParseRes {
        StreamParseRes {
            done: false,
            need_more_data: false,
        }
    }
}
// struct NestedJsonEntryHandler<'a> {
//     parent_processor: &'a DumbJsonProcessor,
// }

// impl<'a> NestedJsonEntryHandler<'a> {
//     fn new(parent_processor: &'a DumbJsonProcessor) -> NestedJsonEntryHandler<'a> {
//         NestedJsonEntryHandler {
//             parent_processor,
//          }
//     }
// }

// impl<'a> JsonEntryHandler for NestedJsonEntryHandler<'a> {
//     fn handle_json_entry(&self, json_entry: &JsonEntry) {
//         self.parent_processor.__handle_child_json_entry(json_entry);
//     }
// }

#[derive(Debug, Clone)]
pub enum JsonFieldValue {
    String(String),
    Whole(i32),
    Decimal(f64),
    Boolean(bool),
    Null,
}

impl JsonFieldValue {
    fn new_str(v: String) -> JsonFieldValue {
        JsonFieldValue::String(v)
    }
    fn new_none_str(v: String) -> Result<JsonFieldValue, DumbError> {
        if v == "null" {
            Ok(JsonFieldValue::Null)
        } else if v == "true" {
            Ok(JsonFieldValue::Boolean(true))
        } else if v == "false" {
            Ok(JsonFieldValue::Boolean(false))
        } else {
            if v.contains('.') {
                let n = v.parse::<f64>();
                if n.is_ok() {
                    Ok(JsonFieldValue::Decimal(n.unwrap()))
                } else {
                    Err(DumbError::from(format!("Invalid decimal number '{}'", v)))
                }
            } else {
                let n = v.parse::<i32>();
                if n.is_ok() {
                    Ok(JsonFieldValue::Whole(n.unwrap()))
                } else {
                    Err(DumbError::from(format!("Invalid whole number '{}'", v)))
                }
            }
        }
    }
    pub fn to_string(&self) -> String {
        match *self {
            JsonFieldValue::Null => "null".to_string(),
            JsonFieldValue::Boolean(v) => v.to_string(),
            JsonFieldValue::Whole(v) => v.to_string(),
            JsonFieldValue::Decimal(v) => v.to_string(),
            JsonFieldValue::String(ref v) => v.clone(),
        }
    }
}

impl fmt::Display for JsonFieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_string();
        write!(f, "{}", s)
    }
}

pub struct JsonEntry {
    pub field_name: String,
    pub field_value: JsonFieldValue,
}

pub trait JsonEntryHandler {
    fn handle_json_entry(&mut self, json_entry: &JsonEntry);
}

pub struct InPlaceJsonEntryHandler {
    f: Box<dyn Fn(&JsonEntry)>,
}
impl InPlaceJsonEntryHandler {
    pub fn new<F: 'static + Fn(&JsonEntry)>(f: F) -> InPlaceJsonEntryHandler {
        InPlaceJsonEntryHandler { f: Box::new(f) }
    }
}
impl JsonEntryHandler for InPlaceJsonEntryHandler {
    fn handle_json_entry(&mut self, json_entry: &JsonEntry) {
        let f = &self.f;
        f(json_entry);
    }
}
