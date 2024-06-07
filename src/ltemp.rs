//! A simple line template for formatting a line, which can be use for printing values as a line with some template -- [`crate::ltemp::DumbLineTemplate`]

#![deny(warnings)]
#![allow(unused)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::manual_map)]
#![allow(clippy::upper_case_acronyms)]

use core::fmt;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    error::Error,
    path::Display,
};

use crate::arg::DumbArgBuilder;

/// for internal use only
pub const FLEXIBLE_WIDTH_EX: bool = true;
//pub const FLEXIBLE_WIDTH: bool = false;

type WIDTH = u16;

/// use this macro to compose [`DumbLineTemplate`] components
///
/// for example that also involve ASCII escaped strings:
/// ```
/// use rusty_dumb_tools::prelude::*;
/// use std::collections::HashMap;
/// let name = "Trevor Lee";
/// let lt_comps = dlt_comps![
///     "| ",
///     ("{1b}[7m(\u{1b}[0m", 1),
///     dltc!("key", align='C'),
///     ("{1b}[7m)\u{1b}[0m", 1),
///     " |"
/// ];
/// let ltemp = DumbLineTemplate::new_fixed_width(15, &lt_comps);
/// let line = ltemp.format(&HashMap::from([("key", String::from("value"))])).unwrap();
/// assert_eq!(line, "| {1b}[7m(\u{1b}[0m  value  {1b}[7m)\u{1b}[0m |");
/// ```
/// notes:
/// * `"| "`: a fixed string
/// * `("{1b}[7m(\u{1b}[0m", 1)`:
///   - `{1b}[7m(\u{1b}[0m` is the ASCII escaped string for `(`
///   -  the `1` specifies that the ASCII escaped string has "visual" length 1
/// * `dltc!("key", align='C')`:
///   - a value-mapped component
///   - require a mapped value for key `key` when calling [`DumbLineTemplate::format`]
///   - also see the macro [`crate::dltc!`]
#[macro_export]
macro_rules! dlt_comps {
  ($($x:expr$(,)?)*) => {{
    let mut comps: Vec<LineTempComp> = Vec::new();
    $(
      let comp = $x.to_line_temp_comp();
      comps.push(comp);
    )*
    comps
  }};
}

/// use this macro to construct a value-mapped [`DumbLineTemplate`] component (a [`MappedLineTempCompBuilder`] to be precise), and it is expected to be use together with [`dlt_comps!`]
///
/// the compulsory argument is the key for the line template component;
/// the other optional "ordered but named" arguments are
/// * `fixed_width` - like calling [`MappedLineTempCompBuilder::fixed_width`]
/// * `min_width` - like calling [`MappedLineTempCompBuilder::min_width`]
/// * `max_width` - like calling [`MappedLineTempCompBuilder::max_width`]
/// * `align` - like calling [`MappedLineTempCompBuilder::align`]
/// * `optional` - like calling [`MappedLineTempCompBuilder::optional`]
#[macro_export]
macro_rules! dltc {
    ($x:expr
        $(, fixed_width=$fixed_width:expr)?
        $(, min_width=$min_width:expr)?
        $(, max_width=$max_width:expr)?
        $(, align=$align:expr)?
        $(, optional=$optional:expr)?
        ) => {{
      let mut builder = MappedLineTempCompBuilder::new($x);
      $(builder.optional($optional);)?
      $(builder.fixed_width($fixed_width);)?
      $(builder.min_width($min_width);)?
      $(builder.max_width($max_width);)?
      $(builder.align($align);)?
      builder
    }};
}

// #[macro_export]
// macro_rules! dltc_escaped {
//     ($val:expr, $len:expr) => {{
//         EscapedLineTempComp::new($val.to_string(), $len as u16)
//     }};
//     () => {

//     };
// }

