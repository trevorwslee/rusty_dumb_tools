#![deny(warnings)]
#![allow(unused)]

use crate::arg::DumbArgBuilder;

#[macro_export]
macro_rules! slt_comps {
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
macro_rules! sltc_mapped {
    ($x:expr) => {
        MappedLineTempCompBuilder::new($x)
    };
}

/// For internal debugging use only.
#[test]
fn debug_ltemp() {
    let key1_temp = MappedLineTempCompBuilder::new("key1")
        .optional(true)
        .min_width(3)
        .max_width(5)
        .build();
    println!("key1_temp: {:?}", key1_temp);

    let ltemp = slt_comps!["abc", sltc_mapped!("key"), "def".to_string()];
    println!("ltemp: {:?}", ltemp);
}

struct DumbLineTemplate {
    components: Vec<LineTempComp>,
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

pub struct MappedLineTempCompBuilder {
    optional: bool,
    min_width: u32, // can be 0
    max_width: u32, // can be u32:MAX
    key: String,
}
impl MappedLineTempCompBuilder {
    pub fn new(key: &str) -> Self {
        Self {
            optional: false,
            min_width: 0,
            max_width: u32::MAX,
            key: key.to_string(),
        }
    }
    pub fn optional(&mut self, optional: bool) -> &mut MappedLineTempCompBuilder {
        self.optional = optional;
        self
    }
    pub fn min_width(&mut self, min_width: u32) -> &mut MappedLineTempCompBuilder {
        self.min_width = min_width;
        self
    }
    pub fn max_width(&mut self, max_width: u32) -> &mut MappedLineTempCompBuilder {
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
    Fixed(FixedLineTempComp),
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
        unimplemented!("get_needed_width");
    }
}

#[derive(Debug)]
pub struct FixedLineTempComp {
    min_width: u16, // can be 0
    max_width: u16, // can be u32:MAX
    fixed_width: u16,
}
impl FixedLineTempComp {
    fn get_min_width(&self) -> u16 {
        self.min_width
    }
    fn get_max_width(&self) -> u16 {
        self.max_width
    }
}
impl FixedLineTempComp {
    pub fn new(fixed_width: u16) -> Self {
        Self {
            min_width: fixed_width,
            max_width: fixed_width,
            fixed_width,
        }
    }
}

pub trait LineTempCompTrait {
    fn as_line_temp_comp(&self) -> LineTempComp;
}
impl LineTempCompTrait for MappedLineTempCompBuilder {
    fn as_line_temp_comp(&self) -> LineTempComp {
        let mapped_line_temp_comp = self.build();
        LineTempComp::Mapped(mapped_line_temp_comp)
    }
}
impl LineTempCompTrait for String {
    fn as_line_temp_comp(&self) -> LineTempComp {
        let fixed_line_temp_comp = FixedLineTempComp::new(self.len() as u16);
        LineTempComp::Fixed(fixed_line_temp_comp)
    }
}
impl LineTempCompTrait for &'static str {
    fn as_line_temp_comp(&self) -> LineTempComp {
        let fixed_line_temp_comp = FixedLineTempComp::new(self.len() as u16);
        LineTempComp::Fixed(fixed_line_temp_comp)
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
