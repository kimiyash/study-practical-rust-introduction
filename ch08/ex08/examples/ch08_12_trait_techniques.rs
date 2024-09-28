use std::fmt;

#[derive(Debug)]
enum Either<A, B> {
    Hoge(A),
    Uhi(B),
}

impl<A, B> fmt::Display for Either<A, B>
where
    A: fmt::Display,
    B: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Either::Hoge(a) => a.fmt(f),
            Either::Uhi(b) => b.fmt(f),
        }
    }
}

fn main() {
    // Vec<Either<bool, i32>>として宣言しておく
    let mut v: Vec<Either<bool, i32>> = vec![];
    // Either の値をいれる
    v.push(Either::Hoge(true));
    v.push(Either::Uhi(1_i32));
    //すると `{}` で表示できる
    for e in v {
        println!("{}", e);
    }
}