use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("hoge".to_string());
    v.push("piyo".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"piyo".to_string()));
    println!("{:?}", v);
    let _ = v.pop();
    println!("{:?}", v);
    v.push("uhi".to_string());
    v.push("piyo".to_string());
    println!("{:?}", v);
    v.push("uhi".to_string());
    v.push("piyo".to_string());
    println!("{:?}", v);
    println!("{}", v.get_or(0, &"default".to_string()));
    println!("{}", v.get_or(100, &"default".to_string()));

    let mut iter = v.iter();
    // v.push("test".to_string()); // これはコンパイルエラー push は可変の参照を得ようとするが、iter が生存してるので不変の参照が有効
    // 21 |     let mut iter = v.iter();
    //    |                    - immutable borrow occurs here
    // 22 |     v.push("test".to_string());
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
    // 23 |
    // 24 |     assert_eq!(iter.next(), Some(&"hoge".to_string()));
    //    |                ---- immutable borrow later used here
    assert_eq!(iter.next(), Some(&"hoge".to_string()));

    v.push("test".to_string());

    println!();
    for msg in &v {
        println!("{:?}", msg);
    }
}
