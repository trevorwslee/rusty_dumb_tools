#![deny(warnings)]
#![allow(unused)]

use std::{
    io::{Read, Write},
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

pub fn handle_demo_json(parser: DumbArgParser) {
    let country = parser.get::<String>("country").unwrap();
    println!("! query universities of country: [{}] ...", country);
    //let query = format!("search?country={}", country);
    let response = match TcpStream::connect("universities.hipolabs.com:80") {
        Ok(mut stream) => {
            let request = format!(
              "GET /search?country={} HTTP/1.1\r\nHost: universities.hipolabs.com\r\nAccept: application/json\r\nConnection: close\r\n\r\n",
              country.replace(" ", "%20")
          );
            //println!("[\n{}\n]", request);
            match stream.write_all(request.as_bytes()) {
                Ok(_) => {
                    println!("...");
                    let mut response = String::new();
                    match stream.read_to_string(&mut response) {
                        Ok(_) => Ok(response),
                        Err(e) => Err(format!("failed to read response: {}", e)),
                    }
                }
                Err(e) => Err(format!("failed to send request: {}", e)),
            }
        }
        Err(e) => Err(format!("failed to connect: {}", e)),
    };
    match response {
        Ok(response) => {
            let jsons = response
                .chars()
                .skip(response.find("\r\n\r\n").unwrap() + 4)
                .collect::<String>();
            //println!("! jsons: [{}]", jsons);
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
