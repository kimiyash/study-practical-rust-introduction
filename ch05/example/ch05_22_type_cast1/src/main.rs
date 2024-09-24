fn main() {
    let i1 = 42;
    let _f1 = i1 as f64;

    let c1 = 'a';
    assert_eq!(97, c1 as u32);

    let i2 = 300;
    let u1 = i2 as u8; // オーバーフローのチェックはしない
    assert_eq!(u1, 44);

    let t1 = ('a', 42);
    // コンパイルエラー an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    // let t2 = t1 as (u32, u8);
    let _t3 = (t1.0 as u32, t1.1 as u8);

    let v1 = vec![b'h', b'e', b'l', b'l', b'o'];
    // コンパイルエラー an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    // let v2 = v1 as Vec<u16>;
    let _v3 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();

    // &str は Vec<u8> へのFromトレイトを実装してる
    let v4: Vec<u8> = From::from("hello");
    assert_eq!(v4, v1);
}
