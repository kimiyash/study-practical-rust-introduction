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

}
