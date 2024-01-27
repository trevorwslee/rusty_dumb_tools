//! A simple line template for formatting a line, say for use of terminal-oriented UI -- [`crate::ltemp::DumbLineTemplate``]

#![deny(warnings)]
#![allow(unused)]

use core::fmt;
use std::{cmp, collections::HashMap, path::Display};

use crate::arg::DumbArgBuilder;

pub const FLEXIBLE_WIDTH: bool = false; // TODO: debug and make it works

#[macro_export]
macro_rules! dlt_comps {
  ($($x:expr),*) => {{
    let mut comps: Vec<LineTempComp> = Vec::new();
    $(
      let comp = $x.as_line_temp_comp();
      comps.push(comp);
    )*
    comps
  }};
}

#[macro_export]
macro_rules! dltc {
    ($x:expr $(, optional=$optional:expr)? $(, fixed_width=$fixed_width:expr)? $(, min_width=$min_width:expr)? $(, max_width=$max_width:expr)? $(, align=$align:expr)?) => {{
      let mut builder = DumbLineTempCompBuilder::new($x);
      $(builder.optional($optional);)?
      $(builder.fixed_width($fixed_width);)?
      $(builder.min_width($min_width);)?
      $(builder.max_width($max_width);)?
      $(builder.align($align);)?
      builder
    }};
}

#[test]
fn debug_ltemp() {
    let key1_temp = DumbLineTempCompBuilder::new("key1")
        .optional(true)
        .min_width(3)
        .max_width(5)
        .build();
    println!("key1_temp: {:?}", key1_temp);

    let lt_comps = dlt_comps![
        "abc",
        dltc!("key1"),
        "def".to_string(),
        dltc!("key2", optional = true, min_width = 1, max_width = 10)
    ];
    println!("lt_comps: {:?}", lt_comps);

    // let lt_comps = dlt_comps![
    //   dltc!("key1")
    // ];
    // println!("lt_comps: {:?}", lt_comps);

    let ltemp = DumbLineTemplate::new(0, 100, &lt_comps);
    println!("ltemp: {:?}", ltemp);

    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    println!("formatted: [{}]", formatted);
}

