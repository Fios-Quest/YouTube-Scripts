
fn main() {
    
    // --- Sum ---
    
    assert_eq!(
        vec![1, 2, 3, 4].iter().sum::<i32>(),
        10
    );
    
    // --- Product ---

    assert_eq!(
        vec![1, 2, 3, 4].iter().product::<i32>(),
        24
    );

    // --- Option ---
    
    let v: Vec<Option<usize> > = vec![
        Some(10),
        Some(20),
        Some(12),
    ];

    // Note: the Option needs to be owned, references won't work, so we'll use .into_iter()
    let total: Option<usize> = v.into_iter().sum();
    assert_eq!(total, Some(42));
    
    // --- Min / Max ---

    let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

    assert_eq!(v.iter().min(), Some(&'H'));
    assert_eq!(v.iter().max(), Some(&'w'));

    // --- Count ---
    
    let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

    let iter = v.clone().into_iter();
    assert_eq!(iter.count(), v.len()); // iter has same number of items as v is long
    // iter no longer exists
    
    // --- Len ---

    let v = vec!['H', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];

    let mut iter = v.iter();
    assert_eq!(iter.len(), v.len());
    assert_eq!(iter.next(), Some(&'H'));
}
