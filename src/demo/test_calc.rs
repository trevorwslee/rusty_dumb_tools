#![deny(warnings)]
#![allow(unused)]

use crate::prelude::*;

macro_rules! test_calc_push {
    ($units:expr, $res:expr) => {
        let units = $units;
        let res = $res;
        let mut calc = DumbCalcProcessor::new();
        for unit in units {
            calc.push(unit);
        }
        calc.eval().unwrap();
        assert_calc_eq_result!(calc, res);
    };
}

macro_rules! test_calc_prase_and_push {
    ($units:expr, $res:expr) => {
        let units = $units;
        let res = $res;
        let mut calc = DumbCalcProcessor::new();
        calc.parse_and_push(units).unwrap();
        calc.eval().unwrap();
        assert_calc_eq_result!(calc, res);
    };
}

macro_rules! test_calc_parse_and_push_error {
    ($units:expr) => {
        let units = $units;
        let mut calc = DumbCalcProcessor::new();
        calc.parse_and_push(units).unwrap();
        calc.eval();
        assert_calc_result_error!(&calc);
    };
}

macro_rules! assert_calc_eq_result {
    ($calc:expr, $res:expr) => {
        let calc = $calc;
        let res = $res;
        let calc_res = calc.get_result();
        let check_calc_res = match calc_res {
            CalcResult::Final(calc_res) => calc_res,
            _ => res + 1.0,
        };
        let res_diff = (check_calc_res - res).abs();
        if res_diff > 0.000000001 {
            println!("XXX");
            println!("XXX calc_res(={calc_res:?}) != res(={res}) ... calc={calc:?}");
            println!("XXX");
            assert_eq!(check_calc_res, res);
        }
    };
}

macro_rules! assert_calc_result_error {
    ($calc:expr) => {
        let calc = $calc;
        let calc_res = calc.get_result();
        if calc_res.is_ok() {
            println!("XXX");
            println!("XXX calc_res(={calc_res:?}) is not error ... calc={calc:?}");
            println!("XXX");
        }
        assert!(calc_res.is_err());
    };
}

