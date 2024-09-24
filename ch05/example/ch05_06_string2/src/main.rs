fn main() {
    let i = 42;
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;
    assert_eq!(f.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.2}", f), "4.40");

    let t = (1, "ABC");
    assert_eq!(format!("{:?}", t), r#"(1, "ABC")"#);

    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));

    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r2);

    let cs = ['t', 'r', 'u', 's', 't'];
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust");

    let bad_utf8: [u8; 7] = [
        b'a', // a
        0xf0, 0x90, 0x80, // でたらめなバイト列
        0xe3, 0x81, 0x82, // あ
    ];
    let s = String::from_utf8_lossy(&bad_utf8);
    assert_eq!(s, "a\u{fffd}あ");

    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57]; // UTF16 で ab漢字
    if let Ok(s) = String::from_utf16(&utf16) {
        assert_eq!(s, "ab漢字");
    } else {
        unreachable!();
    }

    // バイト文字列リテラル ASCII 以外の文字のバイトは「\x２桁の16進数」で記述する
    let bs1 = b"abc\xe3\x81\x82"; // UTF-8で "abcあ"
    assert_eq!(bs1, &[b'a', b'b', b'c', 0xe3, 0x81, 0x82]);
    // rawバイト文字列リテラル。エスケープ文字の \ を特別扱いしない。 \n は改行文字でなく \n と解釈される
    let bs2 = br#"ab\ncd"#;
    assert_eq!(bs2, &[b'a', b'b', b'\\', b'n', b'c', b'd']);
    assert_eq!(bs2, b"ab\\ncd");
}
