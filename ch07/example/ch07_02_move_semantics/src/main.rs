#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Droppping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn f1(p: Parent) {
    println!("p1: {:?}", p);
}

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(p1);
    // println!("p1: {:?}", p1); // エラー
    //  25 |     f1(p1);
    //     |        -- value moved here
    //  26 |     println!("p1: {:?}", p1); // エラー
    //     |                          ^^ value borrowed here after move
}
