//! core [`crate::json`] sub-demo code

#![deny(warnings)]
#![allow(unused)]

use std::{
    io::{BufRead, Error, Read, Write},
    net::TcpStream,
};

use crate::prelude::*;

pub fn create_demo_json_parser() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("DumbCalcProcessor command-line input demo.");
    dap_arg!("country", default = "hong kong")
        .set_description(
            "country of info about universities queried via API http://universities.hipolabs.com/",
        )
        .add_to(&mut parser)
        .unwrap();
    dap_arg!("-a", flag2 = "--all", fixed = true)
        .set_description("show all fields")
        .add_to(&mut parser)
        .unwrap();
    parser
}
pub fn handle_demo_json(parser: DumbArgParser) {
    let country = parser.get::<String>("country").unwrap();
    let show_all = parser.get_or_default::<bool>("-a", false);
    println!("*** query universities of country: [{}] ...", country);
    demo_query_universities(&country, show_all);
}

pub fn demo_query_universities(country: &str, show_all: bool) {
    let stream = make_connection(&country);
    let result = match stream {
        Ok(mut stream) => process_connection(&mut stream, show_all),
        Err(e) => Err(format!("XXX error: [{}]", e)),
    };
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn make_connection(country: &str) -> Result<TcpStream, Error> {
    let mut stream: TcpStream = TcpStream::connect("universities.hipolabs.com:80")?;
    let request = format!(
        "GET /search?country={} HTTP/1.1\r\nHost: universities.hipolabs.com\r\nAccept: application/json\r\nConnection: close\r\n\r\n",
        country.replace(" ", "%20")
    );
    stream.write_all(request.as_bytes())?;
    Ok(stream)
}
fn make_connection_get_response(country: &String) -> Result<String, Error> {
    match make_connection(country) {
        Ok(mut stream) => {
            let mut response = String::new();
            stream.read_to_string(&mut response)?;
            Ok(response)
        }
        Err(e) => Err(e),
    }
}
fn process_connection(stream: &mut TcpStream, show_all: bool) -> Result<(), String> {
    let mut handler = InPlaceJsonEntryHandler::new(move |json_entry| {
        let show = show_all || json_entry.field_name == "name";
        if show {
            println!(
                "* `{}` => `{}`",
                json_entry.field_name, json_entry.field_value
            );
        }
    });
    let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
    let mut progress = ProcessJsonProgress::new();
    let mut buf = [0; 32];
    loop {
        match stream.read(&mut buf) {
            Ok(size) => {
                if size == 0 {
                    return Ok(());
                }
                let bytes = &buf[..size];
                json_processor.push_json_bytes(bytes, &mut progress);
            }
            Err(e) => {
                return Err(format!("XXX error: [{}]", e));
            }
        }
    }
}