#[test]
fn debug_ltemp() {
    let key1_temp = MappedLineTempCompBuilder::new("key1")
        .optional(true)
        .min_width(3)
        .max_width(5)
        .build();
    println!("key1_temp: {:?}", key1_temp);

    let lt_comps = dlt_comps![
        "abc",
        dltc!("key1"),
        "def".to_owned(),
        dltc!("key2", min_width = 1, max_width = 10, optional = true)
    ];
    println!("lt_comps: {:?}", lt_comps);

    let ltemp = DumbLineTemplate::new(0, 100, &lt_comps);
    println!("ltemp: {:?}", ltemp);

    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
    map.insert("key2", String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    println!("formatted: [{}]", formatted);

    let formatted = DumbLineTemplate::new_fixed_width(
        50,
        &dlt_comps![
            "abc",
            dltc!("key1"),
            "def".to_owned(),
            dltc!("key2", min_width = 1, max_width = 10, optional = true)
        ],
    )
    .format(&HashMap::from([
        ("key1", String::from("value1")),
        ("key2", String::from("value2")),
    ]))
    .unwrap();
    println!("formatted: [{}]", formatted);
}

/// A simple line template for formatting a line; it can be use for printing values as a line with some template.
///
/// Example:
/// ```
/// use rusty_dumb_tools::prelude::*;
/// use std::collections::HashMap;
///
/// // create the template components
/// let lt_comps = dlt_comps![
///     "| ",
///     dltc!("label", fixed_width = 6, align = 'L'),
///     " : ",
///     dltc!("value", align = 'R'),
///     " |"
/// ];
///
/// // create the template
/// let ltemp = DumbLineTemplate::new_fixed_width(30, &lt_comps);
///
/// // format line1 from the template
/// let name = "Trevor Lee";
/// let map = HashMap::from([
///   ("label", String::from("NAME")),
///   ("value", name.to_string()),
/// ]);
/// let line1 = ltemp.format(&map).unwrap();
///
/// // format line2 from the template
/// let map = HashMap::from([
///  ("label", String::from("AGE")),
///  ("value", String::from("<undisclosed>")),
/// ]);
/// let line2 = ltemp.format(&map).unwrap();
///
/// assert_eq!(line1, "| NAME   :        Trevor Lee |");
/// assert_eq!(line2, "| AGE    :     <undisclosed> |");
/// ```
/// Notes:
/// * `"| "`: a fixed string
/// * `dltc!("label", fixed_width = 6, align = 'L')`:
///   - a value-mapped component
///   - require a mapped value for key `label` when calling [`DumbLineTemplate::format`]
///   - also see the macros [`dlt_comps!`] and [`dltc!`]
///
/// You may want to consider the helper [`crate::lblscreen::DumbLineByLineScreen`] for coding a simple terminal / text-based "screen"
#[derive(Debug)]
pub struct DumbLineTemplate {
    min_width: WIDTH,
    max_width: WIDTH,
    components: Vec<LineTempComp>,
}
impl DumbLineTemplate {
    /// please use the macro [`dlt_comps!`] for construction of the components
    /// * `min_width` - the minimum width of the line
    /// * `max_width` - the maximum width of the line
    /// * `components` - the template components of the line, which can be created using the macro [`dlt_comps!`]
    ///
    /// also see [`DumbLineTemplate::new_fixed_width`]
    pub fn new(
        min_width: WIDTH,
        max_width: WIDTH,
        components: &[LineTempComp],
    ) -> DumbLineTemplate {
        DumbLineTemplate {
            min_width,
            max_width,
            components: components.to_owned(),
        }
    }
    /// the same as [`DumbLineTemplate::new`] but with fixed width
    pub fn new_fixed_width(fixed_width: WIDTH, components: &[LineTempComp]) -> DumbLineTemplate {
        DumbLineTemplate {
            min_width: fixed_width,
            max_width: fixed_width,
            components: components.to_vec(),
        }
    }
    pub fn min_width(&self) -> WIDTH {
        self.min_width
    }
    pub fn max_width(&self) -> WIDTH {
        self.max_width
    }
    /// a helper function that the help to scan for the keys involved in formatting the line
    pub fn scan_for_keys(&self) -> HashSet<String> {
        let mut keys = HashSet::new();
        for comp in self.components.iter() {
            if let LineTempComp::Mapped(mapped_comp) = comp {
                keys.insert(mapped_comp.get_map_key().to_string());
            }
            // match comp {
            //     LineTempComp::Mapped(mapped_comp) => {
            //         keys.insert(mapped_comp.get_map_key().to_string());
            //     }
            //     _ => {}
            // }
        }
        keys
    }
    /// based on the template and the input map of values, format and return a line;
    /// for a more flexible way of formatting, try [`DumbLineTemplate::format_ex`]
    ///
    /// e.g.
    /// ```_no_run
    /// let map = HashMap::from([
    ///    ...
    /// ]);
    /// let line = ltemp.format(&map).unwrap();
    /// ```
    pub fn format<T: LineTempCompMapValueTrait>(
        &self,
        value_mapper: &T,
    ) -> Result<String, Box<dyn Error>> {
        let map_value_fn = |key: &str| -> Option<(T::VALUE, WIDTH)> {
            let mapped_value = value_mapper.map_value(key);
            match mapped_value {
                Some(mapped_value) => Some(mapped_value),
                None => None,
            }
        };
        self.format_ex(map_value_fn)
    }
    /// like [`format`] but accept function that returns the mapped values; each mapped value is supposed to be a tuple of the value and its width
    /// (note that for ASCII escaped string, the "visual" length can be different from the length of the string)
    pub fn format_ex<T: fmt::Display, F: Fn(&str) -> Option<(T, WIDTH)>>(
        &self,
        map_value_fn: F,
    ) -> Result<String, Box<dyn Error>> {
        // let map = map
        // .iter()
        // .map(|(k, v)| (k.to_string(), v.to_string()))
        // .collect::<HashMap<String, String>>();
        let mut total_fixed_width = 0;
        let mut total_mapped_needed_width = 0;
        let mut mapped_comp_indexes = Vec::new();
        let mut mapped_comps = Vec::new();
        let mut mapped_needed_widths = Vec::new();
        //let mut total_mapped_min_width = 0;
        //let mut mapped_min_widths = Vec::new();
        //let mut total_mapped_max_width = 0_u64;
        //let mut mapped_max_widths = Vec::new();
        let mut total_mapped_room = 0_u64;
        let mut mapped_rooms = Vec::new();
        for (index, comp) in self.components.iter().enumerate() {
            match comp {
                LineTempComp::Mapped(mapped_comp) => {
                    mapped_comp_indexes.push(Some(mapped_comps.len()));
                    let map_value = match map_value_fn(mapped_comp.get_map_key()) {
                        Some(map_value) => map_value,
                        None => {
                            if mapped_comp.is_optional() {
                                continue;
                            } else {
                                return Err(format!(
                                    "missing required key: {}",
                                    mapped_comp.get_map_key()
                                )
                                .into());
                            }
                        }
                    };
                    let needed_width = mapped_comp.get_needed_width(map_value.1);
                    total_mapped_needed_width += needed_width;
                    mapped_comps.push(mapped_comp);
                    mapped_needed_widths.push(needed_width);
                    let min_width = mapped_comp.get_min_width();
                    //total_mapped_min_width += min_width;
                    //mapped_min_widths.push(min_width);
                    let max_width = mapped_comp.get_max_width();
                    //total_mapped_max_width += max_width as u64;
                    //mapped_max_widths.push(max_width);
                    if min_width > max_width {
                        panic!("min_width {} > max_width {}", min_width, max_width);
                    }
                    let room = max_width - min_width;
                    //println!("***** room={}; min_width={}; max_width={}", room, min_width, max_width);
                    total_mapped_room += room as u64;
                    mapped_rooms.push(room);
                }
                // LineTempComp::Fixed(fixed_comp) => {
                //     mapped_comp_indexes.push(None);
                //     let fixed_width = fixed_comp.len() as u32;
                //     total_fixed_width += fixed_width;
                // }
                LineTempComp::Fixed(escaped_comp, width) => {
                    mapped_comp_indexes.push(None);
                    let fixed_width = width;
                    total_fixed_width += fixed_width;
                }
            }
        }
        loop {
            let total_need_width = total_fixed_width + total_mapped_needed_width;
            if total_need_width < self.min_width {
                if FLEXIBLE_WIDTH_EX {
                    let mut remain_width_to_add = self.min_width - total_need_width;
                    let mut loop_total_mapped_room = total_mapped_room;
                    loop {
                        let loop_total_width_to_add = remain_width_to_add;
                        for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                            let max_width = mapped_comp.get_max_width();
                            let assigned_width = mapped_needed_widths[index];
                            let width_to_add: WIDTH = if loop_total_mapped_room == 0 {
                                0
                            } else {
                                let proportion = max_width as f64 / loop_total_mapped_room as f64;
                                let width_to_add =
                                    (proportion * loop_total_width_to_add as f64).ceil();
                                width_to_add as WIDTH
                            };
                            let width_to_add = cmp::min(width_to_add, remain_width_to_add);
                            let new_assigned_width =
                                cmp::min(assigned_width + width_to_add, max_width);
                            if new_assigned_width != assigned_width {
                                mapped_needed_widths[index] = new_assigned_width;
                                remain_width_to_add -= new_assigned_width - assigned_width;
                            }
                            if remain_width_to_add == 0 {
                                break;
                            }
                        }
                        if remain_width_to_add == 0 {
                            break;
                        }
                        if loop_total_width_to_add == remain_width_to_add {
                            return Err(format!(
                                "too big a line ... {} extra, on top of min {}",
                                remain_width_to_add, self.min_width
                            )
                            .into());
                        }
                        loop_total_mapped_room -=
                            (loop_total_width_to_add - remain_width_to_add) as u64;
                    }
                } else {
                    let total_width_to_add = self.min_width - total_need_width;
                    let mut remain_width_to_add = total_width_to_add;
                    for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                        if remain_width_to_add == 0 {
                            break;
                        }
                        let max_width = mapped_comp.get_max_width();
                        let assigned_width = mapped_needed_widths[index];
                        let max_width_to_add = remain_width_to_add;
                        let width_add = cmp::min(max_width - assigned_width, max_width_to_add);
                        // println!(
                        //     "***** remain_width_to_add={}; max_width_to_add={}; width_add={}; assigned_width={}",
                        //     remain_width_to_add, max_width_to_add, width_add, assigned_width
                        // );
                        if width_add > 0 {
                            mapped_needed_widths[index] = assigned_width + width_add;
                            remain_width_to_add -= width_add;
                        }
                    }
                    if remain_width_to_add > 0 {
                        return Err(format!(
                            "too big a line ... {} extra, on top of min {}",
                            remain_width_to_add, self.min_width
                        )
                        .into());
                    }
                }
            } else if total_need_width > self.max_width {
                if FLEXIBLE_WIDTH_EX {
                    let mut remain_width_to_reduce = total_need_width - self.max_width;
                    let mut loop_total_mapped_room = total_mapped_room;
                    loop {
                        let loop_total_width_to_reduce = remain_width_to_reduce;
                        for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                            let min_width = mapped_comp.get_min_width();
                            let assigned_width = mapped_needed_widths[index];
                            let width_to_reduce: WIDTH = if loop_total_mapped_room == 0 {
                                0 /*remain_width_to_reduce*/
                            } else {
                                let proportion = min_width as f64 / loop_total_mapped_room as f64;
                                let width_to_reduce =
                                    (proportion * loop_total_width_to_reduce as f64).ceil() as u32;
                                width_to_reduce as WIDTH
                            };
                            let width_to_reduce = cmp::min(width_to_reduce, remain_width_to_reduce);
                            let width_to_reduce = cmp::min(width_to_reduce, assigned_width);
                            let new_assigned_width =
                                cmp::max(assigned_width - width_to_reduce, min_width);
                            // println!(
                            //     "***** width_to_reduce={}; min_width={}",
                            //     width_to_reduce, min_width
                            // );
                            // println!(
                            //     "***** new_assigned_width={}; assigned_width={}",
                            //     new_assigned_width, assigned_width
                            // );
                            if new_assigned_width != assigned_width {
                                mapped_needed_widths[index] = new_assigned_width;
                                remain_width_to_reduce -= assigned_width - new_assigned_width;
                            }
                            if remain_width_to_reduce == 0 {
                                break;
                            }
                        }
                        if remain_width_to_reduce == 0 {
                            break;
                        }
                        if loop_total_width_to_reduce == remain_width_to_reduce {
                            return Err(format!(
                                "too small a line ... still need {}, on top of max {}",
                                remain_width_to_reduce, self.max_width
                            )
                            .into());
                        }
                        loop_total_mapped_room -=
                            (loop_total_width_to_reduce - remain_width_to_reduce) as u64;
                    }
                } else {
                    let total_width_to_reduce = total_need_width - self.max_width;
                    let mut remain_width_to_reduce = total_width_to_reduce;
                    for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                        if remain_width_to_reduce == 0 {
                            break;
                        }
                        let min_width = mapped_comp.get_min_width();
                        let assigned_width = mapped_needed_widths[index];
                        let max_width_to_reduce = remain_width_to_reduce;
                        let width_reduce =
                            cmp::min(assigned_width - min_width, max_width_to_reduce);
                        // println!(
                        //     "***** remain_width_to_reduce={}; max_width_to_reduce={}; width_reduce={}; assigned_width={}",
                        //     remain_width_to_reduce, max_width_to_reduce, width_reduce, assigned_width
                        // );
                        if width_reduce > 0 {
                            mapped_needed_widths[index] = assigned_width - width_reduce;
                            remain_width_to_reduce -= width_reduce;
                        }
                    }
                    if remain_width_to_reduce > 0 {
                        return Err(format!(
                            "too small a line ... still need {}, on top of max {}",
                            remain_width_to_reduce, self.max_width
                        )
                        .into());
                    }
                }
            }
            let mut changed = false;
            for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                let assigned_width = mapped_needed_widths[index];
                let assigned_width_delta = mapped_comp.veto_assigned_width(
                    map_value_fn(mapped_comp.get_map_key()).unwrap().1,
                    assigned_width,
                );
                if assigned_width_delta != 0 {
                    mapped_needed_widths[index] =
                        ((assigned_width as i32) + assigned_width_delta) as WIDTH;
                    total_mapped_needed_width =
                        ((total_mapped_needed_width as i32) + assigned_width_delta) as WIDTH;
                    changed = true
                }
            }
            if !changed {
                break;
            }
        }
        let mut formatted = String::new();
        for (index, comp) in self.components.iter().enumerate() {
            match comp {
                LineTempComp::Mapped(mapped_comp) => {
                    if let Some(map_value) = map_value_fn(mapped_comp.get_map_key()) {
                        let mapped_comp_index = mapped_comp_indexes[index].unwrap();
                        let assigned_width = mapped_needed_widths[mapped_comp_index];
                        let formatted_comp = mapped_comp.format(
                            &map_value.0.to_string(),
                            map_value.1,
                            assigned_width,
                        );
                        formatted.push_str(&formatted_comp);
                    }
                }
                // LineTempComp::Fixed(fixed_comp) => {
                //     formatted.push_str(fixed_comp);
                // }
                LineTempComp::Fixed(escaped_comp, width) => {
                    formatted.push_str(escaped_comp);
                }
            }
        }
        Ok(formatted)
    }
}

