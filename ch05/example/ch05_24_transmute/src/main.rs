fn main() {
    let p1 = Box::new(10);
    // box ポインタを生ポインタ *mut i32 に変換したいが型キャストできない
    // コンパイルエラー、an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
    // let p2 = p1 as *mut i32;

    // Boxポインタと*mutポインタは同じビット幅なので transmute できる
    let _p3: *mut i32 = unsafe { std::mem::transmute(p1) };

    let f1 = 5.6789e+3_f32; // 5678.9
    let i1 = f1 as i32;
    println!("{}", i1);

    let i2: i32 = unsafe { std::mem::transmute(f1) };
    println!("{}", i2);
}
