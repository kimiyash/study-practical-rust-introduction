use std::os::raw::{c_int, c_void};

// ownership.c で定義した C の関数をインポートする
#[link(name = "ownership", kind = "static")]
extern "C" {
    fn take_ownership(i: *const c_int, dtor: unsafe extern "C" fn(i: *mut c_int)) -> c_void;
}

// デストラクタ関数。C に渡した所有権を Rust に返してもらうためのもの
unsafe extern "C" fn drop_pointer(i: *mut c_int) {
    // ポインタから Box に復元することで所有権を取り戻す
    Box::from_raw(i);
    // ここで Box のライフタイムが尽きるので、メモリが解放される
}

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn make_memory() -> *mut c_int;
}

fn main() {
    let i = Box::new(1);
    // C 側に所有権を渡すので into_raw を使う
    unsafe { take_ownership(Box::into_raw(i), drop_pointer) };

    unsafe {
        let i = make_memory();

        println!("got {}", *i);

        // C から渡せたメモリは手で解放する必要がある
        libc::free(i as *mut _);
    }
}
