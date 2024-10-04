use onigmo_sys::*;
use std::mem::MaybeUninit;
use std::str::from_utf8_unchecked;

fn main() {
    unsafe {
        // 正規表現のパターン文字
        let pattern = b"a(.*)b|[e-f]+";
        // マッチ対象
        let s = b"zzzzaffffffffb";
        // onig_new_without_alloc で初期化するメモリをスタックに確保する
        let mut req: MaybeUninit<regex_t> = MaybeUninit::uninit();
        let mut einfo: MaybeUninit<OnigErrorInfo> = MaybeUninit::uninit();
        // 正規表現をコンパイルし、req に格納する
        let r = onig_new_without_alloc(
            req.as_mut_ptr(),
            // パターン文字の戦闘
            pattern as *const OnigUChar,
            // パターン文字の末尾
            (pattern as *const OnigUChar).add(pattern.len()),
            // 今回オプションはつけない
            ONIG_OPTION_NONE,
            // Rust の文字列は UTF-8 エンコーディング
            &OnigEncodingUTF_8,
            // Onigmoのデフォルトの構文を使う
            OnigDefaultSyntax,
            einfo.as_mut_ptr(),
        );
        // コンパイル結果の戻りが正常でなければエラー
        if (r as ::std::os::raw::c_uint) != ONIG_NORMAL {
            // エラー情報を取得し出力する
            let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
            onig_error_code_to_str(s as *mut _ as *mut _, r as OnigPosition, einfo.as_ptr());
            println!("ERROR: {}\n", from_utf8_unchecked(s));
            // 正規表現のエラーならそのまま終了
            return;
        }

        // マッチ情報を表すデータを準備する
        let region = onig_region_new();

        // マッチ対象文字列の終端
        let end = (s as *const OnigUChar).add(s.len());
        // マッチ開始位置
        let start = s as *const _;
        // マッチ終了位置
        let range = end;
        // 正規表現でマッチする
        let mut r = onig_search(
            req.as_mut_ptr(),
            s as *const _,
            end,
            start,
            range,
            region,
            ONIG_OPTION_NONE,
        );
        if 0 <= r {
            // 戻り値が 0以上ならマッチ成功
            println!("match as {}", r);
            let region = region.as_ref().unwrap();
            // グルーピングされた部分正規表現ごとにマッチ位置を表示する

            for i in 0..(region.num_regs) {
                println!(
                    "{}: ({:?}-{:?})",
                    i,
                    region.beg.offset(i as isize),
                    region.end.offset(i as isize)
                );
            }
            r = 0;
        } else if (r as ::std::os::raw::c_int) == ONIG_MISMATCH {
            // 戻り値が ONIG_MISMATCH なら正規表現とマッチ失敗
            println!("search fail");
            r = -1;
        } else {
            // それ以外では Onigmo の内部エラー
            let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
            onig_error_code_to_str(s as *mut _ as *mut _, r as OnigPosition, einfo.as_ptr());
            println!("ERROR: {}\n", from_utf8_unchecked(s));
            std::process::exit(-1);
        }
        // 使ったリソースを手動で解放する
        onig_region_free(region, 1);
        onig_free_body(req.as_mut_ptr());
        onig_end();
        std::process::exit(r as i32);

    }
}