use minigrep::ArgsConfig;
fn main() {
    let cmd_args: Vec<String> = std::env::args().collect();

    let args_config = ArgsConfig::build(&cmd_args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", args_config.query);
    println!("In file {}", args_config.filename);

    if let Err(e) = minigrep::run(args_config) { 
        println!("Cannot read the file: {}", e);
        std::process::exit(1);
    }
}