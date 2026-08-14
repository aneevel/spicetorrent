pub fn decode_integer(chunk: &String) -> Result<i32, String> {
    let mut tokens = chunk.chars();

    // First char should always be an 'i'
    if let Some(token) = tokens.next() {
        if token != 'i' {
            return Err("No Prefix".to_string());
        }
        return Ok(5);
    }

    Err("Not Implemented".to_string())
}

pub fn encode_integer(integer: i128) -> Result<String, String> {
    Err("Not Implemented Yet!".to_string())
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
            Err("No Prefix".to_string()),
            "Unable to correctly warn about invalid integer with no prefix!"
        );
    }

    #[test]
    fn decode_invalid_integer_no_affix() {
        let result = decode_integer(&"i10".to_string());
        assert_eq!(
            result,
            Err("No Affix".to_string()),
            "Unable to correctly warn about invalid integer with no affix!"
        );
    }

    #[test]
    fn decode_invalid_integer_leading_zero() {
        let result = decode_integer(&"i01e".to_string());
        assert_eq!(
            result,
            Err("Leading Zero".to_string()),
            "Unable to correctly warn about invalid integer with leading zero!"
        );
    }

    #[test]
    fn decode_invalid_integer_leading_zero_2() {
        let result = decode_integer(&"i001e".to_string());
        assert_eq!(
            result,
            Err("Leading Zero".to_string()),
            "Unable to correctly warn about invalid integer with two leading zeroes!"
        );
    }

    #[test]
    fn decode_invalid_integer_negative_zero() {
        let result = decode_integer(&"i-0e".to_string());
        assert_eq!(
            result,
            Err("Negative Zero".to_string()),
            "Unable to correctly warn about invalid integer with negative zero!"
        );
    }

    #[test]

    fn encode_positive_integer() {
        let result = encode_integer(10);
        assert_eq!(
            result,
            Ok("i10e".to_string()),
            "Unable to correctly encode valid integer 10!"
        );
    }

    #[test]
    fn encode_zero() {
        let result = encode_integer(0);
        assert_eq!(
            result,
            Ok("i0e".to_string()),
            "Unable to correctly encode valid integer zero!"
        );
    }

    #[test]

    fn encode_negative_integer() {
        let result = encode_integer(-10);

        assert_eq!(
            result,
            Ok("i-10e".to_string()),
            "Unable to correctly encode valid integer -10!"
        );
    }
}
