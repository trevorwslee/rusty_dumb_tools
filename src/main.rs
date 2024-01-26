//#![deny(warnings)]
#![allow(unused)]

mod debug;
//mod demo;
// mod demo_arg;
// mod demo_calc;

use std::{env, vec};

use rusty_dumb_tools::{
    arg::{self, DumbArgBuilder, DumbArgParser},
    calc::{self, DumbCalcProcessor},
    demo, sap_arg,
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

    // if true {
    //     demo::run_demo(Some(vec!["arg", "-h"]));
    //     return;
    // }

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
    let in_args = if debug {
        let in_args = vec![
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
        ];
        Some(in_args)
    } else {
        None
    };
    demo::run_demo(in_args);
}

fn debug_main() {
    //calc::debug_calc();
    debug::debug_calc_processor();

    //arg::debug_arg();

    debug::debug_dumb_arg_parser();
}
