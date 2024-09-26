trait Overload {
    fn call(&self) -> &'static str;
}

impl Overload for i32 {
    fn call(&self) -> &'static str {
        "i32"
    }
}

impl Overload for str {
    fn call(&self) -> &'static str {
        "str"
    }
}

fn main() {
    assert_eq!(1_i32.call(), "i32");
    assert_eq!("str".call(), "str");

    assert_eq!(Overload::call(&1_i32), "i32");
    assert_eq!(Overload::call("str"), "str");
}