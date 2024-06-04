#![deny(warnings)]
#![allow(unused)]

mod debug;
//mod demo;
// mod demo_arg;
// mod demo_calc;

use std::{collections::HashMap, env, vec};

//use crossterm::style::Colorize;
use rusty_dumb_tools::{
    demo::{self, demo_arg::arg_parser_sample},
    json,
    prelude::*,
    progress,
};

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    println!("The version of this crate is: {}", version);

    if true {
        progress::debug_progress(false, 1000, 2);
        progress::debug_progress_single(false, 1000);
        progress::debug_progress_single(true, 1000);
        progress::debug_progress_single(false, 1000);
        return;
    }

    // if true {
    //     test_query_universities(false);
    //     return;
    // }

    if false {
        arg_parser_sample(false);
        return;
    }

    if false {
        println!("ABC…XYZ...({})", "…".len());
        println!("ABCD?YZ");
        return;
    }

    if false {
        test_chars();
        return;
    }
    if false {
        demo::run_demo(Some(vec!["calculator", "rich"]));
        //demo::demo_calculator::debug_demo_calculator(true);
        return;
    }
    if false {
        demo::demo_ltemp::show_table("012345678901234567890");
        return;
    }

    if true {
        released_main();
    } else {
        debug_main();
    }
}

fn released_main() {
    // e.g. cargo run -- calc 1.1 + 2.2 * (4.3 - 2.4) + 5
    // e.g. cargo run -- arg -f 0.2 5 --string2 VAL1 false 1 2 3
    let debug = false;
    let in_args = if debug {
        let in_args = vec![
            "arg",
            "-f",
            "0.2",
            "5",
            "--string2",
            "VAL1",
            "false",
            "1",
            "2",
            "3",
        ];
        Some(in_args)
    } else {
        None
    };
    demo::run_demo(in_args);
}

fn test_chars() {
    if true {
        let c = '✖' as u32 - 1000;
        for i in 0..=2000 {
            let v = c + i;
            let char1 = std::char::from_u32(v).unwrap();
            print!("{} ", char1);
        }
        println!();
    }
    if true {
        let c = '０' as u32 - 1000;
        for i in 0..=2000 {
            let v = c + i;
            let char1 = std::char::from_u32(v).unwrap();
            print!("{} ", char1);
        }
        println!();
    }
    if false {
        println!("0️⃣ 1️⃣ 2️⃣ 3️⃣ 4️⃣ 5️⃣ 6️⃣ 7️⃣ 8️⃣ 9️⃣ ");
        println!("０１２３４５６７８９");
        println!("𝟎 𝟏 𝟐 𝟑 𝟒 𝟓 𝟔 𝟕 𝟖 𝟗");
        println!("𝟘 𝟙 𝟚 𝟛 𝟜 𝟝 𝟞 𝟟 𝟠 𝟡");
        println!("⓪①②③④⑤⑥⑦⑧⑨");
        println!("0123456789");
        println!("🇦🇨|±|+|=|🇽|÷|=|");
    }

    // 1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣
    // ±
    // ➀ (U+2780)

    // let v = 0x1F600;
    // let character = std::char::from_u32(v).unwrap();
    // let string = character.to_string();

    // // Split the string into grapheme clusters
    // let graphemes: Vec<&str> = string.graphemes(true).collect();

    // // Print each grapheme cluster
    // for grapheme in graphemes {
    //     println!("{}", grapheme);
    // }

    // use unicode_segmentation::UnicodeSegmentation;

    // let string = "😄👋🏽";
    // let graphemes: Vec<&str> = string.graphemes(true).collect();
    // let num_graphemes = graphemes.len();

    // println!("Number of graphemes: {}", num_graphemes); // Output: Number of graphemes: 3

    // [dependencies]
    // unicode-segmentation = "1.8.0"

    if false {
        // ➕➖✖️➗🟰🇦🇨▪%±
        println!("±\u{2780}ABC1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣ABC±\u{2780}");
        let v = 0x1F600;
        let character = std::char::from_u32(v).unwrap();
        let string = character.to_string();
        println!("{}", string); // Output: 😄
        let a = '😄' as u32;
        let a_char = std::char::from_u32(a).unwrap();
        println!("A:{}", a_char);
    }
}

fn debug_main() {
    //calc::debug_calc();
    debug::debug_calc_processor();

    //arg::debug_arg();

    debug::debug_dumb_arg_parser();
}
