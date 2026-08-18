#[derive(PartialEq, Debug)]
pub enum IntegerDecodingError {
    NoPrefix,
    NoAffix,
    NoValue,
    LeadingZero,
    NegativeZero,
    InvalidInteger,
}

#[derive(PartialEq, Debug)]
pub enum StringDecodingError {
    NoPrefix,
    NoColon,
    IncorrectLengthSpecifier,
    InvalidContent,
}

pub fn decode_string(chunk: &String) -> Result<String, StringDecodingError> {
    let mut tokens = chunk.chars();

    // First char should always be a positive, non-zero integer
    if let Some(token) = tokens.next() {
        let length = match token.to_digit(10) {
            Some(length) => length,
            None => return Err(StringDecodingError::NoPrefix),
        };

        let mut length = match i32::try_from(length) {
            Ok(length) => length,
            Err(_) => return Err(StringDecodingError::NoPrefix),
        };

        // Grab until we have the entire integer
        let mut lookahead = tokens.clone();
        let mut multiplier = 10;

        // Go until we reach a colon
        while let Some(specifier) = lookahead.next() {
            if specifier == ':' {
                break;
            } else if !specifier.is_digit(10) {
                return Err(StringDecodingError::NoColon);
            } else {
                let specifier = match specifier.to_digit(10) {
                    Some(specifier) => specifier,
                    None => return Err(StringDecodingError::NoPrefix),
                };

                let specifier = match i32::try_from(specifier) {
                    Ok(specifier) => specifier,
                    Err(_) => return Err(StringDecodingError::NoPrefix),
                };

                length = length * multiplier + specifier;
                multiplier = multiplier * 10;
            }
        }

        // Throw away colon
        tokens.next();

        let mut result = String::from("");

        // Pop tokens until we hit that length
        for _ in 0..length {
            match tokens.next() {
                Some(token) => result.push(token),
                None => return Err(StringDecodingError::IncorrectLengthSpecifier),
            };
        }

        if result.len() as i32 != length {
            return Err(StringDecodingError::IncorrectLengthSpecifier);
        }

        if result.len() == 0 {
            return Err(StringDecodingError::InvalidContent);
        }

        return Ok(result);
    }
    return Err(StringDecodingError::InvalidContent);
}

fn encode_string(string: &String) -> String {
    return format!("{}:{}", string.len(), string);
}

pub fn decode_integer(chunk: &String) -> Result<i32, IntegerDecodingError> {
    let mut tokens = chunk.chars();

    // First char should always be an 'i'
    if let Some(token) = tokens.next() {
        if token != 'i' {
            return Err(IntegerDecodingError::NoPrefix);
        }
    }

    let mut multiplier = 1;

    let mut is_negative = false;
    let mut result: Option<i32> = None;
    while let Some(remaining) = tokens.next() {
        if remaining == 'e' {
            if let Some(_) = result {
                if is_negative {
                    return Ok(result.unwrap() * -1);
                } else {
                    return Ok(result.unwrap());
                }
            } else {
                return Err(IntegerDecodingError::NoValue);
            }
        } else if remaining == '-' {
            // Look ahead - we can only tolerate one negative symbol
            let mut lookahead = tokens.clone();
            let next_value = lookahead.next().unwrap();

            // Handle special negative zero case
            if next_value == '0' {
                return Err(IntegerDecodingError::NegativeZero);
            } else if next_value.is_numeric() == true {
                is_negative = true;
            } else {
                return Err(IntegerDecodingError::InvalidInteger);
            }
        } else if remaining == '0' {
            // If we already have a result generated, this is just another digit and should be
            // processed as one
            if result.is_some() {
                result = result.map(|x| x * multiplier);

                multiplier = multiplier * 10;
            } else {
                // Otherwise, we have a leading zero
                // Look ahead - we can only tolerate one zero, and the next must be the ending symbol.
                let mut lookahead = tokens.clone();

                if lookahead.next().unwrap() != 'e' {
                    return Err(IntegerDecodingError::LeadingZero);
                } else {
                    return Ok(0);
                }
            }
        } else {
            let remainder_as_digit = match remaining.to_digit(10) {
                Some(remainder) => remainder,

                None => return Err(IntegerDecodingError::InvalidInteger),
            };

            let remainder = match i32::try_from(remainder_as_digit) {
                Ok(remainder) => remainder,

                Err(_) => return Err(IntegerDecodingError::InvalidInteger),
            };

            result = Some(result.map_or(remainder, |x| (x * multiplier) + remainder));

            multiplier = multiplier * 10;
        }
    }

    // If we got here with no case culling, we have no affix
    Err(IntegerDecodingError::NoAffix)
}

pub fn encode_integer(integer: i128) -> String {
    return format!("i{}e", integer);
}

#[cfg(test)]
mod tests {

    use crate::bencode::*;

    #[test]
    fn decode_valid_integer_random_integer() {
        let result = decode_integer(&"i10e".to_string());
        assert_eq!(result, Ok(10), "Unable to decode integer \"10\"!");
    }

