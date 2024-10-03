use libc::{suseconds_t, time_t};
use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::ptr;

// #[repr(C)] をつけることで C と相互運用できる型になる
// メモリ上の表現が C 五感になるというだけで、それ以外は普通の Rust の構造体として扱える
// struct timeval {
//   time_t tv_sec; /* seconds */
//   suseconds_t tv_usec; /* microseconds */
// };
#[repr(C)]
#[derive(Debug)]
struct TimeVal {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

// struct timezone {
//   int tz_minuteswest; /* minutes west of Greenwich */
//   int tz_dsttime; /* type of DST correction */
// };
#[repr(C)]
#[derive(Debug)]
struct Timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

extern "C" {
    // 上記で定義した型を FFI の方に使える
    // int gettimeofday(struct timeval *tv, struct timezone *tz);
    // Option でくるまなかった場合
    fn gettimeofday(tv: *mut TimeVal, tz: *mut Timezone) -> c_int;
    // Optionでくるんだ場合（うまく動作しない）
    // fn gettimeofday(tv: Option<*mut TimeVal>, tz: Option<*mut Timezone>) -> c_int;
}

fn main() {
    unsafe {
        // C によって初期化するメモリは mem::uninitialized で確保できる
        // もちろん、Rust の構造体の初期化構文でもつかえる
        let mut tv: MaybeUninit<TimeVal> = MaybeUninit::uninit();
        // Option でくるまなかった場合
        // あるいは NULL を渡したい場合は ptr::null_mut も使える
        let tz: *mut Timezone = ptr::null_mut();
        let ret = gettimeofday(tv.as_mut_ptr(), tz);
        // Optionでくるんだ場合（うまく動作しない）
        // let ret = gettimeofday(Some(tv.as_mut_ptr()), None);
        if ret == -1 {
            println!("failure");
            return;
        }
        println!("{:?}", tv.assume_init());
    }
}
