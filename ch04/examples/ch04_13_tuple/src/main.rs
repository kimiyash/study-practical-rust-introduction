fn main() {
    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    assert!(t1.1);

    // let i = 0;
    // let t1a = t1.i; // コンパイルエラー、unknown field

    let mut t1 = (88, true);
    t1.0 += 100;
    assert_eq!(t1, (188, true));

    let (n1, b1) = (88, true);
    assert_eq!(n1, 88);
    assert!(b1);

    let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
    assert_eq!(x1, 0);
    assert_eq!(y1, 5);
    assert_eq!(x2, 10);
    assert_eq!(y2, -1);

    let ((x1, y1), _) = ((0, 5), (10, -1));
    assert_eq!(x1, 0);
    assert_eq!(y1, 5);

    let mut t1 = ((0, 5), (10, -1));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;
    *x1_ptr += 3;
    *y1_ptr *= -1;
    assert_eq!(t1, ((3, -5), (10, -1)));
}
