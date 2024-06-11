#![deny(warnings)]
#![allow(unused)]

use crate::prelude::*;

#[test]
fn test_progress() {
    _test_progress_iter();
    _test_progress_into_iter();
    _test_progress_iter_objs();
    _test_progress_into_iter_objs();
    _test_progress_mut_objs();
    _test_progress_rev_objs();
    _test_progress_enum_objs();
    _test_progress_zip();
    _test_progress_cycle();
}

fn _test_progress_iter() {
    DumbProgressSetting::set_style(DumbProgressStyle::Simple);
    __test_progress_iter(false, false);
    __test_progress_iter(true, false);
    __test_progress_iter(false, true);
    __test_progress_iter(true, true);
}

fn __test_progress_iter(try_with_total: bool, nested: bool) {
    let v = vec![0, 1, 2, 3, 4];
    let iter = if try_with_total {
        dpi_iter!(v, name = "VEC", desc = "vector")
    } else {
        dpiw!(v.iter(), name = "VEC", desc = "vector")
    };
    let mut result: Vec<i32> = Vec::new();
    for i in iter {
        result.push(*i);
        if nested {
            for j in dpir!(100..102) {
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

fn _test_progress_into_iter() {
    DumbProgressSetting::set_style(DumbProgressStyle::Simple);
    __test_progress_into_iter(false, false);
    __test_progress_into_iter(false, true);
    __test_progress_into_iter(true, true);
}

fn __test_progress_into_iter(range: bool, try_with_total: bool) {
    let iter = if range {
        dpir!(0..10, name = "RANGE", desc = "range")
    } else {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        if try_with_total {
            dpi_into_iter!(v, name = "VEC", desc = "vector")
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

fn _test_progress_iter_objs() {
    struct Obj {
        v: i32,
    }
    impl Obj {
        fn new(v: i32) -> Self {
            Self { v }
        }
    }
    let items: Vec<Obj> = vec![Obj::new(0), Obj::new(1), Obj::new(2)];
    let iter = dpi_iter!(items, name = "VEC", desc = "vector");
    let mut result: Vec<i32> = Vec::new();
    for v in iter {
        result.push(v.v);
    }
    assert_eq!(result, vec![0, 1, 2]);
}

fn _test_progress_into_iter_objs() {
    struct Obj {
        v: i32,
    }
    impl Obj {
        fn new(v: i32) -> Self {
            Self { v }
        }
    }
    let items: Vec<Obj> = vec![Obj::new(0), Obj::new(1), Obj::new(2)];
    let iter = dpi_into_iter!(items, name = "VEC", desc = "vector");
    let mut result: Vec<Obj> = Vec::new();
    for v in iter {
        result.push(v);
        //print!("{}", v.v);
    }
    let result = result.iter().map(|v| v.v).collect::<Vec<i32>>();
    assert_eq!(result, vec![0, 1, 2]);
}

fn _test_progress_mut_objs() {
    struct Obj {
        v: i32,
    }
    impl Obj {
        fn new(v: i32) -> Self {
            Self { v }
        }
    }
    let mut items: Vec<Obj> = vec![Obj::new(0), Obj::new(1), Obj::new(2)];
    let iter = dpiw!(items.iter_mut(), name = "VEC", desc = "vector");
    let mut result: Vec<i32> = Vec::new();
    for v in iter {
        result.push(v.v);
        v.v += 10;
    }
    assert_eq!(result, vec![0, 1, 2]);
    let result = items.iter().map(|v| v.v).collect::<Vec<i32>>();
    assert_eq!(result, vec![10, 11, 12]);
}

fn _test_progress_rev_objs() {
    struct Obj {
        v: i32,
    }
    impl Obj {
        fn new(v: i32) -> Self {
            Self { v }
        }
    }
    let items: Vec<Obj> = vec![Obj::new(0), Obj::new(1), Obj::new(2)];
    let iter = dpiw!(items.iter().rev(), name = "VEC", desc = "vector");
    let mut result: Vec<i32> = Vec::new();
    for v in iter {
        result.push(v.v);
    }
    assert_eq!(result, vec![2, 1, 0]);
}

fn _test_progress_enum_objs() {
    struct Obj {
        v: i32,
    }
    impl Obj {
        fn new(v: i32) -> Self {
            Self { v }
        }
    }
    let items: Vec<Obj> = vec![Obj::new(0), Obj::new(1), Obj::new(2)];
    let iter = dpiw!(items.iter().enumerate(), name = "VEC", desc = "vector");
    let mut result: Vec<i32> = Vec::new();
    for (idx, v) in iter {
        result.push(10 * idx as i32 + v.v);
    }
    assert_eq!(result, vec![0, 11, 22]);
}

fn _test_progress_zip() {
    let v1 = vec![0, 1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8, 9];
    let iter = dpiw!(v1.iter().zip(v2.iter()), name = "VEC", desc = "vector");
    let mut result: Vec<i32> = Vec::new();
    for (v1, v2) in iter {
        result.push(10 * v1 + v2);
    }
    assert_eq!(result, vec![5, 16, 27, 38, 49]);
}

fn _test_progress_cycle() {
    let v = vec![0, 1, 2];
    let iter = dpiw!(v.iter().cycle(), name = "VEC", desc = "vector");
    let mut result: Vec<i32> = Vec::new();
    for v in iter {
        result.push(*v);
        if result.len() == 6 {
            break;
        }
    }
    assert_eq!(result, vec![0, 1, 2, 0, 1, 2]);
}
