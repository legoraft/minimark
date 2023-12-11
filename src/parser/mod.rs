use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod heading;
mod paragraph;
mod list;

pub fn parse(filename: &str) -> Vec<String> {
    let mut tokens: Vec<String> = vec!["<html>\n".to_string(), "<body>\n\n".to_string()];

    let input_filename = Path::new(filename);
    let file = File::open(input_filename).expect("Error, failed to read file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line.unwrap();
        let mut first_char: Vec<char> = line_content.chars().take(1).collect();
        let mut output_line: String;

        match first_char.pop().unwrap() {
            '#' => output_line = heading::compile(&line_content),
            '-' => output_line = list::compile(&line_content),
            _ => output_line = paragraph::compile(&line_content),
        };
    }

    tokens.push("\n</body>\n".to_string());
    tokens.push("</html>".to_string());

    tokens
}