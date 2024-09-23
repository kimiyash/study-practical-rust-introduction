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
}
