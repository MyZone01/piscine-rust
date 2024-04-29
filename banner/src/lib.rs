use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(flag: &str, desc: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", flag.chars().next().unwrap()),
            long_hand: format!("--{}", flag),
            desc: desc.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }

    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        if let Some(callback) = self.flags.get(&flag) {
            match callback(argv[0], argv[1]) {
                Ok(result) => result.to_string(),
                Err(err) => err.to_string(),
            }
        } else {
            "Invalid flag".to_string()
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f32 = a.parse()?;
    let num2: f32 = b.parse()?;
    Ok((num1 / num2).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let num1: f32 = a.parse()?;
    let num2: f32 = b.parse()?;
    Ok((num1 % num2).to_string())
}
