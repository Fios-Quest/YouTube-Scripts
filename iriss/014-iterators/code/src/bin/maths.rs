fn main() {
    // --- Sum ---

    assert_eq!(vec![1, 2, 3, 4].iter().sum::<i32>(), 10);

    // --- Product ---

    assert_eq!(vec![1, 2, 3, 4].iter().product::<i32>(), 24);

    let v: Vec<u8> = vec![1, 2, 3, 4, 5];
    let product: u8 = v.iter().product();

    assert_eq!(product, 0);

    // --- Option ---

    let v: Vec<Option<usize>> = vec![Some(10), Some(20), Some(12)];

    let total: Option<usize> = v.iter().copied().sum();
    assert_eq!(total, Some(42));

    // --- Min / Max ---

    let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

    assert_eq!(v.iter().min(), Some(&'H'));
    assert_eq!(v.iter().max(), Some(&'w'));

    // --- Count ---

    let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

    let iter = v.clone().into_iter();
    assert_eq!(iter.count(), v.len());

    // --- Len ---

    let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

    let mut iter = v.iter();
    assert_eq!(iter.len(), v.len());
    assert_eq!(iter.next(), Some(&'H'));
}
