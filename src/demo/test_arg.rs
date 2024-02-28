#![deny(warnings)]
#![allow(unused)]

use crate::prelude::*;

#[test]
fn test_missing_args() {
    println!("*** MISSING ARGUMENTS ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("i32", value = 0).add_to(&mut parser).unwrap();
    dap_arg!("miss", value = 0).add_to(&mut parser).unwrap();
    let process_res = parser.check_process_args(vec!["123"], true);
    assert_eq!(
        "argument [miss] not provided",
        process_res.unwrap_err().to_string()
    );
}
#[test]
fn test_missing_arg_flags() {
    println!("*** MISSING ARGUMENTS (FLAG) ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-miss", value = 0).add_to(&mut parser);
    dap_arg!("i32", value = 0).add_to(&mut parser);
    let process_res = parser.check_process_args(vec!["123"], true);
    assert_eq!(
        "argument [-miss] not provided",
        process_res.unwrap_err().to_string()
    );
}
#[test]
fn test_allow_missing_arg_flags() {
    println!("*** ALLOW MISSING ARGUMENTS ***");
    let mut parser = DumbArgParser::new();
    parser.set_allow_missing_arguments();
    dap_arg!("-miss1", value = 0).add_to(&mut parser);
    dap_arg!("-miss2", value = "").add_to(&mut parser);
    dap_arg!("i32", value = 0).add_to(&mut parser);
    parser.process_args(vec!["123"]);
    assert!(parser.get::<i32>("-miss1").is_none());
    assert!(parser.get::<String>("-miss2").is_none());
}
#[test]
fn test_invalid_arg() {
    if true {
        println!("*** INVALID ARGUMENT ***");
        let mut parser = DumbArgParser::new();
        dap_arg!("i32", value = 0).add_to(&mut parser);
        let process_res = parser.check_process_args(vec!["abc"], true);
        assert_eq!(
            "failed to parse \"abc\" as i32",
            process_res.unwrap_err().to_string()
        );
        let process_res = parser.check_process_args(vec!["123", "456"], true);
        assert_eq!(
            "unacceptable input argument [456]",
            process_res.unwrap_err().to_string()
        );
    }
}
#[test]
fn test_arg_types() {
    println!("*** ARGUMENT TYPES ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("i32", value = 0 as i32).add_to(&mut parser);
    dap_arg!("i64", value = 0 as i64).add_to(&mut parser);
    dap_arg!("f32", value = 0 as f32).add_to(&mut parser);
    dap_arg!("f64", value = 0 as f64).add_to(&mut parser);
    dap_arg!("bool", value = false).add_to(&mut parser);
    dap_arg!("string", value = "".to_owned()).add_to(&mut parser);
    dap_arg!("string2", value = "STR2").add_to(&mut parser);
    let in_args = vec!["1", "2", "3", "4", "true", "in-string", "in-string2"];
    parser.process_args(in_args);
    assert_eq!(1, parser.get::<i32>("i32").unwrap());
    assert_eq!(2 as i64, parser.get::<i64>("i64").unwrap());
    assert_eq!(3 as f32, parser.get::<f32>("f32").unwrap());
    assert_eq!(4 as f64, parser.get::<f64>("f64").unwrap());
    assert_eq!(true, parser.get::<bool>("bool").unwrap());
    assert_eq!("in-string", parser.get::<String>("string").unwrap());
    assert_eq!("in-string2", parser.get::<String>("string2").unwrap());
    assert_eq!("1", parser.get::<String>("i32").unwrap());
    assert_eq!("2", parser.get::<String>("i64").unwrap());
    assert_eq!("3", parser.get::<String>("f32").unwrap());
    assert_eq!("4", parser.get::<String>("f64").unwrap());
    assert_eq!("true", parser.get::<String>("bool").unwrap());
    let result = std::panic::catch_unwind(|| {
        let param: i32 = parser.get("f64").unwrap();
        std::process::exit(-1); // should not reach here
    });
    assert_eq!("1", parser.get_string("i32").unwrap());
    assert_eq!("2", parser.get_string("i64").unwrap());
    assert_eq!("3", parser.get_string("f32").unwrap());
    assert_eq!("4", parser.get_string("f64").unwrap());
    assert_eq!("true", parser.get_string("bool").unwrap());
    assert!(result.is_err());
}
#[test]
fn test_arg_convert() {
    println!("*** ARG CONVERT ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("arg").add_to(&mut parser);
    dap_arg!("arg_i32", value = 1_i32).add_to(&mut parser);
    dap_arg!("arg_i64", value = 1_i64).add_to(&mut parser);
    dap_arg!("arg_f32", value = 1_f32).add_to(&mut parser);
    dap_arg!("arg_f64", value = 1_f64).add_to(&mut parser);
    parser.process_args(vec!["A", "1", "2", "3.0", "4.0"]);
    assert_eq!("A", parser.get::<String>("arg").unwrap());
    assert_eq!(1, parser.get::<i32>("arg_i32").unwrap());
    assert_eq!(1, parser.get::<i64>("arg_i32").unwrap()); // i32 => i64
    assert_eq!(1.0, parser.get::<f64>("arg_i32").unwrap()); // i32 => f64
    assert_eq!("1", parser.get::<String>("arg_i32").unwrap()); // i32 => String
    assert_eq!(2, parser.get::<i64>("arg_i64").unwrap());
    assert_eq!("2", parser.get::<String>("arg_i64").unwrap()); // i64 => String
    assert_eq!(3.0, parser.get::<f32>("arg_f32").unwrap());
    assert_eq!(3.0, parser.get::<f64>("arg_f32").unwrap()); // f32 => f64
    assert_eq!("3", parser.get::<String>("arg_f32").unwrap()); // f32 => String
    assert_eq!(4.0, parser.get::<f64>("arg_f64").unwrap());
    assert_eq!("4", parser.get::<String>("arg_f64").unwrap()); // f64 => String
}
#[test]
fn test_positional_args() {
    println!("*** POSITIONAL ARGUMENTS ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("i32", value = 0).add_to(&mut parser);
    dap_arg!("string", value = "").add_to(&mut parser);
    dap_arg!("bool", value = false).add_to(&mut parser);
    dap_arg!("i64", default = 888 as i64).add_to(&mut parser);
    let in_args = vec!["123", "string", "true"];
    parser.process_args(in_args);
    let i32_param: i32 = parser.get("i32").unwrap();
    let string_param: String = parser.get("string").unwrap();
    let bool_param: bool = parser.get("bool").unwrap();
    let i64_param: i64 = parser.get("i64").unwrap();
    assert_eq!(123, i32_param);
    assert_eq!("string", string_param);
    assert_eq!(true, bool_param);
    assert_eq!(888, i64_param);
    assert_eq!("123", parser.get::<String>("i32").unwrap());
    assert_eq!("true", parser.get::<String>("bool").unwrap());
    assert_eq!("888", parser.get::<String>("i64").unwrap());
    assert!(parser.get::<i32>("abc").is_none());
}
#[test]
fn test_flag_args() {
    println!("*** FLAG ARGUMENTS ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-a", flag2 = "--A", value = 0).add_to(&mut parser);
    dap_arg!("-b", flag2 = "--B", fixed = 0).add_to(&mut parser);
    dap_arg!("-c", flag2 = "--C", fixed = 999).add_to(&mut parser);
    dap_arg!("string", value = "").add_to(&mut parser);
    dap_arg!("string2", value = "").add_to(&mut parser);
    dap_arg!("i32", default = 888).add_to(&mut parser);
    let in_args: Vec<&str> = vec!["--C", "ABC", "-a", "123", "DEF"]; // flags can be before or after positional
    parser.process_args(in_args);
    assert_eq!(123, parser.get::<i32>("-a").unwrap());
    assert_eq!(123, parser.get::<i32>("--A").unwrap());
    assert_eq!(None, parser.get::<i32>("-b"));
    assert_eq!(999, parser.get::<i32>("-c").unwrap());
    assert_eq!(999, parser.get::<i32>("--C").unwrap());
    assert_eq!("ABC", parser.get::<String>("string").unwrap());
    assert_eq!("DEF", parser.get::<String>("string2").unwrap());
    assert_eq!(888, parser.get::<i32>("i32").unwrap());
}
#[test]
fn test_arg_range() {
    println!("*** ARGUMENT RANGE ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f", value = 0)
        .set_range(100, 200)
        .add_to(&mut parser);
    parser.process_args(vec!["-f", "123"]);
    assert_eq!(123, parser.get::<i32>("-f").unwrap());
    if true {
        let process_res = parser.check_process_args(vec!["-f", "1000"], true);
        assert_eq!(
            "[1000] is out of range [100, 200]",
            process_res.unwrap_err().to_string()
        );
        let process_res = parser.check_process_args(vec!["-f", "50"], true);
        assert_eq!(
            "[50] is out of range [100, 200]",
            process_res.unwrap_err().to_string()
        );
    }
}
#[test]
fn test_arg_string_range() {
    println!("*** ARGUMENT [STRING] RANGE ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f").set_range("bbb", "ddd").add_to(&mut parser);
    parser.process_args(vec!["-f", "ccc"]);
    assert_eq!("ccc", parser.get::<String>("-f").unwrap());
    if true {
        let process_res = parser.check_process_args(vec!["-f", "aaa"], true);
        assert_eq!(
            "[aaa] is out of range [bbb, ddd]",
            process_res.unwrap_err().to_string()
        );
        let process_res = parser.check_process_args(vec!["-f", "eee"], true);
        assert_eq!(
            "[eee] is out of range [bbb, ddd]",
            process_res.unwrap_err().to_string()
        );
    }
}
#[test]
fn test_arg_enum() {
    println!("*** ARGUMENT ENUM ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f", value = 0)
        .set_enums(vec![100, 200])
        .add_to(&mut parser);
    parser.process_args(vec!["-f", "200"]);
    assert_eq!(200, parser.get::<i32>("-f").unwrap());
    if true {
        let process_res = parser.check_process_args(vec!["-f", "1000"], true);
        assert_eq!(
            "[1000] doesn't match any of the enum values [100, 200]",
            process_res.unwrap_err().to_string()
        );
    }
}
#[test]
fn test_arg_string_enum() {
    println!("*** ARGUMENT [STRING] ENUM ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f").set_enums(vec!["A", "B"]).add_to(&mut parser);
    parser.process_args(vec!["-f", "A"]);
    assert_eq!("A", parser.get::<String>("-f").unwrap());
    if true {
        let process_res = parser.check_process_args(vec!["-f", "aaa"], true);
        assert_eq!(
            "[aaa] doesn't match any of the enum values [A, B]",
            process_res.unwrap_err().to_string()
        );
    }
}
#[test]
fn test_args() {
    println!("*** ARGUMENTS ***");
    let mut parser = DumbArgParser::new_with_name("pgm");
    dap_arg!("str-arg").add_to(&mut parser); // positional argument "str-arg" (of type String)
    dap_arg!("-v", flag2 = "--v", fixed = true).add_to(&mut parser); // argument flag "-v" / "--v" with fixed value (false)
    dap_arg!("-v2", flag2 = "--v2", fixed = true).add_to(&mut parser); // argument flag "-v" / "--v" with fixed value (false)
    dap_arg!("-name", default = "nobody").add_to(&mut parser); // argument "-name" requiring value, with default "unknown"
    dap_arg!("i32-arg").value(123).add_to(&mut parser); // positional argument "i32-arg" of type i32 (inferred from the value 123)
    let process_res = parser.check_process_args(vec!["--v2", "STR-ARG", "999"], true);
    assert!(process_res.is_ok() && process_res.unwrap());
    println!(". str-arg: {:?}", parser.get::<String>("str-arg"));
    println!(". i32-arg: {:?}", parser.get::<i32>("i32-arg"));
    println!(". -v: {:?}", parser.get::<bool>("-v"));
    println!(". --v: {:?}", parser.get::<bool>("--v")); // will be the same parameter value as "-v"
    println!(". string: {:?}", parser.get::<String>("-name"));
    let usage = parser.compose_usage();
    let parameters = parser.compose_inputs();
    assert_eq!("pgm [-h] [-v] [-v2] [-name name] str-arg i32-arg", usage);
    assert_eq!("STR-ARG -v2 -name nobody 999", parameters);
}
#[test]
fn test_multi_arg() {
    println!("*** MULTI-ARGUMENT ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f", flag2 = "--F", default = 20)
        .set_enums(vec![1, 20, 300])
        .set_multi()
        .add_to(&mut parser);
    parser.process_args(vec![]);
    assert_eq!(vec![20], parser.get_multi::<i32>("--F").unwrap());
    assert_eq!(vec!["20"], parser.get_multi_strings("--F").unwrap());
    parser.process_args(vec!["-f", "20", "1", "300"]);
    assert_eq!(vec![20, 1, 300], parser.get_multi::<i32>("--F").unwrap());
    if true {
        let process_res = parser.check_process_args(vec!["-f", "20", "123", "300"], true);
        assert_eq!(
            "[123] doesn't match any of the enum values [1, 20, 300]",
            process_res.unwrap_err().to_string()
        );
    }
}
#[test]
fn test_multi_arg_string() {
    println!("*** MULTI-ARGUMENT (STRING) ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f", flag2 = "--F", default = 777).add_to(&mut parser);
    dap_arg!("str", value = "S").set_multi().add_to(&mut parser);
    parser.process_args(vec!["A", "B", "C"]);
    assert_eq!(777, parser.get::<i32>("--F").unwrap());
    assert_eq!(
        vec!["A", "B", "C"],
        parser.get_multi::<String>("str").unwrap()
    );
}
#[test]
fn test_rest_multi_arg() {
    println!("*** REST MULTI-ARGUMENT ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f", flag2 = "--F", default = 777).add_to(&mut parser);
    dap_arg!("str", value = 888).set_rest().add_to(&mut parser);
    parser.process_args(vec!["888", "A", "B", "C"]);
    assert_eq!(777, parser.get::<i32>("--F").unwrap());
    assert_eq!(888, parser.get::<i32>("str").unwrap());
    assert_eq!(vec!["A", "B", "C"], parser.get_rest("str").unwrap());
}
#[test]
fn test_rest_parsed() {
    println!("*** REST PARSED ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f", flag2 = "--F", default = 777).add_to(&mut parser);
    dap_arg!("a", value = 888).set_rest().add_to(&mut parser);
    parser.process_args(vec!["888", "--name", "trevor"]);
    assert_eq!(777, parser.get::<i32>("--F").unwrap());
    assert_eq!(888, parser.get::<i32>("a").unwrap());
    assert_eq!(vec!["--name", "trevor"], parser.get_rest("a").unwrap());
    let mut sub_parser = DumbArgParser::new();
    sap_arg!("-n", "--name").value("").add_to(&mut sub_parser);
    parser.process_rest_args("a", &mut sub_parser);
    assert_eq!("trevor", sub_parser.get::<String>("--name").unwrap());
}

#[test]
fn test_overall() {
    println!("*** -- OVERALL ***");
    let mut parser = DumbArgParser::new_with_name("pgm");
    dap_arg!("-f", flag2 = "--F", default = 777).add_to(&mut parser);
    dap_arg!("-v", flag2 = "--V", fixed = true).add_to(&mut parser);
    dap_arg!("--").set_multi().add_to(&mut parser); // multi-argument "--" that capture the rest of the arguments after "--"
    parser.process_args(vec!["-v", "--", "-v", "--", "ABC"]);
    assert_eq!(777, parser.get::<i32>("--F").unwrap());
    assert_eq!(true, parser.get::<bool>("--V").unwrap());
    assert_eq!(
        vec!["-v", "--", "ABC"],
        parser.get_multi::<String>("--").unwrap()
    );
    assert_eq!("pgm [-h] [-f F] [-v] -- ", parser.compose_usage());
    assert_eq!("-f 777 -v -- -v", parser.compose_inputs());
}
#[test]
fn test_arg__() {
    println!("*** -- ARGUMENT ***");
    let mut parser = DumbArgParser::new();
    dap_arg!("-f", flag2 = "--F", default = 777).add_to(&mut parser);
    dap_arg!("-v", flag2 = "--V", fixed = true).add_to(&mut parser);
    dap_arg!("str", default = "DEF")
        .set_multi()
        .add_to(&mut parser);
    parser.process_args(vec![]);
    assert_eq!("-f 777 DEF", parser.compose_inputs());
    assert_eq!(777, parser.get::<i32>("--F").unwrap());
    assert!(parser.get::<bool>("--V").is_none());
    assert_eq!("DEF", parser.get::<String>("str").unwrap());
    parser.process_args(vec!["-v", "a", "-", "b"]);
    assert_eq!(777, parser.get::<i32>("--F").unwrap());
    assert_eq!(true, parser.get::<bool>("--V").unwrap());
    assert_eq!("a", parser.get::<String>("str").unwrap());
    assert_eq!(
        vec!["a", "-", "b"],
        parser.get_multi::<String>("str").unwrap()
    );
}
