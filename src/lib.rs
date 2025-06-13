use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(index = 1)]
    pub query: String,

    #[arg(index = 2)]
    pub filename: String,

    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    #[arg(short = 'n', long = "line-number")]
    pub line_number: bool,

}

pub struct SearchConfig {
    pub ignore_case: bool,
    pub line_number: bool,
}

impl SearchConfig {
    pub fn from_args(args: &Args) -> Self {
        Self {
            ignore_case: args.ignore_case,
            line_number: args.line_number,
        }
    }
}

// search関数の定義
pub fn search<'a>(
    query: &str,  
    contents: &'a str,
    config: &SearchConfig
) -> Vec<String> {
    contents
        .lines() 
        .enumerate() 
        .filter(|(_, line)| {
            let line_to_check = if config.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            let query_to_check = if config.ignore_case {
                query.to_lowercase()
            } else {
                query.to_string()
            };
            // 含むものだけ返す
            line_to_check.contains(&query_to_check)
        })
        .map(|(line_num, line)| {
            if config.line_number {
                format!("{}:{}", line_num + 1, line)
            } else {
                line.to_string()
            }
        })
        // 最終的な出力を Vec<String> で返す
        .collect()
}

// BoxはErrorトレイトを実装する型を返すことを意味する
pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(&args.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let config = SearchConfig::from_args(&args);
    let results = search(&args.query, &contents, &config);

    for line in results {
        println!("{}", line);
    }

    Ok(())
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

        let config = SearchConfig {
            ignore_case: false,
            line_number: false,
        };

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, &config)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        let config = SearchConfig {
            ignore_case: true,
            line_number: false,
        };

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, &config)
        );
    }

    #[test]
    fn with_line_number() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let config = SearchConfig {
            ignore_case: false,
            line_number: true,
        };

        assert_eq!(
            vec!["2:safe, fast, productive."],
            search(query, contents, &config)
        );
    }

    #[test]
    fn case_insensitive_with_line_number() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        let config = SearchConfig {
            ignore_case: true,
            line_number: true,
        };

        assert_eq!(
            vec!["1:Rust:", "3:Trust me."],
            search(query, contents, &config)
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