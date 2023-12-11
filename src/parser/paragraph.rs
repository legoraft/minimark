pub fn compile(line: &String) -> String {
    let tags: Vec<&str> = vec!["<p>", "</p>"];

    let output_line: String = format!("{}{line}{}\n", tags[0], tags[1]);

    output_line
}