// pub trait LineTempCompMapValueTrait {
//     type VALUE: fmt::Display;
//     fn map_value(&self, key: &str) -> Option<(Self::VALUE, WIDTH)>;
// }
// impl LineTempCompMapValueTrait for HashMap<&str, String> {
//     type VALUE = String;
//     fn map_value(&self, key: &str) -> Option<(Self::VALUE, WIDTH)> {
//         let value = self.get(key);
//         if value.is_some() {
//             let value = value.unwrap();
//             Some((value.clone(), value.len() as WIDTH))
//         } else {
//             None
//         }
//     }
// }
// impl LineTempCompMapValueTrait for HashMap<&str, &str> {
//     type VALUE = String;
//     fn map_value(&self, key: &str) -> Option<(Self::VALUE, WIDTH)> {
//         let value = self.get(key);
//         if value.is_some() {
//             let value = value.unwrap();
//             Some((value.to_string(), value.len() as WIDTH))
//         } else {
//             None
//         }
//     }
// }

// #[derive(Debug)]
// pub struct LineTemplateError {
//     message: String,
// }
// impl Error for LineTemplateError {}
// impl fmt::Display for LineTemplateError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.message)
//     }
// }

pub trait LineTempCompMapValueTrait {
    type VALUE: fmt::Display;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, WIDTH)>;
}
impl<T: AsRef<str>> LineTempCompMapValueTrait for HashMap<&str, T> {
    type VALUE = String;
    fn map_value(&self, key: &str) -> Option<(String, WIDTH)> {
        let value = self.get(key);
        if let Some(value) = value {
            let value = value.as_ref();
            Some((value.to_string(), value.len() as WIDTH))
        } else {
            None
        }
    }
}
// impl LineTempCompMapValueTrait for HashMap<&str, String> {
//     type VALUE = String;
//     fn map_value(&self, key: &str) -> Option<(String, WIDTH)> {
//         let value = self.get(key);
//         if let Some(value) = value {
//             //let value = value.unwrap();
//             Some((value.clone(), value.len() as WIDTH))
//         } else {
//             None
//         }
//     }
// }
// impl LineTempCompMapValueTrait for HashMap<&str, &str> {
//     type VALUE = String;
//     fn map_value(&self, key: &str) -> Option<(String, WIDTH)> {
//         let value = self.get(key);
//         if let Some(value) = value {
//             //let value = value.unwrap();
//             Some((value.to_string(), value.len() as WIDTH))
//         } else {
//             None
//         }
//     }
// }

