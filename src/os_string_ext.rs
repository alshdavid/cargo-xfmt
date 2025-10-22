#![allow(dead_code)]
use std::ffi::OsStr;
use std::ffi::OsString;

pub trait OsStringExt {
    fn try_to_string(self) -> std::io::Result<String>;
}

impl OsStringExt for OsString {
    fn try_to_string(self) -> std::io::Result<String> {
        match self.into_string() {
            Ok(name) => Ok(name),
            Err(_) => Err(std::io::Error::other(
                "Unable to convert OsString to String",
            )),
        }
    }
}

impl OsStringExt for &OsStr {
    fn try_to_string(self) -> std::io::Result<String> {
        match self.to_str() {
            Some(name) => Ok(name.to_string()),
            None => Err(std::io::Error::other(
                "Unable to convert OsString to String",
            )),
        }
    }
}

impl OsStringExt for Option<&OsStr> {
    fn try_to_string(self) -> std::io::Result<String> {
        match self {
            Some(name) => Ok(name.try_to_string()?),
            None => Err(std::io::Error::other(
                "Unable to convert OsString to String",
            )),
        }
    }
}
