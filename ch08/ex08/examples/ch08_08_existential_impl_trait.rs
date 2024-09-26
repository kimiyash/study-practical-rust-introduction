use std::iter::Filter;
use std::ops::Range;
use std::fmt;

fn to_n(n: i32) -> impl Iterator {
    0..n
}
// // impl Trait を柄はない場合以下のようは型になる
// use std::ops::Range;
// fn to_n(n: i32) -> Range<i32>
//     0..n
// }

// fn to_n_even(n: i32) -> Filter<Range<i32>, fn(&i32) -> bool> {
//     (0..n).filter(|i| i % 2 == 0)
// }
// これは下記のようにかける
fn to_n_even(n: i32) -> impl Iterator {
    (0..n).filter(|i| i % 2 == 0)
}

fn one() -> impl fmt::Display {
    1_i32
}

fn main() {
    let n = one();
    println!("{}", n + n); // これはエラーになる
    // 28 |     println!("{}", n + n); // これはエラーになる
    //    |                    - ^ - impl std::fmt::Display
    //    |                    |
    //    |                    impl std::fmt::Display
}