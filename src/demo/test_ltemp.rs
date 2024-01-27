#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::{
    dlt_comps, dltc,
    ltemp::{
        DumbLineTempCompBuilder, DumbLineTemplate, LineTempComp, LineTempCompTrait, FLEXIBLE_WIDTH,
    },
};

#[test]
fn test_ltemp_fit() {
    let lt_comps = dlt_comps![
        "|abc>",
        dltc!("key1"),
        "_def_".to_string(),
        dltc!("key2", optional = true, min_width = 1, max_width = 100),
        "<ghi|".to_string()
    ];
    let ltemp = DumbLineTemplate::new(0, 100, &lt_comps);

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

#[test]
fn test_ltemp_over() {
    let lt_comps = dlt_comps![
        "|abc>",
        dltc!("key1", min_width = 3),
        "_def_".to_string(),
        dltc!("key2", optional = true, min_width = 3),
        "<ghi|".to_string()
    ];

    let ltemp = DumbLineTemplate::new(5, 22, &lt_comps);
    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 5 && formatted.len() <= 22);
    assert_eq!(formatted, "|abc>val_def_valu<ghi|");

    let ltemp = DumbLineTemplate::new(5, 18, &lt_comps);
    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 5 && formatted.len() <= 18);
    assert_eq!(formatted, "|abc>val_def_<ghi|");

    let ltemp = DumbLineTemplate::new(5, 10, &lt_comps);
    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    let formatted = ltemp.format(&map);
    assert!(formatted.is_err());
    assert_eq!(
        formatted.err().unwrap(),
        "too small a line ... still need 8, on top of max 10"
    );
}

#[test]
fn test_ltemp_under() {
    let lt_comps = dlt_comps![
        "|abc>",
        dltc!("key1", max_width = 10),
        "_def_".to_string(),
        dltc!("key2", optional = true, max_width = 10),
        "<ghi|".to_string()
    ];

    let ltemp = DumbLineTemplate::new(30, 100, &lt_comps);
    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 30 && formatted.len() <= 100);
    if FLEXIBLE_WIDTH {
        assert_eq!(formatted, "|abc>value1  _def_value2 <ghi|");
    } else {
        assert_eq!(formatted, "|abc>value1   _def_value2<ghi|");
    }

    let ltemp = DumbLineTemplate::new(25, 100, &lt_comps);
    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 25 && formatted.len() <= 100);
    assert_eq!(formatted, "|abc>value1    _def_<ghi|");

    let ltemp = DumbLineTemplate::new(50, 100, &lt_comps);
    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    let formatted = ltemp.format(&map);
    assert!(formatted.is_err());
    assert_eq!(
        formatted.err().unwrap(),
        "too big a line ... 25 extra, on top of min 50"
    );
}

#[test]
fn test_ltemp_align() {
    let lt_comps = dlt_comps![
        "|abc>",
        dltc!("key1", max_width = 10, align = 'L'),
        "|".to_string(),
        dltc!("key2", max_width = 10, align = 'C'),
        "|".to_string(),
        dltc!("key3", max_width = 10, align = 'R'),
        "<ghi|".to_string()
    ];

    let ltemp = DumbLineTemplate::new(37, 100, &lt_comps);
    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));
    map.insert(String::from("key3"), String::from("value3"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 37 && formatted.len() <= 100);
    if FLEXIBLE_WIDTH {
        assert_eq!(formatted, "|abc>value1   |  value2 | value3<ghi|");
    } else {
        assert_eq!(formatted, "|abc>value1    |  value2 |value3<ghi|");
    }
}
