use std::os::raw::{c_char, c_int};

// オペーク型を表す型を導入する
// バリアントのない列挙型は値が作れないのでユーザーが勝手にインスタンスを作ることはできない
// この列挙型へのポインタでオペーク型へのポインタを表す
enum File {}

extern "C" {
    // C の `FILE`型の実態がわからないので Rust 側では実体に言及しない型でマッピングする

    // FILE *fopen(const char *path, const char *mode);
    fn fopen(fname: *const c_char, mode: *const c_char) -> *mut File;

    // int fgetc(FILE *stream);
    fn fgetc(stream: *mut File) -> c_int;

    // int fclose(FILE *stream);
    fn fclose(stream: *mut File) -> c_int;
}

fn main() {
    unsafe {
        // C の文字列を作る。ここでは NULL 終端したバイト列を作ってキャストしている
        let fname: *const c_char = b"Cargo.toml\0".as_ptr() as *const _;
        let mode: *const c_char = b"r\0".as_ptr() as *const _;
        // FiLE は Rust では本来実体の無い型なので C 関数を通してのみ初期化できる
        let file = fopen(fname, mode);
        if file.is_null() {
            println!("open file failed");
            return;
        }
        loop {
            // Rust にとってはよくわからない値のまま C の関数にわたす
            let c = fgetc(file);
            if c == -1 {
                break;
            } else {
                let c = c as u8 as char;
                print!("{}", c);
            }
        }
        // 同じく実体のよくわからないまま C の関数で終了処理をする
        if fclose(file) == -1 {
            println!("close file failed");
        }
    }
}