/// a simple line template for formatting a line, say for use of terminal-oriented UI
///
/// example:
/// ```
/// use rusty_dumb_tools::{
///    dlt_comps, dltc,
///    ltemp::{DumbLineTempCompBuilder, DumbLineTemplate, LineTempComp, LineTempCompTrait},
/// };
/// use std::collections::HashMap;
/// let name = "Trevor Lee";
/// let lt_comps = dlt_comps![
///     "| ",
///     dltc!("label", fixed_width = 6, align = 'L'),
///     " : ",
///     dltc!("value", align = 'R'),
///     " |"
/// ];
/// let ltemp = DumbLineTemplate::new(30, 30, &lt_comps);
///
/// let mut map = HashMap::new();
/// map.insert(String::from("label"), String::from("NAME"));
/// map.insert(String::from("value"), name.to_string());
/// let line1 = ltemp.format(&map).unwrap();
///
/// let mut map = HashMap::new();
/// map.insert(String::from("label"), String::from("AGE"));
/// map.insert(String::from("value"), String::from("<undisclosed>"));
/// let line2 = ltemp.format(&map).unwrap();
///
/// assert_eq!(line1, "| NAME   :        Trevor Lee |");
/// assert_eq!(line2, "| AGE    :     <undisclosed> |");
/// ```
#[derive(Debug)]
pub struct DumbLineTemplate {
    min_width: u32,
    max_width: u32,
    components: Vec<LineTempComp>,
}
impl DumbLineTemplate {
    /// please use the macro [`dlt_comps!`] for construction of the components
    /// * `min_width` - the minimum width of the line
    /// * `max_width` - the maximum width of the line
    /// * `components` - the components of the line
    pub fn new(min_width: u32, max_width: u32, components: &Vec<LineTempComp>) -> DumbLineTemplate {
        DumbLineTemplate {
            min_width,
            max_width,
            components: components.clone(),
        }
    }
    /// based on the template and the input map of values, format and return a line
    pub fn format<T: fmt::Display>(&self, map: &HashMap<String, T>) -> Result<String, String> {
        let map = map
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>();
        let map_value_provide_fn = |key: &str| -> Option<&str> {
            match map.get(key) {
                Some(value) => Some(value),
                None => None,
            }
        };
        return self.format_ex(map_value_provide_fn);
    }
    /// like [`format`] but accept function that returns the mapped values
    pub fn format_ex<'a, F: Fn(&str) -> Option<&'a str>>(
        &self,
        map_value_provide_fn: F,
    ) -> Result<String, String> {
        // let map = map
        // .iter()
        // .map(|(k, v)| (k.to_string(), v.to_string()))
        // .collect::<HashMap<String, String>>();
        let mut total_fixed_width: u32 = 0;
        let mut total_mapped_needed_width: u32 = 0;
        let mut mapped_comp_indexes = Vec::new();
        let mut mapped_comps = Vec::new();
        let mut mapped_needed_widths = Vec::new();
        let mut total_mapped_min_width = 0;
        let mut mapped_min_widths = Vec::new();
        let mut total_mapped_max_width = 0_u64;
        let mut mapped_max_widths = Vec::new();
        for (index, comp) in self.components.iter().enumerate() {
            match comp {
                LineTempComp::Mapped(mapped_comp) => {
                    mapped_comp_indexes.push(Some(mapped_comps.len()));
                    let map_value = match map_value_provide_fn(&mapped_comp.get_map_key()) {
                        Some(map_value) => map_value,
                        None => {
                            if mapped_comp.is_optional() {
                                continue;
                            } else {
                                return Err(format!(
                                    "missing required key: {}",
                                    mapped_comp.get_map_key()
                                ));
                            }
                        }
                    };
                    let needed_width = mapped_comp.get_needed_width(&map_value);
                    total_mapped_needed_width += needed_width;
                    mapped_comps.push(mapped_comp);
                    mapped_needed_widths.push(needed_width);
                    let min_width = mapped_comp.get_min_width();
                    total_mapped_min_width += min_width;
                    mapped_min_widths.push(min_width);
                    let max_width = mapped_comp.get_max_width();
                    total_mapped_max_width += max_width as u64;
                    mapped_max_widths.push(max_width);
                }
                LineTempComp::Fixed(fixed_comp) => {
                    mapped_comp_indexes.push(None);
                    let fixed_width = fixed_comp.len() as u32;
                    total_fixed_width += fixed_width;
                }
            }
        }
        loop {
            let total_need_width = total_fixed_width + total_mapped_needed_width;
            if total_need_width < self.min_width {
                let total_width_to_add = self.min_width - total_need_width;
                let mut remain_width_to_add = total_width_to_add;
                for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                    if remain_width_to_add == 0 {
                        break;
                    }
                    let max_width = mapped_comp.get_max_width();
                    let assigned_width = mapped_needed_widths[index];
                    let max_width_to_add = if FLEXIBLE_WIDTH {
                        let proportion = max_width as f64 / total_mapped_max_width as f64;
                        let max_width_to_add =
                            (proportion * total_width_to_add as f64).ceil() as u32; //remain_width_to_add;
                        let max_width_to_add: u32 = cmp::min(max_width_to_add, remain_width_to_add);
                        max_width_to_add
                    } else {
                        remain_width_to_add
                    };
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
                    ));
                }
            } else if total_need_width > self.max_width {
                let total_width_to_reduce = total_need_width - self.max_width;
                let mut remain_width_to_reduce = total_width_to_reduce;
                for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                    if remain_width_to_reduce == 0 {
                        break;
                    }
                    let min_width = mapped_comp.get_min_width();
                    let assigned_width = mapped_needed_widths[index];
                    let max_width_to_reduce = if FLEXIBLE_WIDTH {
                        let max_width_to_reduce = if total_mapped_min_width == 0 {
                            remain_width_to_reduce
                        } else {
                            let proportion = min_width as f64 / total_mapped_min_width as f64;
                            (proportion * total_width_to_reduce as f64).ceil() as u32
                            //remain_width_to_reduce
                        };
                        let max_width_to_reduce: u32 =
                            cmp::min(max_width_to_reduce, remain_width_to_reduce);
                        max_width_to_reduce
                    } else {
                        remain_width_to_reduce
                    };
                    let width_reduce = cmp::min(assigned_width - min_width, max_width_to_reduce);
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
                    ));
                }
            }
            let mut changed = false;
            for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                let assigned_width = mapped_needed_widths[index];
                let assigned_width_delta = mapped_comp.veto_assigned_width(
                    &map_value_provide_fn(mapped_comp.get_map_key()).unwrap(),
                    assigned_width,
                );
                if assigned_width_delta != 0 {
                    mapped_needed_widths[index] =
                        ((assigned_width as i32) + assigned_width_delta) as u32;
                    total_mapped_needed_width =
                        ((total_mapped_needed_width as i32) + assigned_width_delta) as u32;
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
                    if let Some(map_value) = map_value_provide_fn(mapped_comp.get_map_key()) {
                        let mapped_comp_index = mapped_comp_indexes[index].unwrap();
                        let assigned_width = mapped_needed_widths[mapped_comp_index];
                        let formatted_comp = mapped_comp.format(&map_value, assigned_width);
                        formatted.push_str(&formatted_comp);
                    }
                }
                LineTempComp::Fixed(fixed_comp) => {
                    formatted.push_str(fixed_comp);
                }
            }
        }
        Ok(formatted)
    }
    // /// like [`format`] but accept a map of "boxed" [`fmt::Display`] values;
    // /// and therefore, the map of values can be of different types that implement the trait [`fmt::Display`]
    // pub fn format_ex(&self, map: &HashMap<String, Box<dyn fmt::Display>>) -> Result<String, String> {
    //     let mut new_map: HashMap<String, String> = HashMap::new();
    //     for (key, value) in map.iter() {
    //         new_map.insert(key.clone(), value.to_string());
    //     }
    //     self.format(&new_map)
    // }
}

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

