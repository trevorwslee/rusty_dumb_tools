use calculator::Calculator;
use rusty_dumb_tools::prelude::*;

mod calculator;

fn main() {
    let mut parser = DumbArgParser::new();
    parser.set_description("DumbCalculator demo.");
    dap_arg!("mode", default = "text")
        .set_description("calculator mode")
        .set_with_desc_enums(vec!["text:text based", "rich:richer text-based"])
        .add_to(&mut parser)
        .unwrap();

    parser.parse_args();

    let mode = parser.get::<String>("mode").unwrap();
    let richer = mode == "rich";
    if richer {
        Calculator::<true>::new_and_init().run()
    } else {
        Calculator::<false>::new_and_init().run()
    };
}
