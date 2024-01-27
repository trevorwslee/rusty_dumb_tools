#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::{dlt_comps, dltc, ltemp::{DumbLineTemplate, DumbLineTempCompBuilder, LineTempComp, LineTempCompTrait}};

#[test]
fn test_ltemp_fit() {
    let lt_comps = dlt_comps![
        "|abc>",
        dltc!("key1"),
        "_def_".to_string(),
        dltc!("key2", optional = true, min_width = 1, max_width = 100),
        "<ghi|".to_string()
    ];
    let ltemp = DumbLineTemplate::new(0, 100, lt_comps);

    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    assert_eq!(formatted, "|abc>value1_def_value2<ghi|");

    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    let formatted = ltemp.format(&map).unwrap();
    assert_eq!(formatted, "|abc>value1_def_<ghi|");

  }
