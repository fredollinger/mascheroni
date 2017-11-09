use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

/// Write a number to a given filename
///
/// If the file all ready exists, the number will be appended.
///
/// Each number gets its own line and is comma delimited so one can
/// diff the output of two files more easily as well as import the data
/// into other application.
///
/// Example:
///
/// write_num(1, "foo.txt");
pub fn write_num(num: u64, fname: &str) -> std::io::Result<()>
{
    let s: String = num.to_string() + ",\n";
    if Path::new(fname).exists() {
        let mut file = OpenOptions::new().append(true).open(fname)?;
        file.write_all(s.as_bytes())?;
    }
    else {
        let mut file = File::create(fname)?;
        file.write_all(s.as_bytes())?;
    }
    return Ok(());
}

pub fn public_function() {
    println!("called rary's `public_function()`");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
