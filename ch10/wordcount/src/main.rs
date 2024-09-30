use std::env;
use std::fs::File;
use std::io::BufReader;

use bicycle_book_wordcount::count;

fn main() {
    // 1. コマンドラインで指定した引数を読み込む
    let filename = env::args().nth(1).expect("1 arugument FILENAME required");
    // 2. 指定子たファイルを開く
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    // 3. ファイルから1行ずつ読み込む
    let freqs = count(reader, Default::default());
    println!("{:#?}", freqs);
}
