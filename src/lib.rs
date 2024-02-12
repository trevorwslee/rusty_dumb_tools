//! A collection of simple tools in ***Rust*** as ***Rust*** modules:
//! * [`crate::arg::DumbArgParser`]: A simple argument parser.
//! It can be useful for handling command line argument parsing.
//! * [`crate::calc::DumbCalcProcessor`]: A simple infix calculation processor.
//! * [`crate::calculator::DumbCalculator`]: A simple calculator that accepts input keys acting like a real calculator.
//! It can be used to implement a simple calculator UI.
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
//! You may also want to visit the source in [*rusty_dumb_tools* GitHub repo](https://github.com/trevorwslee/rusty_dumb_tools)
//!
//! Enjoy!
//!
//! Greeting from the author Trevor Lee:
//! > ***Peace be with you! May God bless you! Jesus loves you! Amazing Grace!***

#[macro_use]
pub mod shared;
pub mod arg;
pub mod calc;
pub mod lblscreen;
pub mod ltemp;

pub mod calculator;

pub mod demo;

pub mod prelude {
    //! All the common 'uses' of this crate -- ```use rusty_dumb_tools::prelude::*;```
    pub use crate::arg::*;
    pub use crate::calc::*;
    pub use crate::calculator::*;
    pub use crate::dap_arg;
    pub use crate::dlt_comps;
    pub use crate::dltc;
    pub use crate::lblscreen::*;
    pub use crate::ltemp::*;
    pub use crate::sap_arg;
    pub use crate::shared::*;
}
