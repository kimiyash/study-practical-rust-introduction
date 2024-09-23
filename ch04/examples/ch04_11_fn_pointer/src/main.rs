fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}

fn main() {
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);
    f = abs;
    assert_eq!(f(-42), 42);

    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

    // let mut f_bad = double; // fn(i32) -> i32 {double} の型になる
    // f_bad = abs; // 関数は厳密には別の型になっているためコンパイルエラー
    // let f_bad: fn(i32) -> i32 {double} = double: // こういう書き方はできない

    let f_bad = double;
    assert_eq!(std::mem::size_of_val(&f_bad), 0); // 不思議、コンパイラはそれぞれの関数の位置が自明だから、とのことらしい
}
