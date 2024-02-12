use rusty_dumb_tools::demo;

use rusty_dumb_tools::prelude::*;

pub fn debug_calc_processor() {
    println!("DumbCalc:");
    let mut calc = DumbCalcProcessor::new();

    println!("2");
    calc.push("2").unwrap();
    calc.eval().unwrap();
    println!("= {}", calc.get_result());
    assert_eq!(2.0, calc.get_result().unwrap());

    println!("CLEAR");
    calc.reset();
    println!("= {}", calc.get_result());
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
    println!("= {}", calc.get_result());
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
    println!("= {}", calc.get_result());
    assert_eq!(24.0, calc.get_result().unwrap());

    println!("- 24");
    calc.push("-").unwrap();
    calc.push("24.0").unwrap();
    calc.eval().unwrap();
    println!("= {}", calc.get_result());
    assert_eq!(0.0, calc.get_result().unwrap());

    // calc.reset();
    // debug_dumb_tools_push_str(&mut calc, " 1 + 2 * 3 + 4 * 5 + 6 ", 33.0);

    // calc.reset();
    // debug_dumb_tools_push_str(&mut calc, "", 0.0);
    // debug_dumb_tools_push_str(&mut calc, "()", 0.0);

    //debug_simple_calc_push_str(&mut calc, "2()", 0.0);  // == 2 * () ==> 2 * 0
}

pub fn debug_dumb_arg_parser() {
    let mut parser = demo::demo_arg::create_debug_arg_parser();
    println!("parser: {:?}", parser);
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^");
    parser.parse_args();
    println!("==========================");
}
