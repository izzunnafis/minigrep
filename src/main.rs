use minigrep::ArgsConfig;
fn main() {
    let cmd_args: Vec<String> = std::env::args().collect();

    let args_config = ArgsConfig::build(&cmd_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(args_config) { 
        eprintln!("Cannot read the file: {}", e);
        std::process::exit(1);
    }
}