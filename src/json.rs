const DEBUG_ON: bool = true;

#[test]
fn test_json_processor() {
    struct TestJsonEntryHandler {}
    impl JsonEntryHandler for TestJsonEntryHandler {
        fn handle_json_entry(&self, json_entry: &JsonEntry) {
            println!("Json item: {} => {}", json_entry.key, json_entry.value);
        }
    }
    let handler = TestJsonEntryHandler {};
    let mut json_processor = DumbJsonProcessor::new(Box::new(handler));
    let json_data = "{}";
    json_processor.sink_json_data(json_data);

    let handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "In PlaceJson item: {} => {}",
            json_entry.key, json_entry.value
        );
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(handler));
    json_processor.sink_json_data(json_data);
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
}

impl DumbJsonProcessor {
    pub fn new(json_entry_handler: Box<dyn JsonEntryHandler>) -> DumbJsonProcessor {
        DumbJsonProcessor { json_entry_handler }
    }
    // pub fn new_test<F: 'static + Fn(&str, &str)>(f: F) -> DumbJsonProcessor {
    //   DumbJsonProcessor {
    //     json_entry_handler: Box::new(f)
    //   }
    // }
    pub fn sink_json_data(&mut self, json_data: &str) {
        if DEBUG_ON {
            println!("Json data: {}", json_data);
        }
        let key = "key";
        let value = "value";
        let json_entry = JsonEntry { key, value };
        self.json_entry_handler.handle_json_entry(&json_entry);
    }
}

pub struct JsonEntry<'a> {
    pub key: &'a str,
    pub value: &'a str,
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
