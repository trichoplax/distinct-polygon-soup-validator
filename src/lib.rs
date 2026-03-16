use std::{fmt, num::ParseIntError};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_distinct_polygon_soup(soup: &str) -> String {
    let ordinate_result = Ordinate::try_new(soup);
    match ordinate_result {
        Ok(ordinate) => format!("{}", ordinate),
        Err(e) => format!("{}", e),
    }
}

struct Ordinate {
    value: i32,
}

impl Ordinate {
    fn try_new(ordinate_string: &str) -> Result<Self, OrdinateError> {
        let parsed_ordinate = ordinate_string.trim().parse();
        match parsed_ordinate {
            Err(e) => Err(OrdinateError::Parse {
                input: ordinate_string.to_string(),
                error: e,
            }),
            Ok(0..=65) => Ok(Ordinate {
                value: parsed_ordinate.expect("The parsed ordinate is known to be Ok."),
            }),
            Ok(_) => Err(OrdinateError::Range(
                parsed_ordinate.expect("The parsed ordinate is known to be Ok."),
            )),
        }
    }
}

impl fmt::Display for Ordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
enum OrdinateError {
    Parse { input: String, error: ParseIntError },
    Range(i32),
}

impl fmt::Display for OrdinateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrdinateError::Range(v) => write!(f, "{} is not in the range 0 to 65 inclusive.", v),
            OrdinateError::Parse { input: s, error: e } => write!(
                f,
                "The string '{}' cannot be parsed as an integer. Cause: {}.",
                s, e
            ),
        }
    }
}
