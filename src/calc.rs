//! A simple infix calculation processor -- [`crate::calc::DumbCalcProcessor`]

#![deny(warnings)]
#![allow(unused)]

use std::{fmt, num::ParseFloatError};

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
        if check_calc_res != res {
            println!("XXX");
            println!("XXX calc_res(={calc_res:?}) != res(={res}) ... calc={calc:?}");
            println!("XXX");
        }
        assert_eq!(check_calc_res, res);
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
pub fn test_calc_empty() {
    let mut calc = DumbCalcProcessor::new();
    assert_calc_eq_result!(&calc, 0.0);
    calc.parse_and_push("123");
    calc.eval().unwrap();
    assert_eq!(123.0, calc.calc_impl.result);
    calc.eval().unwrap();
    assert_eq!(123.0, calc.calc_impl.result);
    calc.reset();
    println!(". calc={:?}", calc);
    assert_eq!(0.0, calc.calc_impl.result);
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
        test_calc_prase_and_push!("2 inv", 0.5);
        test_calc_prase_and_push!("0 exp", 1.0);
        test_calc_prase_and_push!("1 exp", 1.0_f64.exp());
        test_calc_prase_and_push!("50%", 0.5);
    }
    if true {
        test_calc_prase_and_push!(" 0 cos ", 1.0);
        test_calc_prase_and_push!(" ( 1 - 1 ) cos * ( 2 + 1)", 3.0);
        test_calc_prase_and_push!(" 50% + 50 %", 1.0);
        test_calc_prase_and_push!(" 2 * PI ", 2.0 * std::f64::consts::PI);
        test_calc_prase_and_push!(" 3 * E ", 3.0 * std::f64::consts::E);
    }
}

/// For internal debugging use only.
pub fn debug_calc() {
    if true {
        println!("raw");
        let open: Unit = Unit::OpenBracket;
        let close = Unit::CloseBracket;
        let add = Unit::Operator(Op::ADD);
        let digit = Unit::Operand(888.0);
        println!("{} | {} | {} | {}", open, close, add, digit);
    }

    if true {
        println!("infix");
        let infix = vec![
            Unit::Operand(1.0),
            Unit::Operator(Op::ADD),
            Unit::Operand(2.0),
            Unit::Operator(Op::MULTIPLY),
            Unit::OpenBracket,
            Unit::Operand(3.0),
            Unit::Operator(Op::SUBTRACT),
            Unit::Operand(1.0),
            Unit::CloseBracket,
            Unit::Operator(Op::DIVIDE),
            Unit::Operand(3.0),
        ];
        print_infix(&infix);
    }

    if true {
        let mut calc = CalcImpl::new();

        println!("2 * (3 + 4) - 1");
        calc.push(Unit::Operand(2.0));
        println!(". calc={:?}", calc);
        calc.push(Unit::Operator(Op::MULTIPLY));
        println!(". calc={:?}", calc);
        calc.push(Unit::OpenBracket);
        println!(". calc={:?}", calc);
        calc.push(Unit::Operand(3.0));
        println!(". calc={:?}", calc);
        calc.push(Unit::Operator(Op::ADD));
        println!(". calc={:?}", calc);
        calc.push(Unit::Operand(4.0));
        println!(". calc={:?}", calc);
        calc.push(Unit::CloseBracket);
        println!(". calc={:?}", calc);
        calc.push(Unit::Operator(Op::SUBTRACT));
        println!(". calc={:?}", calc);
        calc.push(Unit::Operand(1.0));
        println!(". calc={:?}", calc);
        calc.eval();
        println!(". calc={:?}", calc);
        println!("= {}", calc.result);
        assert_eq!(13.0, calc.result);

        println!("4 / 2");
        calc.push(Unit::Operand(4.0));
        println!(". calc={:?}", calc);
        calc.push(Unit::Operator(Op::DIVIDE));
        println!(". calc={:?}", calc);
        calc.push(Unit::Operand(2.0));
        println!(". calc={:?}", calc);
        calc.eval();
        println!(". calc={:?}", calc);
        println!("= {}", calc.result);
        assert_eq!(2.0, calc.result);

        println!("+ 5");
        calc.push(Unit::Operator(Op::ADD));
        println!(". calc={:?}", calc);
        calc.push(Unit::Operand(5.0));
        println!(". calc={:?}", calc);
        calc.eval();
        println!(". calc={:?}", calc);
        println!("= {}", calc.result);
        assert_eq!(7.0, calc.result);

        // if true {
        //     println!(")");
        //     calc.push(Unit::CloseBracket);
        //     println!(". calc={:?}", calc);
        //     println!("= {}", calc.result.unwrap());
        //     assert_eq!(7.0, calc.result.unwrap());
        // }

        println!("CLEAR");
        calc.reset();
        println!(". calc={:?}", calc);
        assert_eq!(0.0, calc.result);

        // println!("(+ 5)");

        // println!("3 (1 + 1)) - 2");
        // // if true {
        // //     calc.push(Unit::Operand(5.0));
        // //     calc.push(Unit::Operator(Op::MULTIPLY));
        // // }
        // calc.push(Unit::Operand(3.0));
        // println!(". calc={:?}", calc);
        // calc.push(Unit::OpenBracket);
        // println!(". calc={:?}", calc);
        // calc.push(Unit::Operand(1.0));
        // println!(". calc={:?}", calc);
        // calc.push(Unit::Operator(Op::ADD));
        // println!(". calc={:?}", calc);
        // calc.push(Unit::Operand(1.0));
        // println!(". calc={:?}", calc);
        // calc.push(Unit::CloseBracket);
        // println!(". calc={:?}", calc);
        // calc.push(Unit::CloseBracket);
        // println!(". calc={:?}", calc);
        // calc.push(Unit::Operator(Op::SUBTRACT));
        // println!(". calc={:?}", calc);
        // calc.push(Unit::Operand(2.0));
        // println!(". calc={:?}", calc);
    }
}

