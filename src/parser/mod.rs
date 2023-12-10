use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod heading;

pub fn parse(filename: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let input_filename = Path::new(filename);
    let file = File::open(input_filename).expect("Error, failed to read file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line.unwrap();
        let output_line: String;

        if line_content.contains("# ") {
            output_line = heading::compile(line_content);
        } else {
            output_line = format!("<p>{}</p>\n", &line_content);
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    tokens
}