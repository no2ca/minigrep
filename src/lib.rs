use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use clap::Parser;

// 外から使うため pub を使う

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(index=1)]
    pub query: String,

    #[arg(index=2)]
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
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );

    }

    // clapによる引数パースのテスト
    #[test]
    fn parse_args_basic() {
        let args = Args::try_parse_from(&["minigrep", "test", "sample.txt"]).unwrap();
        assert_eq!(args.query, "test");
        assert_eq!(args.filename, "sample.txt");
        assert_eq!(args.ignore_case, false);
    }

    #[test]
    fn parse_args_with_ignore_case_short() {
        let args = Args::try_parse_from(&["minigrep", "test", "sample.txt", "-i"]).unwrap();
        assert_eq!(args.query, "test");
        assert_eq!(args.filename, "sample.txt");
        assert_eq!(args.ignore_case, true);
    }

    #[test]
    fn parse_args_with_ignore_case_long() {
        let args = Args::try_parse_from(&["minigrep", "test", "sample.txt", "--ignore-case"]).unwrap();
        assert_eq!(args.query, "test");
        assert_eq!(args.filename, "sample.txt");
        assert_eq!(args.ignore_case, true);
    }

    #[test]
    fn parse_args_flag_before_positional() {
        let args = Args::try_parse_from(&["minigrep", "-i", "test", "sample.txt"]).unwrap();
        assert_eq!(args.query, "test");
        assert_eq!(args.filename, "sample.txt");
        assert_eq!(args.ignore_case, true);
    }

    #[test]
    fn parse_args_missing_filename() {
        let result = Args::try_parse_from(&["minigrep", "test"]);
        assert!(result.is_err());
    }

    #[test]
    fn parse_args_missing_query() {
        let result = Args::try_parse_from(&["minigrep"]);
        assert!(result.is_err());
    }

    #[test]
    fn parse_args_too_many_args() {
        let result = Args::try_parse_from(&["minigrep", "test", "sample.txt", "extra"]);
        assert!(result.is_err());
    }

    #[test]
    fn parse_args_unknown_flag() {
        let result = Args::try_parse_from(&["minigrep", "test", "sample.txt", "--unknown"]);
        assert!(result.is_err());
    }
}