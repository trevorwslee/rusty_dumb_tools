#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::{
    arg::{DumbArgBuilder, DumbArgParser},
    dap_arg,
};

use crate::{
    dlt_comps, dltc,
    ltemp::{DumbLineTempCompBuilder, DumbLineTemplate, LineTempComp, LineTempCompTrait},
};

pub fn handle_demo_ltemp(parser: DumbArgParser) {
    let name = match parser.get::<String>("name") {
        Some(t) => t,
        None => {
            panic!("No name specified.");
        }
    };
    show_table(&name);
}

pub fn create_demo_ltemp_parser() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("This is a simple line template demo.");
    dap_arg!("name")
        .set_description("your name please")
        .add_to(&mut parser)
        .unwrap();
    parser
}

pub fn show_table(name: &str) {
    let lt_comps = dlt_comps![
        "| ",
        dltc!("label", fixed_width = 6, align = 'L'),
        " : ",
        dltc!("value", align = 'R'),
        " |"
    ];
    let ltemp = DumbLineTemplate::new(30, 30, &lt_comps);

    let mut map = HashMap::new();
    map.insert(String::from("label"), String::from("NAME"));
    map.insert(String::from("value"), name.to_string());
    let line1 = ltemp.format(&map).unwrap();

    let mut map = HashMap::new();
    map.insert(String::from("label"), String::from("AGE"));
    map.insert(String::from("value"), String::from("<undisclosed>"));
    let line2 = ltemp.format(&map).unwrap();

    println!("{}", line1);
    println!("{}", line2);
}
