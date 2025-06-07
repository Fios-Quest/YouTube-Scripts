fn main() {
    // ---- Range ----

    for n in 0..5 {
        println!("Number: {n}");
    }

    assert_eq!((0..5).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);

    // ---- Repeat ----

    use std::iter::repeat;

    let mut banana_phone = repeat("Ring");

    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));
    assert_eq!(banana_phone.next(), Some("Ring"));

    // ---- Cycle ----

    let mut iter = vec![0, 1, 2].into_iter().cycle();

    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(0)); // <- Repeat
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(0)); // <- Repeat
    // ...and so on

    // ---- Cycle Not Clonable ----

    #[derive(Debug, PartialEq)]
    struct NotCloneable;

    // Value will own our data
    let value = NotCloneable;

    // The vec we base the iterator on will contain a reference
    let mut iter = vec![&value].into_iter().cycle();

    assert_eq!(iter.next(), Some(&NotCloneable));
    assert_eq!(iter.next(), Some(&NotCloneable));
    assert_eq!(iter.next(), Some(&NotCloneable));

    // ---- Chain ----

    let i1 = vec![0, 1, 2].into_iter();
    let i2 = vec![3, 4, 5].into_iter();

    assert_eq!(i1.chain(i2).collect::<Vec<_>>(), vec![0, 1, 2, 3, 4, 5]);

    // --- str ---

    let script = r#"Many other Types in Rust can also be broken
        down into Iterators.

        The script for this video, for example, can be represented as one large
        `str`, which you can break the data down by `.lines()`, `.chars()` or
         `.bytes()` all of which produce iterators.

        ## Using Iterators - Mathematics"#;

    let script_lines = script.lines();
    let script_chars = script.chars();
    let script_bytes = script.bytes();
}
