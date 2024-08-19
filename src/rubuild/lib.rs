use std::{
    env::{self, var},
    error::Error,
    fs,
};
pub struct Config {
    query: String,
    filename: String,
    pub is_open: bool,
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.is_open {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}
impl Config {
    // 这里没有所有权  更改
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();
        // clone获取所有权 但是会有性能损耗
        // let query = args[1].clone();
        // let filename = args[2].clone();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("error"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("error"),
        };

        let is_open = env::var("IS_OPEN").is_err();
        Ok(Config {
            query,
            filename,
            is_open,
        })
    }
}