/// a simple infix calculation processor that accepts a stream of "calculation units" and evaluate the result;
/// please refer to [`DumbCalcProcessor::push`] for the acceptable "calculation units"
///
/// example:
/// ```
/// use rusty_dumb_tools::calc::DumbCalcProcessor;
/// let mut calc = DumbCalcProcessor::new();
/// calc.push("1.5");  // push a single "calculation unit", like a number or operator
/// calc.eval().unwrap();  // evaluate the pushed "calculation units" and get the result
/// assert_eq!(1.5, calc.get_result().unwrap());
/// calc.parse_and_push("+ 2.5 * 3 - 4"); // based on last calculation result, parse and push additional "calculation units"
/// calc.eval().unwrap();  // evaluate the pushed "calculation units" and get the result
/// assert_eq!(5.0, calc.get_result().unwrap());
/// ```
#[derive(Debug)]
pub struct DumbCalcProcessor {
    calc_impl: CalcImpl,
}
impl DumbCalcProcessor {
    pub fn new() -> DumbCalcProcessor {
        DumbCalcProcessor {
            calc_impl: CalcImpl::new(),
        }
    }
    /// push a "calculation unit":
    /// * a bracket: "(", ")"
    /// * a number: e.g. "0", "1", "2.3", "-4", "-5.67"
    /// * a binary operator: "+", "-", "*", "/"
    ///   <br>note that these binary operators have the usual precedence
    /// * an unary operator: "neg", "sin", "cos", "tan", "asin", "acos", "atan", "log", "ln", "sqrt", "exp", "inv"
    ///   <br>notes:
    ///   - an unary operator should come after the operand that it operates on;
    ///   - these unary operators have the same highest precedence
    /// * a constant: "PI", "E"
    /// * a "=", which will evaluate the pushed "calculation units"
    ///
    /// please use [`DumbCalcProcessor::parse_and_push`] if you want to push multiple "calculation units" in a string, like a string of a complete infix expression
    pub fn push(&mut self, unit: &str) -> Result<(), String> {
        let unit = unit.trim();
        if unit == "=" {
            self.eval();
            return Ok(());
        } else {
            let push_unit = match unit {
                "(" => Unit::OpenBracket,
                ")" => Unit::CloseBracket,
                "+" => Unit::Operator(Op::ADD),
                "-" => Unit::Operator(Op::SUBTRACT),
                "*" => Unit::Operator(Op::MULTIPLY),
                "/" => Unit::Operator(Op::DIVIDE),
                "neg" => Unit::Operator(Op::NEGATE),
                "sin" => Unit::Operator(Op::SIN),
                "cos" => Unit::Operator(Op::COS),
                "tan" => Unit::Operator(Op::TAN),
                "asin" => Unit::Operator(Op::ASIN),
                "acos" => Unit::Operator(Op::ACOS),
                "atan" => Unit::Operator(Op::ATAN),
                "log" => Unit::Operator(Op::LOG),
                "ln" => Unit::Operator(Op::LN),
                "sqrt" => Unit::Operator(Op::SQRT),
                "square" => Unit::Operator(Op::SQUARE),
                "inv" => Unit::Operator(Op::INVERSE),
                "exp" => Unit::Operator(Op::EXP),
                "mod" => Unit::Operator(Op::MOD),
                "abs" => Unit::Operator(Op::ABS),
                "%" => Unit::Operator(Op::PERCENT),
                "PI" => Unit::Operand(std::f64::consts::PI),
                "E" => Unit::Operand(std::f64::consts::E),
                _ => match unit.parse::<f64>() {
                    Ok(operand) => Unit::Operand(operand),
                    Err(_) => {
                        let err_msg = format!("'{}' is not a valid unit", unit);
                        return Err(err_msg);
                    }
                },
            };
            self.calc_impl.push(push_unit);
            Ok(())
        }
    }
    /// parse and push multiple "calculation units" in a string, like a string of a complete infix expression;
    /// each parsed "calculation unit" will be pushed one by one with [`DumbCalcProcessor::push`]
    ///
    /// note: please consider unary operators as ***not parsable***
    pub fn parse_and_push<T: AsRef<str>>(&mut self, units: T) -> Result<(), String> {
        let units = _parse_units_from_str(units.as_ref())?;
        for unit in units {
            self.push(&unit)?;
        }
        Ok(())
    }
    /// evaluate the pushed "calculation units" and get the result;
    /// the result will also be assigned to the internal `result`, which can be used as the "initial" value of the next sequence of "calculation units";
    /// please refer to [`DumbCalcProcessor::get_result`] for the result
    pub fn eval(&mut self) -> Result<f64, String> {
        self.calc_impl.eval();
        match self.get_result() {
            CalcResult::Final(result) => Ok(result),
            CalcResult::Intermediate(result) => panic!("unexpected intermediate result {}", result),
            CalcResult::Error(err_msg) => Err(err_msg),
        }
    }
    /// return the calculation result so far; call [`DumbCalcProcessor::eval`] to evaluate the pushed "calculation units", and assign the result to it (as final result)
    ///
    /// note that the result is a [`CalcResult`] enum, that can be one of three kinds -- final, intermediate, or error
    pub fn get_result(&self) -> CalcResult {
        let result = self.calc_impl.result;
        if result.is_nan() {
            CalcResult::Error("result is NaN".to_string())
        } else if result.is_infinite() {
            CalcResult::Error("result is infinity".to_string())
        } else {
            let scanned = &self.calc_impl.scanned;
            if scanned.len() > 0 {
                let intermediate_result = scanned.last().unwrap();
                CalcResult::Intermediate(*intermediate_result)
            } else {
                CalcResult::Final(result)
            }
        }
    }
    // /// like [`DumbCalcProcessor::get_result`]
    // pub fn get_unwrapped_result(&self) -> f64 {
    //     self.get_result().unwrap()
    // }
    /// reset for new input
    pub fn reset(&mut self) {
        self.calc_impl.reset();
    }
}

