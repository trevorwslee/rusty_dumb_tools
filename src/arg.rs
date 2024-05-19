//! A simple argument parser -- [`crate::arg::DumbArgParser`]

#![deny(warnings)]
#![allow(unused)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::inherent_to_string_shadow_display)]
#![allow(clippy::manual_strip)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::new_without_default)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::redundant_field_names)]

use core::panic;
use std::{
    collections::{btree_map::Values, HashMap},
    env,
    error::Error,
    fmt::{self, Formatter},
    i32,
    num::ParseIntError,
    path::Path,
};

use crate::{arg, shared::DumbError};

/// ***please consider using the macro [`crate::dap_arg!`] instead, since this macro will be deprecated***
///
/// use this macro to create a [`DumbArgBuilder`] instance to build argument object (argument specification) to be added to [`DumbArgParser`] with [`DumbArgBuilder::add_to`]
/// the macro accepts one for more strings (positional argument name or flags) like
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let mut parser = DumbArgParser::new();
/// sap_arg!("-f", "--flag").add_to(&mut parser); // e.g. ... -f flag-value ...
/// sap_arg!("-v", "--verbose").fixed(true).add_to(&mut parser); // e.g. ... -v ... -- this will turn "on" -v with value true
/// sap_arg!("position1").add_to(&mut parser); // e.g. ... -f flag-value positional-1-value
/// sap_arg!("position2").default("def").add_to(&mut parser); // e.g. ... -f flag-value positional-1-value positional-2-value
/// assert_eq!("<program> [-h] -f flag [-v] <position1> [position2]", parser.compose_usage());
/// ```
/// also see [`DumbArgParser`]
#[macro_export]
macro_rules! sap_arg {
  ($($x:expr),*) => {
      {
          let mut name_or_flags = Vec::new();
          $(name_or_flags.push($x.to_string());)*
          DumbArgBuilder::new(name_or_flags)
      }
  };
}

/// use this macro to create a [`DumbArgBuilder`] instance to build argument object (argument specification) to be added to [`DumbArgParser`] with [`DumbArgBuilder::add_to`]
/// the macro accepts one for more strings (positional argument name or flags) like
/// ```
/// use rusty_dumb_tools::prelude::*;
/// let mut parser = DumbArgParser::new();
/// dap_arg!("-f", flag2="--flag").add_to(&mut parser); // e.g. ... -f flag-value ...
/// dap_arg!("-v", flag2="--verbose", fixed=true).add_to(&mut parser); // e.g. ... -v ... -- this will turn "on" -v with value true
/// dap_arg!("position1").add_to(&mut parser); // e.g. ... -f flag-value positional-1-value
/// dap_arg!("position2", default="def").add_to(&mut parser); // e.g. ... -f flag-value positional-1-value positional-2-value
/// assert_eq!("<program> [-h] -f flag [-v] <position1> [position2]", parser.compose_usage());
/// ```
///
/// the compulsory argument is the name of the argument, be it a positional argument or a flag argument like "-v";
/// the other optional "ordered but named" arguments are:
/// - `flag2` - the the second (alias) flag name, like "--verbose" in the above example
/// - `value` - like calling [`DumbArgBuilder::value`]
/// - `default` - like calling [`DumbArgBuilder::default`]
/// - `fixed` - like calling [`DumbArgBuilder::fixed`]
///
/// note that after it parsing the arguments, you retrieve the argument value with [`DumbArgParser::get`] by providing the argument name (or flag name like "-v" / "-verbose")
#[macro_export]
macro_rules! dap_arg {
    // for case like "name, flag2=flag2, value=value, default=default, fixed=fixed"
    ($name:expr
        $(, flag2=$flag2:expr)?
        $(, value=$value:expr)?
        $(, default=$default:expr)?
        $(, fixed=$fixed:expr)?) => {
          {
              let mut name_or_flags = Vec::new();
              name_or_flags.push($name.to_string());
              $(name_or_flags.push($flag2.to_string());)?
              let mut builder = DumbArgBuilder::new(name_or_flags);
              $(builder.value($value);)?
              $(builder.default($default);)?
              $(builder.fixed($fixed);)?
              builder
          }
      };
    // for case like "-v", "--verbose"
    ($name:expr, $name2:expr) => {
        {
            let mut name_or_flags = Vec::new();
            name_or_flags.push($name.to_string());
            name_or_flags.push($name2.to_string());
            DumbArgBuilder::new(name_or_flags)
        }
    };
    // for case like "pos"
    ($name:expr) => {{
        let mut name_or_flags = Vec::new();
        name_or_flags.push($name.to_string());
        DumbArgBuilder::new(name_or_flags)
    }};
    // ($($x:expr),*) => {
    //     {
    //         let mut name_or_flags = Vec::new();
    //         $(name_or_flags.push($x.to_string());)*
    //         DumbArgBuilder::new(name_or_flags)
    //     }
    // };
  }

#[test]
fn test_arg() {}

/// for use by [`DumbArgParser`] internally for debugging.
#[test]
fn debug_arg() {
    let mut parser = DumbArgParser::new();
    parser.set_description("This is a simple argument parser.");
    println!("description: {:?}", parser.description);

    dap_arg!("-d", flag2 = "--def", default = "default").add_to(&mut parser);
    dap_arg!("-v", flag2 = "--verbose", fixed = false).add_to(&mut parser);
    dap_arg!("i32", value = 0).add_to(&mut parser);
    dap_arg!("string", value = "string").add_to(&mut parser);
    dap_arg!("bool", value = false).add_to(&mut parser);
    println!("parser: {:?}", parser);

    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^");

    let in_args = vec!["-v", "123", "string", "true"];
    parser.process_args(in_args);

    println!("==========================");
    println!(". -d: {:?}", parser.get::<String>("-d"));
    //println!(". -d: {:?}", parser.get::<&str>("-d"));
    println!(". -d: {:?}", parser.get::<String>("-d"));
    //println!(". -d: {:?}", parser.get::<&str>("-d"));
    println!(". string: {:?}", parser.get::<String>("string"));
    //println!(". string: {:?}", parser.get::<&str>("string"));
    println!(". i32: {:?}", parser.get::<i32>("i32"));
    println!(". i32 as string: {:?}", parser.get::<String>("i32"));
    println!(". -d as string: {:?}", parser.get::<String>("-d"));
}