    #[test]
    fn decode_valid_integer_zero() {
        let result = decode_integer(&"i0e".to_string());
        assert_eq!(result, Ok(0), "Unable to decode integer \"0\"!");
    }

    #[test]
    fn decode_valid_integer_negative_integer() {
        let result = decode_integer(&"i-3e".to_string());
        assert_eq!(result, Ok(-3), "Unable to decode integer \"-3\"!");
    }

    #[test]
    fn decode_invalid_integer_no_prefix() {
        let result = decode_integer(&"10".to_string());
        assert_eq!(
            result,
            Err(IntegerDecodingError::NoPrefix),
            "Unable to correctly warn about invalid integer with no prefix!"
        );
    }

    #[test]
    fn decode_invalid_integer_no_affix() {
        let result = decode_integer(&"i10".to_string());
        assert_eq!(
            result,
            Err(IntegerDecodingError::NoAffix),
            "Unable to correctly warn about invalid integer with no affix!"
        );
    }

    #[test]
    fn decode_invalid_integer_leading_zero() {
        let result = decode_integer(&"i01e".to_string());
        assert_eq!(
            result,
            Err(IntegerDecodingError::LeadingZero),
            "Unable to correctly warn about invalid integer with leading zero!"
        );
    }

    #[test]
    fn decode_invalid_integer_leading_zero_2() {
        let result = decode_integer(&"i001e".to_string());
        assert_eq!(
            result,
            Err(IntegerDecodingError::LeadingZero),
            "Unable to correctly warn about invalid integer with two leading zeroes!"
        );
    }

    #[test]
    fn decode_invalid_integer_negative_zero() {
        let result = decode_integer(&"i-0e".to_string());
        assert_eq!(
            result,
            Err(IntegerDecodingError::NegativeZero),
            "Unable to correctly warn about invalid integer with negative zero!"
        );
    }

    #[test]
    fn decode_invalid_integer_no_integer() {
        let result = decode_integer(&"ie".to_string());

        assert_eq!(
            result,
            Err(IntegerDecodingError::NoValue),
            "Unable to correctly warn about string with no integer!"
        );
    }

    #[test]
    fn decode_invalid_integer_incorrect_affix() {
        let result = decode_integer(&"if".to_string());

        assert_eq!(
            result,
            Err(IntegerDecodingError::InvalidInteger),
            "Unable to correctly warn about string with invalid char in place of integer!"
        );
    }

    #[test]
    fn decode_invalid_integer_incorrect_negative_symbol() {
        let result = decode_integer(&"i--1e".to_string());

        assert_eq!(
            result,
            Err(IntegerDecodingError::InvalidInteger),
            "Unable to correctly warn about string with multiple negative symbols!"
        );
    }

    #[test]
    fn encode_positive_integer() {
        let result = encode_integer(10);
        assert_eq!(
            result,
            "i10e".to_string(),
            "Unable to correctly encode valid integer 10!"
        );
    }

    #[test]
    fn encode_zero() {
        let result = encode_integer(0);
        assert_eq!(
            result,
            "i0e".to_string(),
            "Unable to correctly encode valid integer zero!"
        );
    }

    #[test]
    fn encode_negative_integer() {
        let result = encode_integer(-10);

        assert_eq!(
            result,
            "i-10e".to_string(),
            "Unable to correctly encode valid integer -10!"
        );
    }

    #[test]
    fn decode_valid_string() {
        let result = decode_string(&"6:coding".to_string());

        assert_eq!(
            result,
            Ok("coding".to_string()),
            "Unable to correctly decode valid string '6:coding'"
        );
    }

    #[test]
    fn decode_valid_string_2() {
        let result = decode_string(&"4:spam".to_string());

        assert_eq!(
            result,
            Ok("spam".to_string()),
            "Unable to correctly decode valid string '4:spam'"
        );
    }

    #[test]
    fn decode_invalid_string_no_prefix_2() {
        let result = decode_string(&"-3:eggs".to_string());

        assert_eq!(
            result,
            Err(StringDecodingError::NoPrefix),
            "Unable to correctly report error of no prefix!"
        );
    }

    #[test]
    fn decode_invalid_string_no_content() {
        let result = decode_string(&"0:".to_string());

        assert_eq!(
            result,
            Err(StringDecodingError::InvalidContent),
            "Unable to correctly report error of invalid string content!"
        );
    }

    #[test]
    fn decode_invalid_string_no_colon() {
        let result = decode_string(&"4spam".to_string());

        assert_eq!(
            result,
            Err(StringDecodingError::NoColon),
            "Unable to correctly report error of no colon separator!"
        );
    }

    #[test]
    fn decode_invalid_string_incorrect_length_specifier() {
        let result = decode_string(&"4:egg".to_string());

        assert_eq!(
            result,
            Err(StringDecodingError::IncorrectLengthSpecifier),
            "Unable to correctly report error of incorrect length specifier!"
        );
    }

    #[test]
    fn encode_random_string() {
        let result = encode_string(&"eggs".to_string());

        assert_eq!(
            result, "4:eggs",
            "Unable to correctly encode valid string 'eggs'!"
        );
    }
}
