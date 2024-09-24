#[derive(Debug, PartialEq)]
struct UniqueValue; // フィールドがない構造体。トレイトを実装したいとき役立つらしい

fn main() {
    let uv1 = UniqueValue;
    let uv2 = UniqueValue;
    assert_eq!(uv1, uv2);
}
