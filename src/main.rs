extern crate minigrep;
use std::process;
use clap::Parser;

use minigrep::Args;
fn main() {
    // 引数をパースする
    let args = Args::parse();
    
    if let Err(e) = minigrep::run(args) {
        println!("Application error: {}", e);
        // エラーコード1で終了する
        process::exit(1);
    }
}