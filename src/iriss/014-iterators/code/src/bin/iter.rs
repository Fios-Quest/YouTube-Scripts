fn main() {
    let hello = String::from("Hello");
    let world = String::from("World");

    let v = vec![hello, world];

    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&String::from("Hello")));
    assert_eq!(iter.next(), Some(&String::from("World")));
    assert_eq!(iter.next(), None);

    assert_eq!(v, vec![String::from("Hello"), String::from("World")]);

    // ---

    let hello = String::from("Hello");
    let world = String::from("World");

    let v = vec![&hello, &world];

    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&&hello));
    assert_eq!(iter.next(), Some(&&world));
    assert_eq!(iter.next(), None);

    assert_eq!(v, vec![&hello, &world]);
}
