//! Home for `rusty_dumb_tools` demos.

pub mod demo_arg;
pub mod demo_calc;

// use std::io;
// use std::io::Write;

use crate::{
    arg::{DumbArgBuilder, DumbArgParser},
    sap_arg,
};

use crate::demo::{
    demo_arg::handle_demo_arg,
    demo_calc::{create_demo_parser_calc, handle_demo_calc, handle_demo_calc_repl},
};

///
/// Run the demo.
/// * in_args: if None, parse from command-line; otherwise, parse from in_args.
pub fn run_demo(in_args: Option<Vec<&str>>) {
    // e.g. cargo run -- calc 1.1 + 2.2 * (4.3 - 2.4) + 5
    // e.g. cargo run -- arg -f 0.2 5 --string2 VAL1 false 1 2 3
    let mut parser = create_demo_parser();
    if in_args.is_some() {
        let in_args = in_args.unwrap();
        parser.process_args(in_args);
    } else {
        parser.parse_args();
    }
    handle_demo(parser);
}

pub fn create_demo_parser() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("A collection of rusty_dumb_tools demos.");
    sap_arg!("tool")
        .value("calc")
        .set_description("a demo")
        .set_with_desc_enums(vec![
            "calc:DumbCalcProcessor command-line input demo",
            "calc-repl:DumbCalcProcessor REPL demo",
            "arg:DumbArgParser debug",
        ])
        .set_rest()
        .add_to(&mut parser)
        .unwrap();
    parser
}
pub fn handle_demo(parser: DumbArgParser) {
    let tool = match parser.get::<String>("tool") {
        Some(t) => t,
        None => {
            return;
        }
    };
    match tool.as_str() {
        "arg" => {
            let mut demo_parser = demo_arg::create_debug_arg_parser();
            parser.process_rest_args("tool", &mut demo_parser);
            handle_demo_arg(demo_parser);
        }
        "calc" => {
            let mut demo_parser = create_demo_parser_calc();
            parser.process_rest_args("tool", &mut demo_parser);
            handle_demo_calc(demo_parser);
        }
        "calc-repl" => {
            handle_demo_calc_repl();
        }
        _ => panic!("Unknown tool: {}", tool),
    };
    //println!("demo_parser: {:?}", demo_parser);
}
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
//             println!("| = {}", calc.get_result());
//         }
//         Err(e) => {
//             println!("| Error: {}", e);
//         }
//     }
//     println!("|");
// }
// fn _handle_demo_calc_repl() {
//     println!();
//     println!("* enter an infix expression");
//     println!("* can split the infix expression into multiple lines; e.g. a \"unit\" a line");
//     println!("* finally, enter \"=\" (or an empty line) to evaluate it");
//     println!("* can then continue to enter another infix expression ...");
//     println!();
//     let mut calc = calc::DumbCalcProcessor::new();
//     let mut units = String::new();

//     loop {
//         print!("> {}", units);
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         std::io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         let mut unit = input.trim();
//         if unit == "" {
//             unit = "=";
//         }

//         if unit.to_lowercase() == "c" {
//             units.clear();
//             calc.reset();
//             continue;
//         }

//         if unit == "=" {
//             calc.eval();
//         } else {
//             let push_res = calc.parse_and_push(unit);
//             match push_res {
//                 Ok(_) => {}
//                 Err(e) => {
//                     println!("| Error: {}", e);
//                     continue;
//                 }
//             }
//         }

//         units.push_str(unit);
//         units.push(' ');

//         let result = calc.get_result();
//         let sep = match result {
//             CalcResult::Final(_) => {
//                 units.clear();
//                 "="
//             }
//             CalcResult::Intermediate(_) => ":",
//             CalcResult::Error(_) => {
//                 units.clear();
//                 calc.reset();
//                 "!"
//             }
//         };
//         println!("| {} {}", sep, result);
//     }
// }
