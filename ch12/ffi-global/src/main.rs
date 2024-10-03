use std::os::raw::c_int;

#[link(name = "readline")]
extern "C" {
    // rust の static と同じく static 名前: 型; で宣言する
    static rl_readline_version: c_int;
}

fn main() {
    unsafe {
        // readline のバージョンは 16 進数なので :x で 16 進表示する
        println!("using readline version {:x}", rl_readline_version);
    }
}