fn _parse_units_from_str(units: &str) -> Result<Vec<String>, String> {
    let units: Vec<char> = units.chars().collect();
    _parse_units_from_chars(&units)
}

fn _parse_units_from_chars(units: &Vec<char>) -> Result<Vec<String>, String> {
    let mut parsed_units: Vec<String> = Vec::new();
    let max_idx = units.len();
    let mut idx = 0;
    while idx < max_idx {
        let token = _to_next_unit_token(idx, units);
        match token {
            Some((start_idx, end_idx)) => {
                if start_idx == end_idx {
                    idx += 1;
                    continue;
                }
                let unit: String = units[start_idx..end_idx].iter().collect();
                idx = end_idx;
                parsed_units.push(unit)
            }
            None => {
                return Err("failed to extract token".to_string());
            }
        }
    }
    Ok(parsed_units)
}

fn _to_next_unit_token(mut idx: usize, s: &Vec<char>) -> Option<(usize, usize)> {
    let max_idx = s.len();
    let mut start_idx: i32 = -1;
    let mut end_idx = max_idx;
    while idx < max_idx {
        let c = s[idx];
        if start_idx == -1 {
            if c == '('
                || c == ')'
                || c == '+'
                || c == '-'
                || c == '*'
                || c == '/'
                || c == '%'
                || c == '='
            {
                return Some((idx, idx + 1));
            }
            if c.is_whitespace() {
                idx += 1;
                continue;
            }
            start_idx = idx as i32;
            idx += 1;
            continue;
        }
        if c.is_whitespace()
            || c == '('
            || c == ')'
            || c == '+'
            || c == '-'
            || c == '*'
            || c == '/'
            || c == '%'
            || c == '='
        {
            end_idx = idx;
            break;
        }
        idx += 1;
        continue;
    }
    if start_idx == -1 {
        if idx == end_idx {
            return Some((end_idx, idx));
        }
        return None;
    } else {
        return Some((start_idx as usize, end_idx));
    }
}

