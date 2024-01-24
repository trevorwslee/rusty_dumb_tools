//! A collection of simple tools in ***Rust*** as ***Rust*** modules:
//! * [`crate::arg::DumbArgParser`]: A simple argument parser.
//! It can be useful for handling command line argument parsing for a ***Rust*** program.
//! * [`crate::calc::DumbCalcProcessor`]: A simple infix calculation processor.
//! It can be used to implement a simple calculator in ***Rust***.
//!
//! For a demo program of using the tools, you may want to run the included demo function [`demo::run_demo`] like
//! ```
//! use rusty_dumb_tools::demo;
//! // demo::run_demo(None);  // get arguments from command-line         
//! demo::run_demo(Some(vec!["arg", "-h"]));  // pass in explicit arguments        
//! ````
//!
//! Greeting from the author Trevor Lee:
//! > ***Peace be with you! May God bless you! Jesus loves you! Amazing Grace!***

#[macro_use]
pub mod arg;
pub mod calc;

pub mod demo;
