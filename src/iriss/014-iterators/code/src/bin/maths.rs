fn main() {
    // --- Sum ---

    assert_eq!(vec![1, 2, 3, 4].iter().sum::<i32>(), 10);

    // --- Product ---

    assert_eq!(vec![1, 2, 3, 4].iter().product::<i32>(), 24);

    let v: Vec<u8> = vec![1, 2, 3, 4, 5];
    let product: u8 = v.iter().product();

    assert_eq!(product, 120);

    // --- Option ---

    let v: Vec<Option<i32>> = vec![Some(10), Some(20), Some(12)];

    let total: Option<i32> = v.iter().copied().sum();
    assert_eq!(total, Some(42));

    // --- Min / Max ---

    let hello = "Helloworld";

    assert_eq!(hello.chars().min(), Some('H'));
    assert_eq!(hello.chars().max(), Some('w'));

    // --- Count ---

    let v = vec!["Hello".to_string(), "world".to_string()];

    let mut i = v.iter();

    assert_eq!(i.count(), 2); // `i` can't be used after this
    assert_eq!(i.next(), Some(&"Hello".to_string()));

    let mut i = v.iter();
    assert_eq!(i.len(), 2); // `i` is not consumed
    assert_eq!(i.next(), Some(&"Hello".to_string()));

    // --- Len ---

    let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

    let mut iter = v.iter();
    assert_eq!(iter.len(), v.len());
    assert_eq!(iter.next(), Some(&'H'));
}
