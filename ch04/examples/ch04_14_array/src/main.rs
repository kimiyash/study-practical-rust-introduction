fn main() {
    let _a1 = [false, true, false];
    let a2 = [0.0, -1.0, 1.0, 0.5];
    assert_eq!(a2.len(), 4);

    let a3 = [0; 100];
    assert_eq!(a3.len(), 100);

    let _a4 = [['a', 'b'], ['c', 'd']]; // 2次元配列

    // let a5 = [false, 'a']; // コンパイルエラー、別の型のものを一緒にできない

    // let size = 100;
    // let a1 = [0; size]; // コンパイルエラー、実行時に配列の長さを指定できない

    // const ならできる
    {
        const SIZE: usize = 100;
        let _a1 = [0; SIZE];
    }

    let size = 100;
    let mut v1 = vec![0; size];
    assert_eq!(v1.len(), 100);

    v1.push(1);
    assert_eq!(v1.len(), 101);
    assert_eq!(v1.pop(), Some(1));
    assert_eq!(v1.len(), 100);
}
