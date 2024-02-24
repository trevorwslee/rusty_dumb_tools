# RustyDumbTools (v0.1.8)

A [collection of simple tools](https://crates.io/crates/rusty_dumb_tools) in ***Rust*** as ***Rust*** modules:
* [`crate::arg::DumbArgParser`](https://docs.rs/rusty_dumb_tools/latest/rusty_dumb_tools/arg/struct.DumbArgParser.html): A simple argument parser.
  It can be useful for handling command line argument parsing for a ***Rust*** program.
* [`crate::calc::DumbCalcProcessor`](https://docs.rs/rusty_dumb_tools/latest/rusty_dumb_tools/calc/index.html): A simple infix calculation processor 
  It can be used to implement a simple calculator in ***Rust***.
* [`crate::calculator::DumbCalculator`](https://docs.rs/rusty_dumb_tools/latest/rusty_dumb_tools/calculator/struct.DumbCalculator.html): A simple calculator that accepts input keys acting like a real calculator.
  It can be used to implement a simple calculator UI in **Rust**.
* [`crate::ltemp::DumbLineTemplate`](https://docs.rs/rusty_dumb_tools/latest/rusty_dumb_tools/ltemp/struct.DumbLineTemplate.html): A simple line template for formatting a line.
  It can be usee for printing values as a line with some template.
* [`crate::lblscreen::DumbLineByLineScreen`](https://docs.rs/rusty_dumb_tools/latest/rusty_dumb_tools/lblscreen/struct.DumbLineByLineScreen.html): A terminal / text-based "screen" update helper.
It is extended from `crate::ltemp::DumbLineTemplate`, and should be helpful in managing the updates of the formatted lines that acts as a "screen".
  
  


# Demo

For a demo program of using the tools, which is itself an application using `DumbArgParser`, you may want to run the included demo function [`rusty_dumb_tools::demo::run_demo`](https://docs.rs/rusty_dumb_tools/latest/rusty_dumb_tools/demo/fn.run_demo.html) like
```
use rusty_dumb_tools::demo;
demo::run_demo(None);  // get arguments from command-line         
````


Assuming new ***Rust*** project with `main.rs` like
```
use rusty_dumb_tools::demo;
fn main() {
    demo::run_demo(None);
}
```
the demo can be ***cargo*** run like
* `cargo run -- -h`
  <br>the input demonstrates using `DumbArgParser` for showing "help message"
* `cargo run -- calc -h`
  <br>`DumbArgParser` is set up to parse arguments for a sub-command (with another `DumbArgParser` object);
  and the above input demonstrates showing of "help message" of the sub-command
* `cargo run -- calc 1.1 + 2.2 * (4.3 - 2.4) + 5`
  <br>the above demonstrates how to use a [sub-command] `DumbArgParser` to parse arguments for the sub-command `calc`,
  which in turn will show how to use `DumbCalcProcessor` for performing calculation of the sub-command arguments
* `cargo run -- calc-repl`
  <br>the above demonstrates how to invoke the sub-command `calc-repl`, which in turn show how `DumbCalcProcessor` like a REPL
* `cargo run -- ltemp Trevor`
  <br>the above demonstrates how to use `DumbLineTemplate` to format lines to show data
* `cargo run -- lblscreen`
  <br>the above demonstrates how to use `DumbLineByLineScreen` to implement a "progress info panel"
* `cargo run -- arg -f 0.2 5 --string2 VAL1 false 1 2 3`

The output of running `cargo run -- -h`:
```
| USAGE: rusty_dumb_tools [-h] demo
| : Demos of rusty_dumb_tools.
| . -h, --help : HELP
| . demo ... : REQUIRED; e.g. calc ...
|   : a demo
|   : . [calc] : DumbCalcProcessor command-line input demo
|   : . [calc-repl] : DumbCalcProcessor REPL demo
|   : . [arg] : DumbArgParser demo (more like debugging)
```

The output of running `cargo run -- calc -h`:
```
| USAGE: rusty_dumb_tools calc [-h] input
| : DumbCalcProcessor command-line input demo.
| . -h, --help : HELP
| . input ... : REQUIRED; e.g. 123 ...
|   : infix expression
```

The output of running `cargo run -- calc 1.1 + 2.2 * (4.3 - 2.4) + 5`:
```
|
| = 10.28.
|
```

After running `cargo run -- calc-repl`, the demo will get in a loop to get input from the prompt:
```
* enter an infix expression
* can split the infix expression into multiple lines; e.g. a "unit" a line
* finally, enter "=" (or an empty line) to evaluate it
* can then continue to enter another infix expression ...

>
```

After running `cargo run -- ltemp Trevor`, the demo will show something like
```
===============================
| NAME :               Trevor |
| AGE  :        <undisclosed> |
|      :     and counting ... |
| +  1 | #                    |
===============================
```
`+  1 | #` acts like a "progress indicator"; after 20 seconds:
```
===============================
| NAME :               Trevor |
| AGE  :        <undisclosed> |
|      :     and counting ... |
| + 20 | #################### |
===============================
```

After running `cargo run -- lblscreen`, the screen will show something like
```
----------------------------------------
|      ... wait ... loading 0% ...     |
| ........ |                    :   0% |
----------------------------------------
```
after 20 seconds, when 100% done, the screen will be like
```
|     ... wait ... loading 100% ...    |
| ........ |>>>>>>>>>>>>>>>>>>>>: 100% |
----------------------------------------
```

# Additional Demos

* [DumbCalculator Text-based Demo](demos/text_based_calculator/README.md)
* [DumbCalculator Web-based Demo](demos/web_based_calculator/README.md)
  - you may try it: [<img src="images/dumb_calculator.png" width="150"/>](https://trevorwslee.github.io/DumbCalculator/)
  - and may want to refer to some [brief introduction](https://github.com/trevorwslee/wasm_calculator) on how it is implemented 




# Thank You!

Greeting from the author Trevor Lee:

> Peace be with you!
> May God bless you!
> Jesus loves you!
> Amazing Grace!


# License

MIT


# Change History:

* v0.1.8
  - bug fix

* v0.1.7
  - bug fix
  - enhanced DumbCalcProcessor and DumbCalculator

* v0.1.6
  - bug fix

* v0.1.5
  - bug fix
  - moved demo code around
  - added web-based calculator 

* v0.1.4
  - bug fix

* v0.1.3
  - bug fix
  - added richer text-based calculator demo 

* v0.1.2
  - bug fix
  - added `DumbLineTemplate`, `DumbLineByLineScreen` and `DumbLineByLineScreen`

* v0.1.1
  - bug fix
  - added more documentations, and via `run_demo` function

* v0.1.0
  - initial release
