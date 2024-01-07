pub fn run(args_config: ArgsConfig) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = std::fs::read_to_string(args_config.filename)?;
    
    let query_result = if args_config.ignore_case {
        search_case_insensitive(&args_config.query, &file_contents)
    } else {
        search(&args_config.query, &file_contents)
    };

    for line in query_result {
        println!("{}", line);
    }
    
    Ok(())
}

pub struct ArgsConfig {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl ArgsConfig {
    pub fn build(args: &[String])-> Result<ArgsConfig, &'static str> {
        if args.len() < 3 {
            return  Err("Not enough arguments. Please provide atleast 2 arguments for the query and the filename");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        
        Ok(ArgsConfig { 
            query,
            filename,
            ignore_case,
         })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        );
    }
}
