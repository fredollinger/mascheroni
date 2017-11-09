use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

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
