struct UserName(String);
struct Id(u64);
struct Timestamp(u64);
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

fn main() {
    let id = Id(100);
    let now = Timestamp(4567890123);
    let bad_user = new_user(String::from("kazuki"), now, id);
    // コンパイエラーになる
    // 13 |     let bad_user = new_user(String::from("kazuki"), now, id);
    //    |                    ^^^^^^^^ ----------------------  ---  -- expected `Timestamp`, found `Id`
    //    |                             |                       |
    //    |                             |                       expected `Id`, found `Timestamp`
    //    |                             expected `UserName`, found `String`
}
