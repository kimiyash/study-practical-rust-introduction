use std::env;
use std::path::PathBuf;

fn main() {
    // oniguruma の共有ライブラリをリンク
    println!("cargo:rustc-link-lib=onigmo");

    // bindgen の設定
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")  // wrapper.h を指定
        .generate()
        .expect("Unable to generate bindings");

    // 生成されたバインディングを OUT_DIR に書き出す
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
