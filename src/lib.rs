pub fn run(args_config: ArgsConfig) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(args_config.filename)?;

    println!("\n With text:\n {}", file_contents);
    
    Ok(())
}

pub struct ArgsConfig {
    pub query: String,
    pub filename: String,
}

impl ArgsConfig {
    pub fn build(args: &[String])-> Result<ArgsConfig, &'static str> {
        if args.len() < 3 {
            return  Err("Not enough arguments. Please provide atleast 2 arguments for the query and the filename");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(ArgsConfig { query, filename })
    }
}