#[test]
fn debug_arg_sap() {
    let mut parser = DumbArgParser::new();
    parser.set_description("This is a simple argument parser.");
    println!("description: {:?}", parser.description);

    sap_arg!("-d", "--def")
        .default("default")
        .add_to(&mut parser);
    sap_arg!("-v", "--verbose").fixed(false).add_to(&mut parser);
    sap_arg!("i32").value(0).add_to(&mut parser);
    sap_arg!("string")
        .value("string".to_owned())
        .add_to(&mut parser);
    sap_arg!("bool").value(false).add_to(&mut parser);
    println!("parser: {:?}", parser);

    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^");

    let in_args = vec!["-v", "123", "string", "true"];
    parser.process_args(in_args);

    println!("==========================");
    println!(". -d: {:?}", parser.get::<String>("-d"));
    //println!(". -d: {:?}", parser.get::<&str>("-d"));
    println!(". -d: {:?}", parser.get::<String>("-d"));
    //println!(". -d: {:?}", parser.get::<&str>("-d"));
    println!(". string: {:?}", parser.get::<String>("string"));
    //println!(". string: {:?}", parser.get::<&str>("string"));
    println!(". i32: {:?}", parser.get::<i32>("i32"));
    println!(". i32 as string: {:?}", parser.get::<String>("i32"));
    println!(". -d as string: {:?}", parser.get::<String>("-d"));
}

