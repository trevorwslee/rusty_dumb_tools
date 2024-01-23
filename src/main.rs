//#![deny(warnings)]
#![allow(unused)]

mod debug;
mod demo;

use std::env;

use rusty_dumb_tools::{
    arg::{self, DumbArgBuilder, DumbArgParser},
    calc::{self, DumbCalcProcessor},
    sap_arg,
};

fn main() {
    //demo::demo();

    // let released = if env::var("CARGO_PKG_NAME").is_ok() {
    //     println!("Running with cargo run");
    //     false
    // } else {
    //     println!("Running as an installed binary");
    //     true
    // };

    let released: bool = true;
    if released {
        released_main();
    } else {
        debug_main();
    }
}

fn released_main() {
    // e.g. cargo run -- calc 1.1 + 2.2 * (4.3 - 2.4) + 5
    // e.g. cargo run -- arg -f 0.2 5 --string2 VAL1 false 1 2 3
    let debug = false;

    let mut parser = demo::create_demo_parser();
    if debug {
        //parser.process_args(vec!["arg", "-h"]);
        parser.process_args(vec![
            "arg",
            "-f",
            "0.2",
            "5",
            "--string2",
            "VAL1",
            "false",
            "1",
            "2",
            "3",
        ]);
        //parser.process_args(vec![]);
    } else {
        parser.parse_args();
    }
    demo::handle_demo(parser);
}

// fn _create_demo_parser() -> DumbArgParser {
//     let mut parser = DumbArgParser::new();
//     parser.set_description("A collection of rusty_dumb_tools demos.");
//     sap_arg!("tool")
//         .value("calc")
//         .set_description("a demo")
//         .set_with_desc_enums(vec![
//             "calc:DumbCalcProcessor command-line input demo",
//             "arg:DumbArgParser debug",
//         ])
//         .set_rest()
//         .add_to(&mut parser)
//         .unwrap();
//     parser
// }
// fn _handle_demo(parser: DumbArgParser) {
//     let tool = match parser.get::<String>("tool") {
//         Some(t) => t,
//         None => {
//             return;
//         }
//     };
//     match tool.as_str() {
//         "arg" => {
//             let mut demo_parser = _create_debug_arg_parser();
//             parser.process_rest_args("tool", &mut demo_parser);
//             _handle_demo_arg(demo_parser);
//         }
//         "calc" => {
//             let mut demo_parser = _create_demo_parser_calc();
//             parser.process_rest_args("tool", &mut demo_parser);
//             _handle_demo_calc(demo_parser);
//         }
//         _ => panic!("Unknown tool: {}", tool),
//     };
//     //println!("demo_parser: {:?}", demo_parser);
// }
// fn _handle_demo_arg(parser: DumbArgParser) {
//     let i32_arg = parser.get::<i32>("i32").unwrap();
//     let bool_arg = parser.get::<bool>("bool").unwrap();
//     let float_arg = parser.get::<f64>("--float").unwrap();
//     let verbose_arg = parser.get::<bool>("-v").unwrap_or(false);
//     let string_arg = parser.get::<String>("-s").unwrap();
//     let string2_arg = parser.get::<String>("--string2").unwrap();
//     let multi = parser.get_multi::<i32>("multi").unwrap();

//     println!(". i32_arg: {}", i32_arg);
//     println!(". bool_arg: {}", bool_arg);
//     println!(". float_arg: {}", float_arg);
//     println!(". verbose_arg: {}", verbose_arg);
//     println!(". string_arg: {}", string_arg);
//     println!(". string2_arg: {}", string2_arg);
//     println!(". multi: {:?}", multi);
// }
// fn _create_demo_parser_calc() -> DumbArgParser {
//     let mut parser = DumbArgParser::new();
//     parser.set_description("DumbCalcProcessor command-line input demo.");
//     sap_arg!("input")
//         .value("123")
//         .set_multi()
//         .set_description("infix expression")
//         .add_to(&mut parser)
//         .unwrap();
//     parser
// }
// fn _handle_demo_calc(parser: DumbArgParser) {
//     //println!("calc demo parser: {:?}", parser);
//     let input = parser.get_multi::<String>("input").unwrap();
//     let mut calc = calc::DumbCalcProcessor::new();
//     for i in input {
//         calc.parse_and_push(&i).unwrap()
//     }
//     println!("|");
//     match calc.eval() {
//         Ok(_) => {
//             println!("| = {}", calc.get_result().unwrap());
//         }
//         Err(e) => {
//             println!("| Error: {}", e);
//         }
//     }
//     println!("|");
// }

