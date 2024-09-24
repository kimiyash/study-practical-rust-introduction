use std::collections::HashMap;

fn main() {
    let mut m1 = HashMap::new();
    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);
    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);
    // d が存在するならその参照をえる、存在しないなら d に対して 0 を登録してから参照を返す
    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));

    // イテレータとcollectを使って HasMap を生成する
    let m2 = vec![("a", 1), ("b", 3)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    println!("{:?}", m2);
}