/// a simple argument parser.
///
/// example usage:
/// ```
/// use rusty_dumb_tools::{arg::{DumbArgParser, DumbArgBuilder}, dap_arg};
/// let mut parser = DumbArgParser::new();
/// parser.set_description("This is a simple argument parser.");
/// parser.set_allow_missing_arguments();  // normal should not do this
/// dap_arg!("-v", flag2="--verbose", fixed=true).add_to(&mut parser);  // argument flag "-v" / "--verbose" with fixed value (true) when the flag is present
/// dap_arg!("-n", flag2="--name", default="nobody").add_to(&mut parser);  // argument "-n" / "--name" requiring input value, with default "nobody"
/// dap_arg!("str-arg").add_to(&mut parser);  // positional argument "str-arg" (of type String)
/// dap_arg!("i32-arg", value=123).add_to(&mut parser);  // positional argument "i32-arg" of type i32 (inferred from the value 123)
/// dap_arg!("multi-arg").set_multi().add_to(&mut parser);  // positional multi-argument "multi-arg" that will accept multiple values (one + rest)
/// parser.parse_args();  // parse from command-line arguments
/// println!(". -v: {:?}", parser.get::<bool>("-v"));
/// println!(". --verbose: {:?}", parser.get::<bool>("--verbose"));  // will be the same parameter value as "-v"
/// println!(". --name: {:?}", parser.get::<String>("--name"));  // can use "-n" as well
/// println!(". str-arg: {:?}", parser.get::<String>("str-arg"));
/// println!(". i32-arg: {:?}", parser.get::<i32>("i32-arg"));
/// println!(". multi-arg: {:?}", parser.get_multi::<String>("multi-arg"));
/// ```
/// notes:
/// * -h and --help are reserved for showing help message; after showing the help message, the program will exit
/// * in case of invalid input argument, will show the error message as well as the help message, then the program will exit
/// * arguments are typed; the default is [`String`]; others are [`std::i32`], [`std::i64`], [`std::f32`], [`std::f64`] and [`bool`]
/// * also see the macro [`dap_arg`]
///
/// you may want to refer to [`crate::demo::run_demo`] for a demo program that uses [`DumbArgParser`].
#[derive(Debug)]
pub struct DumbArgParser {
    args: Vec<Arg>,
    input_arg_values: Vec<Option<ArgValue>>,
    input_arg_index_map: HashMap<String, usize>,
    input_multi_arg_data: Option<(Vec<String>, Vec<ArgValue>)>,
    input_rest_arg_data: Option<(Vec<String>, Vec<String>)>,
    program_name: Option<String>,
    description: Option<String>,
    allow_none: bool,
}
impl DumbArgParser {
    /// create and instance of [DumbArgParser]; program name will be extracted from [`env::args`] when [`DumbArgParser::parse_args`] is called
    /// use [`DumbArgParser::new_with_name`] if want to create an instance with a specific program name
    pub fn new() -> DumbArgParser {
        Self::_new(None)
    }
    /// create an instance of [DumbArgParser] with the specific program name to be shown in the help message
    pub fn new_with_name(program_name: &str) -> DumbArgParser {
        Self::_new(Some(program_name.to_string()))
    }
    /// create an instance of [DumbArgParser] with the specific settings [`DumbArgParserSettings`]
    fn new_ex(settings: ArgParserSettings) -> DumbArgParser {
        // not yet used, since not very useful
        DumbArgParser {
            args: Vec::new(),
            input_arg_values: Vec::new(),
            input_arg_index_map: HashMap::new(),
            input_multi_arg_data: None,
            input_rest_arg_data: None,
            program_name: settings.program_name,
            description: settings.description,
            allow_none: settings.allow_none,
        }
    }
    fn _new(program_name: Option<String>) -> DumbArgParser {
        DumbArgParser {
            args: Vec::new(),
            input_arg_values: Vec::new(),
            input_arg_index_map: HashMap::new(),
            input_multi_arg_data: None,
            input_rest_arg_data: None,
            program_name: program_name,
            description: None,
            allow_none: false,
        }
    }
    /// set the program description to be shown in the help message
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }
    /// in case of missing arguments, [`DumbArgParser::get`] will return [`None`] instead of exiting the program
    pub fn set_allow_missing_arguments(&mut self) {
        self.allow_none = true;
    }
    fn add_arg(&mut self, arg: Arg) {
        self.args.push(arg);
    }
    /// compose the "usage" part of the help message, mostly for illustration use
    pub fn compose_usage(&self) -> String {
        let (flag_args, position_args) = self._split_args();
        self._compose_usage(&flag_args, &position_args)
    }
    fn _split_args(&self) -> (Vec<Arg>, Vec<Arg>) {
        let mut position_args: Vec<Arg> = Vec::new();
        let mut flag_args: Vec<Arg> = Vec::new();
        for arg in self.args.iter() {
            match &arg.key {
                ArgKey::Name(name) => {
                    position_args.push(arg.clone());
                }
                ArgKey::Flags(_, flags) => {
                    flag_args.push(arg.clone());
                }
            }
        }
        (flag_args, position_args)
    }
    /// compose the equivalent argument inputs after parsing, mostly for illustration use
    pub fn compose_inputs(&self) -> String {
        let mut parameters = String::new();
        for arg in self.args.iter() {
            let (key, flag) = match &arg.key {
                ArgKey::Name(name) => (name, None),
                ArgKey::Flags(_, flags) => (&flags[0], Some(&flags[0])),
            };
            let value = match self.input_arg_index_map.get(key) {
                Some(idx) => self.input_arg_values[*idx].as_ref(),
                None => None,
            };
            if value.is_none() {
                continue;
            }
            if !parameters.is_empty() {
                parameters.push(' ');
            }
            if let ArgNature::Fixed = arg.nature {
                parameters.push_str(flag.unwrap());
            } else {
                let value = value.unwrap();
                if let Some(flag) = flag {
                    parameters.push_str(flag);
                    parameters.push(' ');
                }
                parameters.push_str(value.to_string().as_str());
            }
        }
        parameters
    }
    /// get the parsed -- [`DumbArgParser::parse_args`] -- argument value (parameter) assigned to the given argument name
    /// * `arg_name` - the argument name of which the value is to be retrieved; it can be a positional argument name,
    ///                or can be a flag argument name (including `flag2`, which is just an alias to the flag argument)
    ///
    /// e.g.
    /// ```_no_run
    /// let param: i32 = parser.get("i32-arg").unwrap();
    /// ```
    /// or
    /// ```_no_run
    /// let param = parser.get::<i32>("i32-arg").unwrap();
    /// ```
    /// note: except that all types can be implicitly converted to [`String`], no other implicit type conversion; if type does not agree, will panic
    pub fn get<T: ArgValueTrait>(&self, arg_name: &str) -> Option<T> {
        return self._get(arg_name, None);
    }
    /// like [`DumbArgParser::get`] but returns a default value if the argument value is not supplied;
    /// note that if the argument is specified to have default (set with [`DumbArgBuilder::default()`]), that default will be used instead of the one provided here
    pub fn get_or_default<T: ArgValueTrait>(&self, arg_name: &str, default: T) -> T {
        return self._get(arg_name, Some(default)).unwrap();
    }
    fn _get<T: ArgValueTrait>(&self, arg_name: &str, default: Option<T>) -> Option<T> {
        let arg_idx = match self.input_arg_index_map.get(arg_name) {
            Some(arg_idx) => arg_idx,
            None => return default,
        };
        let arg_value = match &self.input_arg_values[*arg_idx] {
            Some(arg_value) => arg_value,
            None => return default,
        };
        match T::from_arg_value(arg_value.clone()) {
            Ok(value) => Some(*value),
            Err(err) => panic!("{}", err),
        }
    }
    /// like [`DumbArgParser::get`] but returns a [`String`]
    pub fn get_string(&self, arg_name: &str) -> Option<String> {
        self.get::<String>(arg_name)
    }
    /// get the parsed -- [`DumbArgParser::parse_args`] -- multi-argument values -- see [`DumbArgBuilder::set_multi`] -- associated with the given argument name
    /// * `arg_name` - the argument name of which the values are to be retrieved
    ///
    /// note: like [`DumbArgParser::get`], except when target type is [`String`], no implicit type conversion
    pub fn get_multi<T: ArgValueTrait>(&self, arg_name: &str) -> Option<Vec<T>> {
        match &self.input_multi_arg_data {
            Some(input_multi_arg_data) => {
                let names = &input_multi_arg_data.0;
                let arg_values = &input_multi_arg_data.1;
                if names.contains(&arg_name.to_string()) {
                    let mut values = Vec::new();
                    for arg_value in arg_values.iter() {
                        let value = match T::from_arg_value(arg_value.clone()) {
                            Ok(value) => *value,
                            Err(err) => panic!("{}", err),
                        };
                        values.push(value);
                    }
                    Some(values)
                } else {
                    None
                }
            }
            None => None,
        }
    }
    /// like [`DumbArgParser::get_multi`] but returns a [`Vec`] of [`String`]
    pub fn get_multi_strings(&self, arg_name: &str) -> Option<Vec<String>> {
        self.get_multi::<String>(arg_name)
    }
    /// get the parsed -- [`DumbArgParser::parse_args`] -- "rest" multi-argument values -- see [`DumbArgBuilder::set_rest`] -- associated with the given argument name
    /// * `arg_name` - the argument name of which the values are to be retrieved
    pub fn get_rest(&self, arg_name: &str) -> Option<Vec<String>> {
        match &self.input_rest_arg_data {
            Some(input_rest_arg_data) => {
                let in_names = &input_rest_arg_data.0;
                let in_values = &input_rest_arg_data.1;
                if in_names.contains(&arg_name.to_string()) {
                    let mut values = Vec::new();
                    for in_value in in_values.iter() {
                        values.push(in_value.clone());
                    }
                    Some(values)
                } else {
                    None
                }
            }
            None => None,
        }
    }
    /// IMPORTANT: assume [`DumbArgParser::get_rest`] is able to retrieve the "rest" multi-argument values
    pub fn process_rest_args(&self, arg_name: &str, parser: &mut DumbArgParser) {
        let arg_idx = match self.input_arg_index_map.get(arg_name) {
            Some(arg_idx) => arg_idx,
            None => panic!("no argument [{}] found", arg_name),
        };
        let arg = &self.args[*arg_idx];
        let arg_value = self.input_arg_values[*arg_idx].clone().unwrap();
        let arg_value = arg_value.to_string();
        let program_name = self._get_program_name();
        let sub_program_name = match arg.key {
            ArgKey::Name(ref name) => arg_value.clone(),
            ArgKey::Flags(_, ref flags) => format!("{} {}", flags[0].clone(), arg_value),
        };
        parser.program_name = Some(format!("{} {}", program_name, sub_program_name));
        let check_parse_result = self.check_process_rest_args(arg_name, parser, true);
        parser._exit_program_after_check_parse(check_parse_result);
    }
    /// IMPORTANT: assume [`DumbArgParser::get_rest`] is able to retrieve the "rest" multi-argument values
    pub fn check_process_rest_args(
        &self,
        arg_name: &str,
        parser: &mut DumbArgParser,
        show_help_if_needed: bool,
    ) -> Result<bool, DumbError> {
        let rest = self.get_rest(arg_name);
        if rest.is_none() {
            panic!("no \"rest\" multi-argument for [{}] found", arg_name);
        }
        let rest = rest.unwrap();
        let rest = rest.iter().map(|s| s.as_str()).collect();
        parser.check_process_args(rest, show_help_if_needed)
    }
    /// parse from the input program arguments -- [`env::args`] -- for argument values (parameters);
    /// after parsing, the argument values can be retrieved by [`DumbArgParser::get`]
    pub fn parse_args(&mut self) {
        let in_args = self._prepare_in_args_from_os();
        let in_args = in_args.iter().map(|s| s.as_str()).collect();
        self.process_args(in_args);
    }
    /// like [`DumbArgParser::parse_args`] but returns a [`Result`] instead of exiting the program
    pub fn check_parse_args(&mut self, show_help_if_needed: bool) -> Result<bool, DumbError> {
        let in_args = self._prepare_in_args_from_os();
        let in_args = in_args.iter().map(|s| s.as_str()).collect();
        self.check_process_args(in_args, show_help_if_needed)
    }
    pub fn show_help(&self, err_msg: Option<String>, and_exit: bool) {
        let (flag_args, position_args) = self._split_args();
        self._show_help(&flag_args, &position_args, &err_msg);
        if (and_exit) {
            let check_parse_result = match err_msg {
                //Some(err) => Err(err.clone()),
                Some(err) => Err(err.into()),
                None => Ok(false),
            };
            self._exit_program_after_check_parse(check_parse_result);
        }
    }
    fn _prepare_in_args_from_os(&mut self) -> Vec<String> {
        let os_args: Vec<String> = env::args().collect();
        let program_path = &os_args[0];
        let program_name = DumbArgParser::_extract_program_name(program_path);
        let in_args = os_args[1..].to_vec();
        //let in_args = in_args.iter().map(|s| s.as_str()).collect();
        if self.program_name.is_none() {
            self.program_name = Some(program_name.clone());
        }
        in_args
    }
    fn _extract_program_name(program_path: &str) -> String {
        let path = Path::new(program_path);
        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        file_stem.to_string()
    }
    /// like [`DumbArgParser::parse_args`] but the input arguments to parse are provided explicitly
    pub fn process_args(&mut self, in_args: Vec<&str>) {
        let check_parse_result = self.check_process_args(in_args, true);
        self._exit_program_after_check_parse(check_parse_result);
    }
    /// like [`DumbArgParser::process_args`] but returns a [`Result`] instead of exiting the program
    pub fn check_process_args(
        &mut self,
        in_args: Vec<&str>,
        show_help_if_needed: bool,
    ) -> Result<bool, DumbError> {
        self.input_arg_values.clear();
        self.input_arg_index_map.clear();
        self.input_multi_arg_data = None;
        for i in 0..self.args.len() {
            self.input_arg_values.push(None);
        }
        let (flag_args, position_args) = self._split_args();
        for arg_idx in 0..self.args.len() {
            let arg = &self.args[arg_idx];
            let rest_arg_values = if arg.multi_mode != ArgMultiMode::None {
                if (arg.nature == ArgNature::Fixed) {
                    panic!("multi-argument [{}] cannot be fixed", arg.key.get_a_name());
                }
                Some(vec![])
            } else {
                None
            };
            if arg.nature == ArgNature::Optional {
                self._set_arg_value(arg_idx, arg.value.clone(), rest_arg_values)
                    .unwrap();
            }
        }
        let (need_help, err_msg) = match self._scan_args(in_args) {
            Ok((show_help, err_msg)) => (show_help, err_msg),
            Err(err_msg) => (false, Some(err_msg.to_string())),
        };
        if need_help || err_msg.is_some() {
            if show_help_if_needed {
                self._show_help(&flag_args, &position_args, &err_msg);
            }
            if let Some(err_msg) = err_msg {
                Err(err_msg.into())
            } else {
                Ok(false)
            }
        } else {
            Ok(true)
        }
    }
    fn _exit_program_after_check_parse(&self, check_parse_result: Result<bool, DumbError>) {
        match check_parse_result {
            Ok(ok) => {
                if ok {
                    //return;
                } else {
                    println!();
                    println!("~~~ [{}] EXITED normally ~~~", self._get_program_name());
                    println!();
                    std::process::exit(0);
                }
            }
            Err(err) => {
                println!();
                println!(
                    "~~~ [{}] EXITED with error \"{}\" ~~~",
                    self._get_program_name(),
                    err
                );
                println!();
                std::process::exit(-1);
            }
        }
    }
    fn _scan_args(&mut self, in_args: Vec<&str>) -> Result<(bool, Option<String>), Box<dyn Error>> {
        let mut err_msg: Option<String> = None;
        let mut pos_idx: usize = 0;
        let in_args_len = in_args.len(); // if nothing provided, show help
        let mut in_arg_idx: usize = 0;
        let mut need_help = false; //in_args_len == 0;
        loop {
            if in_arg_idx >= in_args_len {
                break;
            }
            let in_arg = in_args[in_arg_idx];
            in_arg_idx += 1;
            //println!("- {}", in_arg);
            if in_arg == "-h" || in_arg == "--help" {
                need_help = true;
                break;
            }
            let (arg_idx, arg, arg_value) = if in_arg.starts_with('-') {
                let arg_idx = DumbArgParser::_scan_arg_index(&self.args, in_arg, -1);
                if arg_idx.is_none() {
                    err_msg = Some(format!("unknown input argument [{}]", in_arg));
                    //return Err(format!("unknown argument [{}]", in_arg));
                    break;
                }
                let arg_idx = arg_idx.unwrap();
                let arg = &self.args[arg_idx];
                let arg_value;
                if arg.nature == ArgNature::Fixed {
                    arg_value = Ok(arg.value.clone());
                } else {
                    if in_arg_idx >= in_args_len {
                        err_msg = Some(format!("missing input argument after [{}]", in_arg));
                        //return Err(format!("unknown argument [{}]", in_arg));
                        break;
                    }
                    arg_value = arg.convert_in(in_args[in_arg_idx]);
                    in_arg_idx += 1;
                };
                (arg_idx, arg, arg_value)
            } else {
                let arg_idx = DumbArgParser::_scan_arg_index(&self.args, in_arg, pos_idx as i32);
                if arg_idx.is_none() {
                    err_msg = Some(format!("unacceptable input argument [{}]", in_arg));
                    //return Err(format!("unacceptable argument [{}]", in_arg));
                    break;
                }
                let arg_idx = arg_idx.unwrap();
                let arg = &self.args[arg_idx];
                let arg_value = arg.convert_in(in_arg);
                pos_idx += 1;
                (arg_idx, arg, arg_value)
            };
            match arg_value {
                Ok(arg_value) => {
                    let mut in_rest_args = Vec::new();
                    if arg.multi_mode == ArgMultiMode::Regular
                        || arg.multi_mode == ArgMultiMode::Rest
                    {
                        loop {
                            if in_arg_idx >= in_args_len {
                                break;
                            }
                            let in_rest_arg = in_args[in_arg_idx];
                            // let rest_arg_value = match arg.from_value(in_rest_arg) {
                            //     Ok(rest_arg_value) => rest_arg_value,
                            //     Err(err) => {
                            //         err_msg = Some(err);
                            //         break;
                            //     }
                            // };
                            in_rest_args.push(in_rest_arg.to_string());
                            in_arg_idx += 1;
                        }
                    }
                    self._set_arg_value(arg_idx, arg_value, Some(in_rest_args))?;
                }
                Err(err) => {
                    err_msg = Some(err);
                    break;
                }
            }
        }
        if !need_help && err_msg.is_none() {
            for (index, arg) in self.args.iter().enumerate() {
                if arg.nature == ArgNature::Fixed {
                    continue;
                }
                let value = &self.input_arg_values[index];
                if value.is_none() {
                    if !self.allow_none
                    /* && arg.multi_mode == ArgMultiMode::None*/
                    {
                        let msg = format!("argument [{}] not provided", arg.key.get_a_name());
                        err_msg = Some(msg);
                        break;
                    }
                }
            }
        }
        Ok((need_help, err_msg))
    }
    fn _scan_arg_index(args: &[Arg], flag: &str, pos_idx: i32) -> Option<usize> {
        let mut arg_pos_idx: usize = 0;
        for (arg_idx, arg) in args.iter().enumerate() {
            match &arg.key {
                ArgKey::Name(_) => {
                    if pos_idx >= 0 && arg_pos_idx == pos_idx as usize {
                        return Some(arg_idx);
                    }
                    arg_pos_idx += 1;
                }
                ArgKey::Flags(_, flags) => {
                    for f in flags.iter() {
                        if f == flag {
                            return Some(arg_idx);
                        }
                    }
                }
            }
        }
        None
    }
    fn _set_arg_value(
        &mut self,
        arg_idx: usize,
        arg_value: ArgValue,
        in_rest_args: Option<Vec<String>>,
    ) -> Result<(), String> {
        let arg: &Arg = &self.args[arg_idx];
        Self::_verify_arg_range(arg, &arg_value)?;
        // match &arg.range {
        //     ArgRange::Enum(enum_values) => {
        //         let mut found = false;
        //         for enum_value in enum_values.iter() {
        //             if enum_value.compare(&arg_value) == 0 {
        //                 found = true;
        //                 break;
        //             }
        //         }
        //         if !found {
        //             let mut values = String::new();
        //             for enum_value in enum_values.iter() {
        //                 let value = enum_value.to_string();
        //                 if values.len() > 0 {
        //                     values.push_str(", ");
        //                 }
        //                 values.push_str(value.as_str());
        //             }
        //             return Err(format!(
        //                 "[{}] doesn't match any of the enum values [{}]",
        //                 arg_value.to_string(),
        //                 values
        //             ));
        //         }
        //     }
        //     ArgRange::Range(min, max) => {
        //         if arg_value.compare(min) < 0 || arg_value.compare(max) > 0 {
        //             return Err(format!(
        //                 "[{}] is out of range [{}, {}]",
        //                 arg_value.to_string(),
        //                 min,
        //                 max
        //             ));
        //         }
        //     }
        //     ArgRange::None => {}
        // }
        self.input_arg_values[arg_idx] = Some(arg_value.clone());
        let (multi_arg_values, rest_arg_values) = if let Some(in_rest_args) = in_rest_args {
            //let in_rest_args = in_rest_args.unwrap();
            if arg.multi_mode == ArgMultiMode::Regular {
                let mut multi_arg_values = vec![arg_value];
                for in_rest_arg in in_rest_args.iter() {
                    let rest_arg_value = match arg.convert_in(in_rest_arg) {
                        Ok(rest_arg_value) => rest_arg_value,
                        Err(err) => {
                            return Err(err /*.into()*/);
                        }
                    };
                    Self::_verify_arg_range(arg, &rest_arg_value)?;
                    multi_arg_values.push(rest_arg_value.clone());
                }
                (Some(multi_arg_values), None)
            } else if arg.multi_mode == ArgMultiMode::Rest {
                (None, Some(in_rest_args))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };
        match &arg.key {
            ArgKey::Name(name) => {
                self.input_arg_index_map.insert(name.clone(), arg_idx);
                if let Some(multi_arg_values) = multi_arg_values {
                    assert!(rest_arg_values.is_none());
                    self.input_multi_arg_data = Some((vec![name.clone()], multi_arg_values));
                } else if let Some(rest_arg_values) = rest_arg_values {
                    self.input_rest_arg_data = Some((vec![name.clone()], rest_arg_values));
                }
            }
            ArgKey::Flags(_, flags) => {
                for flag in flags.iter() {
                    self.input_arg_index_map.insert(flag.clone(), arg_idx);
                }
                if let Some(multi_arg_values) = multi_arg_values {
                    assert!(rest_arg_values.is_none());
                    self.input_multi_arg_data = Some((flags.clone(), multi_arg_values));
                } else if let Some(rest_arg_values) = rest_arg_values {
                    self.input_rest_arg_data = Some((flags.clone(), rest_arg_values));
                }
            }
        }
        Ok(())
    }
    fn _verify_arg_range(arg: &Arg, arg_value: &ArgValue) -> Result<(), String> {
        match &arg.constraint {
            ArgConstraint::Enums(enum_values) => {
                let mut found = false;
                for enum_value in enum_values.iter() {
                    if enum_value.compare(arg_value) == 0 {
                        found = true;
                        break;
                    }
                }
                if !found {
                    let mut values = String::new();
                    for enum_value in enum_values.iter() {
                        let value = enum_value.to_string();
                        if !values.is_empty() {
                            values.push_str(", ");
                        }
                        values.push_str(value.as_str());
                    }
                    return Err(format!(
                        "[{}] doesn't match any of the enum values [{}]",
                        arg_value.to_string(),
                        values
                    ));
                }
            }
            ArgConstraint::Range(min, max) => {
                if arg_value.compare(min) < 0 || arg_value.compare(max) > 0 {
                    return Err(format!(
                        "[{}] is out of range [{}, {}]",
                        arg_value.to_string(),
                        min,
                        max
                    ));
                }
            }
            ArgConstraint::None => {}
        }
        Ok(())
    }
    fn _show_help(&self, flag_args: &[Arg], position_args: &[Arg], err_msg: &Option<String>) {
        println!();
        if let Some(err_msg) = err_msg {
            println!("| !!!");
            println!("| !!! INVALID INPUT ARGUMENT: {}", err_msg);
            println!("| !!!");
        }
        let usage = self._compose_usage(flag_args, position_args);
        print!("| USAGE: {usage}");
        println!();
        match &self.description {
            Some(description) => {
                println!("| : {description}");
            }
            None => {
                println!();
            }
        }
        println!("| . -h, --help : HELP");
        for flag_arg in flag_args.iter() {
            let (name, flags) = match &flag_arg.key {
                ArgKey::Flags(name, flags) => (name, flags),
                _ => panic!(),
            };
            print!("| . ");
            for (index, flag) in flags.iter().enumerate() {
                if index > 0 {
                    print!(", ");
                }
                let f = if flag_arg.nature == ArgNature::Fixed {
                    flag.to_string()
                } else {
                    format!("{flag} {name}")
                };
                print!("{f}");
            }
            self._show_help_arg_desc(flag_arg);
            // if flag_arg.nature == ArgNature::Fixed {
            //     println!(" : FLAG [{}]", flag_arg.value);
            // } else if flag_arg.nature == ArgNature::Optional {
            //     println!(" : OPTIONAL; default [{}]", flag_arg.value);
            // } else {
            //     println!(" : REQUIRED; e.g. [{}]", flag_arg.value);
            // }
            // match &flag_arg.description {
            //     Some(description) => {
            //         println!("|   -- {description}")
            //     }
            //     None => {}
            // }
        }
        for position_arg in position_args.iter() {
            let name = match &position_arg.key {
                ArgKey::Name(name) => name,
                _ => panic!(),
            };
            print!("| . <{name}>");
            self._show_help_arg_desc(position_arg);
            // print!("| . {name}");
            // if position_arg.nature == ArgNature::Optional {
            //     println!(" : OPTIONAL; default [{}]", position_arg.value);
            // } else {
            //     println!(" : REQUIRED; e.g. [{}]", position_arg.value);
            // }
            // match &position_arg.description {
            //     Some(description) => {
            //         println!("|   -- {description}")
            //     }
            //     None => {}
            // }
        }
    }
    fn _show_help_arg_desc(&self, arg: &Arg) {
        if arg.multi_mode != ArgMultiMode::None {
            print!(" ...");
        }
        if arg.nature == ArgNature::Fixed {
            println!(" : FLAG [{}]", arg.value);
        } else if arg.nature == ArgNature::Optional {
            println!(" : OPTIONAL; default [{}]", arg.value);
        } else {
            print!(" : REQUIRED; e.g. {}", arg.value);
            if arg.multi_mode != ArgMultiMode::None {
                print!(" ...");
            }
            println!();
        }
        match &arg.description {
            Some(description) => {
                println!("|   : {description}")
            }
            None => {}
        }
        match &arg.constraint {
            ArgConstraint::Range(min, max) => {
                println!("|   : range: [{}, {}]", min.to_string(), max.to_string());
            }
            ArgConstraint::Enums(enum_values) => {
                let mut single_line = true;
                for enum_value in enum_values.iter() {
                    if enum_value.description.is_some() {
                        single_line = false;
                        break;
                    }
                }
                if single_line {
                    let mut values = String::new();
                    for enum_value in enum_values.iter() {
                        let value = enum_value.to_string();
                        if !values.is_empty() {
                            values.push_str(", ");
                        }
                        values.push_str(value.as_str());
                    }
                    println!("|   : enum values: [{}]", values);
                } else {
                    for enum_value in enum_values.iter() {
                        let value = enum_value.to_string();
                        print!("|   : . [{}]", value);
                        match &enum_value.description {
                            Some(description) => {
                                println!(" : {description}");
                            }
                            None => {
                                println!();
                            }
                        }
                    }
                }
            }
            ArgConstraint::None => {}
        }
    }
    fn _compose_usage(&self, flag_args: &[Arg], position_args: &[Arg]) -> String {
        let mut usage = String::new();
        let program_name = self._get_program_name();
        usage.push_str(program_name.as_str());
        usage.push_str(" [-h]");
        for flag_arg in flag_args.iter() {
            let (val_name, flags) = match &flag_arg.key {
                ArgKey::Flags(name, flags) => (name, flags),
                _ => panic!(),
            };
            let flag = &flags[0];
            let f = if flag_arg.nature == ArgNature::Fixed {
                flag.to_string()
            } else {
                format!("{flag} {val_name}")
            };
            let f = if flag_arg.nature == ArgNature::Fixed || flag_arg.nature == ArgNature::Optional
            {
                format!(" [{f}]")
            } else {
                format!(" {f}")
            };
            //println!("==============FLAG===== f: {}", f);
            usage.push_str(f.as_str());
        }
        for position_arg in position_args.iter() {
            let name = match &position_arg.key {
                ArgKey::Name(name) => name,
                _ => panic!(),
            };
            let f = if position_arg.nature == ArgNature::Optional {
                format!(" [{name}]")
            } else {
                format!(" <{name}>")
            };
            //println!("==============POS===== f: {}", f);
            usage.push_str(f.as_str());
        }
        usage
    }
    fn _get_program_name(&self) -> String {
        if let Some(program_name) = &self.program_name {
            program_name.clone()
        } else {
            "<program>".to_owned()
        }
    }
}

