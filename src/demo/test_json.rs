use crate::json::{DumbJsonProcessor, InPlaceJsonEntryHandler};

#[test]
pub fn test_json_standard() {
    let handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "In PlaceJson item: {} => {}",
            json_entry.field_name, json_entry.field_value
        );
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(handler));
    let json_segment = r#"{"hello":"world"}"#;
    let res = json_processor.push_json_segment(json_segment);
    assert!(res.is_some() && res.unwrap().is_empty());
    print!("~~~")
}
