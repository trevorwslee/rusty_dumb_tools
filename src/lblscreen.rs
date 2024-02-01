//! A simple terminal / text-based "screen" update helper -- [`crate::lblscreen::DumbLineByLineScreen`]

#![deny(warnings)]
#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::ltemp::DumbLineTemplate;

/// settings for [`DumbLineByLineScreen`]
/// * line_prefix: the prefix to be printed before each formatted line
/// * line_suffix: the suffix to be printed after each formatted line
/// * top_line: the top line to be printed before the formatted lines
///   - it will not be prefixed or suffixed
///   - and it may contain newline characters
/// * bottom_line: the bottom line to be printed after the formatted lines
///   - it will not be prefixed or suffixed
///   - and it may contain newline characters
/// * screen_height_adjustment:
///   - normally not needed since screen height will be automatically calculated
///   - but if the calculation is not correct, can use this to adjust the screen height
pub struct LBLScreenSettings {
    pub line_prefix: Option<String>,
    pub line_suffix: Option<String>,
    pub top_line: Option<String>,
    pub bottom_line: Option<String>,
    pub screen_height_adjustment: i32,
}
impl Default for LBLScreenSettings {
    fn default() -> Self {
        Self {
            line_prefix: None,
            line_suffix: None,
            top_line: None,
            bottom_line: None,
            screen_height_adjustment: 0,
        }
    }
}

