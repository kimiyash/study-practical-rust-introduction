fn main() {
    let v1: Vec<u8> = vec![3, 4, 5]; // i32 と推論されるのを u8 として認識させる
    assert_eq!(Some(&3_u8), v1.first());

    let mut s1 = String::from("Type coercion ");
    let s2 = String::from("is actually easy.");

    // 型強制によって s1 が String型から &mut String型へ変換
    // &s2 は&String型から &str型へと変換
    s1.push_str(&s2);

    let mut _i1 = 0u8; // i1 は u8 と推論される
    _i1 = 255; // 255 は u8型になる

    let p1 = &&&&[1, 2, 3, 4]; // &&&&[i32; 4] 型
                               // 型強制が &&&&a1 -> &&&a1 -> &&a1 -> &a1 の順に推移的に作用ｓる
    let _p2: &[i32; 4] = p1;

    let p3 = &&[1, 2, 3, 4];
    let p4: &[i32; 4] = p3;
    let _p5: &[i32] = p4;
    // let p6: &[i32] = p3; // expected `&[i32]`, found `&&[i32; 4]`
}
