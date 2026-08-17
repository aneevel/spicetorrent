#[derive(PartialEq, Debug)]
pub enum IntegerDecodingError {
    NoPrefix,
    NoAffix,
    NoValue,
    LeadingZero,
    NegativeZero,
    InvalidInteger,
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
}
