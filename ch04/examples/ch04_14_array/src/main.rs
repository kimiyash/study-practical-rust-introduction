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

    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    // インデックスは定数でなくても良い
    let mut index = 0;
    assert_eq!(array2[index], 0);
    index += 1;
    assert_eq!(array2[index], 10);

    let array3 = [0, 1];
    // array3[2]; // コンパイルエラー、index out of bounds: the length is 2 but the index is 2

    // 実行時にエラーになるケース
    // let index = 2;
    // array3[index];
    // error: this operation will panic at runtime
    //    --> src/main.rs:49:5
    //    |
    // 49 |     array3[index];
    //    |     ^^^^^^^^^^^^^ index out of bounds: the length is 2 but the index is 2

    assert_eq!(array3.get(1), Some(&1));
    assert_eq!(array3.get(2), None);

    let array4 = ['a'; 50];
    for ch in array4.iter() {
        print!("{}", *ch);
    }

    let mut array5 = [1; 50];
    for n in array5.iter_mut() {
        *n *= 2;
    }
}
