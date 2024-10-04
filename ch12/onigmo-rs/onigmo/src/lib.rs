use onigmo_sys::*;
use std::error;
use std::fmt;
use std::mem::MaybeUninit;
use std::ops::Drop;
use std::ops::Range;
use std::ptr::NonNull;

// 本来は OnigPostion のままでなく enum を定義して変換したほうがいいが
// 長くなるのでここでは省略する
#[derive(Debug, Clone)]
pub struct Error(OnigPosition, Option<MaybeUninit<OnigErrorInfo>>, String);
type Result<T> = ::std::result::Result<T, Error>;

impl Error {
    // 中身はほぼ onigmo-sys での記述のままだが、
    // 最後に String を作っているので Rust から扱いやすくなっている。
    // また、ここで unsafe を一つ閉じ込めている
    fn new(pos: OnigPosition, error_info: Option<MaybeUninit<OnigErrorInfo>>) -> Self {
        use std::str::from_utf8;
        let s: &mut [OnigUChar] = &mut [0; ONIG_MAX_ERROR_MESSAGE_LEN as usize];
        unsafe {
            let size = match error_info {
                Some(ei) => onig_error_code_to_str(s as *mut _ as *mut _, pos, ei),
                None => onig_error_code_to_str(s as *mut _ as *mut _, pos),
            };
            let size = size as usize;
            let s = from_utf8(&s[0..size]).unwrap().to_string();
            Error(pos, error_info, s)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ERROR: {}", self.2)
    }
}

impl error::Error for Error {}

// Regx は動作の主体となり、主に &mut self の形で使われるのでここではポインタは使わない
pub struct Regex(MaybeUninit<regex_t>);
impl Regex {
    pub fn new(pattern: &str) -> Result<Self> {
        // 長く見えるが、実体は onig_new_without_allocを呼んでいるだけ
        unsafe {
            let mut req: MaybeUninit<regex_t> = MaybeUninit::uninit();
            let pattern = pattern.as_bytes();
            let mut einfo: MaybeUninit<OnigErrorInfo> = MaybeUninit::uninit();
            let r = onig_new_without_alloc(
                req.as_mut_ptr(),
                pattern.as_ptr() as *const OnigUChar,
                (pattern.as_ptr() as *const OnigUChar).add(pattern.len()),
                ONIG_OPTION_NONE,
                &OnigEncodingUTF_8,
                OnigDefaultSyntax,
                einfo.as_mut_ptr(),
            );
            if (r as ::std::os::raw::c_uint) == ONIG_NORMAL {
                Ok(Regex(req))
            } else {
                // 先ほど定義したエラーもしっかり使う
                Err(Error::new(r as OnigPosition, Some(einfo)))
            }
        }
    }

    // 検索対象はリードオンリーなので &str で受け取る
    pub fn search(&mut self, s: &str) -> Option<Region> {
        unsafe {
            let s = s.as_bytes();
            let start = s.as_ptr();
            let end = start.add(s.len());
            let range = end;
            let mut region = Region::new()?;

            let pos = onig_search(
                self.0.as_mut_ptr(),
                start,
                end,
                start,
                range,
                region.as_ptr_mut(),
                ONIG_OPTION_NONE,
            );
            if 0 <= pos {
                Some(region)
            } else {
                // Onigmo のソースコードにならい、
                // デバッグビルドのときは戻り値が ONIG_MISMATCH でなければ
                // パニックするようにする
                debug_assert!(pos as std::os::raw::c_int == ONIG_MISMATCH);
                None
            }
        }
    }
}

impl Drop for Regex {
    /// デストラクタ
    fn drop(&mut self) {
        unsafe { onig_free_body(self.0.as_mut_ptr()) }
    }
}

// Region は Onigmo の API がそうなっているのと、
// 主に戻り値でそのまま返されるので内部の値はポインタで指す
#[derive(Debug)]
pub struct Region(NonNull<OnigRegion>);

impl Region {
    pub fn new() -> Option<Self> {
        // コンストラクタは onig_region_new を呼ぶ
        unsafe {
            let region: *mut OnigRegion = onig_region_new();
            Some(Region(NonNull::new(region)?))
        }
    }

    fn as_ptr_mut(&mut self) -> *mut OnigRegion {
        self.0.as_ptr()
    }

    // fn as_ptr(&self) -> *const OnigRegion {
    //     self.0.as_ptr()
    // }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe { onig_region_free(self.0.as_ptr(), 1) }
    }
}

impl Clone for Region {
    fn clone(&self) -> Self {
        // onig_region_copy で Clone を実装
        unsafe {
            let mut to: MaybeUninit<OnigRegion> = MaybeUninit::uninit();
            onig_region_copy(to.as_mut_ptr(), self.0.as_ptr());
            Region(NonNull::new_unchecked(to.as_mut_ptr()))
        }
    }
}

// データを持っている Region とそのインデックスのイテレータ
/// Region から取り出すイテレータの型
#[derive(Debug, Clone)]
pub struct PositionIter<'a>(&'a Region, Range<i32>);

impl Region {
    // Region の imple にこの関数を追記する
    /// 位置情報のイテレータを取り出す
    pub fn position(&self) -> PositionIter {
        let num_regs;
        // リージョン数の情報からインデックスのイテレータを作る
        unsafe {
            num_regs = (*self.0.as_ptr()).num_regs;
        }
        PositionIter(self, 0..num_regs)
    }
}

impl<'a> Iterator for PositionIter<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let region = *(self.0).0.as_ptr();
            // self.1 がイテレータになっているのでそえｒを使ってイテレータを実装する
            self.1.next().map(|i| {
                (
                    *region.beg.offset(i as isize) as usize,
                    *region.end.offset(i as isize) as usize,
                )
            })
        }
    }
}
