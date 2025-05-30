extern crate minigrep;
use std::env;
use std::process;

use minigrep::Config;
fn main() {
    // 引数を文字列型を格納するベクトルとして集結させる
    let args: Vec<String> = env::args().collect();
    
    // Configのnewを呼び出してインスタンスを生成
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        // エラーコード1で終了する
        process::exit(1);
    }
}