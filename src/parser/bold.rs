pub fn compile(line: &String) -> String {
    let tags: Vec<&str> = line.split("**").collect();
    dbg!(tags);

    // Create a vec with all the ** tags and content
    // Find first instance of **, get content until next **
    // Repeat for any other instances of **

    let tags: Vec<&str> = vec!["<b>", "</b>"];

    let output_line: String = format!("{}Hello, world!{}\n", tags[0], tags[1]);

    output_line
}