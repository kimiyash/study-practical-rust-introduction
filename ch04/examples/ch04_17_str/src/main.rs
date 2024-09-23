fn main() {
    let s1 = "abc1";
    let s2 = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);

    let s3 = "文字列を副業にわたって書くと
        改行はスペースが入る";
    println!("{}", s3);
    
    let s4 = "行末にバックスラッシュをつけると\
                    改行などが入らない";
    println!("{}", s4);

    let s5 = "文字列に\"と\\を含める";
    println!("{}", s5);

    let s6 = r#"文字列に"と\を含める"#; // raw 文字列リテラル
    println!("{}", s6);

    let s7 = r###"このように#の数を増やすと"##"があっても大丈夫"###;
    println!("{}", s7);

    let s8 = "もちろん絵文字\u{1f600}も使える";
    println!("{}", s8);

    let fruits = "あかりんご、　 あおりんご\nラズベリー、　 ブラックベリー";
    let mut lines = fruits.lines();
    let apple_line = lines.next();
    assert_eq!(apple_line, Some("あかりんご、　 あおりんご"));
    assert_eq!(lines.next(), Some("ラズベリー、　 ブラックベリー"));
    assert_eq!(lines.next(), None);

    if let Some(apples) = apple_line {
        assert!(apples.starts_with("あか"));
        assert!(apples.contains("りんご"));
        assert_eq!(apples.find("あお"), Some(22)); // 0始まりなので18バイト目

        let mut apple_iter = apples.split("、");
        assert_eq!(apple_iter.next(), Some("あかりんご"));
        let green = apple_iter.next();
        assert_eq!(green, Some("　 あおりんご"));
        assert_eq!(green.map(str::trim), Some("あおりんご"));
        assert_eq!(apple_iter.next(), None);
    } else {
        unreachable!();
    }

    let s1 = "a";
    let s2 = "あ";
    let s3 = "😀";
    let s4 = "🇯🇵";

    // len() メソッドはUTF-8のバイト数を返す
    assert_eq!(s1.len(), 1);
    assert_eq!(s2.len(), 3);
    assert_eq!(s3.len(), 4);
    assert_eq!(s4.len(), 8);

    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..4), None); // UTF-8 として解釈できない場合

    let s = "かか\u{3099}く"; // \u{3099} は濁点文字
    println!("{}", s);

    let mut iter = s.chars();
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('か'));
    assert_eq!(iter.next(), Some('\u{3099}'));
    assert_eq!(iter.next(), Some('く'));
    assert_eq!(iter.next(), None);

}
