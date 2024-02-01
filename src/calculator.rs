//! A simple calculator that accepts input keys acting like a real calculator. It is base on [`crate::calc::DumbCalcProcessor`] for it's calculation processing.

#![deny(warnings)]
#![allow(unused)]

use crate::calc::{self, CalcResult};

#[test]
fn test_calculator() {}

pub struct DumbCalculator {
    entering: EnteringMode,
    calc: calc::DumbCalcProcessor,
}

/// a simple calculator that accepts input keys acting like a real calculator;
/// it may task is the keep track of key presses and turn them into "calculation units";
/// it uses a [`crate::calc::DumbCalcProcessor`] to handle the actual calculation processing
///
/// for example:
/// ```
/// use rusty_dumb_tools::calculator::DumbCalculator;
/// let mut calculator = DumbCalculator::new();
/// calculator.push("1").unwrap();
/// calculator.push(".").unwrap();
/// calculator.push("0").unwrap();
/// calculator.push("2").unwrap();
/// assert_eq!(calculator.get_display(), "1.02");
/// calculator.push("*").unwrap();
/// calculator.push("3").unwrap();
/// assert_eq!(calculator.get_display(), "3");
/// calculator.push("=").unwrap();
/// assert_eq!(calculator.get_display(), "3.06");
/// ```
///
/// for a fuller sample code, please refer to the "calculator" sub-demo of [`crate::demo::run_demo`]
impl DumbCalculator {
    /// create a new [`DumbCalculator`] instance
    pub fn new() -> Self {
        Self {
            entering: EnteringMode::Not,
            calc: calc::DumbCalcProcessor::new(),
        }
    }
    /// push a key input:
    /// * a digit, including a "."
    /// * operators accepted by [`crate::calc::DumbCalcProcessor::push`] like:
    ///   - binary operators; e.g. "+", "-", "*", "/"
    ///   - unary operators; e.g. "neg", "sin", "cos", "tan", "asin", "acos", "atan", "sqrt", "ln", "log
    ///   - constants; e.g. "PI", "E"
    ///   - "="
    pub fn push(&mut self, key: &str) -> Result<(), String> {
        if key == "." {
            match &self.entering {
                EnteringMode::Not => {
                    self.entering = EnteringMode::Decimal(0, String::from(""));
                }
                EnteringMode::Integer(i) => {
                    self.entering = EnteringMode::Decimal(*i, String::from(""));
                }
                EnteringMode::Decimal(i, d) => {
                    self.entering = EnteringMode::Decimal(*i, d.clone());
                }
            }
        } else if key >= "0" && key <= "9" {
            let digit = key.parse::<u32>().unwrap();
            match &self.entering {
                EnteringMode::Not => {
                    self.entering = EnteringMode::Integer(digit); // TODO: move self.entering out
                }
                EnteringMode::Integer(i) => {
                    self.entering = EnteringMode::Integer(*i * 10 + digit);
                }
                EnteringMode::Decimal(i, d) => {
                    let digit_str = digit.to_string();
                    if d == "" {
                        self.entering = EnteringMode::Decimal(*i, digit_str);
                    } else {
                        let mut new_d = d.clone();
                        new_d.push_str(&digit_str /*&digit.to_string()*/);
                        self.entering = EnteringMode::Decimal(*i, new_d);
                    }
                }
            }
        } else {
            match &self.entering {
                EnteringMode::Not => {}
                EnteringMode::Integer(i) => {
                    self.calc.push(i.to_string().as_str()).unwrap();
                    self.entering = EnteringMode::Not;
                }
                EnteringMode::Decimal(i, d) => {
                    let num = if d == "" {
                        format!("{}.0", i)
                    } else {
                        format!("{}.{}", i, d)
                    };
                    self.calc.push(num.as_str()).unwrap();
                    self.entering = EnteringMode::Not;
                }
            }
            self.calc.push(key)?;
        }
        Ok(())
    }
    /// like [`DumbCalculator::push`] but each characters of the input will be pushed individually one by one
    pub fn push_chars(&mut self, keys: &str) -> Result<(), String> {
        for key in keys.chars() {
            if key != ' ' {
                self.push(key.to_string().as_str())?;
            }
        }
        Ok(())
    }
    pub fn reset(&mut self) {
        self.entering = EnteringMode::Not;
        self.calc.reset();
    }
    // pub fn get_display(&self) -> CalculatorDisplay {
    //     match self.entering {
    //         EnteringMode::Not => {
    //             match self.calc.get_result() {
    //                 CalcResult::Final(r) => CalculatorDisplay::Normal(r),
    //                 CalcResult::Intermediate(r) => CalculatorDisplay::Normal(r),
    //                 CalcResult::Error(e) => CalculatorDisplay::Error(e),
    //             }
    //         }
    //         EnteringMode::Integer(i) => CalculatorDisplay::Normal(i as f64),
    //         EnteringMode::Decimal(i, d) => {
    //             let d_str = d.to_string();
    //             let divider = 10_f64.powf(d_str.len() as f64);
    //             let r = i as f64 + d as f64 / divider;
    //             CalculatorDisplay::Normal(r)
    //         }
    //     }
    // }
    pub fn get_display(&self) -> String {
        return self._get_display(None);
    }
    pub fn get_display_sized(&self, result_width: usize) -> String {
        return self._get_display(Some(result_width));
    }
    fn _get_display(&self, result_width: Option<usize>) -> String {
        let (mut display_result, result) = match &self.entering {
            EnteringMode::Not => {
                let result = match self.calc.get_result() {
                    CalcResult::Final(r) => r,
                    CalcResult::Intermediate(r) => r,
                    CalcResult::Error(e) => return String::from("Error"),
                };
                let display_result = format!("{}", result);
                (display_result, result)
            }
            EnteringMode::Integer(i) => {
                let result = *i as f64;
                let display_result = format!("{}", result);
                (display_result, result)
            }
            EnteringMode::Decimal(i, d) => {
                if d == "" {
                    let display_result = format!("{}.0", i);
                    let result = *i as f64;
                    (display_result, result)
                } else {
                    let display_result = format!("{}.{}", i, d);
                    let divider = 10_f64.powf(d.len() as f64);
                    let d = d.parse::<u32>().unwrap();
                    let result = *i as f64 + d as f64 / divider;
                    (display_result, result)
                }
            }
        };
        //let result = -21.2345;
        //let result = -0.123456789123456789;
        //let result = -1234567891234.0;
        if result_width.is_some() {
            let result_width = result_width.unwrap();
            if display_result.len() < result_width {
                let room = result_width - display_result.len();
                display_result = format!("{}{}", " ".repeat(room), display_result);
            } else {
                let room = result_width - (if result < 0.0 { 5 } else { 4 });
                display_result = format!("{:.*}", room, result);
                if display_result.len() > result_width {
                    let room = result_width - (if result < 0.0 { 8 } else { 7 });
                    display_result = format!("{:.*e}", room, result);
                }
            }
        }
        display_result
        //let display_result = format!("\x1B[7m {} \x1B[0m", display_result);
        //Some((display_result, DISPLAY_WIDTH))
    }
}

// pub enum CalculatorDisplay {
//     Normal(f64),
//     Error(String),
// }

enum EnteringMode {
    Not,
    Integer(u32),
    Decimal(u32, String),
}
