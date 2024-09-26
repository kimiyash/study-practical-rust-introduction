trait Overload<T> {
    fn call(&self, t: T) -> &'static str;
}

impl Overload<i32> for i32 {
    fn call(&self, _: i32) -> &'static str {
        "(i32, i32)"
    }
}

impl Overload <char> for i32 {
    fn call(&self, _: char) -> &'static str {
        "(i32, char)"
    }
}

fn main() {
    assert_eq!(1_i32.call(2_i32), "(i32, i32)");
    assert_eq!(1_i32.call('c'), "(i32, char)");
}