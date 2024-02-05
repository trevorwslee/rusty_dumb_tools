//#![deny(warnings)]
#![allow(unused)]

mod debug;
//mod demo;
// mod demo_arg;
// mod demo_calc;

use std::{collections::HashMap, env, vec};

use crossterm::style::Colorize;
use rusty_dumb_tools::{
    arg::{self, DumbArgBuilder, DumbArgParser},
    calc::{self, DumbCalcProcessor},
    demo, dlt_comps, dltc,
    ltemp::{LineTempComp, LineTempCompTrait, MappedLineTempCompBuilder},
    sap_arg,
};

// fn test_ltemp_align() {
//     let lt_comps = dlt_comps![
//         "|abc>",
//         dltc!("key1", max_width = 10, align=LineTempCompAlign::Left),
//         "|".to_string(),
//         dltc!("key2", max_width = 10, align=LineTempCompAlign::Left),
//         "|".to_string(),
//         dltc!("key3", max_width = 10, align=LineTempCompAlign::Right),
//         "<ghi|".to_string()
//     ];

//     let ltemp = DumbLineTemplate::new(34, 100, &lt_comps);
//     let mut map = HashMap::new();
//     map.insert(String::from("key1"), String::from("value1"));
//     map.insert(String::from("key2"), String::from("value2"));
//     map.insert(String::from("key3"), String::from("value3"));
//     let formatted = ltemp.format(&map).unwrap();
//     //assert!(formatted.len() >= 34 && formatted.len() <= 100);
//     assert_eq!(formatted, "");
// }

fn main() {
    if false {
        println!("ABC…XYZ...({})", "…".len());
        println!("ABCD?YZ");
        return;
    }

    if false {
        test_chars();
        return;
    }
    // if false {
    //     demo::demo_calculator_gui::handle_demo_calculator_gui();
    //     return;
    // }
    if false {
        demo::run_demo(Some(vec!["calculator", "rich"]));
        //demo::demo_calculator::debug_demo_calculator(true);
        return;
    }
    if false {
        demo::demo_ltemp::show_table("012345678901234567890");
        return;
    }

    // let released = if env::var("CARGO_PKG_NAME").is_ok() {
    //     println!("Running with cargo run");
    //     false
    // } else {
    //     println!("Running as an installed binary");
    //     true
    // };

    // if true {
    //     demo::run_demo(Some(vec!["arg", "-h"]));
    //     return;
    // }

    let released: bool = true;
    if released {
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
        println!("{} :({}):{} ", "1️⃣", "1️⃣".len(), "123".red());
        for i in 0..=9 {
            let v = i + 0x277f;
            let char1 = std::char::from_u32(v).unwrap();
            print!("{} ", char1);
        }
        println!();
    }

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
