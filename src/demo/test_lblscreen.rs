#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::prelude::*;

#[test]
fn test_lblscreen_general() {
    let mut lbl_demo_screen = {
        /// template for the line that ends up like "|     ... wait ... loading 100% ...    |"
        let mut comps = dlt_comps![
            "| ",
            dltc!("description", align = 'C').set_truncate_indicator("..."),
            " |"
        ];
        let temp1 = DumbLineTemplate::new_fixed_width(40, &comps);

        /// template for the line that ends up like "| ........ |>>>>>>>>>>>>>>>>>>>>: 100% |"
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
            top_line: Some("-".repeat(40)),    // the top line of the "screen"
            bottom_line: Some("-".repeat(40)), // the bottom line of the "screen"
            ..LBLScreenSettings::default()
        };
        DumbLineByLineScreen::new(vec![temp1, temp2], settings)
    };
    println!("The following is the \"screen\":");
    lbl_demo_screen.init();

    // setup a map of values for the "screen"
    let mut state = HashMap::<&str, String>::new();
    let mut progress_done_percent = 100;
    let progress_percent = format!("{}%", progress_done_percent);
    let description = format!("... wait ... loading {} ...", progress_done_percent);
    let progress_bar = ">".repeat(progress_done_percent / 5 as usize);
    state.insert("description", description);
    state.insert("progress-bar", progress_bar);
    state.insert("progress%", progress_percent);

    let updated_line_count = lbl_demo_screen.refresh(&state); // update the "screen" according to the mapped values
    assert_eq!(updated_line_count, 2);

    let updated_line_count = lbl_demo_screen.refresh(&state); // update the "screen" according to the mapped values
    assert_eq!(updated_line_count, 0);

    state.insert("description", "???".to_owned());
    let updated_line_count = lbl_demo_screen.refresh(&state); // update the "screen" according to the mapped values
    assert_eq!(updated_line_count, 1);

    let updated_line_count = lbl_demo_screen.refresh(&state); // update the "screen" according to the mapped values
    assert_eq!(updated_line_count, 0);
}
