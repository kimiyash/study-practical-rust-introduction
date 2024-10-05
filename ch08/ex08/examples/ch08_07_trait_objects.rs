use std::fmt::Display;
use std::string::ToString;

fn stringfy(t: Box<dyn ToString>) -> String {
    t.to_string()
}

fn stringfy_i32(t: i32) -> String {
    <i32 as ToString>::to_string(&t)
}

fn stringfy_u64(t: u64) -> String {
    <u64 as ToString>::to_string(&t)
}

fn main() {
    // stringfy(1_i32);
    // stringfy::<i32>(1_i32);
    stringfy(Box::new(1));
    stringfy(Box::new("test".to_string()));

    stringfy_i32(1_i32);
    stringfy_u64(1_u64);

    // // Vec はトレイトでで宣言できない、コンパイル時にサイズが決まらないといけない
    // let mut v: Vec<Display> = vec![]; // doesn't have a size known at compile-time
    // v.push(true);
    // v.push(1_i32);

    // トレイトオブジェクトにすると入れられる
    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1_i32);
}
