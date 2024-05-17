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
    dap_arg!("country", default = "united kingdom")
        .set_description("country in which to query universities for")
        .add_to(&mut parser)
        .unwrap();
    parser
}

fn make_connection(country: &String) -> Result<TcpStream, Error> {
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
fn process_connection(stream: &mut TcpStream) -> Result<(), String> {
    let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
        println!(
            "* `{}` => `{}`",
            json_entry.field_name, json_entry.field_value
        );
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
                return Err(format!("! error: [{}]", e));
            }
        }
    }
    //Ok(())
}
pub fn handle_demo_json(parser: DumbArgParser) {
    let country = parser.get::<String>("country").unwrap();
    println!("! query universities of country: [{}] ...", country);
    if true {
        let stream = make_connection(&country);
        println!("!");
        println!("!");
        let result = match stream {
            Ok(mut stream) => process_connection(&mut stream),
            Err(e) => Err(format!("! error: [{}]", e)),
        };
        println!("!");
        println!("!");
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
    } else {
        match make_connection_get_response(&country) {
            Ok(response) => {
                let jsons = response
                    .chars()
                    .skip(response.find("\r\n\r\n").unwrap() + 4)
                    .collect::<String>();
                //println!("let jsons = r#\"{}\"#;", jsons);
                let mut handler = InPlaceJsonEntryHandler::new(|json_entry| {
                    println!(
                        "* `{}` => `{}`",
                        json_entry.field_name, json_entry.field_value
                    );
                });
                let mut json_processor = DumbJsonProcessor::new(Box::new(&mut handler));
                if true {
                    let mut input = String::new();
                    input.push_str("{\"universiunities\":");
                    input.push_str(&jsons);
                    input.push_str("}");
                    //println!("{}", input);
                    json_processor.push_json(&input);
                } else {
                    // TODO: the following is very slow ... find out why
                    let mut progress = ProcessJsonProgress::new();
                    let mut input = jsons;
                    loop {
                        json_processor.push_json_piece(&input, &mut progress);
                        if progress.is_done() {
                            input = progress.get_remaining();
                            if input.is_empty() {
                                break;
                            }
                        }
                        println!("... more jsons ...");
                    }
                }
            }
            Err(e) => {
                println!("! error: [{}]", e);
            }
        }
    }
}
