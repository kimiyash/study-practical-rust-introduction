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
}