fn debug_main() {
    calc::debug_calc();
    debug::debug_calc_processor();

    arg::debug_arg();

    debug::debug_dumb_arg_parser();
}

// fn debug_dumb_tools() {
//     println!("DumbCalc:");
//     let mut calc = calc::DumbCalcProcessor::new();

//     println!("2");
//     calc.push("2").unwrap();
//     calc.eval().unwrap();
//     println!("= {}", calc.get_result().unwrap());
//     assert_eq!(2.0, calc.get_result().unwrap());

//     println!("CLEAR");
//     calc.reset();
//     println!("= {}", calc.get_result().unwrap());
//     assert_eq!(0.0, calc.get_result().unwrap());

//     println!("2 + (3 * 4) - 1");
//     calc.push("2").unwrap();
//     calc.push("+").unwrap();
//     calc.push("(").unwrap();
//     calc.push("3").unwrap();
//     calc.push("*").unwrap();
//     calc.push("4").unwrap();
//     calc.push(")").unwrap();
//     calc.push("-").unwrap();
//     calc.push("1").unwrap();
//     calc.eval().unwrap();
//     println!("= {}", calc.get_result().unwrap());
//     assert_eq!(13.0, calc.get_result().unwrap());

//     println!("+ (3 * 4) - 1");
//     calc.push("+").unwrap();
//     calc.push("(").unwrap();
//     calc.push("3").unwrap();
//     calc.push("*").unwrap();
//     calc.push("4").unwrap();
//     calc.push(")").unwrap();
//     calc.push("-").unwrap();
//     calc.push("1").unwrap();
//     calc.eval().unwrap();
//     println!("= {}", calc.get_result().unwrap());
//     assert_eq!(24.0, calc.get_result().unwrap());

//     println!("- 24");
//     calc.push("-").unwrap();
//     calc.push("24.0").unwrap();
//     calc.eval().unwrap();
//     println!("= {}", calc.get_result().unwrap());
//     assert_eq!(0.0, calc.get_result().unwrap());

//     // calc.reset();
//     // debug_dumb_tools_push_str(&mut calc, " 1 + 2 * 3 + 4 * 5 + 6 ", 33.0);

//     // calc.reset();
//     // debug_dumb_tools_push_str(&mut calc, "", 0.0);
//     // debug_dumb_tools_push_str(&mut calc, "()", 0.0);

//     //debug_simple_calc_push_str(&mut calc, "2()", 0.0);  // == 2 * () ==> 2 * 0
// }

// fn debug_dumb_tools_push_str(calc: &mut DumbCalcProcessor, units: &str, res: f64) {
//     println!("\"{}\"", units);
//     calc.push_str(units).unwrap();
//     calc.eval().unwrap();
//     println!("= {}", calc.get_unwrapped_result());
//     calc::assert_eq_calc_result(calc, res);
// }

// fn debug_dumb_arg_parser() {
//     let mut parser = debug::create_debug_arg_parser();
//     println!("parser: {:?}", parser);
//     println!("^^^^^^^^^^^^^^^^^^^^^^^^^^");
//     parser.parse_args();
//     println!("==========================");
// }

// fn _create_debug_arg_parser() -> DumbArgParser {
//     let mut parser = DumbArgParser::new();
//     parser.set_description("This is a simple argument parser.");
//     sap_arg!("i32")
//         .value(0)
//         .set_description("an integer")
//         .set_range(1, 10)
//         .add_to(&mut parser)
//         .unwrap();
//     sap_arg!("bool").default(true).add_to(&mut parser).unwrap();
//     sap_arg!("-f", "--float")
//         .value(0.1)
//         .add_to(&mut parser)
//         .unwrap();
//     sap_arg!("-v")
//         .fixed(true)
//         .set_description("turn on verbose mode")
//         .add_to(&mut parser)
//         .unwrap();
//     sap_arg!("-s", "--string")
//         .default("V1")
//         .set_with_desc_enums(vec!["V1:version 1", "V2:version 2", "V3:version 3"])
//         .add_to(&mut parser)
//         .unwrap();
//     sap_arg!("--string2")
//         .value("VAL2")
//         .set_description("this is the second string")
//         .set_enums(vec!["VAL1", "VAL2", "VAL3"])
//         .add_to(&mut parser)
//         .unwrap();
//     sap_arg!("multi")
//         .value(1)
//         .set_multi()
//         .add_to(&mut parser)
//         .unwrap();
//     parser
// }
