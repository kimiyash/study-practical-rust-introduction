use std::str::FromStr;

trait Server {
    // type 型名で関連型を宣言できる
    type Response;
    // あるいはtype型名: トレイト境界で境界を設定することもできる
    type Request: FromStr;

    // 関連型にはSelf::型名でアクセスする
    fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;
impl Server for EchoServer {
    // トップレベルと同じようにtype 型名 = 型名で定義できる
    type Response = String;
    // トレイト境界のついた型も同じように定義できる
    // トレイト境界を満たさない型を書くとコンパイルエラーになる
    type Request = String;

    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
}

// // S::Response のように Serverの関連型を参照できる
// // 関連型については特別指定しなければ任意の関連型を受け付ける
// fn handle<S: Server>(server: S, req: &str) -> S::Response {
//     // 関連型にトレイト境界がついているのでトレイト関数を呼び出すこともできる
//     let req = S::Request::from_str(&req).unwrap();
//     server.handle(req)
// }

// あるいは、関連型が特定の型を持っていることを指定したければ
// トレイト名<関連型名 = 型> のように指定できる
// この場合RequestにStringを持つServerの実装しか受け付けない
fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
    server.handle(req.to_string())
}

fn main() {
    let server = EchoServer;
    assert_eq!(handle(server, "Hello"), "Hello");
}

// 定義じにパラメータか名前をつけるのかの違いがある
// trait Foo<T> {}
// trait Bar {type T;}

// 参照するときはTかSelf::Tかが違う
trait Foo<T> {
    fn new(t: T) -> Self;
}

trait Bar {
    type T;
    fn new(t: Self::T) -> Self;
}

// トレイト境界に書くときにジェネリクスは引数の型が必要だが、関連型は推論できるなら省略できる
fn some_fun_foo<S, T: Foo<S>>(t: T) {}
fn some_fun_bar<T: Bar>(t: T) {}

// トレイト境界で特定の型を指定するときも指定方法が異なる
fn some_fun_foo_2<T: Foo<u32>>(t: T) {}
fn some_fun_bar_2<T: Bar<T = u32>>(t: T) {}

// データ型への実装をジェネリクスにするときは、ジェネリクスのほうが簡単に書けるが、関連型は成約がある
struct Buz;
impl<T> Foo<T> for Buz {
    fn new(t: T) -> Self {
        Buz
    }
}
// // 以下は書けない、implの部分でTが使われてないため
// // エラーの原因は、トレイト Bar を Buz に対してジェネリックに実装している際に、Rust の型システムが期待する T 型に関する情報が不足しているためです。
// impl<T> Bar for Buz {
//     type T = T; // ここで T に関する情報が不足している
//     fn new(t: Self::T) -> Self {
//         Buz
//     }
// }
// Qux>T> のように実装するデータ型のほうがジェネリクスなら可能
struct Qux<T> {
    hoge: T,
}
impl<T> Bar for Qux<T> {
    type T = T;
    fn new(t: Self::T) -> Self {
        Qux { hoge: t }
    }
}

// 同じ型へのパラメータを変えた複数の実装はジェネリクスじゃないとかけない
// ジェネリクスは一対多
impl Foo<i32> for Buz {}
impl Foo<char> for Buz {}
// 関連型は一対一
impl Bar for Baz { type T = i32; }
// 関連型だと2つ目以降はエラー
// impl Bar for Baz { type T = char; }

// 一方関連型でないとできないこともある
// 関連型は外部からも参照できる
fn get_from_foo<D: Foo<VeryLong<Type, Name>>>(d: D) -> (VeryLong<Type, Name>, VeryLong<Type, Name>) {..})
fn get_from_bar<D: Bar<T = VeryLong<Type, Name>>>(d: D) -> (D::T, D::T) {..}