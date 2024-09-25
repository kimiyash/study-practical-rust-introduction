use std::collections::HashMap;

fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    // get_mut が返す可変の参照が存在している間は map の可変の借用が有効
    match map.get_mut(&key) {
        // value が可変の参照に束縛される
        Some(value) => value.push_str(", world!"),
        None => {
            // このブロックでは get_mut のよる map の可変の借用が終了
            // insert は map の可変の借用をとる
            map.insert(key, Default::default());
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
    println!("{:?}", map);
    process_or_default('i', &mut map);
    println!("{:?}", map);
}
