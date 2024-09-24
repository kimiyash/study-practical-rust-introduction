fn main() {
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");
    // Rust 1.19 より前のバージョンでは性能上の理由から to_owned() が推奨されていたが、いまは関係なし
    let s3 = "ストロベリー".to_owned();
    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");
    s2.push('と');
    // push_strが受け付けるのは &str のみだが & をつけると型強制という仕組みで &String から &str へ変換される
    s2.push_str(&s3);
    assert_eq!(s2, "ブラックベリーとストロベリー");
    println!("{}", s3);
}
