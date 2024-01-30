// DumbPrintedScreenUpdater
// * startoff assume @ the right place to start printing ... and screen can be fit in terminal
// * pass in
//   - optional line prefix ... printed before printing every line,
//   - optional line suffix ... printed after printing every line,
//   - optional top and bottom lines (will be printed with prefix and suffix)
// * pass in a list of DumbLineTemplate ... one per line
// * pass in map_value_fn
// * can update all lines
// * can update for certain changed keys
//   - for each line, keep set of keys, only update the line if key values changed

#![deny(warnings)]
#![allow(unused)]

use std::fmt;

use crate::ltemp::DumbLineTemplate;

/// * line_prefix: the prefix to be printed before each formatted line
/// * line_suffix: the suffix to be printed after each formatted line
/// * top_line: the top line to be printed before the formatted lines; note that this line will not be prefixed or suffixed
/// * bottom_line: the bottom line to be printed after the formatted lines; note that this line will not be prefixed or suffixed
/// * screen_height_adjustment:
///   normally not needed, but if top_line and/or bottom_line contains newlines, then this adjusts the screen height that is calculated automatically
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

pub struct DumbLineByLineScreen {
    line_temps: Vec<DumbLineTemplate>,
    line_prefix: Option<String>,
    line_suffix: Option<String>,
    top_line: Option<String>,
    bottom_line: Option<String>,
    screen_height_adjustment: i32,
    initialized: bool,
}
impl DumbLineByLineScreen {
    /// must call [`DumbLineByLineScreen::refresh`]` once afterward
    ///
    /// note that printing will start at the current cursor position; after the first refresh, [`DumbLineByLineScreen`] will take it from there
    pub fn new(
        line_temps: Vec<DumbLineTemplate>,
        settings: LBLScreenSettings,
    ) -> Self {
        Self {
            line_temps,
            line_prefix: settings.line_prefix,
            line_suffix: settings.line_suffix,
            top_line: settings.top_line,
            bottom_line: settings.bottom_line,
            screen_height_adjustment: settings.screen_height_adjustment,
            initialized: false,
        }
    }
    pub fn refresh<T: LBLScreenMapValueTrait>(&mut self, map_value_provider: &T) {
        if self.initialized {
            // move cursor up to top first
            let mut by = self.line_temps.len() as i32 + self.screen_height_adjustment;
            if self.top_line.is_some() {
                by += 1;
            }
            if self.bottom_line.is_some() {
                by += 1;
            }
            let seq = format!("\x1B[{}A", by);
            print!("{}", seq)
        }
        self._update(None, map_value_provider)
    }
    /// refresh the screen assuming only the values of the given keys changed
    pub fn refresh_for_keys<T: LBLScreenMapValueTrait>(&mut self, keys: Vec<String>, map_value_provider: &T) {
        if !self.initialized {
            panic!("must call refresh() once first");
        }
        self._update(Some(keys), map_value_provider)
    }
    fn _update<T: LBLScreenMapValueTrait>(&mut self, keys: Option<Vec<String>>, map_value_provider: &T) {
        // for each line, keep set of keys, only update the line if key values changed
        let map_value_fn = |key: &str| -> Option<(T::VALUE, u16)> {
            let mapped_value = map_value_provider.map_value(key);
            mapped_value
        };
        if self.top_line.is_some() {
            println!("{}", self.top_line.as_ref().unwrap());
        }
        for line_temp in &self.line_temps {
            let line = line_temp.format_ex(map_value_fn).unwrap();
            if self.line_prefix.is_some() {
                print!("{}", self.line_prefix.as_ref().unwrap());
            }
            print!("{}", line); // | is the line prefix and suffix
            if self.line_suffix.is_some() {
                print!("{}", self.line_suffix.as_ref().unwrap());
            }
            println!()
        }
        if self.bottom_line.is_some() {
            println!("{}", self.bottom_line.as_ref().unwrap());
        }
        self.initialized = true;
    }
}

pub trait LBLScreenMapValueTrait {
    type VALUE: fmt::Display;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, u16)>;

}


// use crossterm::cursor;

// let mut cursor = cursor();
// let (x, y) = cursor.pos();

// println!("Cursor position: X: {}, Y: {}", x, y);


// [dependencies]
// crossterm = "0.19"
