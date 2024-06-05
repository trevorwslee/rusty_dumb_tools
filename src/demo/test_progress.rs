#![deny(warnings)]
#![allow(unused)]

use crate::prelude::*;

#[test]
fn test_progress_iter() {
    DumbProgressSetting::set_style(DumbProgressStyle::Simple);
    _test_progress_iter(false, false);
    _test_progress_iter(true, false);
    _test_progress_iter(false, true);
    _test_progress_iter(true, true);
}

fn _test_progress_iter(try_with_total: bool, nested: bool) {
    let v = vec![0, 1, 2, 3, 4];
    let iter = if try_with_total {
        dpiter!(v, name = "VEC", desc = "vector")
    } else {
        dpiw!(v.iter(), name = "VEC", desc = "vector")
    };
    let mut result: Vec<i32> = Vec::new();
    for i in iter {
        result.push(*i);
        if nested {
            for j in dprange!(100..102) {
                result.push(j + *i);
            }
        }
    }
    if nested {
        assert_eq!(
            result,
            vec![0, 100, 101, 1, 101, 102, 2, 102, 103, 3, 103, 104, 4, 104, 105]
        );
    } else {
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }
}

#[test]
fn test_progress_into() {
    DumbProgressSetting::set_style(DumbProgressStyle::Simple);
    _test_progress_into(false, false);
    _test_progress_into(false, true);
    _test_progress_into(true, true);
}

fn _test_progress_into(range: bool, try_with_total: bool) {
    let iter = if range {
        dprange!(0..10, name = "RANGE", desc = "range")
    } else {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        if try_with_total {
            dpintoiter!(v, name = "VEC", desc = "vector")
        } else {
            dpiw!(v.into_iter(), name = "VEC", desc = "vector")
        }
    };
    let mut result: Vec<i32> = Vec::new();
    for i in iter {
        result.push(i);
    }
    assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
