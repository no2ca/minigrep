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

    #[arg(short = 'v', long = "invert-match")]
    pub invert_match: bool,

    #[arg(short = 'w', long = "whole-word")]
    pub whole_word: bool,

    #[arg(short = 'E', long = "regex")]
    pub regex: bool,

}

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub ignore_case: bool,
    pub line_number: bool,
    pub invert_match: bool,
    pub whole_word: bool,
    pub regex: bool,
}

impl SearchConfig {
    pub fn from_args(args: &Args) -> Self {
        Self {
            ignore_case: args.ignore_case,
            line_number: args.line_number,
            invert_match: args.invert_match,
            whole_word: args.whole_word,
            regex: args.regex,
        }
    }
}

// search関数の定義
pub fn search<'a>(
    query: &str,  
    contents: &'a str,
    config: &SearchConfig
) -> Result<Vec<String>, Box<dyn Error>> {
    let processed_query = if config.ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    let results: Result<Vec<String>, Box<dyn Error>> = contents
        .lines() 
        .enumerate() 
        .filter_map(|(line_num, line)| {
            match match_line(line, &processed_query, config) {
                Ok(matches) => {
                    // 該当する行が無いならNoneを返す
                    if matches ^ config.invert_match {
                        Some(Ok((line_num, line)))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(e))
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|pairs| {
            pairs.into_iter()
                .map(|(line_num, line)| format_output(line_num, line, config))
                .collect()
        });

        results
        
}

fn match_line(line: &str, query: &str, config: &SearchConfig) -> Result<bool, Box<dyn Error>> {
    let line_to_check = if config.ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };

    if config.regex {
        let regex = regex::Regex::new(query)?;
        Ok(regex.is_match(&line_to_check))
    } else if config.whole_word {
        Ok(line_to_check.split_whitespace().any(|word| word == query))
    } else {
        Ok(line_to_check.contains(query))
    }
}

fn format_output(line_num: usize, line: &str, config: &SearchConfig) -> String {
    if config.line_number {
        format!("{}:{}", line_num + 1, line)
    } else {
        line.to_string()
    }
}

// BoxはErrorトレイトを実装する型を返すことを意味する
pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(&args.filename)?;
    
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let config = SearchConfig::from_args(&args);
    let results = search(&args.query, &contents, &config)?;

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
            invert_match: false,
            whole_word: false,
            regex: false,
        };

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, &config).unwrap()
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
            invert_match: false,
            whole_word: false,
            regex: false,
        };

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, &config).unwrap()
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
            invert_match: false,
            whole_word: false,
            regex: false,
        };

        assert_eq!(
            vec!["2:safe, fast, productive."],
            search(query, contents, &config).unwrap()
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
            invert_match: false,
            whole_word: false,
            regex: false,
        };

        assert_eq!(
            vec!["1:Rust:", "3:Trust me."],
            search(query, contents, &config).unwrap()
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

    #[test]
    fn invert_match() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let config = SearchConfig {
            ignore_case: false,
            line_number: false,
            invert_match: true,
            whole_word: false,
            regex: false,
        };

        assert_eq!(
            vec!["Rust:", "Pick three.", "Trust me."],
            search(query, contents, &config).unwrap()
        );
    }

    #[test]
    fn invert_match_with_line_number() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let config = SearchConfig {
            ignore_case: false,
            line_number: true,
            invert_match: true,
            whole_word: false,
            regex: false,
        };

        assert_eq!(
            vec!["1:Rust:", "3:Pick three.", "4:Trust me."],
            search(query, contents, &config).unwrap()
        );
    }

    #[test]
    fn whole_word_match() {
        let query = "rust";
        let contents = "\
Rust language
Trust me with rust
rust is great
rusty old car";

        let config = SearchConfig {
            ignore_case: true,
            line_number: false,
            invert_match: false,
            whole_word: true,
            regex: false,
        };

        assert_eq!(
            vec!["Rust language", "Trust me with rust", "rust is great"],
            search(query, contents, &config).unwrap()
        );
    }

    #[test]
    fn whole_word_no_match() {
        let query = "car";
        let contents = "\
I care about cars
Careful with the car
scar on my arm";

        let config = SearchConfig {
            ignore_case: false,
            line_number: false,
            invert_match: false,
            whole_word: true,
            regex: false,
        };

        assert_eq!(
            vec!["Careful with the car"],
            search(query, contents, &config).unwrap()
        );
    }

    #[test]
    fn whole_word_with_line_number() {
        let query = "me";
        let contents = "\
Trust me
Some text here
Meet me at home
Welcome to the party";

        let config = SearchConfig {
            ignore_case: false,
            line_number: true,
            invert_match: false,
            whole_word: true,
            regex: false,
        };

        assert_eq!(
            vec!["1:Trust me", "3:Meet me at home"],
            search(query, contents, &config).unwrap()
        );
    }

    #[test]
    fn invert_match_and_whole_word() {
        let query = "rust";
        let contents = "\
Rust language
Trust me with rust
rust is great
rusty old car
Python programming";

        let config = SearchConfig {
            ignore_case: true,
            line_number: false,
            invert_match: true,
            whole_word: true,
            regex: false,
        };

        assert_eq!(
            vec!["rusty old car", "Python programming"],
            search(query, contents, &config).unwrap()
        );
    }

    #[test]
    fn regex_basic() {
        let query = r"r.st";
        let contents = "\
Rust programming
Python code
Trust me
rest well";

        let config = SearchConfig {
            ignore_case: false,
            line_number: false,
            invert_match: false,
            whole_word: false,
            regex: true,
        };

        assert_eq!(
            vec!["Trust me", "rest well"],
            search(query, contents, &config).unwrap()
        );
    }

    #[test]
    fn regex_case_insensitive() {
        let query = r"RUST";
        let contents = "\
Rust programming
Python code
Trust with rust";

        let config = SearchConfig {
            ignore_case: true,
            line_number: false,
            invert_match: false,
            whole_word: false,  
            regex: true,
        };

        assert_eq!(
            vec!["Rust programming", "Trust with rust"],
            search(query, contents, &config).unwrap()
        );
    }  
    #[test]
    fn invalid_regex_should_return_error() {
        // 不正な正規表現の例： `*` は何かに続く必要があるが、単独で使われている
        let query = r"*";
        let contents = "some text\nto search through";

        let config = SearchConfig {
            ignore_case: false,
            line_number: false,
            invert_match: false,
            whole_word: false,
            regex: true, // regex モードは有効
        };

        // search関数は Result を返すと仮定
        let result = search(query, contents, &config);

        // 戻り値が Err であることを確認する
        assert!(result.is_err(), "Expected an error for invalid regex, but got Ok");

    }

    #[test]
    fn search_in_empty_contents() {
        let query = "a";
        let contents = ""; // 検索対象が空
        let config = SearchConfig {
            ignore_case: false,
            line_number: false,
            invert_match: false,
            whole_word: false,
            regex: true,
        };

        assert_eq!(
            Vec::<&str>::new(), // 空のベクタが返されることを期待
            search(query, contents, &config).unwrap() // このケースは成功するので unwrap してOK
        );
    }

}