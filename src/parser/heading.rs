pub fn compile(line: String) -> String {
    let (tag, content) = line.split_once(" ").unwrap();

    let tag = format!("<h{0}> </h{0}>", tag.len() as i64);
    let tags: Vec<&str> = tag.split_whitespace().collect();

    let output_line: String = format!("{}{content}{}\n", tags[0], tags[1]);

    output_line
}