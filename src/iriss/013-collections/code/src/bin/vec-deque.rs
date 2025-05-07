fn main() {
    use std::collections::VecDeque;

    let mut v = VecDeque::from([0, 1, 2, 3, 4, 5]);

    v.push_back(6);
    v.push_front(-1);

    assert_eq!(v, [-1, 0, 1, 2, 3, 4, 5, 6]);

    assert_eq!(v.pop_back(), Some(6));
    assert_eq!(v.pop_front(), Some(-1));
    assert_eq!(v.pop_front(), Some(0));

    assert_eq!(v, [1, 2, 3, 4, 5]);
}
