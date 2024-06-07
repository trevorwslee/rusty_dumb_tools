use std::{thread, time::Duration};

use crate::prelude::*;

pub fn create_demo_progress_parser() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("This is a simple iteration progress demo.");
    dap_arg!("-b", flag2 = "--break", fixed = true)
        .set_description("break out in the middle")
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("-n", flag2 = "--nested", fixed = true)
        .set_description("nesterd iteration")
        .add_to(&mut parser)
        .unwrap();
    parser
}

pub fn handle_demo_progress(parser: DumbArgParser) {
    let break_out: bool = parser.get_or_default("-b", false);
    let nested: bool = parser.get_or_default("-n", false);
    if break_out {
        try_simple_progress_range_open_ended();
    } else {
        if nested {
            try_nested_progress();
        } else {
            try_simple_progress_range();
        }
    }
}

pub fn try_progress(sleep_millis: u64, level: usize, try_with_total: bool) {
    DumbProgressSetting::set_style(DumbProgressStyle::Default);
    let items = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let desc = format!("level {}", level);
    let name = format!("L{}", level);
    let mut iter = if try_with_total {
        dpi_into_iter!(items, name = name, desc = desc)
    } else {
        dpiw!(items.into_iter(), name = name, desc = desc)
    };
    while let Some(_item) = iter.next() {
        // if show_items {
        //     println!("          * iter(): {}", item);
        // }
        if sleep_millis > 0 {
            thread::sleep(Duration::from_millis(sleep_millis));
        }
        if level > 0 {
            try_progress(sleep_millis, level - 1, try_with_total);
        }
    }
    // if true {
    //     for item in items.iter() {
    //         println!("- iter(): {}", item);
    //     }
    // }
}
pub fn try_progress_single(show_items: bool, sleep_millis: u64, try_with_total: bool) {
    DumbProgressSetting::set_style(DumbProgressStyle::Default);
    if true {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];
        {
            let mut iter = if try_with_total {
                dpi_iter!(items, name = "ITER")
            } else {
                dpiw!(items.iter(), name = "ITER")
            };
            while let Some(item) = iter.next() {
                if show_items {
                    println!("          * iter(): {}", item);
                }
                if sleep_millis > 0 {
                    thread::sleep(Duration::from_millis(sleep_millis));
                }
            }
        }
        if true {
            for item in items.iter() {
                println!("- iter(): {}", item);
            }
        }
    }
}
pub fn try_progress_range(
    show_items: bool,
    sleep_millis: u64,
    _level: usize,
    _try_with_total: bool,
) {
    DumbProgressSetting::set_style(DumbProgressStyle::Simple);
    let iter = dpir!(0..3, name = "Range");
    for i in iter {
        if show_items {
            println!("          * i: {}", i);
        }
        if sleep_millis > 0 {
            thread::sleep(Duration::from_millis(sleep_millis));
        }
    }
}

pub fn try_simple_progress_range_open_ended() {
    for i in dpiw!(0.., name = "RANGE", desc = "demo iteration of range") {
        println!(" i is {}", i);
        thread::sleep(Duration::from_millis(1000));
        if i > 6 {
            break;
        }
    }
}
pub fn try_simple_progress_range() {
    for i in dpir!(0..6, name = "RANGE", desc = "demo iteration of range") {
        println!(" i is {}", i);
        thread::sleep(Duration::from_millis(1000));
    }
}
pub fn try_nested_progress() {
    //DumbProgressSetting::set_max_nested_progress_bar_count(1);
    for i in dpir!(0..3, name = "RANGE") {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("orange"),
        ];
        for item in dpi_iter!(items, name = "VECTOR") {
            println!(" i is {}; item is {}", i, item);
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