#[derive(Debug)]
struct CalcImpl {
    scanned: Vec<f64>,
    stack: Vec<Unit>, // can only be ) or Op
    last_pushed: Option<Unit>,
    result: f64,
}
impl CalcImpl {
    fn new() -> CalcImpl {
        CalcImpl {
            scanned: Vec::new(),
            stack: Vec::new(),
            last_pushed: None,
            result: 0.0,
        }
    }
    fn push(&mut self, push_unit: Unit) {
        //println!("* {:?}", push_unit);
        let last_pushed = self.last_pushed;
        self.last_pushed = Some(push_unit.clone());
        match last_pushed {
            Some(last_pushed_unit) => match last_pushed_unit {
                Unit::Operand(_) => {
                    // if push_unit == Unit::OpenBracket {
                    //     self._push(Unit::Operator(Op::MULTIPLY)); // add a * between if next is an open bracket
                    // }
                    match push_unit {
                        Unit::OpenBracket => {
                            self._push(Unit::Operator(Op::MULTIPLY)); // add a * between if next is an open bracket
                        }
                        Unit::Operand(operand) => {
                            self.scanned.pop(); // consecutive operands => replace the last one
                        }
                        _ => {}
                    }
                }
                Unit::OpenBracket => match push_unit {
                    Unit::Operator(op) => {
                        if op.is_binary() {
                            self._push(Unit::Operand(0.0)); // add a 0 after ( if next is a binary op
                        }
                    }
                    Unit::CloseBracket => {
                        self._push(Unit::Operand(0.0)); // add a 0 after ( if next is a )
                    }
                    _ => {}
                },
                Unit::Operator(last_op) => {
                    if let Unit::Operator(op) = push_unit {
                        if last_op.is_binary() && op.is_binary() {
                            self.stack.pop(); // consecutive binary ops => replace the last one
                        }
                    }
                }
                _ => {}
            },
            None => {}
        }
        self._push(push_unit)
    }
    fn _push(&mut self, push_unit: Unit) {
        match push_unit {
            Unit::OpenBracket => {
                // if the scanned character is a left parenthesis, push it onto the stack
                self.stack.push(push_unit.clone());
            }
            Unit::CloseBracket => {
                // if the scanned character is a right parenthesis, pop operators from the stack and append them to the postfix expression until a left parenthesis is found
                self._push_all_to_scalled(true);
                // while self.stack.len() > 0 {
                //     let stack_unit = self.stack.pop().unwrap();
                //     if stack_unit == Unit::OpenBracket {
                //         break;
                //     }
                //     self._push_to_scanned(&stack_unit);
                // }
            }
            Unit::Operator(op) => {
                let order = op.get_order();
                while self.stack.len() > 0 {
                    let stack_unit = self.stack.last().unwrap();
                    match stack_unit {
                        Unit::Operator(stack_unit_op) => {
                            let stack_unit_order = stack_unit_op.get_order();
                            if stack_unit_order < order {
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                    let unit = self.stack.pop().unwrap();
                    self._push_to_scanned(&unit);
                }
                self.stack.push(push_unit.clone());
            }
            Unit::Operand(operand) => self.scanned.push(operand),
        }
    }
    fn eval(&mut self) {
        self.last_pushed = None;
        self._push_all_to_scalled(false);
        self.result = if self.scanned.len() > 0 {
            if self.scanned.len() != 1 {
                panic!("'scanned' should have a single element ... self={:?}", self);
            }
            self.scanned.pop().unwrap()
        } else {
            self.result
        };
    }
    fn reset(&mut self) {
        self.scanned.clear();
        self.stack.clear();
        self.result = 0.0;
    }
}
impl CalcImpl {
    fn _push_all_to_scalled(&mut self, until_open_bracket: bool) {
        while self.stack.len() > 0 {
            let stack_unit = self.stack.pop().unwrap();
            if until_open_bracket && stack_unit == Unit::OpenBracket {
                break;
            }
            self._push_to_scanned(&stack_unit);
        }
    }
    fn _push_to_scanned(&mut self, unit: &Unit) {
        match unit {
            Unit::Operator(op) => {
                let result = if op.is_unary() {
                    let operand = self.scanned.pop().unwrap();
                    op.evaluate_unary(operand)
                } else {
                    let right = match self.scanned.pop() {
                        Some(r) => r,
                        None => return,
                    };
                    let left = match self.scanned.pop() {
                        Some(l) => l,
                        None => self.result,
                    };
                    op.evaluate_binary(left, right)
                };
                self.scanned.push(result)
            }
            Unit::OpenBracket => {} // if it an open (, ignore it
            _ => panic!("unexpected unit {:?} ... self={:?}", unit, self),
        }
    }
}

/// calculation result, which can be one of three kinds
/// * final: final calculation result
/// * intermediate: intermediate result during "calculation units" being pushed
/// * error: error like calculating 1 / 0
#[derive(Debug)]
pub enum CalcResult {
    Final(f64),
    Intermediate(f64),
    Error(String),
}
impl CalcResult {
    pub fn unwrap(&self) -> f64 {
        match *self {
            CalcResult::Final(result) => result,
            CalcResult::Intermediate(result) => result,
            CalcResult::Error(ref err_msg) => panic!("Error: {}", err_msg),
        }
    }
    pub fn is_final(&self) -> bool {
        match *self {
            CalcResult::Final(_) => true,
            _ => false,
        }
    }
    pub fn is_intermediate(&self) -> bool {
        match *self {
            CalcResult::Intermediate(_) => true,
            _ => false,
        }
    }
    pub fn is_ok(&self) -> bool {
        match *self {
            CalcResult::Error(_) => false,
            _ => true,
        }
    }
    pub fn is_err(&self) -> bool {
        match *self {
            CalcResult::Error(_) => true,
            _ => false,
        }
    }
}
impl fmt::Display for CalcResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CalcResult::Final(result) => write!(f, "{}.", result),
            CalcResult::Intermediate(result) => write!(f, "{}", result),
            CalcResult::Error(ref err_msg) => write!(f, "Error: {}", err_msg),
        }
    }
}

