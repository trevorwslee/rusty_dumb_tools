//! A simple terminal / text-based "screen" update helper -- [`crate::lblscreen::DumbLineByLineScreen`]

#![deny(warnings)]
#![allow(unused)]

use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    fmt,
};

use crate::ltemp::{DumbLineTemplate, LineTempCompMapValueTrait};

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
    //pub screen_height_adjustment: i32,
}
impl Default for LBLScreenSettings {
    fn default() -> Self {
        Self {
            line_prefix: None,
            line_suffix: None,
            top_line: None,
            bottom_line: None,
            //screen_height_adjustment: 0,
        }
    }
}

/// a simple terminal / text-based "screen" update helper, which relies on [`DumbLineTemplate`] to format each "screen" lines
///
/// for example:
/// ```
/// use std::collections::HashMap;
/// use rusty_dumb_tools::prelude::*;
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
/// please refer to [`DumbLineTemplate`] for more details on the line formatting of the different lines of the "screen";
/// for a fuller sample code, please refer to the "calculator" sub-demo of [`crate::demo::run_demo`]
pub struct DumbLineByLineScreen {
    line_temps: Vec<DumbLineTemplate>,
    line_prefix: Option<String>,
    line_suffix: Option<String>,
    top_line: Option<String>,
    bottom_line: Option<String>,
    bottom_line_height: usize,
    //line_start: usize,
    //screen_height: usize,
    line_keys: Vec<HashSet<String>>,
    line_cache: RefCell<Vec<Option<String>>>,
    initialized: bool,
}
impl DumbLineByLineScreen {
    /// must call [`DumbLineByLineScreen::init`] after instantiation;
    /// note that printing will start at the current cursor position (likely should be start of a line); as long as the cursor position is not changed externally,
    /// [`DumbLineByLineScreen`] will know where to update which "screen" lines when [`DumbLineByLineScreen::refresh`] / [`DumbLineByLineScreen::refresh_for_keys`] is called
    pub fn new(line_temps: Vec<DumbLineTemplate>, settings: LBLScreenSettings) -> Self {
        let mut line_keys = Vec::new();
        let mut line_cache = Vec::new();
        for line_temp in &line_temps {
            let keys = line_temp.scan_for_keys();
            line_keys.push(keys);
            line_cache.push(None::<String>);
        }
        let bottom_line_height = if let Some(bottom_line) = &settings.bottom_line {
            DumbLineByLineScreen::calc_line_height(bottom_line)
        } else {
            0
        };
        // let line_start = if let Some(top_line) = &settings.top_line {
        //         DumbLineByLineScreen::calc_line_height(top_line)
        //     } else {0};
        // let mut screen_height = line_start + line_temps.len()/*  + settings.screen_height_adjustment*/;
        // // if let Some(top_line) = &settings.top_line {
        // //     screen_height += //DumbLineByLineScreen::calc_line_height(top_line);
        // // }
        // if let Some(bottom_line) = &settings.bottom_line {
        //     screen_height += DumbLineByLineScreen::calc_line_height(bottom_line);
        // }
        Self {
            line_temps,
            line_prefix: settings.line_prefix,
            line_suffix: settings.line_suffix,
            top_line: settings.top_line,
            bottom_line: settings.bottom_line,
            line_keys: line_keys,
            line_cache: RefCell::new(line_cache),
            bottom_line_height: bottom_line_height,
            //line_start: line_start,
            //screen_height: screen_height,
            initialized: false,
        }
    }
    /// call it once after instantiation; before call, make sure the cursor is positioned at the top of the screen
    pub fn init(&mut self) {
        if self.initialized {
            panic!("already initialized");
        }
        if let Some(top_line) = &self.top_line {
            println!("{}", top_line);
        }
        let line_count = self.line_temps.len();
        for i in 0..line_count {
            println!();
        }
        if let Some(bottom_line) = &self.bottom_line {
            println!("{}", bottom_line);
        }
        self.initialized = true;
    }
    /// refresh the screen; since lines are cached, refresh will not reprint any lines not changed;
    /// nevertheless, for a bit better performance, you can use [`DumbLineByLineScreen::refresh_for_keys`] to refresh only the lines that are affected by the given keys
    ///
    /// e.g.
    /// ```_no_run
    /// let mut state = HashMap::<&str, String>::new();
    /// ...
    /// lbl_demo_screen.refresh(&state);
    /// ````
    pub fn refresh<T: LBLScreenMapValueTrait>(&self, value_mapper: &T) {
        if !self.initialized {
            panic!("must call init_screen() once first");
        }
        let map_value_fn = |key: &str| -> Option<(T::VALUE, u16)> {
            let mapped_value = value_mapper.map_value(key);
            match mapped_value {
                Some(mapped_value) => Some(mapped_value),
                None => None,
            }
        };
        self.refresh_ex(map_value_fn)
    }
    pub fn refresh_ex<T: fmt::Display, F: Fn(&str) -> Option<(T, u16)>>(&self, map_value_fn: F) {
        if !self.initialized {
            panic!("must call init_screen() once first");
        }
        self._update(None, map_value_fn)
    }
    /// refresh the screen assuming only the values of the given keys changed; it will be a bit faster, but in general, simply use[`DumbLineByLineScreen::refresh`] to refresh the whole "screen"
    pub fn refresh_for_keys<T: LBLScreenMapValueTrait>(&self, keys: &Vec<&str>, value_mapper: &T) {
        if !self.initialized {
            panic!("must call init_screen() once first");
        }
        let map_value_fn = |key: &str| -> Option<(T::VALUE, u16)> {
            let mapped_value = value_mapper.map_value(key);
            match mapped_value {
                Some(mapped_value) => Some(mapped_value),
                None => None,
            }
        };
        self.refresh_for_keys_ex(keys, map_value_fn)
    }
    pub fn refresh_for_keys_ex<T: fmt::Display, F: Fn(&str) -> Option<(T, u16)>>(
        &self,
        keys: &Vec<&str>,
        map_value_fn: F,
    ) {
        if !self.initialized {
            panic!("must call init_screen() once first");
        }
        self._update(Some(keys), map_value_fn)
    }
    fn calc_line_height(line: &str) -> usize {
        let mut height = 1;
        for c in line.chars() {
            if c == '\n' {
                height += 1;
            }
        }
        height
    }
    fn _update<T: fmt::Display, F: Fn(&str) -> Option<(T, u16)>>(
        &self,
        keys: Option<&Vec<&str>>,
        map_value_fn: F,
    ) {
        if self.initialized {
            //print!("\x1B[{}A", self.screen_height);
            print!("\x1B[{}A", self.line_temps.len() + self.bottom_line_height);
        }
        // for each line, keep set of keys, only update the line if key values changed
        let map_value_fn = |key: &str| -> Option<(T, u16)> {
            let mapped_value = map_value_fn(key);
            mapped_value
        };
        // if let Some(top_line) = &self.top_line {
        //     println!("{}", top_line);
        // }
        for (index, line_temp) in self.line_temps.iter().enumerate() {
            let refresh_line = if let Some(keys) = keys {
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
                let line = line_temp
                    .format_ex(map_value_fn)
                    .expect(format!("line[{}]", index).as_str());
                let cache = &mut self.line_cache.borrow_mut();
                let cached_line = cache.get_mut(index).unwrap();
                let line = if cached_line.is_none() || cached_line.as_ref().unwrap() != &line {
                    cached_line.replace(line.clone());
                    Some(line)
                } else {
                    None
                };
                if let Some(line) = line {
                    print!("\x1B[0K");
                    if let Some(line_prefix) = &self.line_prefix {
                        print!("{}", line_prefix);
                    }
                    print!("{}", line);
                    if let Some(line_suffix) = &self.line_suffix {
                        print!("{}", line_suffix);
                    }
                }
            }
            println!("\r")
        }
        for i in 0..self.bottom_line_height {
            println!();
        }
        // if let Some(bottom_line) = &self.bottom_line {
        //     println!("{}", bottom_line);
        // }
    }
}

pub trait LBLScreenMapValueTrait {
    type VALUE: fmt::Display;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, u16)>;
}
impl LBLScreenMapValueTrait for HashMap<&str, String> {
    type VALUE = String;
    fn map_value(&self, key: &str) -> Option<(String, u16)> {
        let value = self.get(key);
        if let Some(value) = value {
            //let value = value.unwrap();
            Some((value.clone(), value.len() as u16))
        } else {
            None
        }
    }
}
impl LBLScreenMapValueTrait for HashMap<&str, &str> {
    type VALUE = String;
    fn map_value(&self, key: &str) -> Option<(String, u16)> {
        let value = self.get(key);
        if let Some(value) = value {
            //let value = value.unwrap();
            Some((value.to_string(), value.len() as u16))
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
