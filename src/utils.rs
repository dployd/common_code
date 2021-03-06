use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::Path;
use std::process::Command;
use std::{format, fs, print, println, str, usize};

use chrono::NaiveDateTime;
use colored::Colorize;
use itertools::Itertools;
use names::Generator;
use prettytable::Cell;
use string_builder::Builder;

#[must_use]
pub fn vec_to_string(vec: &[String], quotes: bool) -> String {
    let mut builder = Builder::default();
    builder.append("[");
    for (i, item) in vec.iter().enumerate() {
        if quotes {
            builder.append(quote(&item.to_string()));
        } else {
            builder.append(item.as_bytes());
        }
        if i < vec.len() - 1 {
            builder.append(", ");
        }
    }
    builder.append("]");

    builder.string().unwrap_or_default()
}

fn align_content(preferred_size: usize, content_size: usize) -> usize {
    if preferred_size < content_size {
        return 0;
    }
    preferred_size - content_size
}

fn add_whitespaces(builder: &mut Builder, indentation: usize) {
    for _j in 0..indentation {
        builder.append(' ');
    }
}

pub fn ident_and_append(builder: &mut Builder, string: &str, indentation: usize) {
    self::add_whitespaces(builder, indentation);
    builder.append(string);
}

pub fn add_indented_aligned_key_value(
    builder: &mut Builder,
    indentation: usize,
    preferred_size: usize,
    key: &str,
    value: &str,
) {
    ident_and_append(builder, key, indentation);
    add_whitespaces(builder, align_content(preferred_size, key.len()));
    builder.append(" = ");
    builder.append(value);
    builder.append("\n");
}

#[must_use]
pub fn quote(value: &str) -> String {
    let str = value.replace('"', "\\\"");
    let result = str.replace("\\\\\"", "\\\"");
    format!("\"{}\"", result)
}

#[must_use]
pub fn get_random_name() -> String {
    let mut generator = Generator::default();
    if let Some(string) = generator.next() {
        return string;
    }
    String::from("random-name")
}

#[must_use]
pub fn get_ok_or_error(result: bool) -> String {
    if result {
        return format!("[  {}  ]", "OK".green());
    }
    return format!("[{}]", "FAILED".red());
}

pub fn print_with_offset(str: &str) {
    println!("[      ]: {}", str);
}

pub fn print_information(str: &str) {
    println!("[{}]: {}", " INFO ".cyan(), str);
}

pub fn print_message(message: &str, status: bool) {
    println!("{}: {}", get_ok_or_error(status), message);
}

/// # Errors
///
/// Will return `Err` if `filename` could not be written to
pub fn append_to_file(filename: &str, line: String) -> io::Result<()> {
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(filename) {
        let mut lines = BufReader::new(file)
            .lines()
            .filter_map(|l| match l {
                Ok(line) => Some(line),
                Err(_) => None,
            })
            .collect::<Vec<String>>();
        lines.push(line);
        return fs::write(filename, lines.join("\n"));
    }
    return Err(std::io::Error::new(
        ErrorKind::Other,
        format!("can not open {} in append_to_file", filename),
    ));
}

/// # Errors
///
/// Will return `Err` if `filename` could not be written to
pub fn remove_line_from_file(filename: &str, line: &str) -> io::Result<()> {
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(filename) {
        let lines = BufReader::new(file)
            .lines()
            .filter_map(|l| match l {
                Ok(line) => Some(line),
                Err(_) => None,
            })
            .filter(|l| !l.eq(&line))
            .collect::<Vec<String>>()
            .join("\n");
        return fs::write(filename, lines);
    }
    return Err(std::io::Error::new(
        ErrorKind::Other,
        format!("can not open {} in remove_line_from_file", filename),
    ));
}

/// # Errors
///
/// Will return `Err` if `filename` could not be written to
pub fn remove_line_with_substring_from_file(filename: &str, substring: &str) -> io::Result<()> {
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(filename) {
        let lines = BufReader::new(file)
            .lines()
            .filter_map(|l| match l {
                Ok(line) => Some(line),
                Err(_) => None,
            })
            .filter(|l| !l.contains(&substring))
            .collect::<Vec<String>>()
            .join("\n");
        return fs::write(filename, lines);
    }
    return Err(std::io::Error::new(
        ErrorKind::Other,
        format!(
            "can not open {} in remove_line_with_substring_from_file",
            filename
        ),
    ));
}

/// # Errors
///
/// Will return `Err` if `filename` could not be opened for reading and writing
pub fn get_lines_from_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = OpenOptions::new().read(true).write(true).open(filename)?;

    Ok(BufReader::new(file)
        .lines()
        .map(std::result::Result::unwrap)
        .collect::<Vec<String>>())
}

/// # Errors
///
/// Will return `Err` if `filename` could not be written to
pub fn replace_in_file(filename: &str, needle: &str, replacement: &str) -> io::Result<()> {
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(filename) {
        let lines = BufReader::new(file)
            .lines()
            .map_ok(|l| l.replace(needle, replacement))
            .map(std::result::Result::unwrap)
            .collect::<Vec<String>>()
            .join("\n");
        return fs::write(filename, lines);
    }
    return Err(std::io::Error::new(
        ErrorKind::Other,
        format!("can not open {} in replace_in_file", filename),
    ));
}

#[must_use]
pub fn read_line(prompt: &str) -> String {
    print!("{}: ", prompt);
    if let Ok(_flushed) = io::stdout().flush() {}
    let mut string = String::new();
    if let Ok(_flushed) = io::stdin().read_line(&mut string) {}
    string.replace("\n", "")
}

#[must_use]
pub fn get_cell_content_of_string(string: &str) -> Cell {
    Cell::new(string)
}

#[must_use]
pub fn get_cell_content_of_option(content: &Option<String>) -> String {
    match content {
        Some(string) => String::from(string),
        None => String::from("\u{2014}"),
    }
}

#[must_use]
pub fn get_cell_content_of_date(content: &Option<NaiveDateTime>) -> String {
    match content {
        Some(date) => date.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => String::from("\u{2014}"),
    }
}

#[must_use]
pub fn filter_lines_by_substring(lines: &[String], needle: &str) -> Vec<String> {
    lines
        .iter()
        .filter_map(|line| {
            if line.contains(needle) {
                Some(line.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
}

#[must_use]
pub fn remove_colors(line: &str) -> String {
    line.replace("\u{1b}[0m", "")
        .replace("\u{1b}[0;32m", "")
        .replace("\u{1b}[1;32m", "")
}

/// # Errors
///
/// Will return `Err` if `filename` could not be opened for reading and writing
pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut builder = Builder::default();
    for line in get_lines_from_file(filename)? {
        builder.append(line);
    }
    match builder.string() {
        Ok(string) => Ok(string),
        Err(e) => Err(std::io::Error::new(ErrorKind::Other, e)),
    }
}

#[must_use]
pub fn sha256sum_of_file(filepath: &str) -> Option<String> {
    let path = Path::new(filepath);
    if path.exists() {
        if let Ok(output) = Command::new("sha256sum").arg(filepath).output() {
            if output.status.success() {
                if let Ok(str) = str::from_utf8(&output.stdout) {
                    let sum = str.split(' ').collect::<Vec<&str>>();
                    if let Some(sum) = sum.first() {
                        if let Ok(sum) = sum.parse() {
                            return Some(sum);
                        }
                    }
                }
            }
        }
    }
    None
}

#[must_use]
pub fn sha256sum_matches(filepath: &str, checksum: &str) -> bool {
    match sha256sum_of_file(filepath) {
        Some(sum) => sum.eq(checksum),
        None => false,
    }
}
