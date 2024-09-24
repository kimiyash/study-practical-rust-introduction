fn main() {
    let mut v1 = vec![0, 1, 2, 3];
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
    // 過企業ではBox<i32>に変換する前に余分なメモリを持たなくする v1.shrink_to_fit() が暗黙的に呼ばれる
    let s1 = v1.into_boxed_slice();
    let mut v1 = s1.into_vec();
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
    v1.reserve(10);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
    v1.reserve_exact(20);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
}
