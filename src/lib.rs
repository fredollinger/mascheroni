use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

/// Given a number, check to see if it's
/// a is_palindrome.
///
/// A is_palindrome is the same each ways
/// for example:
///
/// 1001 is a palindrome as is 1001001
///
/// But 71 and 1101 are not is_palindromes.
///
/// example:
///
/// let mut num = 2002;
/// println!("Is num palindrome? {} {}", num, is_palindrome(num));
///
/// num = 71;
/// println!("Is num palindrome? {} {}", num, is_palindrome(num));
pub fn is_palindrome(num: u64) -> bool {
    let bar = num.to_string();
    let bytes = bar.as_bytes();
    let len = bytes.len();

    for x in 0..len {
        if bytes[x] != bytes[len-x-1] {
            return false;
        }
    }

    return true;
}

/// Given a string convert it to u64
pub fn string2u64(st: &str) -> std::result::Result<u64, std::num::ParseIntError> {
    let mut ret: u64 = 0;
    let mut ret_str = String::from("");
    for c in st.chars() {
        if c.is_digit(10) {
        ret_str.push(c);
        }
    }
    u64::from_str(&ret_str)
}

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

#[test]
fn test_string2u64() {
    let res = string2u64("asdjflsdjfslj10");
    match res {
        Ok(n) => { assert_eq!(n, 10); },
        Err(_) => { assert!(false); }
    }
}

// #[test]
fn test_is_not_palindrome() {
    assert!(is_palindrome(71), false);
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(1001001), true);
}

