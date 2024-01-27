#![deny(warnings)]
#![allow(unused)]

use core::fmt;
use std::collections::HashMap;

use crate::arg::DumbArgBuilder;

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
    ($x:expr $(, optional=$optional:expr)? $(, min_width=$min_width:expr)? $(, max_width=$max_width:expr)?) => {{
      let mut builder = DumbLineTempCompBuilder::new($x);
      $(builder.optional($optional);)?
      $(builder.min_width($min_width);)?
      $(builder.max_width($max_width);)?
      builder
    }};
}

/// For internal debugging use only.
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

    let ltemp = DumbLineTemplate::new(0, 100, lt_comps);
    println!("ltemp: {:?}", ltemp);

    let mut map = HashMap::new();
    map.insert(String::from("key1"), String::from("value1"));
    map.insert(String::from("key2"), String::from("value2"));
    let formatted = ltemp.format(&map).unwrap();
    println!("formatted: [{}]", formatted);
}

#[derive(Debug)]
pub struct DumbLineTemplate {
    min_width: u32,
    max_width: u32,
    components: Vec<LineTempComp>,
}
impl DumbLineTemplate {
    pub fn new(min_width: u32, max_width: u32, components: Vec<LineTempComp>) -> DumbLineTemplate {
        DumbLineTemplate {
            min_width,
            max_width,
            components,
        }
    }
    pub fn format<T: fmt::Display>(&self, map: &HashMap<String, T>) -> Result<String, String> {
        let map = map
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<HashMap<String, String>>();
        let mut total_fixed_width: u32 = 0;
        let mut total_mapped_needed_width: u32 = 0;
        let mut mapped_comp_indexes = Vec::new();
        let mut mapped_comps = Vec::new();
        let mut mapped_needed_widths = Vec::new();
        for (index, comp) in self.components.iter().enumerate() {
            match comp {
                LineTempComp::Mapped(mapped_comp) => {
                    mapped_comp_indexes.push(Some(mapped_comps.len()));
                    let map_value = match map.get(mapped_comp.get_map_key()) {
                        Some(map_value) => map_value,
                        None => {
                            if mapped_comp.is_optional() {
                                continue;
                            } else {
                                return Err(format!("missing required key: {}", mapped_comp.get_map_key()));
                            }
                        }
                    };
                    let needed_width = mapped_comp.get_needed_width(map_value);
                    total_mapped_needed_width += needed_width;
                    mapped_comps.push(mapped_comp);
                    mapped_needed_widths.push(needed_width);
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
                for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                  if total_width_to_add == 0 {
                    break;
                  }
                  let max_width = mapped_comp.get_max_width();
                  let assigned_width = mapped_needed_widths[index];
                  let width_add = max_width - assigned_width;
                  if width_add > 0 {
                    mapped_needed_widths[index] = assigned_width + width_add;
                  }
                }
                if total_width_to_add > 0 {
                  return Err(format!("too big a line ... {} extra", total_width_to_add))
                }
            } else if total_need_width > self.max_width {
              let total_width_to_reduce = total_need_width - self.max_width;
              for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                if total_width_to_reduce == 0 {
                  break;
                }
                let min_width = mapped_comp.get_min_width();
                let assigned_width = mapped_needed_widths[index];
                let width_reduce = assigned_width - min_width;
                if width_reduce > 0 {
                  mapped_needed_widths[index] = assigned_width - width_reduce;
                }
              }
              if total_width_to_reduce > 0 {
                return Err(format!("too small a line ... still need {}", total_width_to_reduce))
              }
          }
            let mut changed = false;
            for (index, mapped_comp) in mapped_comps.iter().enumerate() {
                let assigned_width = mapped_needed_widths[index];
                let assigned_width_delta = mapped_comp
                    .veto_assigned_width(&map[mapped_comp.get_map_key()], assigned_width);
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
                    if let Some(map_value) = map.get(mapped_comp.get_map_key()) {
                      let mapped_comp_index = mapped_comp_indexes[index].unwrap();
                      let assigned_width = mapped_needed_widths[mapped_comp_index];
                      let formatted_comp = mapped_comp.format(map_value, assigned_width);
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
    key: String,
}
impl DumbLineTempCompBuilder {
    pub fn new(key: &str) -> Self {
        Self {
            optional: false,
            min_width: 0,
            max_width: u32::MAX,
            key: key.to_string(),
        }
    }
    pub fn optional(&mut self, optional: bool) -> &mut DumbLineTempCompBuilder {
        self.optional = optional;
        self
    }
    pub fn min_width(&mut self, min_width: u32) -> &mut DumbLineTempCompBuilder {
        self.min_width = min_width;
        self
    }
    pub fn max_width(&mut self, max_width: u32) -> &mut DumbLineTempCompBuilder {
        self.max_width = max_width;
        self
    }
    pub fn build(&self) -> MappedLineTempComp {
        MappedLineTempComp {
            optional: self.optional,
            min_width: self.min_width,
            max_width: self.max_width,
            key: self.key.clone(),
        }
    }
}

#[derive(Debug)]
pub enum LineTempComp {
    Mapped(MappedLineTempComp),
    Fixed(String),
}

#[derive(Debug)]
pub struct MappedLineTempComp {
    optional: bool,
    min_width: u32, // can be 0
    max_width: u32, // can be u32:MAX
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
        let formatted = if assigned_width > self.max_width {
            panic!("not expected")
        } else if assigned_width < needed_width {
            map_value[..assigned_width as usize].to_string()
        } else {
            map_value.to_string()
        };
        formatted
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
