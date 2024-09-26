use std::cell::RefCell;

struct A {
    c: char,
    s: String,
}

struct B {
    c: char,
    s: RefCell<String>,
}

fn main() {
    let a = A { c: 'a', s: "alex".to_string()};
    let r = &a;
    // r.s.push('a'); // コンパルエラー

    let b = B { c: 'a', s: RefCell::new("alex".to_string())};
    let rb = &b;
    b.s.borrow_mut().push('a');
    {
        let rbs = b.s.borrow();
        assert_eq!(&*rbs, "alexa");
        //b.s.borrow_mut(); // この時点で不変の参照 rbs が有効なのでパニックになる already borrowed: BorrowMutError

        // try_borrow_mut ならパニックせずにエラーになる
        assert!(b.s.try_borrow_mut().is_err());
    }
    // rbs のスコープを抜けてるので利用可能
    assert!(b.s.try_borrow_mut().is_ok());

}