#[test]
pub fn test_calc_push() {
    let mut calc = DumbCalcProcessor::new();
    assert_calc_eq_result!(&calc, 0.0);
    calc.push("123");
    calc.eval().unwrap();
    assert_eq!(123.0, calc.get_result().unwrap());
    calc.eval().unwrap();
    assert_eq!(123.0, calc.get_result().unwrap());
    calc.reset();
    println!(". calc={:?}", calc);
    assert_eq!(0.0, calc.get_result().unwrap());
    calc.push("777");
    calc.eval().unwrap();
    assert_eq!(777.0, calc.get_result().unwrap());
    calc.push("neg");
    calc.eval().unwrap();
    assert_eq!(-777.0, calc.get_result().unwrap());
    calc.reset();
    calc.push("15");
    calc.push("%");
    assert_eq!(0.15, calc.get_result().unwrap());
}
#[test]
pub fn test_calc_general() {
    let mut calc = DumbCalcProcessor::new();
    calc.parse_and_push("1.5");
    calc.eval().unwrap();
    assert_eq!(1.5, calc.get_result().unwrap());
    calc.parse_and_push("+ 2.5 * 3 - 4");
    calc.eval().unwrap();
    assert_eq!(5.0, calc.get_result().unwrap());
    calc.parse_and_push("12 neg");
    calc.eval().unwrap();
    assert_eq!(-12.0, calc.get_result().unwrap());
    calc.parse_and_push("15%");
    calc.eval().unwrap();
    assert_eq!(0.15, calc.get_result().unwrap());
}
#[test]
pub fn test_calc_result() {
    let mut calc = DumbCalcProcessor::new();
    calc.push("1");
    assert_eq!(1.0, calc.get_result().unwrap());
    calc.push("+");
    assert_eq!(1.0, calc.get_result().unwrap());
    calc.push("2");
    assert_eq!(2.0, calc.get_result().unwrap());
    calc.push("*");
    assert_eq!(2.0, calc.get_result().unwrap());
    calc.push("3");
    assert_eq!(3.0, calc.get_result().unwrap());
    calc.push("-");
    assert_eq!(7.0, calc.get_result().unwrap());
    calc.push("1");
    assert_eq!(1.0, calc.get_result().unwrap());
    calc.push("+");
    assert_eq!(6.0, calc.get_result().unwrap());
    calc.push("2");
    assert_eq!(2.0, calc.get_result().unwrap());
    calc.push("square");
    assert_eq!(4.0, calc.get_result().unwrap());
    calc.eval();
    assert_eq!(10.0, calc.get_result().unwrap());
}
#[test]
pub fn test_calc_result_2() {
    let mut calc = DumbCalcProcessor::new();
    calc.push("(");
    assert_eq!(0.0, calc.get_result().unwrap());
    calc.push("1");
    assert_eq!(1.0, calc.get_result().unwrap());
    calc.push(")");
    assert_eq!(1.0, calc.get_result().unwrap());

    let mut calc = DumbCalcProcessor::new();
    calc.push("(");
    calc.push("(");
    assert_eq!(0.0, calc.get_result().unwrap());
    calc.push("1");
    assert_eq!(1.0, calc.get_result().unwrap());
    calc.push(")");
    calc.push(")");
    assert_eq!(1.0, calc.get_result().unwrap());
}
#[test]
pub fn test_calc_parse() {
    test_calc_prase_and_push!(" 2 + 2 * ( 1 + 1 ) - ( 2 + 2 ) / (1 + 1) ", 4.0);
    test_calc_prase_and_push!("2+3*(4+5-6)-(2+3)/(1+1)", 8.5);
    test_calc_prase_and_push!(" 2 + 3 * (4 + 5 - 6)", 11.0);
    test_calc_prase_and_push!(" (2 + 4) ", 6.0);
    test_calc_prase_and_push!(" 1 + 2 * 3 - 4 / 2", 5.0);
    test_calc_prase_and_push!(" 1 + 2 * 3 - 4", 3.0);
    test_calc_prase_and_push!(" 123.0 + 100 + 0.1 - 23", 200.1);
    test_calc_prase_and_push!(" 123.0 + 100", 223.0);
    test_calc_prase_and_push!(" 123.0 ", 123.0);
    test_calc_prase_and_push!(" -2 ", -2.0);
    test_calc_prase_and_push!(" +2 ", 2.0);
    test_calc_prase_and_push!(" *2 ", 0.0);
    test_calc_prase_and_push!(" /2 ", 0.0);
}
#[test]
pub fn test_calc_non_standard() {
    test_calc_prase_and_push!(" ( ) ", 0.0);
    test_calc_prase_and_push!(" ( ", 0.0);
    test_calc_prase_and_push!(" ) ", 0.0);
    test_calc_prase_and_push!(" 2 + ( ) ", 2.0);
    test_calc_prase_and_push!(" 2 + (  ", 2.0);
    test_calc_prase_and_push!(" 2 + ) ", 2.0);
    test_calc_prase_and_push!(" 10 + 2 + ) ", 12.0);
    test_calc_prase_and_push!(" 2 * () ", 0.0);
    test_calc_prase_and_push!(" 10 + 2 * () ", 10.0);
    test_calc_prase_and_push!(" 2 () ", 0.0);
    test_calc_prase_and_push!(" 10 + 2 () ", 10.0);
    test_calc_prase_and_push!(" 2 + ( * 5) ", 2.0);
    test_calc_prase_and_push!(" 3 * + 4 ", 7.0);
}
#[test]
pub fn test_calc_error() {
    test_calc_parse_and_push_error!(" 0 / 0 ");
    test_calc_parse_and_push_error!(" / () ");
}
#[test]
pub fn test_calc_unary() {
    if true {
        test_calc_push!(vec!["1"], 1.0);
        test_calc_push!(vec!["1", "+", "2.5"], 3.5);
    }
    if true {
        test_calc_push!(vec!["1.5", "neg"], -1.5);
        test_calc_push!(vec!["1.5", "neg", "+", "1.5"], 0.0);
        test_calc_push!(vec!["1.5", "neg", "neg"], 1.5);
        test_calc_push!(vec!["1.5", "neg", "neg", "neg"], -1.5);
        test_calc_push!(vec!["1.5", "neg", "neg", "-", "1.5"], 0.0);
        test_calc_push!(vec!["1.5", "neg", "-", "1.5", "neg"], 0.0);
        test_calc_push!(vec!["50", "%"], 0.5);
    }
    if true {
        test_calc_push!(vec!["0", "cos"], 1.0);
        test_calc_push!(vec!["0", "sin"], 0.0);
        test_calc_push!(vec!["0", "tan"], 0.0);
        test_calc_push!(vec!["0", "cos", "neg"], -1.0);
    }
    if true {
        test_calc_prase_and_push!(" --2.5 ", -2.5);
        test_calc_prase_and_push!("0 cos * 3", 3.0);
        test_calc_prase_and_push!("50% + 5", 5.5);
    }
}
#[test]
pub fn test_calc_op() {
    test_calc_prase_and_push!("1 neg", -1.0);
    test_calc_prase_and_push!("1 neg abs", 1.0);
    test_calc_prase_and_push!("0 cos", 1.0);
    test_calc_prase_and_push!("0 sin", 0.0);
    test_calc_prase_and_push!("0 tan", 0.0);
    test_calc_prase_and_push!("1 acos", 0.0);
    test_calc_prase_and_push!("0 asin", 0.0);
    test_calc_prase_and_push!("0 atan", 0.0);
    test_calc_prase_and_push!("1 log", 0.0);
    test_calc_prase_and_push!("1 ln", 0.0);
    test_calc_prase_and_push!("4 sqrt", 2.0);
    test_calc_prase_and_push!("4 square", 16.0);
    test_calc_prase_and_push!("2 pow10", 100.0);
    test_calc_prase_and_push!("2 inv", 0.5);
    test_calc_prase_and_push!("0 exp", 1.0);
    test_calc_prase_and_push!("1 exp", 1.0_f64.exp());
    test_calc_prase_and_push!("50%", 0.5);

    test_calc_prase_and_push!("10 ^ 2", 100.0);
}
#[test]
pub fn test_calc_const() {
    test_calc_prase_and_push!(" 0 cos ", 1.0);
    test_calc_prase_and_push!(" ( 1 - 1 ) cos * ( 2 + 1)", 3.0);
    test_calc_prase_and_push!(" 50% + 50 %", 1.0);
    test_calc_prase_and_push!(" 2 * PI ", 2.0 * std::f64::consts::PI);
    test_calc_prase_and_push!(" 3 * E ", 3.0 * std::f64::consts::E);
}
#[test]
pub fn test_calc_priority() {
    test_calc_prase_and_push!(" 1+2*3 ", 7.0);
    test_calc_prase_and_push!(" 2*3+1 ", 7.0);
    test_calc_prase_and_push!(" (1+2)*3 ", 9.0);
    test_calc_prase_and_push!(" 2*(3+1) ", 8.0);
    test_calc_prase_and_push!(" 1+2-3 ", 0.0);
    test_calc_prase_and_push!(" 4/2*3 ", 6.0);
    let mut calc = DumbCalcProcessor::new();
    calc.parse_and_push("1+2");
    calc.push("*");
    calc.parse_and_push("(10 + 20) cos");
    calc.eval();
    assert_calc_eq_result!(&calc, 2.7320508075688776);
}
#[test]
pub fn test_calc_implicit_op() {
    test_calc_prase_and_push!("2(3)4", 24.0);
    test_calc_prase_and_push!("2(1+1)", 4.0);
    test_calc_prase_and_push!("(1+1)2", 4.0);
    test_calc_prase_and_push!("8/2(1+3)", 1.0);
    test_calc_prase_and_push!("8/(1+3)2", 1.0);
    test_calc_prase_and_push!("(1+2)(3+4)", 21.0);
}   //test_calc_prase_and_push!(" 123 321", 123.0);
 