// #[derive(Debug)]
// pub struct DumbArgError {
//     message: String,
// }
// impl Error for DumbArgError {}
// impl fmt::Display for DumbArgError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

#[derive(Debug, Clone)]
enum ArgKey {
    Name(String),
    Flags(String, Vec<String>),
}

impl ArgKey {
    fn get_a_name(&self) -> String {
        match self {
            ArgKey::Name(name) => name.clone(),
            ArgKey::Flags(_, flags) => flags[0].clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Arg {
    key: ArgKey,
    value: ArgValue,
    constraint: ArgConstraint,
    nature: ArgNature,
    multi_mode: ArgMultiMode,
    description: Option<String>,
}
impl Arg {
    fn new(
        key: ArgKey,
        value: ArgValue,
        constraint: ArgConstraint,
        nature: ArgNature,
        multi_mode: ArgMultiMode,
        description: &Option<String>,
    ) -> Arg {
        Arg {
            key,
            value,
            constraint,
            nature,
            multi_mode,
            description: description.clone(),
        }
    }
    fn convert_in(&self, val: &str) -> Result<ArgValue, String> {
        let value = match self.value {
            ArgValue::I32(_) => {
                let v = match val.parse::<i32>() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("failed to parse \"{}\" as i32", val)),
                };
                ArgValue::I32(v)
            }
            ArgValue::I64(_) => {
                let v = match val.parse::<i64>() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("failed to parse \"{}\" as i64", val)),
                };
                ArgValue::I64(v)
            }
            ArgValue::F32(_) => {
                let v = match val.parse::<f32>() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("failed to parse \"{}\" as f32", val)),
                };
                ArgValue::F32(v)
            }
            ArgValue::F64(_) => {
                let v = match val.parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("failed to parse \"{}\" as f64", val)),
                };
                ArgValue::F64(v)
            }
            ArgValue::Bool(_) => {
                let v = match val.parse::<bool>() {
                    Ok(v) => v,
                    Err(e) => return Err(format!("failed to parse \"{}\" as bool", val)),
                };
                ArgValue::Bool(v)
            }
            ArgValue::String(_) => ArgValue::String(val.to_string()),
            //ArgValue::StaticStr(_) => ArgValue::String(val.to_string()),
        };
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ArgNature {
    Regular,
    Optional,
    Fixed,
}

