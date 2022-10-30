
use std::fmt::Display;
use std::fmt::Debug;
use std::error::Error;
use std::fmt::Result;
use std::fmt::Formatter;

// custom exception with struct 
pub struct MyCustomError {
}

// implementation must for defining custom exception
impl Error for MyCustomError {
}

impl Display for MyCustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        return write!(f, "Issue seen, raising Exception");
    }
}
impl Debug for MyCustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        return write!(f, "Issue seen, raising Exception");
    }
}

// custom exception with enum
pub enum CustomErrorGroup {
    DummyException1,
    DummyException2
}

// implementation must for defining custom exception
impl  Error for CustomErrorGroup {
}

impl Display for  CustomErrorGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        return match &self {
            CustomErrorGroup::DummyException1 => return write!(f, "Issue seen, DummyException1 Exception"),
            CustomErrorGroup::DummyException2 => return return write!(f, "Issue seen, DummyException2 Exception"),
            _ => return write!(f, "Issue seen, Unknown Exception")
        };
    }
}
impl Debug for CustomErrorGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        return match &self {
            CustomErrorGroup::DummyException1 => return write!(f, "Issue seen, DummyException1 Exception"),
            CustomErrorGroup::DummyException2 => return return write!(f, "Issue seen, DummyException2 Exception"),
            _ => return write!(f, "Issue seen, Unknown Exception")
        }; 
    }
}

// using custom exceptions
pub fn error_gen_1(x: u8) -> std::result::Result<u8, CustomErrorGroup> {
    if x < 10 {
        return Err(CustomErrorGroup::DummyException1);
    }
    else {
        return Ok(23);
    }
}

// using custom exceptions
pub fn error_gen_2(x: u8) -> std::result::Result<u8, MyCustomError> {
    if x < 10 {
        return Err(MyCustomError{});
    }
    else {
        return Ok(37);
    }
}
