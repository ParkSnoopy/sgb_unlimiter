use std::error;
use std::fmt;

pub type DecodeResult<T> = std::result::Result<T, DecodeError>;

#[derive(Debug, Clone)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "decode prebuilt string failed")
    }
}

impl error::Error for DecodeError {}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