pub struct DumbLineTempCompBuilder {
    optional: bool,
    min_width: u32, // can be 0
    max_width: u32, // can be u32:MAX
    align: char,
    key: String,
}
/// a builder for a component to be a component of [`DumbLineTemplate`]
impl DumbLineTempCompBuilder {
    /// use the macro [`dltc!`] instead
    pub fn new(key: &str) -> Self {
        Self {
            optional: false,
            min_width: 1,
            max_width: u32::MAX,
            align: 'L',
            key: key.to_string(),
        }
    }
    /// set the component to be optional
    pub fn optional(&mut self, optional: bool) -> &mut DumbLineTempCompBuilder {
        self.optional = optional;
        self
    }
    /// set the min and max widths of the component to be the same fixed width
    pub fn fixed_width(&mut self, fixed_width: u32) -> &mut DumbLineTempCompBuilder {
        self.min_width = fixed_width;
        self.max_width = fixed_width;
        self
    }
    // set the min width of the component, keeping the max width unchanged
    pub fn min_width(&mut self, min_width: u32) -> &mut DumbLineTempCompBuilder {
        self.min_width = min_width;
        self
    }
    /// set the max width of the component, keeping the min width unchanged
    pub fn max_width(&mut self, max_width: u32) -> &mut DumbLineTempCompBuilder {
        self.max_width = max_width;
        self
    }
    /// set the alignment of the component
    /// * align - 'L' for left, 'R' for right, 'C' for center
    pub fn align(&mut self, align: char) -> &mut DumbLineTempCompBuilder {
        self.align = align;
        self
    }
    pub fn build(&self) -> MappedLineTempComp {
        MappedLineTempComp {
            optional: self.optional,
            min_width: self.min_width,
            max_width: self.max_width,
            align: self.align,
            key: self.key.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineTempComp {
    Mapped(MappedLineTempComp),
    Fixed(String),
}

//#[derive(Debug, Copy, Clone)]
// pub enum LineTempCompAlign {
//     Left,
//     Right,
//     Center,
// }

#[derive(Debug, Clone)]
pub struct MappedLineTempComp {
    optional: bool,
    min_width: u32, // can be 0
    max_width: u32, // can be u32:MAX
    align: char,
    key: String,
}
impl MappedLineTempComp {
    fn get_min_width(&self) -> u32 {
        self.min_width
    }
    fn get_max_width(&self) -> u32 {
        self.max_width
    }
    fn is_optional(&self) -> bool {
        self.optional
    }
    fn get_map_key(&self) -> &str {
        &self.key
    }
    fn get_needed_width(&self, map_value: &str) -> u32 {
        let needed_width = map_value.len() as u32;
        let needed_width = if needed_width < self.min_width {
            self.min_width
        } else if needed_width > self.max_width {
            self.max_width
        } else {
            needed_width
        };
        needed_width
    }
    fn veto_assigned_width(&self, map_value: &str, assigned_width: u32) -> i32 {
        let needed_width = self.get_needed_width(map_value);
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
    fn format(&self, map_value: &str, assigned_width: u32) -> String {
        let needed_width = self.get_needed_width(map_value);
        if assigned_width > self.max_width {
            panic!("not expected")
        } else if assigned_width < needed_width {
            return map_value[..assigned_width as usize].to_string();
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
        let value_width = map_value.len() as u32;
        match self.align {
            'L' => {
                let mut formatted = map_value.to_string();
                formatted
                    .push_str(&" ".repeat((assigned_width - value_width/*needed_width*/) as usize));
                formatted
            }
            'R' => {
                let mut formatted =
                    " ".repeat((assigned_width - value_width/*needed_width*/) as usize);
                formatted.push_str(map_value);
                formatted
            }
            'C' => {
                let left_count =
                    ((assigned_width as f64 - needed_width as f64) / 2.0).ceil() as usize;
                let right_count =
                    ((assigned_width as f64 - needed_width as f64) / 2.0).floor() as usize;
                let mut formatted = " ".repeat(left_count);
                formatted.push_str(map_value);
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
    fn as_line_temp_comp(&self) -> LineTempComp;
}
impl LineTempCompTrait for DumbLineTempCompBuilder {
    fn as_line_temp_comp(&self) -> LineTempComp {
        let mapped_line_temp_comp = self.build();
        LineTempComp::Mapped(mapped_line_temp_comp)
    }
}
impl LineTempCompTrait for String {
    fn as_line_temp_comp(&self) -> LineTempComp {
        LineTempComp::Fixed(self.clone())
    }
}
impl LineTempCompTrait for &'static str {
    fn as_line_temp_comp(&self) -> LineTempComp {
        LineTempComp::Fixed(self.to_string())
    }
}

// impl LineTempCompTrait for MappedLineTempComp {
//     fn get_min_width(&self) -> u32 {
//         self.min_width
//     }
//     fn get_max_width(&self) -> u32 {
//         self.max_width
//     }
// }
// impl MappedLineTempCompTrait for MappedLineTempComp {
//     fn is_optional(&self) -> bool {
//         self.optional
//     }
//     fn get_map_key(&self) -> &str {
//         &self.key
//     }
//     fn get_needed_width(&self, map_value: &str) -> u32 {
//         unimplemented!("get_needed_width");
//     }
// }

// impl LineTempCompTrait for String {
//     fn get_min_width(&self) -> u32 {
//         self.len() as u32
//     }
//     fn get_max_width(&self) -> u32 {
//         self.len() as u32
//     }
// }
// impl FixedLineTempCompTrait for String {
//     fn get_fixed_width(&self) -> u32 {
//         self.len() as u32
//     }
// }

// impl LineTempCompTrait for &'static str {
//   fn get_min_width(&self) -> u32 {
//       self.len() as u32
//   }
//   fn get_max_width(&self) -> u32 {
//       self.len() as u32
//   }
// }
// impl FixedLineTempCompTrait for &'static str {
//   fn get_fixed_width(&self) -> u32 {
//       self.len() as u32
//   }
// }
