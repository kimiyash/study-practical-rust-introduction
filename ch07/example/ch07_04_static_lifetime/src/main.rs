static I0: i32 = 42;

fn take_static<T: 'static>(_x: T) {}

fn main() {
    let mut s0: &'static str;
    let s1 = "42";
    let s2 = 42.to_string();
    s0 = s1;
    //s0 = &s2; // コンパイルエラー、borrowed value does not live long enough

    take_static(s1);
    //take_static(&s2); // コンパイルエラー、borrowed value does not live long enough
    take_static(s2);
}
