#![deny(warnings)]
#![allow(unused)]

use crate::calculator::{DumbCalculator, DumbCalculatorSettings};

#[test]
fn test_calculator_push() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display(), "0");
    calculator.push("1").unwrap();
    assert_eq!(calculator.get_display(), "1");
    calculator.push(".").unwrap();
    assert_eq!(calculator.get_display(), "1.0");
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
    let mut calculator = DumbCalculator::new_ex(DumbCalculatorSettings {
        enable_undo: true,
        ..DumbCalculatorSettings::default()
    });
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
