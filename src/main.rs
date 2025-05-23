// 2モジュール以上ネストされている
use std::env;
fn main() {
    // 引数を文字列型を格納するベクトルとして集結させる
    let args: Vec<String> = env::args().collect();
    // ?はデバッグ成型機
    println!("{:?}", args);
}