#[test]
pub fn test_calc_angle() {
    test_calc_prase_and_push!(" 0 cos ", 1.0);
    test_calc_prase_and_push!(" 90 cos ", 0.0);
    test_calc_prase_and_push!(" 45 cos ", 0.7071067811865476);

    let mut calc = DumbCalcProcessor::new();
    calc.parse_and_push("30 cos");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.8660254037844387);
    calc.push("acos");
    calc.eval();
    assert_calc_eq_result!(&calc, 30.0);

    let mut calc = DumbCalcProcessor::new();
    calc.parse_and_push("30 sin");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.49999999999999994);
    calc.push("asin");
    calc.eval();
    assert_calc_eq_result!(&calc, 30.0);

    let mut calc = DumbCalcProcessor::new();
    calc.parse_and_push("30 tan");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.5773502691896257);
    calc.push("atan");
    calc.eval();
    assert_calc_eq_result!(&calc, 30.0);

    let mut calc = DumbCalcProcessor::new();
    calc.use_angle_mode("rad");
    calc.parse_and_push("0.5 cos");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.8775825618903728);
    calc.push("acos");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.5);

    calc.reset();
    calc.parse_and_push("0.5 sin");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.479425538604203);
    calc.push("asin");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.5);

    calc.reset();
    calc.parse_and_push("0.5 tan");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.5463024898437905);
    calc.push("atan");
    calc.eval();
    assert_calc_eq_result!(&calc, 0.5);
}
#[test]
pub fn test_calc_underscore() {
    test_calc_prase_and_push!("1_2_3", 123.0);
    test_calc_prase_and_push!("1_2_3_+_4_5_6", 579.0);
    test_calc_prase_and_push!("1_2_3_neg_", -123.0);
    test_calc_prase_and_push!("1_2_3_neg_*_2_", -246.0);
}
#[test]
pub fn test_calc_backup_and_restore() {
    let mut calc = DumbCalcProcessor::new();
    assert_calc_eq_result!(&calc, 0.0);
    calc.push("123.4");
    assert_eq!(123.4, calc.get_result().unwrap());
    let backup = calc.backup();
    calc.push("*");
    calc.push("2");
    calc.eval().unwrap();
    assert_eq!(246.8, calc.get_result().unwrap());
    calc.restore(backup);
    assert_eq!(123.4, calc.get_result().unwrap());
    calc.push("*");
    calc.push("20");
    calc.eval().unwrap();
    assert_eq!(2468.0, calc.get_result().unwrap());
}
