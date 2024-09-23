fn print_info(name: &str, sl: &[char]) {
    println!(
        "{:9} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),
        sl.first(),
        sl[1],
        sl.last()
    );
}

fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    println!("a1: {:?}", a1);
    print_info("&a1[..]", &a1[..]);
    print_info("&a1", &a1);
    print_info("&a1[1..3]", &a1[1..3]);

    let v1 = vec!['e', 'f', 'g', 'h'];
    println!("\nv1: {:?}", v1);
    print_info("&v1[..]", &v1[..]);
    print_info("&v1", &v1);
    print_info("&v1[1..3]", &v1[1..3]);

    let mut a1 = [5, 4, 3, 2];
    let sl = &mut a1[1..3];
    sl[0] = 6;
    sl[1] *= 10;
    sl.swap(0, 1);
    assert_eq!(sl, [30, 6]);
    assert_eq!(a1, [5, 30, 6, 2]);

    let a2: [i32; 0] = [];
    let s2 = &a2;
    assert!(s2.is_empty());
    assert_eq!(s2.len(), 0);
    assert_eq!(s2.first(), None);

    let a3 = ["zero", "one", "two", "three", "four"];
    let s3 = &a3[1..4];
    assert!(!s3.is_empty());
    assert_eq!(s3.len(), 3);
    assert_eq!(s3.first(), Some(&"one"));
    assert_eq!(s3[1], "two");
    assert_eq!(s3.get(1), Some(&"two"));
    // assert_eq!(s3[3], "?"); // panic!
    assert_eq!(s3.get(3), None);
    assert!(s3.contains(&"two"));
    assert!(s3.starts_with(&["one", "two"]));
    assert!(s3.ends_with(&["two", "three"]));

    let mut a4 = [6, 4, 2, 8, 0, 9, 4, 3, 7, 5, 1, 7];
    let _ = &mut a4[2..6].sort();
    assert_eq!(&a4[2..6], &[0, 2, 8, 9]);

    let (s4a, s4b) = &mut a4.split_at_mut(5);
    s4a.reverse();
    assert_eq!(s4a, &[8, 2, 0, 4, 6]);
    s4b.sort_unstable(); // 安定ソートではない、ゆえにソート前の同順な値の順序が保たれない。一般的に sort() より高速。
    assert_eq!(s4b, &[1, 3, 4, 5, 7, 7, 9]);

    // &mut を省略しても型矯正によって自動的にスライスがつくられる
    a4[2..6].sort();
    let (_s4a, _s4b) = a4.split_at_mut(5);
}