// trait LineTempCompTrait {
//     fn get_min_width(&self) -> u32 {
//         0
//     }
//     fn get_max_width(&self) -> u32 {
//         u32::MAX
//     }
// }
// trait MappedLineTempCompTrait: LineTempCompTrait {
//     fn is_optional(&self) -> bool {
//         false
//     }
//     fn get_map_key(&self) -> &str;
//     fn get_needed_width(&self, map_value: &str) -> u32;
// }
// trait FixedLineTempCompTrait: LineTempCompTrait {
//     fn get_fixed_width(&self) -> u32;
// }

// struct MappedLineTempCompSetup {
//     optional: bool,
//     min_width: u32, // can be 0
//     max_width: u32, // can be u32:MAX
// }
// impl MappedLineTempCompSetup {
//     fn default(key: &str) -> Self {
//         Self {
//             optional: false,
//             min_width: 0,
//             max_width: u32::MAX,
//         }
//     }
// }

pub struct MappedLineTempCompBuilder {
    key: String,
    min_width: WIDTH,
    max_width: WIDTH,
    align: char,
    optional: bool,
    truncate_indicator: Option<String>,
}
/// a builder for a component to be a component of [`DumbLineTemplate`]
impl MappedLineTempCompBuilder {
    /// use the macro [`dltc!`] instead
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            min_width: 1,
            max_width: WIDTH::MAX,
            align: 'L',
            optional: false,
            truncate_indicator: None,
        }
    }
    /// set the component to be optional
    pub fn optional(&mut self, optional: bool) -> &mut MappedLineTempCompBuilder {
        self.optional = optional;
        self
    }
    /// set the min and max widths of the component to be the same fixed width
    pub fn fixed_width(&mut self, fixed_width: WIDTH) -> &mut MappedLineTempCompBuilder {
        self.min_width = fixed_width;
        self.max_width = fixed_width;
        self
    }
    // set the min width of the component, keeping the max width unchanged
    pub fn min_width(&mut self, min_width: WIDTH) -> &mut MappedLineTempCompBuilder {
        self.min_width = min_width;
        self
    }
    /// set the max width of the component, keeping the min width unchanged
    pub fn max_width(&mut self, max_width: WIDTH) -> &mut MappedLineTempCompBuilder {
        self.max_width = max_width;
        self
    }
    /// set the alignment of the component
    /// * align - 'L' for left, 'R' for right, 'C' for center
    pub fn align(&mut self, align: char) -> &mut MappedLineTempCompBuilder {
        self.align = align;
        self
    }
    /// set the truncate indicator of the component;
    /// e.g.
    /// - `…`  or `>>:…` -- truncate to the right using `…` as indicator
    /// - `…:<<` -- truncate to the left using `…` as indicator
    pub fn set_truncate_indicator(
        &mut self,
        truncate_indicator: &str,
    ) -> &mut MappedLineTempCompBuilder {
        self.truncate_indicator = Some(truncate_indicator.to_string());
        self
    }
    pub fn build(&self) -> MappedLineTempComp {
        MappedLineTempComp {
            key: self.key.clone(),
            min_width: self.min_width,
            max_width: self.max_width,
            align: self.align,
            optional: self.optional,
            truncate_indicator: self.truncate_indicator.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineTempComp {
    Mapped(MappedLineTempComp),
    Fixed(String, WIDTH),
}
// impl From<String> for LineTempComp {
//     fn from(value: String) -> Self {
//         LineTempComp::Fixed(value.clone(), value.len() as WIDTH)
//     }
// }
// impl From<&'static str> for LineTempComp {
//     fn from(value: &'static str) -> Self {
//         LineTempComp::Fixed(value.to_string(), value.len() as WIDTH)
//     }
// }

const DEF_MAPPED_LINE_TEMP_COMP_SETTINGS: MappedLineTempCompSettings = MappedLineTempCompSettings {
    min_width: 1,
    max_width: WIDTH::MAX,
    align: 'L',
    optional: false,
    truncate_indicator: None,
};

//#[derive(Debug, Copy, Clone)]
// pub enum LineTempCompAlign {
//     Left,
//     Right,
//     Center,
// }

pub struct EscapedLineTempComp {
    value: String,
    width: WIDTH,
}
impl EscapedLineTempComp {
    pub fn new(value: String, width: WIDTH) -> Self {
        Self { value, width }
    }
}
impl LineTempCompTrait for EscapedLineTempComp {
    fn to_line_temp_comp(&self) -> LineTempComp {
        LineTempComp::Fixed(self.value.clone(), self.width)
    }
}

#[derive(Debug, Clone)]
pub struct MappedLineTempCompSettings {
    pub min_width: WIDTH,
    pub max_width: WIDTH,
    pub align: char,
    pub optional: bool,
    pub truncate_indicator: Option<String>,
}
impl Default for MappedLineTempCompSettings {
    fn default() -> Self {
        DEF_MAPPED_LINE_TEMP_COMP_SETTINGS
    }
}

#[derive(Debug, Clone)]
pub struct MappedLineTempComp {
    key: String,
    min_width: WIDTH,
    max_width: WIDTH,
    align: char,
    optional: bool,
    truncate_indicator: Option<String>,
}
impl MappedLineTempComp {
    pub fn new(key: &str, settings: &MappedLineTempCompSettings) -> Self {
        Self {
            key: key.to_string(),
            min_width: settings.min_width,
            max_width: settings.max_width,
            align: settings.align,
            optional: settings.optional,
            truncate_indicator: settings.truncate_indicator.clone(),
        }
    }
    pub fn get_min_width(&self) -> WIDTH {
        self.min_width
    }
    pub fn get_max_width(&self) -> WIDTH {
        self.max_width
    }
    pub fn is_optional(&self) -> bool {
        self.optional
    }
    pub fn get_map_key(&self) -> &str {
        &self.key
    }
    fn get_needed_width(&self, mapped_value_width: WIDTH) -> WIDTH {
        let needed_width = mapped_value_width; //map_value.len() as WIDTH;
        if needed_width < self.min_width {
            self.min_width
        } else if needed_width > self.max_width {
            self.max_width
        } else {
            needed_width
        }
    }
    // fn try_add_width(&self, assigned_width: u32, width_to_add: u32) -> u32 {
    //     let max_width = self.get_max_width();
    //     let new_assigned_width = cmp::min(assigned_width + width_to_add, max_width);
    //     new_assigned_width
    // }
    fn veto_assigned_width(&self, mapped_value_width: WIDTH, assigned_width: WIDTH) -> i32 {
        let needed_width = self.get_needed_width(mapped_value_width);
        let mut needed_width_delta: i32 = if assigned_width > self.max_width {
            panic!("not expected")
        } else if assigned_width < needed_width {
            // TODO: can based on how to format in case less room than needed
            0
        } else {
            0
        };
        needed_width_delta
    }
    fn format(
        &self,
        mapped_value: &str,
        mapped_value_width: WIDTH,
        assigned_width: WIDTH,
    ) -> String {
        let needed_width = self.get_needed_width(mapped_value_width);
        if assigned_width > self.max_width {
            panic!("not expected")
        } else if assigned_width < needed_width || assigned_width < mapped_value_width {
            if mapped_value_width != mapped_value.len() as WIDTH {
                panic!("escaped value [{}] cannot be reduced", mapped_value);
            }
            if let Some(truncate_indicator) = &self.truncate_indicator {
                let (truncate_left, truncate_indicator) = {
                    let truncate_right = false;
                    if let Some(truncate_indicator) = truncate_indicator.strip_prefix(">>:") {
                        (false, truncate_indicator)
                    } else if let Some(truncate_indicator) = truncate_indicator.strip_suffix(":<<")
                    {
                        (true, truncate_indicator)
                    } else {
                        (false, truncate_indicator.as_str())
                    }
                };
                let truncate_indicator_len = if truncate_indicator == "…" {
                    1
                } else {
                    truncate_indicator.len()
                };
                let a_width = assigned_width as i32 - truncate_indicator_len as i32;
                if a_width < 0 {
                    panic!(
                        "width {} too small while truncate indicator is [{}]",
                        assigned_width, truncate_indicator
                    )
                }
                let (front, end) = if (truncate_left) {
                    let front = truncate_indicator.to_string();
                    let end = mapped_value[mapped_value.len() - a_width as usize..].to_string();
                    (front, end)
                } else {
                    let front = mapped_value[..a_width as usize].to_string();
                    let end = truncate_indicator.to_string();
                    (front, end)
                };
                return format!("{}{}", front, end);
                // let mut formatted = mapped_value[..a_width as usize].to_string();
                // formatted.push_str(truncate_indicator);
                // return formatted;
            } else {
                return mapped_value[..assigned_width as usize].to_string();
            }
        }
        // let formatted = if assigned_width > self.max_width {
        //     panic!("not expected")
        // } else if assigned_width < needed_width {
        //     return map_value[..assigned_width as usize].to_string()
        // } else if assigned_width > needed_width {
        //     match self.align {
        //         'L' => {
        //             let mut formatted = map_value.to_string();
        //             formatted.push_str(&" ".repeat((assigned_width - needed_width) as usize));
        //             formatted
        //         }
        //         'R' => {
        //             let mut formatted = " ".repeat((assigned_width - needed_width) as usize);
        //             formatted.push_str(map_value);
        //             formatted
        //         }
        //         'C' => {
        //             let left_count =
        //                 ((assigned_width as f64 - needed_width as f64) / 2.0).ceil() as usize;
        //             let right_count =
        //                 ((assigned_width as f64 - needed_width as f64) / 2.0).floor() as usize;
        //             let mut formatted = " ".repeat(left_count);
        //             formatted.push_str(map_value);
        //             formatted.push_str(&" ".repeat(right_count));
        //             formatted
        //         }
        //         _ => panic!("unknown align: {}", self.align),
        //     }
        //     // let mut formatted = map_value.to_string();
        //     // formatted.push_str(&" ".repeat((assigned_width - needed_width) as usize));
        //     // formatted
        // } else {
        //     map_value.to_string()
        // };
        let value_width = mapped_value_width; //map_value.len();
        match self.align {
            'L' => {
                let mut formatted = mapped_value.to_string();
                formatted.push_str(&" ".repeat(
                    assigned_width as usize - mapped_value_width as usize, /*value_width*/
                ));
                formatted
            }
            'R' => {
                let mut formatted = " ".repeat(
                    assigned_width as usize - mapped_value_width as usize, /*value_width*/
                );
                formatted.push_str(mapped_value);
                formatted
            }
            'C' => {
                let left_count =
                    ((assigned_width as f64 - mapped_value_width/*needed_width*/ as f64) / 2.0)
                        .ceil() as usize;
                let right_count =
                    ((assigned_width as f64 - mapped_value_width/*needed_width*/ as f64) / 2.0)
                        .floor() as usize;
                let mut formatted = " ".repeat(left_count);
                formatted.push_str(mapped_value);
                formatted.push_str(&" ".repeat(right_count));
                formatted
            }
            _ => panic!("unknown align: {}", self.align),
        }
    }
}

// #[derive(Debug)]
// pub struct FixedLineTempComp {
//     value: String,
// }
// impl FixedLineTempComp {
//     fn get_min_width(&self) -> u16 {
//         self.value.len() as u16
//     }
//     fn get_max_width(&self) -> u16 {
//         self.value.len() as u16
//     }
// }
// impl FixedLineTempComp {
//     pub fn new(value: String) -> Self {
//         Self { value }
//     }
// }

/// for use by DumbLineTemplate internally.
pub trait LineTempCompTrait {
    fn to_line_temp_comp(&self) -> LineTempComp;
}
impl LineTempCompTrait for MappedLineTempCompBuilder {
    fn to_line_temp_comp(&self) -> LineTempComp {
        let mapped_line_temp_comp = self.build();
        LineTempComp::Mapped(mapped_line_temp_comp)
    }
}
impl LineTempCompTrait for String {
    fn to_line_temp_comp(&self) -> LineTempComp {
        LineTempComp::Fixed(self.clone(), self.len() as WIDTH)
    }
}
impl LineTempCompTrait for &'static str {
    fn to_line_temp_comp(&self) -> LineTempComp {
        LineTempComp::Fixed(self.to_string(), self.len() as WIDTH)
    }
}
impl LineTempCompTrait for (String, usize) {
    fn to_line_temp_comp(&self) -> LineTempComp {
        LineTempComp::Fixed(self.0.clone(), self.1 as WIDTH)
    }
}
impl LineTempCompTrait for (&'static str, usize) {
    fn to_line_temp_comp(&self) -> LineTempComp {
        LineTempComp::Fixed(self.0.to_string(), self.1 as WIDTH)
    }
}
