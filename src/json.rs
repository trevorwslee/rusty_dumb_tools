const DEBUG_ON: bool = true;

#[test]
fn test_json_processor() {
    struct TestJsonEntryHandler {}
    impl JsonEntryHandler for TestJsonEntryHandler {
        fn handle_json_entry(&self, json_entry: &JsonEntry) {
            println!("Json item: {} => {}", json_entry.field_name, json_entry.field_value);
        }
    }
    let handler = TestJsonEntryHandler {};
    let mut json_processor = DumbJsonProcessor::new(Box::new(handler));
    let json_data = "{}";
    json_processor.push_json_segment(json_data);

    let handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "In PlaceJson item: {} => {}",
            json_entry.field_name, json_entry.field_value
        );
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(handler));
    json_processor.push_json_segment(json_data);
}

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

pub struct DumbJsonProcessor {
    json_entry_handler: Box<dyn JsonEntryHandler>,
    for_array: bool,
    unescape_escaped:  bool,
    nested_parser: Option<Box<DumbJsonProcessor>>,
    state: &'static str,
    buffer: String,
    skipping: String,
    finalized:bool,
    field_name: Option<String>,
    field_value: Option<String>,
    count: i16,
}

impl DumbJsonProcessor {
    pub fn new(json_entry_handler: Box<dyn JsonEntryHandler>) -> DumbJsonProcessor {
        DumbJsonProcessor {
            json_entry_handler, 
            for_array: false,
            unescape_escaped: true,
            nested_parser: None,
            state: "",
            buffer: String::new(),
            skipping: String::new(),
            finalized: false,
            field_name: None,
            field_value: None,
            count: 0,
        }
    }
    pub fn push_json_segment(&mut self, json_segment: &str) -> Option<String> {
        return self._push_json_segment(json_segment);
    }
    fn _push_json_segment(&mut self, json_segment: &str) -> Option<String> {
        if DEBUG_ON {
            println!("Json json_segment: {}", json_segment);
        }
        self.buffer.push_str(json_segment);
        loop {
            let stream_parse_res = self._stream_parse();
            if stream_parse_res.need_more_data {
                return None
            }
            if stream_parse_res.done {
                self.finalized = true;
                return Some(self.buffer.clone())
            } else {
                if self.buffer.is_empty() {
                    break
                }
            }
        }
        return None
        // let key = "key";
        // let value = "value";
        // let json_entry = JsonEntry { key, value };
        // self.json_entry_handler.handle_json_entry(&json_entry);
    }
    fn _scan_to(&mut self, what: char, allow_escape: bool) -> Option<i32> {
        let ori_buffer = self.buffer.clone();
        let buf_len = self.buffer.len();
        //let buf_chars: Vec<char> = self.buffer.chars().collect();
        let mut escaping = false;
        let mut i = 0;
        let mut max_i = buf_len;
        while i < max_i {
            let c = self.buffer.chars().nth(i).unwrap();  // TODO: enhance
            if escaping {
                escaping = false;
                if self.unescape_escaped {
                    self.buffer = self.buffer[0..i - 1].to_string() + &self.buffer[i..];
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
            self.buffer = ori_buffer;
            return None
        } else {
            return Some(-1)
        }

    }
    fn __skip(&mut self, what: char, inclusive: bool, allow_escape: bool) -> Option<String> {
        match self._scan_to(what, allow_escape) {
            Some(i) => {
                if i == -1 {
                    self.skipping.push_str(self.buffer.as_str());
                    self.buffer.clear();
                    return Some(String::new())
                } else {
                    let i = i as usize;
                    let skipped = 
                        if inclusive {
                            let skipped = self.skipping.clone() + &self.buffer[0..i+1];
                            //skipped.push_str(&self.buffer[0..i+1]);
                            self.buffer = self.buffer[0..i].to_string();
                            skipped
                        } else {
                            let skipped = self.skipping.clone() + &self.buffer[0..i];
                            //skipped.push_str(&self.buffer[0..i]);    
                            self.buffer = self.buffer[0..i].to_string();
                            skipped
                        };
                    self.skipping.clear();
                    return Some(skipped)
                }
                        }
            None => {
                return None
            }
        }
    }
    fn _skip_to(&mut self, what: char, allow_escape: bool) -> Option<String> {
        return self.__skip(what, true, allow_escape);
    }
    fn _skip_ws(&mut self) -> bool {
        let buf_chars = self.buffer.chars();
        for (i, c) in buf_chars.enumerate() {
            if !c.is_whitespace() {
                self.buffer = self.buffer[i..].to_string();
                return true
            }
        }
        self.buffer.clear();
        return false
    }
    fn _stream_parse(&mut self) -> StreamParseRes {
        if self.state.is_empty() {
            let skip_what = if self.for_array { '[' }  else { '{' };
            if self._skip_to(skip_what, false).is_none() {
                return StreamParseRes::to_be_continued();
            }
            self.state = "{"
        }
        if self.state == "{" {
            if self.for_array {
                self.field_name = Some(self.count.to_string());
                self.state = ":"
            } else {
                let idx = self._scan_to('"', false);
                let close_idx = self._scan_to('}', false);
                if idx.is_none() || close_idx.is_none() {
                    return StreamParseRes::need_more_data();
                }
                let idx = idx.unwrap();
                let close_idx = close_idx.unwrap();
                if close_idx != -1 && (idx == -1 || close_idx < idx) {
                    self.state = "$"
                } else {
                    let skipped_to = self._skip_to('"', false);
                    if skipped_to.is_none() {
                        return StreamParseRes::need_more_data();
                    }
                    let skipped_to: String = skipped_to.unwrap();
                    if skipped_to.is_empty() {
                        return StreamParseRes::to_be_continued();
                    }
                    self.state = "{>"
                }
            }
        }
        if self.state == "{>" {
            let skipped_to = self._skip_to('"', false);
            if skipped_to.is_none() {
                return StreamParseRes::need_more_data();
            }
            let skipped_to = skipped_to.unwrap();
            if skipped_to.is_empty() {
                return StreamParseRes::to_be_continued();
            }
            self.field_name = Some(skipped_to[0..skipped_to.len()-1].to_string());
            self.state = ">:"
        }
        if self.state == ">:" {
            let skipped_to = self._skip_to(':', false);
            if skipped_to.is_none() {
                return StreamParseRes::need_more_data();
            }
            let skipped_to = skipped_to.unwrap();
            if skipped_to.is_empty() {
                return StreamParseRes::to_be_continued();
            }
            self.state = ":"
        }
        if self.state == ":" {
            if !self._skip_ws() {
                return StreamParseRes::to_be_continued();
            }
            self.state = "^"
        }
        if self.state == "^" {
            let c = self.buffer.chars().nth(0).unwrap();
            if c == '{' || c == '[' || c == '"' {
                self.buffer = self.buffer[1..].to_string(); 
                if c == '{' {
                    self.state = "^>{"
                } else if c == '[' {
                    self.state = "^>["
                } else {
                    assert!(c == '"');
                    self.state = "^>\""

                }
            } else {
                self.state = "^>"
            }
        }
        if self.state == "^>{" || self.state == "^>[" {
            let parsing_array = self.state == "^>[";
            let mut json_data = self.buffer.clone();
            if self.nested_parser.is_none() {
                // let parent_field_name = self.field_name.clone().unwrap();
                // let parent_json_entry_handler = &self.json_entry_handler;
                // let handler = InPlaceJsonEntryHandler::new(move |json_entry| {
                //     let field_name = (parent_field_name + "." + json_entry.field_name).as_str();
                //     let field_value = json_entry.field_value;
                //     let json_entry = JsonEntry { field_name: field_name.clone(), field_value: field_value.clone() };
                //     parent_json_entry_handler.handle_json_entry(&json_entry)
                // });
                // let mut json_processor = DumbJsonProcessor::new(Box::new(handler));
                let nested_parser = DumbJsonProcessor::new(Box::new(NestedJsonEntryHandler::new(self)));
                self.nested_parser = Some(Box::new(nested_parser));
                json_data = (if parsing_array { '[' } else { '{' }).to_string() + &json_data
            }
            self.buffer.clear();
            let rest = match self.nested_parser {
                Some(ref mut nested_parser) => {
                    nested_parser._push_json_segment(json_data.as_str())
                }
                None => {
                    assert!(false);  
                    None
                }
            };
            if rest.is_none() {
                return StreamParseRes::need_more_data();
            }
            let rest = rest.unwrap();
            self.nested_parser = None;
            self.buffer = rest;
            self.state = "$";
            self.count += 1
        }
        if self.state == "^>\"" {
            let skipped_to = self._skip_to('"', true);
            if skipped_to.is_none() {
                return StreamParseRes::need_more_data();
                
            }
            let skipped_to = skipped_to.unwrap();
            if skipped_to.is_empty() {
                return StreamParseRes::to_be_continued();
            }
            self.field_value = Some(skipped_to[0..skipped_to.len()-1].to_string());
            self._submit();
            self.count += 1;
            self.state = "$";
        }
        if self.state == "^>" || self.state == "$" {
            let close_token = if self.for_array { ']' } else { '}' };
            let sep_idx = self._scan_to(',', false);
            let close_idx = self._scan_to(close_token, false);
            if sep_idx.is_none() || close_idx.is_none(){
                return StreamParseRes::need_more_data();
            }
            let sep_idx = sep_idx.unwrap();
            let close_idx = close_idx.unwrap();
            let (skipped_to, done) = if sep_idx != -1 && (close_idx == -1 || sep_idx < close_idx) {
                    let skipped_to = self._skip_to(',', false);
                    (skipped_to, false)
                } else {
                    let skipped_to = self._skip_to(close_token, false);
                    (skipped_to, true)
                };
            if skipped_to.is_none() {
                return StreamParseRes::need_more_data();
            }
            let skipped_to = skipped_to.unwrap();    
            if skipped_to.is_empty() {
                return StreamParseRes::to_be_continued();
            }
            if self.state == "^>" {
                let field_value = skipped_to[0..skipped_to.len()-1].to_string(); //skipped.substring(0, skipped.length - 1).trim()
                let field_value_is_empty = field_value.is_empty();
                self.field_value = Some(field_value);
                if !field_value_is_empty {
                    self._submit();
                }
                self.count += 1
            }
            self.state = "{";
            return if done { StreamParseRes::done() } else { StreamParseRes::to_be_continued() };
        }
        return StreamParseRes::to_be_continued();
    }
    fn _submit(& self) {
        let field_name = self.field_name.clone().unwrap();
        let field_value = self.field_value.clone().unwrap();
        let json_entry = JsonEntry { field_name: field_name, field_value: field_value };
        self.json_entry_handler.handle_json_entry(&json_entry);
    }
    pub fn __handle_child_json_entry<'a>(&self, child_json_entry: &'a JsonEntry) {
        let parent_field_name = self.field_name.clone().unwrap();
        let field_name = parent_field_name + "." + &child_json_entry.field_name;
        let field_value = child_json_entry.field_value.clone();
        let json_entry = JsonEntry { field_name: field_name, field_value: field_value };
        self.json_entry_handler.handle_json_entry(&json_entry);
    }
}

struct StreamParseRes {
    done: bool,
    need_more_data: bool,
}
impl StreamParseRes {
    fn done() -> StreamParseRes {
        StreamParseRes { done: true, need_more_data: false }
    }
    fn need_more_data() -> StreamParseRes {
        StreamParseRes { done: false, need_more_data: true }
    }
    fn to_be_continued() -> StreamParseRes {
        StreamParseRes { done: false, need_more_data: false }
    }
}
struct NestedJsonEntryHandler<'a> {
    parent_processor: &'a DumbJsonProcessor,
}

impl<'a> NestedJsonEntryHandler<'a> {
    fn new(parent_processor: &'a DumbJsonProcessor) -> NestedJsonEntryHandler<'a> {
        NestedJsonEntryHandler { 
            parent_processor,
         }
    }
}

impl<'a> JsonEntryHandler for NestedJsonEntryHandler<'a> {
    fn handle_json_entry(&self, json_entry: &JsonEntry) {
        self.parent_processor.__handle_child_json_entry(json_entry);
    }
}



pub struct JsonEntry {
    pub field_name: String,
    pub field_value: String,
}

pub trait JsonEntryHandler {
    fn handle_json_entry(&self, json_entry: &JsonEntry);
}

pub struct InPlaceJsonEntryHandler {
    f: Box<dyn Fn(&JsonEntry)>,
}
impl InPlaceJsonEntryHandler {
    fn new<F: 'static + Fn(&JsonEntry)>(f: F) -> InPlaceJsonEntryHandler {
        InPlaceJsonEntryHandler { f: Box::new(f) }
    }
}
impl JsonEntryHandler for InPlaceJsonEntryHandler {
    fn handle_json_entry(&self, json_entry: &JsonEntry) {
        let f = &self.f;
        f(json_entry);
    }
}
