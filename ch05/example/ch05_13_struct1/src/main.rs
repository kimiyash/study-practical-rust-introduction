struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon {
        // 変数名がフィールド名と合致してるときにフィールド名を省略可
        vertexes,
        stroke_width,
        fill,
    }
}

fn main() {
    let triangle = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };

    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (255, 255, 255));

    let quardrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);
    // パターンマッチでアクセス、不要なフィールドは..で省略可能
    let Polygon {
        vertexes: quad_vx, ..
    } = quardrangle;
    assert_eq!(quad_vx.len(), 4);

    // :以降を省略すると、フィールド名と同じ名前の変数が作られフィールド値に束縛される
    let Polygon { fill, .. } = quardrangle;
    assert_eq!(fill, (0, 0, 0));
    let Polygon { stroke_width, .. } = quardrangle;
    assert_eq!(stroke_width, 1);

    // 構造体の値を変更するには mut が必要
    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexes.len(), 2);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    // すでにある値を下にして、その一部を使った新しい値をつくる
    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 5,
    };
    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (-2, 2)],
        .. triangle1
    };
    assert_ne!(triangle1.vertexes, triangle2.vertexes);
    assert_eq!(triangle1.fill, triangle2.fill);
    assert_eq!(triangle1.stroke_width, triangle2.stroke_width);
}
