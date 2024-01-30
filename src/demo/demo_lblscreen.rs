#![deny(warnings)]
#![allow(unused)]

use std::{collections::HashMap, thread, time::Duration};

use crate::{
    arg::{DumbArgBuilder, DumbArgParser},
    dap_arg,
    lblscreen::{DumbLineByLineScreen, LBLScreenMapValueTrait, LBLScreenSettings},
};

use crate::{
    dlt_comps, dltc,
    ltemp::{DumbLineTemplate, LineTempComp, LineTempCompTrait, MappedLineTempCompBuilder},
};

pub fn handle_demo_lblscreen() {
    let mut lbl_demo_screen = {
        let mut comps = dlt_comps![
            "| ",
            dltc!("description", align = 'C').set_truncate_indicator("..."),
            " |"
        ];
        let temp1 = DumbLineTemplate::new_fixed_width(40, &comps);
        let mut comps = dlt_comps![
            "| ",
            ".".repeat(8),
            " |",
            dltc!("progress-bar"),
            ": ",
            dltc!("progress%", fixed_width = 4, align = 'R'),
            " |"
        ];
        let temp2 = DumbLineTemplate::new_fixed_width(40, &comps);
        let settings = LBLScreenSettings {
            top_line: Some("-".repeat(40)),
            bottom_line: Some("-".repeat(40)),
            screen_height_adjustment: 0,
            ..LBLScreenSettings::default()
        };
        DumbLineByLineScreen::new(vec![temp1, temp2], settings)
    };
    lbl_demo_screen.init();

    let mut state = HashMap::<&str, String>::new();
    let mut progress_done_percent = 0;
    loop {
        let progress_percent = format!("{}%", progress_done_percent);
        let description = format!("... wait ... loading {} ...", progress_percent);
        let progress_bar = ">".repeat(progress_done_percent / 5 as usize);
        state.insert("description", description);
        state.insert("progress-bar", progress_bar);
        state.insert("progress%", progress_percent);
        lbl_demo_screen.refresh(&state);
        thread::sleep(Duration::from_secs(1));
        progress_done_percent += 2;
        if progress_done_percent > 100 {
            break;
        }
    }
}

// fn new_lbl_demo_screen() -> DumbLineByLineScreen {
//     let mut line_temps = Vec::<DumbLineTemplate>::new();

//     let mut comps = dlt_comps![
//         "| ",
//         dltc!("description").set_truncate_indicator("..."),
//         " | ",
//         dltc!("progress-bar", fixed_width = 20),
//         ": ",
//         dltc!("progress%", fixed_width = 4),
//         " |"
//     ];
//     let temp = DumbLineTemplate::new_fixed_width(50, &comps);
//     line_temps.push(temp);

//     DumbLineByLineScreen::new(line_temps, LBLScreenSettings::default())
// }