/// a simple terminal / text-based "screen" update helper, which relies on [`DumbLineTemplate`] to format each "screen" lines
///
/// for example:
/// ```
/// use std::collections::HashMap;
/// use rusty_dumb_tools::{
///     arg::{DumbArgBuilder, DumbArgParser},
///     dap_arg,
///     lblscreen::{DumbLineByLineScreen, LBLScreenMapValueTrait, LBLScreenSettings},
/// };
/// use rusty_dumb_tools::{
///    dlt_comps, dltc,
///    ltemp::{DumbLineTemplate, LineTempComp, LineTempCompTrait, MappedLineTempCompBuilder},
/// };
/// let mut lbl_demo_screen = {
///     /// template for the line that ends up like "|     ... wait ... loading 100% ...    |"
///     let mut comps = dlt_comps![
//         "| ",
///         dltc!("description", align = 'C').set_truncate_indicator("..."),
///         " |"
///     ];
///     let temp1 = DumbLineTemplate::new_fixed_width(40, &comps);
///
///     /// template for the line that ends up like "| ........ |>>>>>>>>>>>>>>>>>>>>: 100% |"
///     let mut comps = dlt_comps![
///         "| ",
///         ".".repeat(8),
///         " |",
///         dltc!("progress-bar"),
///         ": ",
///         dltc!("progress%", fixed_width = 4, align = 'R'),
///         " |"
///     ];
///     let temp2 = DumbLineTemplate::new_fixed_width(40, &comps);
///
///     let settings = LBLScreenSettings {
///         top_line: Some("-".repeat(40)),  // the top line of the "screen"
///         bottom_line: Some("-".repeat(40)),  // the bottom line of the "screen"
///         ..LBLScreenSettings::default()
///     };
///     DumbLineByLineScreen::new(vec![temp1, temp2], settings)
/// };
/// println!("The following is the \"screen\":");
/// lbl_demo_screen.init();
///
/// // setup a map of values for the "screen"
/// let mut state = HashMap::<&str, String>::new();
/// let mut progress_done_percent = 100;
/// let progress_percent = format!("{}%", progress_done_percent);
/// let description = format!("... wait ... loading {} ...", progress_done_percent);
/// let progress_bar = ">".repeat(progress_done_percent / 5 as usize);
/// state.insert("description", description);
/// state.insert("progress-bar", progress_bar);
/// state.insert("progress%", progress_percent);
///
/// lbl_demo_screen.refresh(&state);  // update the "screen" according to the mapped values
/// ```
/// hence, the above code will show:
/// ```none
/// The following is the "screen":
/// ----------------------------------------
/// |     ... wait ... loading 100% ...    |
/// | ........ |>>>>>>>>>>>>>>>>>>>>: 100% |
/// ----------------------------------------
/// ```
///
/// please refer to [`DumbLineTemplate`] for more details on the line formatting of the different lines of the "screen"
pub struct DumbLineByLineScreen {
    line_temps: Vec<DumbLineTemplate>,
    line_prefix: Option<String>,
    line_suffix: Option<String>,
    top_line: Option<String>,
    bottom_line: Option<String>,
    screen_height: usize,
    line_keys: Vec<HashSet<String>>,
    initialized: bool,
}
impl DumbLineByLineScreen {
    /// must call [`DumbLineByLineScreen::init`] after instantiation;
    /// note that printing will start at the current cursor position (likely should be start of a line); as long as the cursor position is not changed externally,
    /// [`DumbLineByLineScreen`] will know where to update which "screen" lines when [`DumbLineByLineScreen::refresh`] / [`DumbLineByLineScreen::refresh_for_keys`] is called
    pub fn new(line_temps: Vec<DumbLineTemplate>, settings: LBLScreenSettings) -> Self {
        let mut line_keys = Vec::new();
        for line_temp in &line_temps {
            let keys = line_temp.scan_for_keys();
            line_keys.push(keys);
        }
        let mut screen_height = line_temps.len() as i32 + settings.screen_height_adjustment;
        if settings.top_line.is_some() {
            screen_height +=
                DumbLineByLineScreen::calc_line_height(settings.top_line.as_ref().unwrap());
        }
        if settings.bottom_line.is_some() {
            screen_height +=
                DumbLineByLineScreen::calc_line_height(settings.bottom_line.as_ref().unwrap());
        }
        Self {
            line_temps,
            line_prefix: settings.line_prefix,
            line_suffix: settings.line_suffix,
            top_line: settings.top_line,
            bottom_line: settings.bottom_line,
            line_keys: line_keys,
            screen_height: screen_height as usize,
            initialized: false,
        }
    }
    /// call it once after instantiation; before call, make sure the cursor is positioned at the top of the screen
    pub fn init(&mut self) {
        if self.initialized {
            panic!("already initialized");
        }
        for i in 0..self.screen_height {
            println!();
        }
        self.initialized = true;
    }
    /// refresh the screen; if only want to refresh when the values of some given keys changed, use [`DumbLineByLineScreen::refresh_for_keys`] instead
    pub fn refresh<T: LBLScreenMapValueTrait>(&self, map_value_provider: &T) {
        if !self.initialized {
            panic!("must call init_screen() once first");
        }
        self._update(None, map_value_provider)
    }
    /// refresh the screen assuming only the values of the given keys changed; if want to refresh the whole screen, use [`DumbLineByLineScreen::refresh`] instead
    pub fn refresh_for_keys<T: LBLScreenMapValueTrait>(
        &self,
        keys: &Vec<&str>,
        map_value_provider: &T,
    ) {
        if !self.initialized {
            panic!("must call init_screen() once first");
        }
        self._update(Some(keys), map_value_provider)
    }
    fn calc_line_height(line: &str) -> i32 {
        let mut height = 1;
        for c in line.chars() {
            if c == '\n' {
                height += 1;
            }
        }
        height
    }
    fn _update<T: LBLScreenMapValueTrait>(&self, keys: Option<&Vec<&str>>, map_value_provider: &T) {
        if self.initialized {
            //let seq = format!("\x1B[{}A", self.screen_height);
            print!("\x1B[{}A", self.screen_height)
        }
        // for each line, keep set of keys, only update the line if key values changed
        let map_value_fn = |key: &str| -> Option<(T::VALUE, u16)> {
            let mapped_value = map_value_provider.map_value(key);
            mapped_value
        };
        if self.top_line.is_some() {
            println!("{}", self.top_line.as_ref().unwrap());
        }
        for (index, line_temp) in self.line_temps.iter().enumerate() {
            let refresh_line = if keys.is_some() {
                let keys = keys.unwrap();
                let line_keys = &self.line_keys[index];
                let mut refresh_line = false;
                for key in keys {
                    if line_keys.contains(*key) {
                        refresh_line = true;
                        break;
                    }
                }
                refresh_line
            } else {
                true
            };
            if refresh_line {
                let line = line_temp.format_ex(map_value_fn).unwrap();
                print!("\x1B[0K");
                if self.line_prefix.is_some() {
                    print!("{}", self.line_prefix.as_ref().unwrap());
                }
                print!("{}", line); // | is the line prefix and suffix
                if self.line_suffix.is_some() {
                    print!("{}", self.line_suffix.as_ref().unwrap());
                }
            }
            println!()
        }
        if self.bottom_line.is_some() {
            println!("{}", self.bottom_line.as_ref().unwrap());
        }
        //self.initialized = true;
    }
}

pub trait LBLScreenMapValueTrait {
    type VALUE: fmt::Display;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, u16)>;
}

impl LBLScreenMapValueTrait for HashMap<&str, String> {
    type VALUE = String;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, u16)> {
        let value = self.get(key);
        if value.is_some() {
            let value = value.unwrap();
            Some((value.clone(), value.len() as u16))
        } else {
            None
        }
    }
}

// use crossterm::cursor;

// let mut cursor = cursor();
// let (x, y) = cursor.pos();

// println!("Cursor position: X: {}, Y: {}", x, y);

// [dependencies]
// crossterm = "0.19"
