type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

fn main() {
    let id = 400;
    let now = 456780123;
    let _user = new_user(String::from("mika"), id, now);

    // Id と Timestamp はおなじ i64 なので間違えてもえらーにならない
    let _bad_user = new_user(String::from("kazuki"), now, id);
}

// 型エイリアスは型のネストが深くなったときに使うと便利
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub type SharedMap<K, V> = Rc<RefCell<HashMap<K, V>>>;