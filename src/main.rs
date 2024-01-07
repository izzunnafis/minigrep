
fn main() {
    let cmd_args: Vec<String> = std::env::args().collect();

    let query = &cmd_args[1];
    let filename = &cmd_args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let file_contents = std::fs::read_to_string(filename)
                            .expect("Something went wrong reading the file");
    
    println!("\n With text:\n {}", file_contents);
}
