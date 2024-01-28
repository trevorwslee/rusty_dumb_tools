#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::{
    dlt_comps, dltc,
    ltemp::{
        DumbLineTempCompBuilder, DumbLineTemplate, EscapedLineTempComp, LineTempComp,
        LineTempCompTrait, FLEXIBLE_WIDTH_EX,
    },
};

#[test]
fn test_ltemp_fit() {
    let lt_comps = dlt_comps![
        "|abc>",
        dltc!("key1"),
        "_def_".to_string(),
        dltc!("key2", min_width = 1, max_width = 100, optional = true),
        "<ghi|".to_string()
    ];
    let ltemp = DumbLineTemplate::new(0, 100, &lt_comps);

    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
    map.insert("key2", String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    assert_eq!(formatted, "|abc>value1_def_value2<ghi|");

    let map = HashMap::from([("key1", "value1")]);
    //let mut map = HashMap::new();
    //map.insert("key1", String::from("value1"));
    let formatted = ltemp.format(&HashMap::from([("key1", "value1")])).unwrap();
    assert_eq!(formatted, "|abc>value1_def_<ghi|");
}

#[test]
fn test_ltemp_over() {
    let lt_comps = dlt_comps![
        "|abc>",
        dltc!("key1", min_width = 3),
        "_def_".to_string(),
        dltc!("key2", min_width = 3, optional = true),
        "<ghi|".to_string()
    ];

    let ltemp = DumbLineTemplate::new(5, 22, &lt_comps);
    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
    map.insert("key2", String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 5 && formatted.len() <= 22);
    assert_eq!(formatted, "|abc>val_def_valu<ghi|");

    let ltemp = DumbLineTemplate::new(5, 18, &lt_comps);
    // let mut map = HashMap::new();
    // map.insert("key1", String::from("value1"));
    let formatted = ltemp
        .format(&HashMap::from([("key1", String::from("value1"))]))
        .unwrap();
    assert!(
        formatted.len() >= ltemp.min_width() as usize
            && formatted.len() <= ltemp.max_width() as usize
    );
    assert_eq!(formatted, "|abc>val_def_<ghi|");

    let ltemp = DumbLineTemplate::new(5, 10, &lt_comps);
    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
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
        dltc!("key2", max_width = 10, optional = true),
        "<ghi|".to_string()
    ];

    let ltemp = DumbLineTemplate::new(30, 100, &lt_comps);
    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
    map.insert("key2", String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 30 && formatted.len() <= 100);
    if FLEXIBLE_WIDTH_EX {
        assert_eq!(formatted, "|abc>value1  _def_value2 <ghi|");
    // } else if FLEXIBLE_WIDTH {
    //     assert_eq!(formatted, "|abc>value1  _def_value2 <ghi|");
    } else {
        assert_eq!(formatted, "|abc>value1   _def_value2<ghi|");
    }

    let ltemp = DumbLineTemplate::new(25, 100, &lt_comps);
    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 25 && formatted.len() <= 100);
    assert_eq!(formatted, "|abc>value1    _def_<ghi|");

    let ltemp = DumbLineTemplate::new(50, 100, &lt_comps);
    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
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
    let map = HashMap::from([("key1", "value1"), ("key2", "value2"), ("key3", "value3")]);
    let formatted = ltemp.format(&map).unwrap();
    assert!(formatted.len() >= 37 && formatted.len() <= 100);
    if FLEXIBLE_WIDTH_EX {
        //assert_eq!(formatted, "|abc>value1   | value2 |  value3<ghi|");
        assert_eq!(formatted, "|abc>value1   |  value2 | value3<ghi|");
    // } else if FLEXIBLE_WIDTH {
    //     assert_eq!(formatted, "|abc>value1   |  value2 | value3<ghi|");
    } else {
        assert_eq!(formatted, "|abc>value1    |  value2 |value3<ghi|");
    }
}

#[test]
fn test_ltemp_escaped() {
    let lt_comps = dlt_comps![
        ("\x1B[7mABC\x1B[0m", 1),
        dltc!("escaped", fixed_width = 6, align = 'C', optional = true)
    ];
    let ltemp = DumbLineTemplate::new(0, 100, &lt_comps);
    let map = HashMap::<&str, String>::new();
    let formatted = ltemp.format(&map).unwrap();
    assert_eq!(formatted, "\u{1b}[7mABC\u{1b}[0m");
    assert_eq!(formatted.len(), 11);
    //println!("*** {}({}) ***", formatted, formatted.len());
    let map_value_provide_fn = |key: &str| -> Option<(&str, u16)> {
        if key == "escaped" {
            Some(("\x1B[7mDEF\x1B[0m", 3))
        } else {
            None
        }
    };
    let formatted = ltemp.format_ex(map_value_provide_fn).unwrap();
    assert_eq!(formatted, "\u{1b}[7mABC\u{1b}[0m  \u{1b}[7mDEF\u{1b}[0m ");
    //println!("*** {}({}) ***", formatted, formatted.len());
}
