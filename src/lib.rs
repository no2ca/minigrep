use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

// 外から使うため pub を使う

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub query: String,

    #[arg(short, long)]
    pub filename: String,

    #[arg(short, long)]
    pub ignore_case: bool,
}

// BoxはErrorトレイトを実装する型を返すことを意味する
pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(args.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if args.ignore_case {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// search関数の定義
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

// 大文字小文字を区別しないsearch関数用のテスト

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );

    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            vec!["Rust", "Trust, me."],
            search(query, contents)
        );

    }
}