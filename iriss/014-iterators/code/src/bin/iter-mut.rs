fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for n in v.iter_mut() {
        *n += 10
    }

    assert_eq!(v, vec![11, 12, 13, 14, 15]);
}
