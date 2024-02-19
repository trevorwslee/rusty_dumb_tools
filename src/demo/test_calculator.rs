#![deny(warnings)]
#![allow(unused)]

use crate::prelude::*;

#[test]
fn test_calculator_push() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display(), "0");
    calculator.push("1").unwrap();
    assert_eq!(calculator.get_display(), "1");
    calculator.push(".").unwrap();
    assert_eq!(calculator.get_display(), "1.");
    calculator.push("0").unwrap();
    assert_eq!(calculator.get_display(), "1.0");
    calculator.push("2").unwrap();
    assert_eq!(calculator.get_display(), "1.02");
    calculator.push(".").unwrap();
    assert_eq!(calculator.get_display(), "1.02");
    calculator.push("+").unwrap();
    assert_eq!(calculator.get_display(), "1.02");
    calculator.push("1").unwrap();
    calculator.push("0").unwrap();
    assert_eq!(calculator.get_display(), "10");
    calculator.push("=").unwrap();
    assert_eq!(calculator.get_display(), "11.02");
    calculator.push("*").unwrap();
    assert_eq!(calculator.get_display(), "11.02");
    calculator.push("4").unwrap();
    assert_eq!(calculator.get_display(), "4");
    calculator.push("=").unwrap();
    assert_eq!(calculator.get_display(), "44.08");
}

#[test]
fn test_calculator_normal() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display(), "0");
    let input = "2*(1+3)-4=-3=";
    let check = "2201134844431";
    let input: Vec<char> = input.chars().collect();
    let check: Vec<char> = check.chars().collect();
    let count = input.len();
    assert_eq!(count, check.len());
    for i in 0..count {
        let input = input[i];
        let check = check[i];
        calculator.push(input.to_string().as_str()).unwrap();
        assert_eq!(calculator.get_display(), check.to_string());
    }
    calculator.push("=").unwrap();
    assert_eq!(calculator.get_display(), "1");
    calculator.reset();
    assert_eq!(calculator.get_display(), "0");
}

#[test]
fn test_calculator_push_chars() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display(), "0");
    if true {
        calculator.push_chars("1.2").unwrap();
        assert_eq!(calculator.get_display(), "1.2");
        calculator.reset();
    }
    calculator.push_chars("1.2+3.4=").unwrap();
    assert_eq!(calculator.get_display(), "4.6");
    calculator.push_chars("2 * (3.4 - 5) -6.7 =").unwrap();
    assert_eq!(calculator.get_display(), "-9.9");
}

#[test]
fn test_calculator_unary() {
    let mut calculator = DumbCalculator::new();
    calculator.push_chars("1%").unwrap();
    assert_eq!(calculator.get_display(), "0.01");
}

#[test]
fn test_calculator_special() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display(), "0");
    calculator.push_chars("2+(3+)=").unwrap();
    assert_eq!(calculator.get_display(), "5");
    calculator.push_chars("2(4)=").unwrap();
    assert_eq!(calculator.get_display(), "8");
    calculator.push_chars("+1=").unwrap();
    assert_eq!(calculator.get_display(), "9");
    calculator.push_chars("2+(+3)=").unwrap();
    assert_eq!(calculator.get_display(), "5");
    calculator.push_chars("+-*/5=").unwrap();
    assert_eq!(calculator.get_display(), "1");
}

#[test]
fn test_calculator_undo() {
    let mut calculator = DumbCalculator::new();
    if true {
        calculator.push_chars("1+2").unwrap();
        calculator.undo();
        calculator.push_chars("3=").unwrap();
        assert_eq!(calculator.get_display(), "4");
    }
    calculator.reset();
    calculator.undo();
    assert_eq!(calculator.get_display(), "0");
    calculator.push_chars("123").unwrap();
    assert_eq!(calculator.get_display(), "123");
    calculator.undo();
    assert_eq!(calculator.get_display(), "12");
    calculator.push_chars(".3*2=").unwrap();
    assert_eq!(calculator.get_display(), "24.6");
    calculator.undo();
    assert_eq!(calculator.get_display(), "2");
    calculator.undo();
    assert_eq!(calculator.get_display(), "12.3");
    calculator.push_chars("-2=").unwrap();
    assert_eq!(calculator.get_display(), "10.3");
    calculator.undo();
    calculator.undo();
    calculator.undo();
    calculator.undo();
    assert_eq!(calculator.get_display(), "12.3");
    calculator.push_chars("4*2=").unwrap();
    assert_eq!(calculator.get_display(), "24.68");
    calculator.reset();
}

