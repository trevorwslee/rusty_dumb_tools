//#![deny(warnings)]
#![allow(unused)]

mod debug;
//mod demo;
// mod demo_arg;
// mod demo_calc;

use std::{collections::HashMap, env, vec};

use rusty_dumb_tools::{
    arg::{self, DumbArgBuilder, DumbArgParser},
    calc::{self, DumbCalcProcessor},
    demo, dlt_comps, dltc,
    ltemp::{DumbLineTempCompBuilder, DumbLineTemplate, LineTempComp, LineTempCompTrait},
    sap_arg,
};

// fn test_ltemp_align() {
//     let lt_comps = dlt_comps![
//         "|abc>",
//         dltc!("key1", max_width = 10, align=LineTempCompAlign::Left),
//         "|".to_string(),
//         dltc!("key2", max_width = 10, align=LineTempCompAlign::Left),
//         "|".to_string(),
//         dltc!("key3", max_width = 10, align=LineTempCompAlign::Right),
//         "<ghi|".to_string()
//     ];

//     let ltemp = DumbLineTemplate::new(34, 100, &lt_comps);
//     let mut map = HashMap::new();
//     map.insert(String::from("key1"), String::from("value1"));
//     map.insert(String::from("key2"), String::from("value2"));
//     map.insert(String::from("key3"), String::from("value3"));
//     let formatted = ltemp.format(&map).unwrap();
//     //assert!(formatted.len() >= 34 && formatted.len() <= 100);
//     assert_eq!(formatted, "");
// }

fn main() {
    // if true {
    //     demo::demo_ltemp::show_table("012345678901234567890");
    //     return;
    // }

    // let released = if env::var("CARGO_PKG_NAME").is_ok() {
    //     println!("Running with cargo run");
    //     false
    // } else {
    //     println!("Running as an installed binary");
    //     true
    // };

    // if true {
    //     demo::run_demo(Some(vec!["arg", "-h"]));
    //     return;
    // }

    let released: bool = true;
    if released {
        released_main();
    } else {
        debug_main();
    }
}

fn released_main() {
    // e.g. cargo run -- calc 1.1 + 2.2 * (4.3 - 2.4) + 5
    // e.g. cargo run -- arg -f 0.2 5 --string2 VAL1 false 1 2 3
    let debug = false;
    let in_args = if debug {
        let in_args = vec![
            "arg",
            "-f",
            "0.2",
            "5",
            "--string2",
            "VAL1",
            "false",
            "1",
            "2",
            "3",
        ];
        Some(in_args)
    } else {
        None
    };
    demo::run_demo(in_args);
}

fn debug_main() {
    //calc::debug_calc();
    debug::debug_calc_processor();

    //arg::debug_arg();

    debug::debug_dumb_arg_parser();
}
