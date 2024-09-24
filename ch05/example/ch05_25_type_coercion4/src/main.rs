fn f1(p: &[i32]) -> i32 {
    p[0]
}

fn f2(p: Box<[i32]>) -> i32 {
    p[0]
}

fn main() {
    let a1 = [1, 2, 3, 4];
    assert_eq!(1, f1(&a1)); // &[i32; 4] -> &[i32]
    assert_eq!(1, f2(Box::new(a1))); // Box<[i32: 4]> -> Box<[i32]>

    // dの型をDebugトレイトのトレイトオブジェクトに指定
    let mut d: Box<dyn std::fmt::Debug>;

    // Debugトレイトを実装する方はトレイトオブジェクトへ型強制できる
    d = Box::new([1, 2]); // Box<[i32; 2]> -> Box<Debug>
    d = Box::new(Some(1)); // Box<Some<i32>> -> Box<Debug>

    // first() のレシーバは Vec<u8> だがその型には first は実装されてない
    // レシーバが暗黙的に &[u8] と型強制されてスライスに定義された first() が呼び出される
    let v1: Vec<u8> = vec![3, 4, 5];
    assert_eq!(v1.first(), Some(&3_u8));
}
