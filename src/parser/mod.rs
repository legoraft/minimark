use std::fs::File;
use std::io::{BufRead, BufReader, empty};
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
        let mut first_char: char = match line_content.chars().nth(0) {
            Some(c) => c,
            None => continue,
        };
        let output_line: String;

        match first_char {
            '#' => output_line = heading::compile(&line_content),
            '-' => output_line = list::compile(&line_content),
            _ => output_line = {
                if first_char.is_numeric() {
                    list::compile(&line_content)
                } else {
                    paragraph::compile(&line_content)
                }
            },
        };

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    tokens.push("\n</body>\n".to_string());
    tokens.push("</html>".to_string());

    tokens
}