#[test]
fn test_calculator_display() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display_sized(5), "    0");

    calculator.push_chars(".123");
    assert_eq!(calculator.get_display_sized(5), "0.123");

    calculator.push_chars("45");
    assert_eq!(calculator.get_display_sized(5), "0.123");

    calculator.push("neg");
    assert_eq!(calculator.get_display_sized(5), "-0.12");
}
#[test]
fn test_calculator_display_roundoff() {
    let mut calculator = DumbCalculator::new();
    calculator.push_chars("5.00005/5.000001=");
    assert_eq!(calculator.get_display_sized(5), "1.000");
    assert_eq!(calculator.get_display_sized(6), "1.0000");
    assert_eq!(calculator.get_display_sized(7), "1.00001");
}
#[test]
fn test_calculator_display_e() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display_sized(5), "    0");

    calculator.push_chars("99.123");
    assert_eq!(calculator.get_display_sized(5), "99.12");
    assert_eq!(calculator.get_display_sized(6), "99.123");
    assert_eq!(calculator.get_display_sized(7), " 99.123");

    calculator.push("neg");
    assert_eq!(calculator.get_display_sized(5), "-99.1");
    assert_eq!(calculator.get_display_sized(6), "-99.12");
    assert_eq!(calculator.get_display_sized(7), "-99.123");
    assert_eq!(calculator.get_display_sized(8), " -99.123");

    calculator.reset();
    calculator.push_chars("123456.7");
    assert_eq!(calculator.get_display_sized(4), "~~~~");
    assert_eq!(calculator.get_display_sized(5), "1.2e5");
    assert_eq!(calculator.get_display_sized(6), "1.23e5");
    assert_eq!(calculator.get_display_sized(7), "1.235e5");
    assert_eq!(calculator.get_display_sized(8), "123456.7");
    assert_eq!(calculator.get_display_sized(9), " 123456.7");

    calculator.reset();
    calculator.push_chars("123456.7");
    calculator.push("neg");
    assert_eq!(calculator.get_display_sized(6), "-1.2e5");
    assert_eq!(calculator.get_display_sized(7), "-1.23e5");
    assert_eq!(calculator.get_display_sized(8), "-1.235e5");
    assert_eq!(calculator.get_display_sized(9), "-123456.7");
    assert_eq!(calculator.get_display_sized(10), " -123456.7");

    calculator.reset();
    calculator.push_chars("0.00001");
    assert_eq!(calculator.get_display_sized(6), "1.0e-5");
    assert_eq!(calculator.get_display_sized(7), "0.00001");
    assert_eq!(calculator.get_display_sized(8), " 0.00001");

    calculator.reset();
    calculator.push_chars("0.00001");
    calculator.push("neg");
    assert_eq!(calculator.get_display_sized(7), "-1.0e-5");
    assert_eq!(calculator.get_display_sized(8), "-0.00001");
    assert_eq!(calculator.get_display_sized(9), " -0.00001");
}
#[test]
fn test_calculator_display_error() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display_sized(5), "    0");

    calculator.push_chars("1/0=");
    assert_eq!(calculator.get_display_sized(1), "E");
    assert_eq!(calculator.get_display_sized(2), " E");
    assert_eq!(calculator.get_display_sized(3), "Err");
    assert_eq!(calculator.get_display_sized(4), " Err");
    assert_eq!(calculator.get_display_sized(5), "Error");
    assert_eq!(calculator.get_display_sized(6), " Error");
}
#[test]
fn test_history_normal() {
    let mut calculator = DumbCalculator::new();
    calculator.push_chars("12.345");
    assert_eq!(calculator.get_history_string(false).unwrap(), "12.345");

    calculator.reset();
    calculator.push_chars(" 12.34 + 5.67 ");
    assert_eq!(calculator.get_history_string(false).unwrap(), "12.34+5.67");

    calculator.reset();
    calculator.push_chars(" 2 * ( 3 + 4 - 1) / 10");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "2*(3+4-1)/10"
    );
    calculator.reset();
}
#[test]
fn test_history_unary() {
    let mut calculator = DumbCalculator::new();
    calculator.push_chars("123.456%");
    assert_eq!(calculator.get_history_string(false).unwrap(), "123.456%");

    calculator.reset();
    calculator.push_chars("123.456");
    calculator.push("neg");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "neg(123.456)"
    );

    calculator.reset();
    calculator.push_chars(" 12 + 34 ");
    calculator.push("neg");
    assert_eq!(calculator.get_history_string(false).unwrap(), "12+neg(34)");

    calculator.reset();
    calculator.push_chars("123");
    calculator.push("neg");
    calculator.push("neg");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "neg(neg(123))"
    );

    calculator.reset();
    calculator.push_chars(" 10 * 123");
    calculator.push("sin");
    calculator.push("neg");
    calculator.push_chars(" * 20");
    calculator.push("cos");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "10*neg(sin(123))*cos(20)"
    );

    calculator.reset();
    calculator.push_chars(" 10 + ( 123");
    calculator.push("sin");
    calculator.push_chars(" * 20) * (1 + 2 + 3");
    calculator.push("neg");
    calculator.push_chars(")");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "10+(sin(123)*20)*(1+2+neg(3))"
    );
}
#[test]
fn test_history_unary_2() {
    let mut calculator = DumbCalculator::new();
    calculator.push_chars("(1)");
    calculator.push("neg");
    assert_eq!(calculator.get_history_string(false).unwrap(), "neg(1)");

    calculator.reset();
    calculator.push_chars("2 * ( 3 + 4 ");
    calculator.push("sin");
    calculator.push_chars(" ) + 123 ");
    calculator.push("cos");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "2*(3+sin(4))+cos(123)"
    );
}
#[test]
fn test_history_unary_finalized() {
    let mut calculator = DumbCalculator::new();
    calculator.push_chars("(1)");
    calculator.push("neg");
    calculator.push("=");
    assert_eq!(calculator.get_history_string(false).unwrap(), "{neg(1)}");
    calculator.push("neg");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "neg({neg(1)})"
    );
}
#[test]
fn test_history_unary_bug() {
    let mut calculator = DumbCalculator::new();
    calculator.push_chars("((2)");
    calculator.push("neg");
    calculator.push_chars(")");
    calculator.push("neg");
    assert_eq!(calculator.get_history_string(false).unwrap(), "neg(neg(2))");

    calculator.reset();
    calculator.push_chars("(((3+4)");
    calculator.push("sin");
    calculator.push_chars(")+5");
    calculator.push("cos");
    calculator.push_chars(")");
    calculator.push("neg");
    assert_eq!(
        calculator.get_history_string(false).unwrap(),
        "neg((sin(3+4))+cos(5))"
    );
}