#[derive(Debug, Clone)]
enum ArgConstraint {
    None,
    Enums(Vec<ArgEnum>),
    Range(ArgValue, ArgValue),
}

#[derive(Debug, Clone)]
struct ArgEnum {
    arg_value: ArgValue,
    description: Option<String>,
}
impl ArgEnum {
    fn new(arg_value: ArgValue, description: Option<String>) -> ArgEnum {
        ArgEnum {
            arg_value,
            description: description.clone(),
        }
    }
    fn compare(&self, arg_value: &ArgValue) -> i32 {
        self.arg_value.compare(arg_value)
    }
    fn to_string(&self) -> String {
        self.arg_value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ArgMultiMode {
    None,
    Regular,
    Rest,
}

#[derive(Debug)]
struct ArgParserSettings {
    // not yet use; not very useful .. when pub, call it DumbArgParserSettings
    pub program_name: Option<String>,
    pub description: Option<String>,
    pub allow_none: bool,
}
impl ArgParserSettings {
    pub fn default() -> ArgParserSettings {
        ArgParserSettings {
            program_name: None,
            description: None,
            allow_none: false,
        }
    }
}

/// For [`DumbArgParser`] to build argument specifications. Please use the macro [`sap_arg!`] to create and instant of it.
#[derive(Debug)]
pub struct DumbArgBuilder {
    name_or_flags: Vec<String>,
    value: ArgValue,
    constraint: ArgConstraint,
    nature: ArgNature,
    multi_mode: ArgMultiMode,
    description: Option<String>,
}
impl DumbArgBuilder {
    /// create an instance of [`DumbArgBuilder`], but suggested to use [`sap_arg!`] macro instead.
    pub fn new(name_or_flags: Vec<String>) -> DumbArgBuilder {
        DumbArgBuilder {
            name_or_flags: name_or_flags,
            value: ArgValue::default(),
            constraint: ArgConstraint::None,
            nature: ArgNature::Regular,
            multi_mode: ArgMultiMode::None,
            description: None,
        }
    }
    /// For argument that requires an argument value passed in.
    /// `value` - used to infer the type of the argument, as well as sample value to shown in help message
    pub fn value<T: ArgValueTrait>(&mut self, value: T) -> &mut DumbArgBuilder {
        self.value = value.to_arg_value();
        self.nature = ArgNature::Regular;
        //self.multi = false;
        self
    }
    /// For argument that has default argument value passed in.
    /// * `value` - the default argument value when it the argument is not provided
    pub fn default<T: ArgValueTrait>(&mut self, value: T) -> &mut DumbArgBuilder {
        self.value = value.to_arg_value();
        self.nature = ArgNature::Optional;
        //self.multi = false;
        self
    }
    /// For argument that has a fixed argument value based simply whether the flag is present or not. E.g. "-v" for turning on verbose mode.
    /// * `value` - the fixed value when the argument flag is provided
    pub fn fixed<T: ArgValueTrait>(&mut self, value: T) -> &mut DumbArgBuilder {
        self.value = value.to_arg_value();
        self.nature = ArgNature::Fixed;
        //self.multi = false;
        self
    }
    /// set the acceptable value range (inclusive) of the argument
    pub fn set_range<T: ArgValueTrait>(&mut self, min: T, max: T) -> &mut DumbArgBuilder {
        self.constraint = ArgConstraint::Range(min.to_arg_value(), max.to_arg_value());
        self
    }
    /// set the acceptable values for the argument; if there are descriptions for the values, use [`DumbArgBuilder::set_with_desc_enums`] instead
    pub fn set_enums<T: ArgValueTrait>(&mut self, values: Vec<T>) -> &mut DumbArgBuilder {
        let mut arg_enums = Vec::new();
        for value in values.iter() {
            let arg_value = value.to_arg_value();
            arg_enums.push(ArgEnum::new(arg_value, None));
        }
        self.constraint = ArgConstraint::Enums(arg_enums);
        self
    }
    /// like [`DumbArgBuilder::set_enums`], set the acceptable values for the argument;
    /// each value is assumed to be suffixed with a description separated by a colon (":"); e.g. "debug:run in debug mode"
    pub fn set_with_desc_enums<T: ArgValueTrait>(&mut self, values: Vec<T>) -> &mut DumbArgBuilder {
        let mut arg_enums = Vec::new();
        for value in values.iter() {
            let arg_value = value.to_arg_value();
            let value_str = arg_value.to_string();
            let mut parts = value_str.split(':');
            let value = parts.next().unwrap();
            let description = parts.next().map(|s| s.to_string()).unwrap();
            arg_enums.push(ArgEnum::new(
                String::to_arg_value(&value.to_string()),
                Some(description),
            ));
        }
        self.constraint = ArgConstraint::Enums(arg_enums);
        self
    }
    /// set it to be a multi-argument -- i.e. one that will accept one + rest of the input argument values, that can be retrieved with [`DumbArgParser::get_multi`]
    ///
    /// *** note that multi-argument cannot be [`DumbArgBuilder::fixed`] ***
    pub fn set_multi(&mut self) -> &mut DumbArgBuilder {
        self.multi_mode = ArgMultiMode::Regular;
        self
    }
    /// like [`DumbArgBuilder::set_multi`], set it to be the form of multi-argument that will accept rest of the input argument values as [`String`], that can be [`DumbArgParser::get_rest`]
    pub fn set_rest(&mut self) -> &mut DumbArgBuilder {
        self.multi_mode = ArgMultiMode::Rest;
        self
    }
    /// set description of the argument to be shown in the help message.
    pub fn set_description(&mut self, description: &str) -> &mut DumbArgBuilder {
        self.description = Some(description.to_string());
        self
    }
    /// add the argument object (argument specification) to the [`DumbArgParser`].
    pub fn add_to(&self, parser: &mut DumbArgParser) -> Result<(), DumbError> {
        let key = self._to_key()?;
        parser.add_arg(Arg::new(
            key,
            self.value.clone(),
            self.constraint.clone(),
            self.nature.clone(),
            self.multi_mode.clone(),
            &self.description,
        ));
        Ok(())
    }
    fn _to_key(&self) -> Result<ArgKey, String> {
        if self.name_or_flags.is_empty() {
            return Err("must provide a name or some flags".to_owned());
        }
        if self.name_or_flags.len() == 1 {
            let name = &self.name_or_flags[0];
            if !name.starts_with('-') {
                return Ok(ArgKey::Name(name.clone()));
            }
        }
        let mut s_name: Option<String> = None;
        let mut d_name: Option<String> = None;
        for flag in self.name_or_flags.iter() {
            if !flag.starts_with('-') {
                return Err(format!("flag [{}] must start with '-'", flag));
            }
            if flag.starts_with("--") {
                if flag.starts_with("---") {
                    return Err(format!("flag [{}] cannot start with more than 2 '-'", flag));
                }
                if d_name.is_none() {
                    d_name = Some(flag[2..].to_string());
                }
            } else {
                if s_name.is_none() {
                    s_name = Some(flag[1..].to_string());
                }
            }
        }
        let name = if let Some(d_name) = d_name {
            d_name
        } else {
            s_name.unwrap()
        };
        Ok(ArgKey::Flags(name, self.name_or_flags.clone()))
    }
}

/// for use by [`DumbArgParser`] internally.
#[derive(Debug, Clone)]
pub enum ArgValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    //StaticStr(&'static str),
}
impl ArgValue {
    fn default() -> ArgValue {
        ArgValue::String("".to_owned())
    }
    fn to_string(&self) -> String {
        match *self {
            ArgValue::Bool(v) => v.to_string(),
            ArgValue::I32(v) => v.to_string(),
            ArgValue::I64(v) => v.to_string(),
            ArgValue::F32(v) => v.to_string(),
            ArgValue::F64(v) => v.to_string(),
            ArgValue::String(ref v) => v.to_string(),
        }
    }
    fn compare(&self, arg_value: &ArgValue) -> i32 {
        match *self {
            ArgValue::I32(_) | ArgValue::I64(_) => {
                let this_value = self.to_string().parse::<i64>().unwrap();
                let other_value = arg_value.to_string().parse::<i64>().unwrap();
                //println!("********** {} .. {}", this_value, other_value);
                if this_value < other_value {
                    -1
                } else if this_value > other_value {
                    1
                } else {
                    0
                }
            }
            ArgValue::F32(_) | ArgValue::F64(_) => {
                let this_value = self.to_string().parse::<f64>().unwrap();
                let other_value = arg_value.to_string().parse::<f64>().unwrap();
                if this_value < other_value {
                    -1
                } else if this_value > other_value {
                    1
                } else {
                    0
                }
            }
            ArgValue::Bool(_) | ArgValue::String(_)/*  | ArgValue::StaticStr(_)*/ => {
                let this_value = self.to_string();
                let other_value = arg_value.to_string();
                //println!("*********** [{}] .. [{}]", this_value, other_value);
                if this_value < other_value {
                    -1
                } else if this_value > other_value {
                    1
                } else {
                    0
                }
            }
        }
    }
}
impl fmt::Display for ArgValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if true {
            let s = self.to_string();
            write!(f, "{}", s)
        } else {
            match *self {
                ArgValue::Bool(v) => write!(f, "{}", v),
                ArgValue::I32(v) => write!(f, "{}", v),
                ArgValue::I64(v) => write!(f, "{}", v),
                ArgValue::F32(v) => write!(f, "{}", v),
                ArgValue::F64(v) => write!(f, "{}", v),
                ArgValue::String(ref v) => write!(f, "{}", v),
                //ArgValue::StaticStr(v) => write!(f, "{}", v),
            }
        }
    }
}
/// for use by [`DumbArgParser`] internally.
pub trait ArgValueTrait {
    fn to_arg_value(&self) -> ArgValue;
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<Self>, String>;
}
impl ArgValueTrait for i32 {
    fn to_arg_value(&self) -> ArgValue {
        ArgValue::I32(*self)
    }
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<i32>, String> {
        match arg_value {
            ArgValue::I32(v) => Ok(Box::new(v)),
            _ => Err(format!("value {:?} is not of type i32", arg_value)),
        }
    }
}
impl ArgValueTrait for i64 {
    fn to_arg_value(&self) -> ArgValue {
        ArgValue::I64(*self)
    }
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<i64>, String> {
        match arg_value {
            ArgValue::I32(v) => Ok(Box::new(v as i64)),
            ArgValue::I64(v) => Ok(Box::new(v)),
            _ => Err(format!("value {:?} is not of type i64", arg_value)),
        }
    }
}
impl ArgValueTrait for f32 {
    fn to_arg_value(&self) -> ArgValue {
        ArgValue::F32(*self)
    }
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<f32>, String> {
        match arg_value {
            ArgValue::F32(v) => Ok(Box::new(v)),
            _ => Err(format!("value {:?} is not of type f32", arg_value)),
        }
    }
}
impl ArgValueTrait for f64 {
    fn to_arg_value(&self) -> ArgValue {
        ArgValue::F64(*self)
    }
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<f64>, String> {
        match arg_value {
            ArgValue::I32(v) => Ok(Box::new(v as f64)),
            ArgValue::F32(v) => Ok(Box::new(v as f64)),
            ArgValue::F64(v) => Ok(Box::new(v)),
            _ => Err(format!("value {:?} is not of type f64", arg_value)),
        }
    }
}
impl ArgValueTrait for bool {
    fn to_arg_value(&self) -> ArgValue {
        ArgValue::Bool(*self)
    }
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<bool>, String> {
        match arg_value {
            ArgValue::Bool(v) => Ok(Box::new(v)),
            _ => Err(format!("value {:?} is not of type bool", arg_value)),
        }
    }
}
impl ArgValueTrait for String {
    fn to_arg_value(&self) -> ArgValue {
        ArgValue::String(self.to_string())
    }
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<String>, String> {
        match arg_value {
            ArgValue::I32(v) => Ok(Box::new(v.to_string())),
            ArgValue::I64(v) => Ok(Box::new(v.to_string())),
            ArgValue::F32(v) => Ok(Box::new(v.to_string())),
            ArgValue::F64(v) => Ok(Box::new(v.to_string())),
            ArgValue::Bool(v) => Ok(Box::new(v.to_string())),
            ArgValue::String(v) => Ok(Box::new(v)),
            //ArgValue::StaticStr(v) => Ok(Box::new(v.to_string())),
        }
    }
}
impl ArgValueTrait for &'static str {
    fn to_arg_value(&self) -> ArgValue {
        ArgValue::String(self.to_string())
    }
    fn from_arg_value(arg_value: ArgValue) -> Result<Box<&'static str>, String> {
        panic!("not expected to be called");
    }
}
