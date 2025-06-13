# minigrep

参考: [入出力プロジェクト：コマンドラインプログラムを構築する - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch12-00-an-io-project.html)

## 概要

Rustのチュートリアルをベースにした文字列検索ツールです。

## 基本的な使い方

```bash
cargo run <検索文字列> <ファイル名>
```

## 追加機能

### 1. Clapライブラリによる引数パース

コマンドライン引数を手動でパースしていましたが、Clapライブラリを使用した引数処理を実装しました。

### 2. 行番号表示機能

`-n` または `--line-number` オプションを使用することで、マッチした行の行番号を表示できます。

### 3. フラグ配置

Clapライブラリにより、フラグを位置引数の前後どちらでも指定できます。

```bash
cargo run -i the poem.txt
cargo run the poem.txt -i
```
