//! A collection of simple tools in ***Rust*** as ***Rust*** modules:
//! * [`crate::arg::DumbArgParser`]: A simple argument parser.
//! It can be useful for handling command line argument parsing.
//! * [`crate::calc::DumbCalcProcessor`]: A simple infix calculation processor.
//! It can be used to implement a simple calculator.
//! * [`crate::ltemp::DumbLineTemplate`]: A simple line template for formatting a line, which can be use to print values as a line with some template.
//! * [`crate::lblscreen::DumbLineByLineScreen`]: A terminal / text-based "screen" update helper, which is extended from [`crate::ltemp::DumbLineTemplate`],
//!   and should be helpful in managing the updates of the formatted lines that acts as a "screen".
//!
//! For a demo program of using the tools, you may want to run the included demo function [`demo::run_demo`] like
//! ```
//! use rusty_dumb_tools::demo;
//! // demo::run_demo(None);  // get arguments from command-line         
//! demo::run_demo(Some(vec!["calc", "-h"]));  // pass in explicit arguments        
//! ````
//!
//! Assuming new ***Rust*** project with `main.rs` like
//! ```no_run
//! use rusty_dumb_tools::demo;
//! fn main() {
//!    demo::run_demo(None);
//! }
//! ```
//! the demo can be ***cargo*** run like
//! * `cargo run -- -h`
//! * `cargo run -- calc -h`
//! * `cargo run -- calc 1.1 + 2.2 * (4.3 - 2.4) + 5`
//! * `cargo run -- calc-repl`
//! * `cargo run -- ltemp Trevor`
//! * `cargo run -- lblscreen`
//! * `cargo run -- arg -f 0.2 5 --string2 VAL1 false 1 2 3`
//!
//! Enjoy!
//!
//! Greeting from the author Trevor Lee:
//! > ***Peace be with you! May God bless you! Jesus loves you! Amazing Grace!***

#[macro_use]
pub mod arg;
pub mod calc;
pub mod lblscreen;
pub mod ltemp;

pub mod calculator;

pub mod demo;
