#![deny(warnings)]
#![allow(unused)]

use crate::calculator::DumbCalculator;

#[test]
fn test_calculator_push() {
    let mut calculator = DumbCalculator::new();
    assert_eq!(calculator.get_display(), "0");
    calculator.push("1").unwrap();
    assert_eq!(calculator.get_display(), "1");
    calculator.push(".").unwrap();
    assert_eq!(calculator.get_display(), "1.0");
    calculator.push("2").unwrap();
    assert_eq!(calculator.get_display(), "1.2");
    calculator.push(".").unwrap();
    assert_eq!(calculator.get_display(), "1.2");
    calculator.push("+").unwrap();
    assert_eq!(calculator.get_display(), "1.2");
    calculator.push("1").unwrap();
    calculator.push("0").unwrap();
    assert_eq!(calculator.get_display(), "10");
    calculator.push("=").unwrap();
    assert_eq!(calculator.get_display(), "11.2");
    calculator.push("*").unwrap();
    assert_eq!(calculator.get_display(), "11.2");
    calculator.push("4").unwrap();
    assert_eq!(calculator.get_display(), "4");
    calculator.push("=").unwrap();
    assert_eq!(calculator.get_display(), "44.8");
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
    calculator.push_chars("1.2+3.4=").unwrap();
    assert_eq!(calculator.get_display(), "4.6");
    calculator.push_chars("2 * (3.4 - 5) -6.7 =").unwrap();
    assert_eq!(calculator.get_display(), "-9.9");
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
