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

// 下記もエラー
// fn one(is_float: bool) -> impl fmt::Display {
//     if is_float {
//         1.0_f32
//     } else {
//         1_i32
// //      ^^^^^ expected `f32`, found `i32`
//     }
// }

// クロージャは匿名型になるので引数の型が特定できない
// fn gen_counter(init: i32) -> ??? {
//     let mut n = init;
//     move || {
//         let ret = n;
//         n += 1;
//         ret
//     }
// }
// こうなら書ける
// fn gen_counter(init: i32) -> Box<dyn FnMut() -> i32> {
//     let mut n = init;
//     Box::new(move || {
//         let ret = n;
//         n += 1;
//         ret
//     })
// }
// トレイトオブジェクトを使わなくするには下記の様に書く
fn gen_counter(init: i32) -> impl FnMut() -> i32 {
    let mut n = init;
    move || {
        let ret = n;
        n += 1;
        ret
    }
}

fn main() {
    let n = one();
    // println!("{}", n + n); // これはエラーになる
    // 28 |     println!("{}", n + n); // これはエラーになる
    //    |                    - ^ - impl std::fmt::Display
    //    |                    |
    //    |                    impl std::fmt::Display
}