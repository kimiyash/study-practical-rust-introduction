fn main() {
    let boxed = Box::new(true);
    // ここで boxed の所有権はムーブしてしまう
    let ptr: *mut bool = Box::into_raw(boxed);
    unsafe {
        // ポイント先のメモリを開放するには Box::from_raw で Box に戻して上げる
        // ここでポインタのデータ型の所有権を boxed が持つことになる
        // 他に参照がないかはユーザーが保証する必要がある
        let boxed = Box::from_raw(ptr);
        // 気をつけないと例えば下記のように２つ目の Box も作れてしまう
        // これは Rust の仮定を破ってしまう
        // let boxed2 = Box::from_raw(ptr);
    }
}
