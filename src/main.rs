mod parser;

use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use crate::parser::heading;

fn main() {
    let arguments: Vec<String> = args().collect();

    match &arguments.len() {
        2 => parse_file(&arguments[1]),
        _ => usage(),
    }
}

fn usage() {
    full_banner();
}

fn parse_file(filename: &str) {
    let mut tokens: Vec<String> = Vec::new();
    let mut output_filename = String::from(&filename[..&filename.len() - 3]);
    output_filename.push_str(".html");

    let mut output_file = File::create(&output_filename)
        .expect("Error, could not create output file!");

    let input_filename = Path::new(filename);
    let file = File::open(input_filename).expect("Error, failed to read file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line.unwrap();
        let mut output_line: String;

        if line_content.contains("# ") {
            output_line = heading::compile(line_content);
        } else {
            output_line = format!("<p>{}</p>\n", &line_content);
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    short_banner();
    println!("Parsing file {:?}...", &filename);
    for line in &tokens {
        output_file.write_all(line.as_bytes()).expect("Error, failed to write line!");
    }
    println!("Finished parsing file, check {:?}!", &output_filename);
}

fn full_banner() {
    short_banner();

    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Usage: minimark <somefile>.md");
}

fn short_banner() {
    println!("{}", get_title());
}

fn get_title() -> String{
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    title.push_str("), ");
    title.push_str(env!("CARGO_PKG_DESCRIPTION"));

    title
}