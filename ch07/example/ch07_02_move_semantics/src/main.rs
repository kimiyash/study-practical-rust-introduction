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

fn main() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("p2: {:?}", p2);
    //  20 |     let mut p1 = Parent(1, Child(11), Child(12));
    //     |         ------ move occurs because `p1` has type `Parent`, which does not implement the `Copy` trait
    //  21 |     let p2 = p1;
    //     |              -- value moved here
    //  22 |     println!("p2: {:?}", p2);
    //  23 |     println!("p1: {:?}", p1); 
    //     |                          ^^ value borrowed here after move
    // println!("p1: {:?}", p1); // 上記のエラーがでる
}
