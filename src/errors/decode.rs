use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Empty Byte Sequence")]
    EmptyByteSequence,
    #[error("Invalid Start Byte")]
    InvalidStartByte,
    #[error("Invalid End Byte For Type: {0}")]
    InvalidEndByte(String),
    #[error("Invalid Integer")]
    InvalidInteger,
    #[error("Negative Zero")]
    NegativeZero,
    #[error("Leading Zero")]
    LeadingZero,
    #[error("Invalid Byte String Length")]
    InvalidByteStringLength,
    #[error("Invalid List")]
    InvalidList,
    #[error("Invalid Dictionary")]
    InvalidDictionary,
    #[error("Invalid Dictionary Key Type")]
    InvalidDictionaryKey,
}
