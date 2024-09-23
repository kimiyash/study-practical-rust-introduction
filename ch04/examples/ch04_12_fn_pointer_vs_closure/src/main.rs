fn main() {
    let x = 4;
    let adder = |n| n + x;
    assert_eq!(adder(2), 4 + 2);

    let mut state = false;
    let mut flipflap = || {
        state = !state;
        state
    };
    assert!(flipflap());
    assert!(!flipflap());
    assert!(flipflap());
    assert!(state);

    let b = 5;
    let mut f = |a: i32| a * 3 + b;
    // f = |a :i32| a * 3 + b; // コンパイルエラー、定義が同じでも別々の型としてクロージャは扱われる

    let mut f: fn(i32) -> i32 = |n| n * 3;
    assert_eq!(f(-42), -126);

    let x = 4;
    // f = |n| n * x; // コンパイルエラー、変数をキャプチャするクロージャは関数ポインタ型になれない

    let v = vec!["I", "love", "Rust"]
        .into_iter()
        .map(|s| s.len())
        .collect::<Vec<_>>();
    let v = vec!["I", "love", "Rust"]
        .into_iter()
        .map(str::len) // len() メソッドは一つの &str をとるので関数ポインタでも型が一致する
        .collect::<Vec<_>>();
}
