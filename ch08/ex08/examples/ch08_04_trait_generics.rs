trait Init<T> {
    fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

fn main() {
    let data = Box::init("foo");
    let data = Box::<f32>::init(0.1);
    let data: Box<f32> = Init::init(0.1);
    let data: Box<_> = Init::<f32>::init(0.1);
}
