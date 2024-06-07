//! A simple calculator that accepts input keys acting like a real calculator. It is base on [`crate::calc::DumbCalcProcessor`] for it's calculation processing.

#![deny(warnings)]
#![allow(unused)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::new_without_default)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::redundant_field_names)]

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
    memory: Option<f64>,
}

/// A simple calculator that accepts input keys acting like a real calculator;
/// it may task is the keep track of key presses and turn them into "calculation units";
/// it uses a [`crate::calc::DumbCalcProcessor`] to handle the actual calculation processing
///
/// For example:
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
/// For a fuller sample code, please refer to the "calculator" sub-demo of [`crate::demo::run_demo`]
impl DumbCalculator {
    /// create a new [`DumbCalculator`] instance with minimum feature (i.e. no undo etc)
    pub fn new_min() -> Self {
        DumbCalculator::new_with_settings(DumbCalculatorSettings {
            enable_history: false,
            enable_undo: false,
            ..DumbCalculatorSettings::default()
        })
    }
    /// create a new [`DumbCalculator`] (with all the default features enabled)
    pub fn new() -> Self {
        DumbCalculator::new_with_settings(DumbCalculatorSettings::default())
    }
    /// like [`DumbCalculator::new`] but with settings
    pub fn new_with_settings(settings: DumbCalculatorSettings) -> Self {
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
            memory: None,
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
    /// there are some special "key input":
    /// * "ac": clear the calculator
    /// * "undo": undo the last "key input" done by [`DumbCalculator::push`], if undo is enabled
    /// * "mc", "mr", "ms", "m+", "m-": memory keys
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
            if key == "ac" {
                self.clear();
                return Ok(());
            } else if key == "undo" {
                self.undo();
                return Ok(());
            }
            let memory_key = match key {
                "mc" | "mr" | "ms" | "m+" | "m-" => Some(key),
                _ => {
                    self._record_undo(key, true);
                    None
                }
            };
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
            match memory_key {
                Some(memory_key) => {
                    self.on_memory_key(memory_key);
                }
                _ => {
                    self.calc.push(key)?;
                }
            }
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
    /// undo the last "key input" done by [`DumbCalculator::push`], if undo is enabled
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
    /// use the "angle mode" for trigonometric functions
    /// * angle_mode: "deg" or "rad"
    pub fn use_angle_mode(&mut self, angle_mode: &str) {
        self.calc.use_angle_mode(angle_mode);
    }
    /// clear the calculator (but will not clear the memory)
    pub fn clear(&mut self) {
        self.entering = EnteringMode::Not;
        self.calc.reset();
        if let Some(undo_stack) = &mut self.undo_stack {
            undo_stack.clear();
        }
        if let Some(history_stack) = &mut self.history_stack {
            history_stack.clear();
        }
    }
    /// reset the calculator (and clear the memory)
    pub fn reset(&mut self) {
        self.clear();
        self.memory = None;
    }
    /// get memory
    pub fn get_memory(&self) -> Option<f64> {
        self.memory
    }
    /// get history of the "key input", if history is enabled
    pub fn get_history(&self) -> Option<&[String]> {
        if let Some(history_stack) = &self.history_stack {
            Some(history_stack)
        } else {
            None
        }
    }
    /// like [`DumbCalculator::get_history`] but returns a string instead
    pub fn get_history_string(&self, better_symbols: bool) -> Option<String> {
        enum Marker {
            NumberStart(usize),
            OpenBracket(usize),
            Finalized,
        }
        enum UnaryBracket {
            No,
            IfNeeded,
            IfNeededNotNum,
        }
        let history = self.get_history();
        if let Some(history) = history {
            let mut hist: String = String::new();
            //let mut prev_h: Option<&str> = None;
            let mut marker_stack: Vec<Marker> = Vec::new();
            let mut prev_c = None::<char>;
            let mut pop_marker = false;
            for h in history {
                let mut new_prev_c = None::<char>;
                let mut new_pop_marker = false;
                if h == "=" {
                    let mut new_hist = String::new();
                    new_hist.push('{');
                    new_hist.push_str(hist.as_str());
                    new_hist.push('}');
                    hist = new_hist;
                    marker_stack.clear();
                    marker_stack.push(Marker::Finalized);
                    new_prev_c = None;
                    new_pop_marker = false;
                } else if DumbCalcProcessor::is_unary_operator(h) {
                    let marker = marker_stack.last();
                    let marker = match marker {
                        Some(Marker::NumberStart(start)) => Some((*start, true)),
                        Some(Marker::OpenBracket(start)) => Some((*start, false)),
                        Some(Marker::Finalized) => Some((0, true)),
                        _ => None,
                    };
                    if h == "%" && !better_symbols {
                        hist.push('%');
                    } else if let Some((start, bracket)) = marker {
                        let prefix = hist[0..start].to_string();
                        let inside = hist[start..].to_string();
                        let (h1, h2, unary_bracket) = if better_symbols && h == "neg" {
                            ("-", "", UnaryBracket::IfNeededNotNum)
                        } else if better_symbols && h == "%" {
                            if inside.parse::<f64>().is_err() {
                                ("", "/100", UnaryBracket::IfNeeded)
                            } else {
                                ("", "%", UnaryBracket::No)
                            }
                        } else if better_symbols && h == "square" {
                            ("", "²", UnaryBracket::IfNeededNotNum)
                        } else if better_symbols && h == "inv" {
                            ("1/", "", UnaryBracket::IfNeededNotNum)
                        } else if better_symbols && h == "sqrt" {
                            ("√", "", UnaryBracket::IfNeededNotNum)
                        } else if better_symbols && h == "abs" {
                            ("|", "|", UnaryBracket::No)
                        } else if better_symbols && h == "pow10" {
                            ("10^", "", UnaryBracket::No)
                        } else if better_symbols && h == "asin" {
                            ("sin⁻¹", "", UnaryBracket::IfNeeded)
                        } else if better_symbols && h == "acos" {
                            ("cos⁻¹", "", UnaryBracket::IfNeeded)
                        } else if better_symbols && h == "atan" {
                            ("tan⁻¹", "", UnaryBracket::IfNeeded)
                        } else {
                            (h.as_str(), "", UnaryBracket::IfNeeded)
                        };
                        let bracket = match unary_bracket {
                            UnaryBracket::No => false,
                            UnaryBracket::IfNeeded => bracket,
                            UnaryBracket::IfNeededNotNum => {
                                bracket
                                    && (inside != "π" && inside != "e")
                                    && inside.parse::<f64>().is_err()
                            }
                        };
                        hist = prefix;
                        hist.push_str(h1);
                        if bracket {
                            hist.push('(');
                        }
                        hist.push_str(inside.as_str());
                        if bracket {
                            hist.push(')');
                        }
                        hist.push_str(h2);
                    } else {
                        let h = format!("_{}_", h);
                        hist.push_str(h.as_str()); // ²
                    }
                    // if marker.is_some() {
                    //     marker_stack.push(marker.unwrap());
                    // }
                    if pop_marker {
                        marker_stack.pop();
                    }
                } else {
                    //let h = h.trim();
                    if pop_marker {
                        marker_stack.pop();
                    }
                    let h = if better_symbols {
                        match h.as_str() {
                            "+" => "+",
                            "-" => "−",
                            "*" => "×",
                            "/" => "÷",
                            "PI" => "π",
                            "E" => "e",
                            _ => h,
                        }
                    } else {
                        h
                    };
                    if h == "(" {
                        marker_stack.push(Marker::OpenBracket(hist.len()));
                    } else if h == ")" {
                        loop {
                            let marker = marker_stack.last();
                            if marker.is_none() {
                                break;
                            }
                            if let Marker::OpenBracket(start) = marker.unwrap() {
                                break;
                            } else {
                                marker_stack.pop();
                            }
                        }
                        new_pop_marker = true;
                    } else if let Some(c) = h.chars().next() {
                        if c == '.' || (c >= '0' && c <= '9') {
                            new_prev_c = Some(c);
                        }
                        if prev_c.is_none() {
                            let marker = marker_stack.last();
                            if let Some(Marker::NumberStart(_)) = marker {
                                marker_stack.pop();
                            }
                            marker_stack.push(Marker::NumberStart(hist.len()));
                        }
                    }
                    hist.push_str(h);
                    // if pop_marker {
                    //     marker_stack.pop();
                    // }
                }
                //prev_h = Some(h);
                prev_c = new_prev_c;
                pop_marker = new_pop_marker;
            }
            Some(hist)
        } else {
            None
        }
    }
    /// get what to show on the calculator's display
    pub fn get_display(&self) -> String {
        self._get_display(None)
    }
    /// get what to show on the calculator's display, but with a fixed display width
    pub fn get_display_sized(&self, result_width: usize) -> String {
        self._get_display(Some(result_width))
    }
    fn _get_display(&self, result_width: Option<usize>) -> String {
        let mut display_result = self.__get_display(result_width);
        if true {
            // turn 0.0000 to 0
            match &self.entering {
                EnteringMode::Not => {
                    let dr = display_result.replace('-', "");
                    let dr = dr.replace('.', "");
                    let dr = dr.replace('0', "");
                    let dr = dr.trim();
                    if dr.is_empty() {
                        display_result = "0".to_string();
                    }
                }
                _ => {}
            }
        }
        if let Some(result_width) = result_width {
            if result_width == 0 {
                panic!("result_width is zero")
            }
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
                    display_result = "~".repeat(result_width).to_string();
                    //display_result = format!("{}", "~".repeat(result_width))
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
            if display_result.len() > result_width {
                if display_result.contains('.') {
                    let dot_idx = display_result.find('.').unwrap();
                    let places: i32 = result_width as i32 - dot_idx as i32 - 1;
                    if places > 0 {
                        display_result = format!("{:.*}", places as usize, result);
                        match self.entering {
                            // since 2024-04-27
                            EnteringMode::Not => {
                                // remove trailing zeros from display_result
                                display_result = display_result
                                    .chars()
                                    .rev()
                                    .skip_while(|c| *c == '0')
                                    .collect();
                                display_result = display_result.chars().rev().collect();
                                if display_result.ends_with('.') {
                                    display_result.push('0');
                                }
                                if display_result == "-0.0" {
                                    display_result = "0.0".to_string();
                                }
                            }
                            _ => {}
                        }
                        if true {
                            if result != 0.0 && display_result == "0.0" {
                                display_result = "?".repeat(result_width + 1); // just so that will oversize, and will be handled later
                                                                               // let dp = result_width as i32 - 5;
                                                                               // if dp > 0 {
                                                                               //     display_result = format!("{:.*e}", dp as usize, result);
                                                                               //     //println!("{}", display_result)
                                                                               // }
                            }
                        }
                    }
                }
                for i in 0..=2 {
                    if display_result.len() > result_width {
                        let mut places = result_width as i32 - (if result < 0.0 { 5 } else { 4 });
                        if (f64::abs(result) < 1.0) {
                            places -= 1;
                        }
                        places -= i;
                        if places > 0 {
                            display_result = format!("{:.*e}", places as usize, result);
                        }
                    }
                    if display_result.len() <= result_width {
                        break;
                    }
                }
                if display_result.len() > result_width {
                    if (f64::abs(result) < 1.0) {
                        display_result = "0".to_string();
                    }
                }
            }
            if false {
                // disabled since 20240513 ... still needed???
                let is_zero = if display_result.len() == result_width {
                    let dr = display_result.replace('-', "");
                    let dr = dr.replace('.', "");
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
    fn on_memory_key(&mut self, key: &str) -> Result<(), DumbError> {
        let mut ori_memory = self.memory.unwrap_or(0.0);
        if key == "mr" {
            let ori_memory = ori_memory.to_string();
            self._record_undo("?", true);
            self.calc.push(ori_memory.as_str())?;
            return Ok(());
        }
        let result = match self.calc.get_result() {
            CalcResult::Final(r) => r,
            CalcResult::Intermediate(r) => r,
            CalcResult::Error(e) => return Ok(()),
        };
        let new_memory = match key {
            "mc" => 0.0,
            "ms" => result,
            "m+" => ori_memory + result,
            "m-" => ori_memory - result,
            _ => panic!("unknown memory key: {}", key),
        };
        if new_memory == 0.0 {
            self.memory = None;
        } else {
            self.memory = Some(new_memory);
        }
        return Ok(());
    }
}

pub struct DumbCalculatorSettings {
    pub enable_undo: bool,
    pub enable_history: bool,
}
impl Default for DumbCalculatorSettings {
    fn default() -> Self {
        Self {
            enable_undo: true,
            enable_history: true,
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
