#![deny(warnings)]
#![allow(unused)]

use crate::calc;

#[test]
fn test_calculator() {
}

pub struct DumbCalculator {
    entering: EnteringMode,
    calc: calc::DumbCalcProcessor,
}

impl DumbCalculator {
    pub fn new() -> Self {
        Self {
            entering: EnteringMode::Not,
            calc: calc::DumbCalcProcessor::new(),
        }
    }
    pub fn push(&mut self, key: &str) -> Result<(), String> {
        if key == "." {
            match self.entering {
                EnteringMode::Not => {
                    self.entering = EnteringMode::Decimal(0, 0);
                }
                EnteringMode::Integer(i) => {
                    self.entering = EnteringMode::Decimal(i, 0);
                }
                EnteringMode::Decimal(i, d) => {
                    self.entering = EnteringMode::Decimal(i, d);
                }
            }
        } else if key >= "0" && key <= "9" {
            let digit = key.parse::<u32>().unwrap();
            match self.entering {
                EnteringMode::Not => {
                    self.entering = EnteringMode::Integer(digit);
                }
                EnteringMode::Integer(i) => {
                    self.entering = EnteringMode::Integer(i * 10 + digit);
                }
                EnteringMode::Decimal(i, d) => {
                    self.entering = EnteringMode::Decimal(i, d * 10 + digit);
                }
            }
        } else {
            match self.entering {
                EnteringMode::Not => {}
                EnteringMode::Integer(i) => {
                    self.calc.push(i.to_string().as_str()).unwrap();
                    self.entering = EnteringMode::Not;
                }
                EnteringMode::Decimal(i, d) => {
                    let num = format!("{}.{}", i, d);
                    self.calc.push(num.as_str()).unwrap();
                    self.entering = EnteringMode::Not;
                }
            }
            self.calc.push(key)?;
        }
        Ok(())
    }
    pub fn get_display(&self) -> String {
        match self.entering {
            EnteringMode::Not => {
                self.calc.get_result().to_string()
            }
            EnteringMode::Integer(i) => {
                format!("{}", i)
            }
            EnteringMode::Decimal(i, d) => {
                format!("{}.{}", i, d)
            }
        }
    }
}

enum EnteringMode {
    Not,
    Integer(u32),
    Decimal(u32, u32),
}
