//! A terminal / text-based "screen" update helper -- [`crate::lblscreen::DumbLineByLineScreen`]

#![deny(warnings)]
#![allow(unused)]

use std::{collections::HashSet, fmt};

use crate::ltemp::DumbLineTemplate;

/// settings for [`DumbLineByLineScreen`]
/// * line_prefix: the prefix to be printed before each formatted line
/// * line_suffix: the suffix to be printed after each formatted line
/// * top_line: the top line to be printed before the formatted lines; note that this line will not be prefixed or suffixed
/// * bottom_line: the bottom line to be printed after the formatted lines; note that this line will not be prefixed or suffixed
/// * screen_height_adjustment:
///   normally not needed since screen height will be automatically,
///   but if the calculation is not correct, can use this to adjust the screen height
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

/// a terminal / text-based "screen" update helper, which relies on [`DumbLineTemplate`] to format each "screen" lines
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
    /// must call [`DumbLineByLineScreen::refresh`]` once afterward
    ///
    /// note that printing will start at the current cursor position; after the first refresh, [`DumbLineByLineScreen`] will take it from there
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
    pub fn refresh<T: LBLScreenMapValueTrait>(&mut self, map_value_provider: &T) {
        self._update(None, map_value_provider)
    }
    /// refresh the screen assuming only the values of the given keys changed
    pub fn refresh_for_keys<T: LBLScreenMapValueTrait>(
        &mut self,
        keys: &Vec<&str>,
        map_value_provider: &T,
    ) {
        if !self.initialized {
            panic!("must call refresh() once first");
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
    fn _update<T: LBLScreenMapValueTrait>(
        &mut self,
        keys: Option<&Vec<&str>>,
        map_value_provider: &T,
    ) {
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
