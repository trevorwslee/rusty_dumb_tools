use std::{thread, time::Duration};

use crate::prelude::*;

pub fn try_progress(sleep_millis: u64, level: usize, try_with_total: bool) {
    let items = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let desc = format!("level {}", level);
    let name = format!("L{}", level);
    let mut iter = if try_with_total {
        dpintoiter!(items, name = name, desc = desc)
    } else {
        dpintoiter_nt!(items, name = name, desc = desc)
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
    if true {
        let items = vec![
            String::from("apple"),
            String::from("banana"),
            String::from("cherry"),
        ];
        {
            let mut iter = if try_with_total {
                dpiter!(items, name = "ITER")
            } else {
                dpiter_nt!(items, name = "ITER")
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
    let iter = dprange!(0..3, name = "Range");
    for i in iter {
        if show_items {
            println!("          * i: {}", i);
        }
        if sleep_millis > 0 {
            thread::sleep(Duration::from_millis(sleep_millis));
        }
    }
}
