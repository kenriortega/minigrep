use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let query = args[2].clone();

        Config { filename, query }
    }
}

pub fn run(cfg: Config) {
    let content = fs::read_to_string(cfg.filename).expect("Can`t read Files");
    let found = search(&cfg.query, &content);
    let wc = found.len();
    for line in found {
        println!("{}", line)
    }
    println!("------------------------------------");
    println!("🎉 WC 📟 ({}) 🔎 [{}] 🎉🎉", wc, &cfg.query);
    println!("------------------------------------");
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
