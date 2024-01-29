// DumbPrintedScreenUpdater
// * startoff assume @ the right place to start printing ... and screen can be fit in terminal
// * pass in
//   - fixed width
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

pub struct LBLScreenSettings {
    pub width: u16,
    pub line_prefix: Option<String>,
    pub line_suffix: Option<String>,
    pub top_line: Option<String>,
    pub bottom_line: Option<String>,
}
impl Default for LBLScreenSettings {
    fn default() -> Self {
        Self {
            width: 80,
            line_prefix: None,
            line_suffix: None,
            top_line: None,
            bottom_line: None,
        }
    }
}

pub struct DumbLineByLineScreen {
    line_temps: Vec<DumbLineTemplate>,
    width: u16,
    line_prefix: Option<String>,
    line_suffix: Option<String>,
    top_line: Option<String>,
    bottom_line: Option<String>,
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
            //map_value_fn: map_value_fn,
            width: settings.width,
            line_prefix: settings.line_prefix,
            line_suffix: settings.line_suffix,
            top_line: settings.top_line,
            bottom_line: settings.bottom_line,
            initialized: false,
        }
    }
    pub fn refresh<T: LBLScreenMapValueTrait>(&self, map_value_provider: &T) {
        if self.initialized {
            // move cursor up to top first
            unimplemented!();
        }
        self._update(None, map_value_provider)
    }
    /// refresh the screen assuming only the values of the given keys changed
    pub fn refresh_for_keys<T: LBLScreenMapValueTrait>(&self, keys: Vec<String>, map_value_provider: &T) {
        if !self.initialized {
            panic!("must call refresh() once first");
        }
        self._update(Some(keys), map_value_provider)
    }
    fn _update<T: LBLScreenMapValueTrait>(&self, keys: Option<Vec<String>>, map_value_provider: &T) {
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
    }
}

pub trait LBLScreenMapValueTrait {
    type VALUE: fmt::Display;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, u16)>;

}