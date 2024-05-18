//! core [`crate::arg`] sub-demo code

#![deny(warnings)]
#![allow(unused)]
#![deny(warnings)]

use crate::{
    arg::{DumbArgBuilder, DumbArgParser},
    dap_arg,
};

pub fn handle_demo_arg(parser: DumbArgParser) {
    let i32_arg = parser.get::<i32>("i32").unwrap();
    let bool_arg = parser.get::<bool>("bool").unwrap();
    let float_arg = parser.get::<f64>("--float").unwrap();
    let verbose_arg = parser.get::<bool>("-v").unwrap_or(false);
    let string_arg = parser.get::<String>("-s").unwrap();
    let string2_arg = parser.get::<String>("--string2").unwrap();
    let multi = parser.get_multi::<i32>("multi").unwrap();

    println!(". i32_arg: {}", i32_arg);
    println!(". bool_arg: {}", bool_arg);
    println!(". float_arg: {}", float_arg);
    println!(". verbose_arg: {}", verbose_arg);
    println!(". string_arg: {}", string_arg);
    println!(". string2_arg: {}", string2_arg);
    println!(". multi: {:?}", multi);
}

pub fn create_debug_arg_parser() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("This is a simple argument parser.");
    dap_arg!("i32", value = 0)
        .set_description("an integer")
        .set_range(1, 10)
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("bool", default = true)
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("-f", flag2 = "--float", value = 0.1)
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("-v", fixed = true)
        .set_description("turn on verbose mode")
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("-s", flag2 = "--string", default = "V1")
        .set_with_desc_enums(vec!["V1:version 1", "V2:version 2", "V3:version 3"])
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("--string2", value = "VAL2")
        .set_description("this is the second string")
        .set_enums(vec!["VAL1", "VAL2", "VAL3"])
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("multi", value = 1)
        .set_multi()
        .add_to(&mut parser)
        .unwrap();
    parser
}

pub fn arg_parser_sample(provide_sample_args: bool) {
    let mut parser = DumbArgParser::new();
    parser.set_description("This is a simple argument parser.");
    dap_arg!("-v", flag2 = "--verbose", fixed = true).add_to(&mut parser); // argument flag "-v" / "--verbose" with fixed value (true) when the flag is present
    dap_arg!("-n", flag2 = "--name", default = "nobody").add_to(&mut parser); // argument "-n" / "--name" requiring input value, with default "nobody"
    dap_arg!("str-arg").add_to(&mut parser); // positional argument "str-arg" (of type String)
    dap_arg!("i32-arg", value = 123).add_to(&mut parser); // positional argument "i32-arg" of type i32 (inferred from the value 123)
    dap_arg!("multi-arg").set_multi().add_to(&mut parser); // positional multi-argument "multi-arg" that will accept multiple values (one + rest)
    if provide_sample_args {
        let in_args: Vec<&str> = vec!["-v", "STR", "888", "m1", "m2", "m3"]; // explicitly provide arguments
        parser.process_args(in_args); // parse from command-line arguments
    } else {
        parser.parse_args(); // parse from command-line arguments
    }
    println!(". -v: {:?}", parser.get::<bool>("-v"));
    println!(". --verbose: {:?}", parser.get::<bool>("--verbose")); // will be the same parameter value as "-v"
    println!(". --name: {:?}", parser.get::<String>("--name")); // can use "-n" as well
    println!(". str-arg: {:?}", parser.get::<String>("str-arg"));
    println!(". i32-arg: {:?}", parser.get::<i32>("i32-arg"));
    println!(". multi-arg: {:?}", parser.get_multi::<String>("multi-arg"));
}
