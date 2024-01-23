use rusty_dumb_tools::{arg::DumbArgBuilder, arg::DumbArgParser, calc, sap_arg};

use crate::debug;

pub fn debug_calc_processor() {
    println!("DumbCalc:");
    let mut calc = calc::DumbCalcProcessor::new();

    println!("2");
    calc.push("2").unwrap();
    calc.eval().unwrap();
    println!("= {}", calc.get_result().unwrap());
    assert_eq!(2.0, calc.get_result().unwrap());

    println!("CLEAR");
    calc.reset();
    println!("= {}", calc.get_result().unwrap());
    assert_eq!(0.0, calc.get_result().unwrap());

    println!("2 + (3 * 4) - 1");
    calc.push("2").unwrap();
    calc.push("+").unwrap();
    calc.push("(").unwrap();
    calc.push("3").unwrap();
    calc.push("*").unwrap();
    calc.push("4").unwrap();
    calc.push(")").unwrap();
    calc.push("-").unwrap();
    calc.push("1").unwrap();
    calc.eval().unwrap();
    println!("= {}", calc.get_result().unwrap());
    assert_eq!(13.0, calc.get_result().unwrap());

    println!("+ (3 * 4) - 1");
    calc.push("+").unwrap();
    calc.push("(").unwrap();
    calc.push("3").unwrap();
    calc.push("*").unwrap();
    calc.push("4").unwrap();
    calc.push(")").unwrap();
    calc.push("-").unwrap();
    calc.push("1").unwrap();
    calc.eval().unwrap();
    println!("= {}", calc.get_result().unwrap());
    assert_eq!(24.0, calc.get_result().unwrap());

    println!("- 24");
    calc.push("-").unwrap();
    calc.push("24.0").unwrap();
    calc.eval().unwrap();
    println!("= {}", calc.get_result().unwrap());
    assert_eq!(0.0, calc.get_result().unwrap());

    // calc.reset();
    // debug_dumb_tools_push_str(&mut calc, " 1 + 2 * 3 + 4 * 5 + 6 ", 33.0);

    // calc.reset();
    // debug_dumb_tools_push_str(&mut calc, "", 0.0);
    // debug_dumb_tools_push_str(&mut calc, "()", 0.0);

    //debug_simple_calc_push_str(&mut calc, "2()", 0.0);  // == 2 * () ==> 2 * 0
}

pub fn debug_dumb_arg_parser() {
    let mut parser = debug::create_debug_arg_parser();
    println!("parser: {:?}", parser);
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^");
    parser.parse_args();
    println!("==========================");
}

pub fn create_debug_arg_parser() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("This is a simple argument parser.");
    sap_arg!("i32")
        .value(0)
        .set_description("an integer")
        .set_range(1, 10)
        .add_to(&mut parser)
        .unwrap();
    sap_arg!("bool").default(true).add_to(&mut parser).unwrap();
    sap_arg!("-f", "--float")
        .value(0.1)
        .add_to(&mut parser)
        .unwrap();
    sap_arg!("-v")
        .fixed(true)
        .set_description("turn on verbose mode")
        .add_to(&mut parser)
        .unwrap();
    sap_arg!("-s", "--string")
        .default("V1")
        .set_with_desc_enums(vec!["V1:version 1", "V2:version 2", "V3:version 3"])
        .add_to(&mut parser)
        .unwrap();
    sap_arg!("--string2")
        .value("VAL2")
        .set_description("this is the second string")
        .set_enums(vec!["VAL1", "VAL2", "VAL3"])
        .add_to(&mut parser)
        .unwrap();
    sap_arg!("multi")
        .value(1)
        .set_multi()
        .add_to(&mut parser)
        .unwrap();
    parser
}
