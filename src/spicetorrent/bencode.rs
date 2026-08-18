use regex::Regex;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum BencodeType {
    Int(i32),
    Str(String),
}

impl fmt::Display for BencodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BencodeType::Int(value) => write!(f, "{}", value),
            BencodeType::Str(value) => write!(f, "{}", value),
        }
    }
}

pub fn decode_list(chunk: &String) -> Vec<BencodeType> {
    return vec![];
}

pub fn encode_list(list: Vec<BencodeType>) -> String {
    let mut encoded = String::from("l");
    for element in list {
        match element {
            BencodeType::Int(value) => encoded.push_str(&encode_integer(value)),
            BencodeType::Str(value) => encoded.push_str(&encode_string(&value)),
        }
    }

    encoded.push('e');
    return encoded;
}

pub fn decode_string(chunk: &String) -> String {
    return String::from("");
}

fn encode_string(string: &String) -> String {
    return format!("{}:{}", string.len(), string);
}

pub fn decode_integer(chunk: &String) -> i32 {
    let regex = Regex::new(r"(i)(-?\d+)(e)").unwrap();
    let Some(captures) = regex.captures(chunk) else {
        return 0;
    };

    for capture in captures.iter() {
        println!("Capture is: {}", capture.unwrap().as_str());
    }
    return captures[2].parse::<i32>().unwrap();
}

pub fn encode_integer(integer: i32) -> String {
    return format!("i{}e", integer);
}

#[cfg(test)]
mod tests {

    use crate::bencode::*;

    #[test]
    fn decode_valid_integer_random_integer() {
        let result = decode_integer(&"i10e".to_string());
        assert_eq!(result, 10, "Unable to decode integer \"10\"!");
    }

    #[test]
    fn decode_valid_integer_zero() {
        let result = decode_integer(&"i0e".to_string());
        assert_eq!(result, 0, "Unable to decode integer \"0\"!");
    }

    #[test]
    fn decode_valid_integer_negative_integer() {
        let result = decode_integer(&"i-3e".to_string());
        assert_eq!(result, -3, "Unable to decode integer \"-3\"!");
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
            "coding".to_string(),
            "Unable to correctly decode valid string '6:coding'"
        );
    }

    #[test]
    fn decode_valid_string_2() {
        let result = decode_string(&"4:spam".to_string());

        assert_eq!(
            result,
            "spam".to_string(),
            "Unable to correctly decode valid string '4:spam'"
        );
    }

    #[test]
    fn decode_valid_string_3() {
        let result = decode_string(&"10:Challenges".to_string());

        assert_eq!(
            result,
            "Challenges".to_string(),
            "Unable to correctly decode valid string '10:Challenges"
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

    #[test]
    fn decode_valid_list_with_strings() {
        let result = decode_list(&"l6:Coding10:Challengese".to_string());

        assert_eq!(
            result,
            vec![
                BencodeType::Str("Coding".to_string()),
                BencodeType::Str("Challenges".to_string())
            ],
            "Unable to correctly decode list of strings!"
        );
    }

    #[test]
    fn decode_valid_list_with_ints() {
        let result = decode_list(&"li10ei5ee".to_string());

        assert_eq!(
            result,
            vec![BencodeType::Int(10), BencodeType::Int(5)],
            "Unable to correctly decode list of ints!"
        );
    }

    #[test]
    fn decode_valid_list_with_strings_and_ints() {
        let result = decode_list(&"li10ei5e6:Coding10:Challengese".to_string());

        assert_eq!(
            result,
            vec![
                BencodeType::Int(10),
                BencodeType::Int(5),
                BencodeType::Str("Coding".to_string()),
                BencodeType::Str("Challenges".to_string())
            ],
            "Unable to correctly decode mixed list of ints and strings"
        );
    }

    #[test]
    fn encode_valid_list_with_strings() {
        let result = encode_list(vec![
            BencodeType::Str("Coding".to_string()),
            BencodeType::Str("Challenges".to_string()),
        ]);

        assert_eq!(
            result, "l6:Coding10:Challengese",
            "Unable to correctly encode valid list of strings"
        );
    }

    #[test]
    fn encode_valid_list_with_ints() {
        let result = encode_list(vec![BencodeType::Int(5), BencodeType::Int(32)]);

        assert_eq!(
            result, "li5ei32ee",
            "Unable to correctly encode valid list of integers"
        );
    }

    #[test]
    fn encode_valid_list_with_strings_and_ints() {
        let result = encode_list(vec![
            BencodeType::Int(5),
            BencodeType::Str("Coding".to_string()),
            BencodeType::Int(32),
        ]);

        assert_eq!(
            result, "li5e6:Codingi32ee",
            "Unable to correctly encode valid list of mixed strings and ints"
        );
    }

    /*
    #[test]
    fn decode_valid_dictionary_with_ints() {
        let result = decode_dictionary("d3:cowi3e3:mooi3e3e");
        let expected: HashMap<String, i32> = HashMap::new();
        expected.insert("cow".to_string(), 3);
        expected.insert("moo".to_string(), 3);

        assert_eq!(
            result, expected,
            "Unable to correctly decode dictionary of String/i32 pairs"
        );
    }*/
}
