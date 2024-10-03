fn main() {
    let x = 1;
    // 参照から const なポインタが作れる
    let xptr: *const i32 = &x;
    // 逆の変換はできない
    // let xref: &i32 = xptr;
    // ポインタへの操作は基本的にあんセーフ
    unsafe {
        // ポインタの参照外しはアンセーフ
        let x = *xptr;
    }

    let mut y = 2;
    // ミュータブルな参照からミュータブルなポインタが作れる
    let yptr: *mut i32 = &mut y;
    unsafe {
        // 書き込みももちろんアンセーフ
        *yptr = 3;
    }

    let z = Box::new(4);
    // Boxを参照としてポインタも作れる
    let zptr: *const i32  = &*z;

    let s: &[u8] = b"abc";
    // スライス(文字列)からポインタが作れる
    let sptr: *const u8 = s.as_ptr();
    unsafe {
        // ポインタからスライス(文字列)も作れるが、こちらはアンセーフ
        let s = std::slice::from_raw_parts(sptr, s.len());
    }
}
