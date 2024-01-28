#![deny(warnings)]
#![allow(unused)]

use std::{collections::HashMap, thread, time::Duration};

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
        dltc!("label", fixed_width = 4, align = 'L'),
        " : ",
        dltc!("value", align = 'R'),
        " |"
    ];
    let ltemp = DumbLineTemplate::new(31, 31, &lt_comps);

    let mut map = HashMap::new();
    map.insert(String::from("label"), String::from("NAME"));
    map.insert(String::from("value"), name.to_string());
    let line1 = ltemp.format(&map).unwrap();

    let mut map = HashMap::new();
    map.insert(String::from("label"), String::from("AGE"));
    map.insert(String::from("value"), String::from("<undisclosed>"));
    let line2 = ltemp.format(&map).unwrap();

    let mut map = HashMap::new();
    map.insert(String::from("label"), String::from(""));
    map.insert(String::from("value"), String::from("and counting ..."));
    let line3 = ltemp.format(&map).unwrap();

    println!("{}", "=".repeat(31));
    println!("{}", line1);
    println!("{}", line2);
    println!("{}", line3);
    println!();
    println!("{}", "=".repeat(31));

    let lt_comps = dlt_comps![
        "| +",
        dltc!("val", fixed_width = 3, align = 'R'),
        " | ",
        dltc!("bar", fixed_width = 20),
        " |"
    ];
    let ltemp = DumbLineTemplate::new(31, 31, &lt_comps);

    for i in 1..=20 {
        let mut map: HashMap<String, String> = HashMap::new();
        map.insert(String::from("bar"), "#".repeat(i));
        map.insert(String::from("val"), i.to_string());
        let line = ltemp.format(&map).unwrap();
        print!("\x1B[1A");
        print!("\x1B[1A");
        print!("{}", line);
        println!();
        println!();
        thread::sleep(Duration::from_secs(1));
    }
}
