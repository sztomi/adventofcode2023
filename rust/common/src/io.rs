use crate::result::Result;
use std::fs;
use std::io::{BufRead, BufReader};
use std::{fs::File, path::Path};

pub fn get_lines<P: AsRef<Path>>(file_name: P) -> Result<Vec<String>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(&file);
    Ok(reader.lines().map(|line| line.unwrap()).collect())
}

pub fn get_chars<P: AsRef<Path>>(file_name: P) -> Result<Vec<char>> {
    Ok(fs::read_to_string(file_name)?.trim().chars().collect())
}

pub fn get_char_map<P: AsRef<Path>>(file_name: P) -> Result<Vec<Vec<char>>> {
    Ok(get_lines(file_name)?.iter().map(|l| l.chars().collect()).collect())
}