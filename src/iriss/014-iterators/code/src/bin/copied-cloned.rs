fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Without .copied()
    let iter: Vec<_> = v.iter().collect();
    assert_eq!(iter, vec![&1, &2, &3, &4, &5]);

    // With .copied()
    let copied: Vec<_> = v.iter().copied().collect();
    assert_eq!(copied, vec![1, 2, 3, 4, 5]);

    let v = vec![String::from("Hello"), String::from("World")];

    // Without .cloned()
    let iter: Vec<_> = v.iter().collect();
    assert_eq!(iter, vec![&String::from("Hello"), &String::from("World")]);

    // With .cloned()
    let cloned: Vec<_> = v.iter().cloned().collect();
    assert_eq!(cloned, vec![String::from("Hello"), String::from("World")]);
}
