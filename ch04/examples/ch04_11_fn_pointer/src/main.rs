fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 { n } else { -n }
}

fn main() {
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);
    f = abs;
    assert_eq!(f(-42), 42);

    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());
}
