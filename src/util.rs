/*
 * Copyright (c) 2019. The information/code/data contained within this file and all other files with the same copyright are protected under US Statutes. You must have explicit written access by Thomas P. Braun in order to access, view, modify, alter, or apply this code in any context commercial or non-commercial. If you have this code but were not given explicit written access by Thomas P. Braun, you must destroy the information herein for legal safety. You agree that if you apply the concepts herein without any written access, Thomas P. Braun will seek the maximum possible legal retribution.
 */

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use bincode::ErrorKind;
use std::error::Error;

#[allow(unused_must_use)]
/// Serializes an entity to the disk
pub(crate) fn serialize_entity_to_disk<T: serde::Serialize>(full_path: String, entity: T) -> Result<(), Box<ErrorKind>> {
    let writer = BufWriter::new(File::create(sanitize_path(full_path)).unwrap());
    bincode::serialize_into(writer, &entity)
}

/// Deserialized an entity to the disk
pub(crate) fn deserialize_entity_from_disk<'de, T: serde::de::DeserializeOwned>(full_path: String) -> Result<T, Box<ErrorKind>> {
    let reader = BufReader::new(File::open(sanitize_path(full_path)).unwrap());
    let entity: Result<T, _> = bincode::config().deserialize_from(reader);
    entity
}

#[allow(dead_code)]
fn condense_str_vec(str_vec: Vec<&str>) -> String {
    str_vec.concat()
}

/// The default name for the default EntropyBank
pub(crate) static ENTROPY_BANK_DEFAULT_FILE: &str = "local_storage";

pub(crate) static HOME_DIR: &str = ".quantum_random";

/// Checks to see if the EntropyBank exists locally
pub(crate) fn entropy_file_exists() -> bool {
    File::open(sanitize_path(format!(
        "{}cfg/{}.entropy",
        get_home_dir(),
        ENTROPY_BANK_DEFAULT_FILE
    )))
        .is_ok()
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(crate) fn sanitize_path(path: String) -> String {
    path.replace("\\", "")
}

#[cfg(any(target_os = "windows"))]
pub(crate) fn sanitize_path(path: String) -> String {
    path.replace("/", "\\")
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
pub(crate) fn get_home_dir() -> String {
    let p: Box<Path> = dirs_2::home_dir().unwrap().into_boxed_path();
    let j = p.to_str().unwrap();
    format!("{}/{}/", j, HOME_DIR)
}

#[cfg(any(target_os = "windows"))]
pub(crate) fn get_home_dir() -> String {
    let p: Box<Path> = dirs_2::home_dir().unwrap().into_boxed_path();
    let j = p.to_str().unwrap();
    format!("{}\\{}\\", j, HOME_DIR)
}

use std::fmt::{Display, Debug, Formatter};
use std::marker::PhantomData;

#[derive(Debug)]
/// Allows easy propagation of errors
pub struct StringedError<'a, T: 'a + ToString + Display + Debug>(T, PhantomData<&'a T>);

/// Convenience wrapper for a boxe'd error
pub type QuantumError<'a> = Box<StringedError<'a, String>>;

/// Allows easy throwing of errors
pub trait Throwable<'a, E: Error> {
    /// Inner type
    /// Throws a returnable error
    fn throw<U, T: ToString + Display>(input: T) -> Result<U, E>;

    /// Prints the error
    fn printf(self) -> Self;
}
impl<'a, T: 'a + ToString + Display + Debug> Error for StringedError<'a, T> {}

impl<'a> Throwable<'a, Self> for QuantumError<'a> {
    fn throw<U, T: ToString + Display>(input: T) -> Result<U, Self> {
        Err(Box::new(StringedError(input.to_string(), PhantomData)))
    }

    fn printf(self) -> Self{
        println!("{}", self.0);
        self
    }
}

impl<'a, T: 'a + ToString + Display + Debug> Display for StringedError<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}