#[allow(non_camel_case_types)]
enum OpPriority {
    BINARY_AM = 1,
    BINARY_MD = 2,
    UNARY = 3,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Op {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NEGATE,
    SIN,
    COS,
    TAN,
    ASIN,
    ACOS,
    ATAN,
    LOG,
    LN,
    SQRT,
    SQUARE,
    INVERSE,
    EXP,
    MOD,
    ABS,
    PERCENT,
}
impl Op {
    fn get_priority(&self) -> OpPriority {
        match self {
            Op::ADD | Op::SUBTRACT => OpPriority::BINARY_AM,
            Op::MULTIPLY | Op::DIVIDE => OpPriority::BINARY_MD,
            Op::NEGATE
            | Op::SIN
            | Op::COS
            | Op::TAN
            | Op::ASIN
            | Op::ACOS
            | Op::ATAN
            | Op::LOG
            | Op::LN
            | Op::SQRT
            | Op::SQUARE
            | Op::INVERSE
            | Op::EXP
            | Op::MOD
            | Op::ABS
            | Op::PERCENT => OpPriority::UNARY,
        }
    }
    fn get_order(&self) -> u8 {
        self.get_priority() as u8
    }
    fn evaluate_binary(&self, left: f64, right: f64) -> f64 {
        match *self {
            Op::ADD => left + right,
            Op::SUBTRACT => left - right,
            Op::MULTIPLY => left * right,
            Op::DIVIDE => left / right,
            _ => panic!("{:?} non-binary operator", self),
        }
    }
    fn is_unary(&self) -> bool {
        return *self == Op::NEGATE
            || *self == Op::COS
            || *self == Op::SIN
            || *self == Op::TAN
            || *self == Op::ASIN
            || *self == Op::ACOS
            || *self == Op::ATAN
            || *self == Op::LOG
            || *self == Op::LN
            || *self == Op::SQRT
            || *self == Op::SQUARE
            || *self == Op::INVERSE
            || *self == Op::EXP
            || *self == Op::MOD
            || *self == Op::ABS
            || *self == Op::PERCENT;
    }
    fn is_binary(&self) -> bool {
        return !self.is_unary();
    }
    fn evaluate_unary(&self, operand: f64) -> f64 {
        match *self {
            Op::NEGATE => -operand,
            Op::SIN => operand.sin(),
            Op::COS => operand.cos(),
            Op::TAN => operand.tan(),
            Op::ASIN => operand.asin(),
            Op::ACOS => operand.acos(),
            Op::ATAN => operand.atan(),
            Op::LOG => operand.log10(),
            Op::LN => operand.ln(),
            Op::SQRT => operand.sqrt(),
            Op::SQUARE => operand * operand,
            Op::INVERSE => 1.0 / operand,
            Op::EXP => operand.exp(),
            Op::MOD => operand % 1.0,
            Op::ABS => operand.abs(),
            Op::PERCENT => operand / 100.0,
            _ => panic!("{:?} non-unary operator", self),
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Op::ADD => write!(f, "+"),
            Op::SUBTRACT => write!(f, "-"),
            Op::MULTIPLY => write!(f, "*"),
            Op::DIVIDE => write!(f, "/"),
            Op::NEGATE => write!(f, "neg"),
            Op::SIN => write!(f, "sin"),
            Op::COS => write!(f, "cos"),
            Op::TAN => write!(f, "tan"),
            Op::ASIN => write!(f, "asin"),
            Op::ACOS => write!(f, "acos"),
            Op::ATAN => write!(f, "atan"),
            Op::LOG => write!(f, "log"),
            Op::LN => write!(f, "ln"),
            Op::SQRT => write!(f, "sqrt"),
            Op::SQUARE => write!(f, "square"),
            Op::INVERSE => write!(f, "inv"),
            Op::EXP => write!(f, "exp"),
            Op::MOD => write!(f, "mod"),
            Op::ABS => write!(f, "abs"),
            Op::PERCENT => write!(f, "%"),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Unit {
    OpenBracket,
    CloseBracket,
    Operand(f64),
    Operator(Op),
}
// impl Unit {
//     fn operator(op: Op) -> Unit {
//         Unit::Operator(op)
//     }
//     fn operand(operand: f64) -> Unit {
//         Unit::Operand(operand)
//     }
// }
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Unit::OpenBracket => write!(f, "("),
            Unit::CloseBracket => write!(f, ")"),
            Unit::Operand(operand) => write!(f, "{}", operand),
            Unit::Operator(operator) => write!(f, "{}", operator),
        }
    }
}

fn print_infix(infix: &Vec<Unit>) {
    let len = infix.len();
    print!("[");
    for i in 0..len {
        if i > 0 {
            print!(", ");
        }
        let unit = infix[i];
        print!("{}", unit);
    }
    println!("]");
}
