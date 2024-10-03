use std::os::raw::c_double;

extern "C" {
    fn cos(x: c_double) -> c_double;
}

fn main() {
    unsafe {
        println!("{}", cos(1.5));
    }
}
