//! A simple calculator that accepts input keys acting like a real calculator. It is base on [`crate::calc::DumbCalcProcessor`] for it's calculation processing.

#![deny(warnings)]
#![allow(unused)]

use core::panic;
use std::{error::Error, ops::Index};

use crate::{
    calc::{self, CalcProcessorBackup, CalcResult, DumbCalcProcessor},
    shared::DumbError,
};

#[test]
fn test_calculator() {}

pub struct DumbCalculator {
    entering: EnteringMode,
    calc: calc::DumbCalcProcessor,
    undo_stack: Option<Vec<UndoStep>>,
    history_stack: Option<Vec<String>>,
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
    /// create a new [`DumbCalculator`] instance with default settings (i.e. no undo etc)
    pub fn new() -> Self {
        DumbCalculator::new_ex(DumbCalculatorSettings::default())
    }
    /// like [`DumbCalculator::new`] but with settings
    pub fn new_ex(settings: DumbCalculatorSettings) -> Self {
        let undo_stack = if settings.enable_undo {
            Some(Vec::new())
        } else {
            None
        };
        let history_stack = if settings.enable_history {
            Some(Vec::new())
        } else {
            None
        };
        Self {
            entering: EnteringMode::Not,
            calc: calc::DumbCalcProcessor::new(),
            undo_stack: undo_stack,
            history_stack: history_stack,
        }
    }
    /// push a "key input":
    /// * a digit, including a "."
    /// * a bracket: “(”, “)”
    /// * an operator accepted by [`crate::calc::DumbCalcProcessor::push`] like:
    ///   - binary operators; e.g. "+", "-", "*", "/", etc
    ///   - unary operators; e.g. "neg", "sin", "cos", "tan", etc
    /// * a constant accepted by [`crate::calc::DumbCalcProcessor::push`] like "PI", etc
    /// * "="
    pub fn push(&mut self, key: &str) -> Result<(), DumbError> {
        if key == "." {
            self._record_undo(key, false);
            self.entering = match &self.entering {
                EnteringMode::Not => EnteringMode::Decimal(0, String::from("")),
                EnteringMode::Integer(i) => EnteringMode::Decimal(*i, String::from("")),
                EnteringMode::Decimal(i, d) => EnteringMode::Decimal(*i, d.clone()),
                EnteringMode::Error => EnteringMode::Error,
            }
        } else if key >= "0" && key <= "9" {
            self._record_undo(key, false);
            let digit = key.parse::<u32>().unwrap();
            self.entering = match &self.entering {
                EnteringMode::Not => EnteringMode::Integer(digit),
                EnteringMode::Integer(i) => {
                    if true {
                        match i.checked_mul(10).and_then(|x| x.checked_add(digit)) {
                            Some(new_i) => EnteringMode::Integer(new_i),
                            None => EnteringMode::Error,
                        }
                    } else {
                        EnteringMode::Integer(*i * 10 + digit)
                    }
                }
                EnteringMode::Decimal(i, d) => {
                    let digit_str = digit.to_string();
                    if d.is_empty() {
                        EnteringMode::Decimal(*i, digit_str)
                    } else {
                        let mut new_d = d.clone();
                        new_d.push_str(&digit_str /*&digit.to_string()*/);
                        EnteringMode::Decimal(*i, new_d)
                    }
                }
                EnteringMode::Error => EnteringMode::Error,
            }
        } else {
            self._record_undo(key, true);
            self.entering = match &self.entering {
                EnteringMode::Not => EnteringMode::Not,
                EnteringMode::Integer(i) => {
                    self.calc.push(i.to_string().as_str()).unwrap();
                    EnteringMode::Not
                }
                EnteringMode::Decimal(i, d) => {
                    let num = if d.is_empty() {
                        format!("{}.0", i)
                    } else {
                        format!("{}.{}", i, d)
                    };
                    self.calc.push(num.as_str()).unwrap();
                    EnteringMode::Not
                }
                EnteringMode::Error => EnteringMode::Error,
            };
            self.calc.push(key)?;
        }
        Ok(())
    }
    /// like [`DumbCalculator::push`] but each characters of the input will be pushed individually one by one
    pub fn push_chars(&mut self, keys: &str) -> Result<(), DumbError> {
        for key in keys.chars() {
            if key != ' ' {
                self.push(key.to_string().as_str())?;
            }
        }
        Ok(())
    }
    /// undo the last "key input" done by [`DumbCalculator::push`]
    pub fn undo(&mut self) {
        if let Some(history_stack) = &mut self.history_stack {
            history_stack.pop();
        }
        if let Some(undo_stack) = &mut self.undo_stack {
            let undo = undo_stack.pop();
            if let Some(undo) = undo {
                match undo {
                    UndoStep::EnteringBackup(entering) => {
                        self.entering = entering;
                    }
                    UndoStep::CalcBackup(backup, entering) => {
                        //println!("backup: {:?}", entering);
                        self.calc.restore(backup);
                        self.entering = entering;
                    }
                }
            } else {
                self.entering = EnteringMode::Not;
                self.calc.reset();
            }
        }
    }
    fn _record_undo(&mut self, key: &str, for_calc: bool) {
        if let Some(history_stack) = &mut self.history_stack {
            history_stack.push(key.to_string());
        }
        if let Some(undo_stack) = &mut self.undo_stack {
            let undo = if for_calc {
                UndoStep::CalcBackup(self.calc.backup(), self.entering.clone())
            } else {
                UndoStep::EnteringBackup(self.entering.clone())
                // match &self.entering {
                //     EnteringMode::Not => None,
                //     EnteringMode::Integer(i) => Some(UndoStep::EnteringInteger(*i)),
                //     EnteringMode::Decimal(i, d) => Some(UndoStep::EnteringDecimal(*i, d.clone())),
                //     EnteringMode::Error => None,
                // }
            };
            undo_stack.push(undo);
        }
    }
    pub fn reset(&mut self) {
        self.entering = EnteringMode::Not;
        self.calc.reset();
        if let Some(undo_stack) = &mut self.undo_stack {
            undo_stack.clear();
        }
        if let Some(history_stack) = &mut self.history_stack {
            history_stack.clear();
        }
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
    pub fn get_history(&self) -> Option<&Vec<String>> {
        if let Some(history_stack) = &self.history_stack {
            Some(history_stack)
        } else {
            None
        }
    }
    pub fn get_history_string(&self) -> Option<String> {
        let history = self.get_history();
        if let Some(history) = history {
            let mut hist: String = String::new();
            for h in history {
                if DumbCalcProcessor::is_unary_operator(h) {
                    hist.push_str(format!("_{}_", h).as_str());
                } else {
                    let h = h.trim();
                    if !h.is_empty() {
                        hist.push_str(h);
                    }
                }
            }
            Some(hist)
        } else {
            None
        }
}
    // pub fn get_history_formatted(&self) -> Option<String> {
    //     if let Some(history_stack) = &self.history_stack {
    //         if true {
    //             let mut formatted = String::new();
    //             for history in history_stack.iter() {
    //                 formatted.push_str(history);
    //             }
    //             Some(formatted)
    //         } else {
    //             let history: Vec<String> = history_stack.iter().map(|s| s.to_string()).collect();
    //             let count = history.len();
    //             let mut formatted = Vec::new();
    //             let mut i = 0;
    //             loop {
    //                 if i >= count {
    //                     break;
    //                 }
    //                 let hist = history.get(i).unwrap();
    //                 if DumbCalcProcessor::is_unary_operator(hist) {
    //                     let f = formatted.pop().unwrap();
    //                     let f = format!(" {}({})", hist, f);
    //                     formatted.push(f);
    //                 } else {
    //                     formatted.push(hist.clone());
    //                 }
    //                 i = i + 1;
    //             }
    //             let formatted = formatted.join("");
    //             Some(formatted)
    //         }
    //     } else {
    //         None
    //     }
    // }
    pub fn get_display(&self) -> String {
        self._get_display(None)
    }
    pub fn get_display_sized(&self, result_width: usize) -> String {
        self._get_display(Some(result_width))
    }
    fn _get_display(&self, result_width: Option<usize>) -> String {
        let mut display_result = self.__get_display(result_width);
        if let Some(result_width) = result_width {
            if result_width == 0 {
                panic!("result_width is zero")
            }
            // if display_result != "Error" {
            //     if display_result.len() != result_width {
            //         // normally should not be the case
            //         display_result = "Error".to_string();
            //     }
            // }
            if display_result == "Error" {
                if result_width < 5 {
                    if result_width < 3 {
                        display_result = format!("{}E", " ".repeat(result_width - 1))
                    } else {
                        display_result = format!("{}Err", " ".repeat(result_width - 3))
                    }
                } else {
                    display_result = format!("{}Error", " ".repeat(result_width - 5))
                }
            }
            if display_result.len() <= result_width {
                let room = result_width - display_result.len();
                display_result = format!("{}{}", " ".repeat(room), display_result);
            } else {
                if true {
                    display_result = format!("{}", "~".repeat(result_width))
                } else {
                    panic!(
                        "cannot fit display_result [{}] ({}) to fixed width {}",
                        display_result,
                        display_result.len(),
                        result_width
                    );
                }
            }
        }
        display_result
    }
    fn __get_display(&self, result_width: Option<usize>) -> String {
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
                if d.is_empty() {
                    let display_result = format!("{}.", i);
                    let result = *i as f64;
                    (display_result, result)
                } else {
                    let display_result = format!("{}.{}", i, d);
                    let divider = 10_f64.powf(d.len() as f64);
                    //let d = d.parse::<u64>().unwrap();
                    let d = match d.parse::<u64>() {
                        Ok(d) => d,
                        Err(_) => {
                            return String::from("Error");
                        }
                    };
                    let result = *i as f64 + d as f64 / divider;
                    (display_result, result)
                }
            }
            EnteringMode::Error => {
                return String::from("Error");
            }
        };
        //let result = -21.2345;
        //let result = -0.123456789123456789;
        //let result = -1234567891234.0;
        if let Some(result_width) = result_width {
            if true {
                // let need_reformat = if display_result.len() == result_width {
                //     let dr = display_result.replace('-', "");
                //     let dr = dr.replace(".", "");
                //     let dr = dr.replace('0', "");
                //     let dr = dr.trim();
                //     dr.is_empty()
                // } else {
                //     display_result.len() > result_width
                // };
                if display_result.len() > result_width {
                    if display_result.contains(".") {
                        let dot_idx = display_result.find('.').unwrap();
                        let places: i32 = result_width as i32 - dot_idx as i32 - 1;
                        if places > 0 {
                            display_result = format!("{:.*}", places as usize, result);
                        }
                    }
                    if display_result.len() > result_width {
                        let places = result_width as i32 - (if result < 0.0 { 5 } else { 4 });
                        if places > 0 {
                            display_result = format!("{:.*e}", places as usize, result);
                        }
                    }
                }
                if true {
                    let is_zero = if display_result.len() == result_width {
                        let dr = display_result.replace('-', "");
                        let dr = dr.replace(".", "");
                        let dr = dr.replace('0', "");
                        let dr = dr.trim();
                        dr.is_empty()
                    } else {
                        false
                    };
                    if is_zero {
                        let places = result_width as i32 - (if result < 0.0 { 6 } else { 5 });
                        if places > 0 {
                            let ori_display_result = display_result;
                            display_result = format!("{:.*e}", places as usize, result);
                            if display_result.len() > result_width {
                                //println!("{}", display_result);
                                // e more than 1 digits
                                display_result = ori_display_result;
                            }
                        }
                    }
                }
            } else {
                if display_result.len() <= result_width {
                    let room = result_width - display_result.len();
                    display_result = format!("{}{}", " ".repeat(room), display_result);
                } else {
                    //let room = result_width - (if result < 0.0 { 5 } else { 4 });
                    let room: i32 = result_width as i32 - (if result < 0.0 { 3 } else { 2 });
                    if room > 0 {
                        display_result = format!("{:.*}", room as usize, result);
                    }
                    if display_result.len() > result_width {
                        println!(
                            "display_result: {} ({}) ... room={}",
                            display_result,
                            display_result.len(),
                            room
                        );
                        //let room = result_width as i32 - (if result < 0.0 { 8 } else { 7 });
                        let room = result_width as i32 - (if result < 0.0 { 5 } else { 4 });
                        if room > 0 {
                            display_result = format!("{:.*e}", room as usize, result);
                        }
                    }
                }
            }
        }
        display_result
        //let display_result = format!("\x1B[7m {} \x1B[0m", display_result);
        //Some((display_result, DISPLAY_WIDTH))
    }
    pub fn get_last_operator(&self) -> Option<String> {
        self.calc.get_last_operator()
    }
    pub fn count_opened_brackets(&self) -> u16 {
        self.calc.count_opened_brackets()
    }
}

pub struct DumbCalculatorSettings {
    pub enable_undo: bool,
    pub enable_history: bool,
}
impl Default for DumbCalculatorSettings {
    fn default() -> Self {
        Self {
            enable_undo: false,
            enable_history: false,
        }
    }
}

// pub enum CalculatorDisplay {
//     Normal(f64),
//     Error(String),
// }

enum UndoStep {
    EnteringBackup(EnteringMode),
    //EnteringInteger(u32),
    //EnteringDecimal(u32, String),
    CalcBackup(CalcProcessorBackup, EnteringMode),
}

#[derive(Debug, Clone)]
enum EnteringMode {
    Not,
    Error,
    Integer(u32),
    Decimal(u32, String),
}
