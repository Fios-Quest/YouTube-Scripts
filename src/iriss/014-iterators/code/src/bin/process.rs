fn main() {
    // --- Filter ---

    let mut iter = (1..=10).filter(|n| n % 2 == 0);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(8));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), None);

    assert_eq!((1..=10).count(), 10);
    assert_eq!((1..=10).filter(|n| n % 2 == 0).count(), 5);

    let mut iter = (1..11).filter(|n| n % 2 == 0);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(8));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), None);

    assert_eq!((1..11).len(), 10);
    assert_eq!((1..11).filter(|n| n % 2 == 0).count(), 5);

    // --- Filter len/count ---

    let full_iter = 1..=10;
    let filtered_iter = full_iter.clone().filter(|n| n % 2 == 0);

    // We need to use count as ranges do not implement ExactSizeIterator
    assert_eq!(full_iter.count(), 10);
    assert_eq!(filtered_iter.count(), 5);

    let full_iter = 1..11;
    let filtered_iter = full_iter.clone().filter(|n| n % 2 == 0);

    // We need to use count as ranges do not implement ExactSizeIterator
    assert_eq!(full_iter.len(), 10);
    assert_eq!(filtered_iter.count(), 5);

    // --- Map ---

    let mut iter = (0..3).map(|n| format!("This is item number {n}"));

    assert_eq!(iter.next(), Some("This is item number 0".to_string()));
    assert_eq!(iter.next(), Some("This is item number 1".to_string()));
    assert_eq!(iter.next(), Some("This is item number 2".to_string()));
    assert_eq!(iter.next(), None);

    // -- Filter Map ---

    let mut iter = (u8::MIN..=u8::MAX).filter_map(|n| n.checked_add(250u8));

    assert_eq!(iter.next(), Some(250)); // 0 + 250
    assert_eq!(iter.next(), Some(251)); // 1 + 250
    assert_eq!(iter.next(), Some(252)); // 2 + 250
    assert_eq!(iter.next(), Some(253)); // 3 + 250
    assert_eq!(iter.next(), Some(254)); // 4 + 250
    assert_eq!(iter.next(), Some(255)); // 5 + 250
    assert_eq!(iter.next(), None);

    // --- Filter Map Count ---

    assert_eq!(
        (u8::MIN..=u8::MAX)
            .filter_map(|n| n.checked_add(250u8))
            .count(),
        6
    );
    assert_eq!(
        (u8::MIN..=u8::MAX).map(|n| n.checked_add(250u8)).count(),
        256
    );

    // --- Take / Skip ---

    let v = vec![1, 2, 3, 4, 5, 6];

    let taken: Vec<_> = v.iter().take(2).collect();
    let skipped: Vec<_> = v.iter().skip(4).collect();

    assert_eq!(taken, vec![&1, &2]);
    assert_eq!(skipped, vec![&5, &6]);

    assert_eq!(
        v.iter().skip(1).take(4).collect::<Vec<_>>(),
        vec![&2, &3, &4, &5]
    );

    // --- repeat take chain ---

    use std::iter::repeat;

    let badger = repeat("badger").take(12);
    let mushroom = repeat("mushroom").take(2);
    let song = badger.chain(mushroom).cycle();

    for line in song.take(42) {
        println!("{line}")
    }

    // --- Enumerate ---

    let initial_data = vec!["This", "sentence", "is", "not", "shorter"];

    let collected: Vec<_> = initial_data
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, s)| s)
        .collect();

    assert_eq!(collected, vec!["This", "is", "shorter"]);

    // --- Enumerate FilterMap ---

    let initial_data = vec!["This", "sentence", "is", "not", "shorter"];

    let collected: Vec<_> = initial_data
        .into_iter()
        .enumerate()
        .filter_map(|(i, s)| (i % 2 == 0).then_some(s))
        .collect();

    assert_eq!(collected, vec!["This", "is", "shorter"]);

    // --- Fold ---

    let v: Vec<u8> = vec![1, 2, 3, 4, 5];

    let good_sum = v.into_iter().fold(Some(0u8), |acc, cur| {
        acc.and_then(|total| total.checked_add(cur))
    });

    assert_eq!(good_sum, Some(15));

    let v: Vec<u8> = vec![101, 102, 103, 104, 105];

    let bad_sum = v.into_iter().fold(Some(0u8), |acc, cur| {
        acc.and_then(|total| total.checked_add(cur))
    });

    assert_eq!(bad_sum, None);

    // --- Try Fold ---

    let v: Vec<u8> = vec![101, 102, 103, 104, 105];

    let bad_sum = v.into_iter().try_fold(0u8, |acc, cur| acc.checked_add(cur));

    assert_eq!(bad_sum, None);

    // --- For Each ---

    struct Fibonacci {
        previous: u8,
        next: Option<u8>,
    }

    impl Fibonacci {
        pub fn new() -> Self {
            Self {
                previous: 0,
                next: Some(1),
            }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            // Store "current" value (we're going to overwrite it)
            let current = self.next?;

            // Update internal values
            self.next = current.checked_add(self.previous);
            self.previous = current;

            // Return the "current" value
            Some(current)
        }
    }

    Fibonacci::new()
        .enumerate()
        .for_each(|(i, f)| println!("{i}: {f}"));
    println!("Loop complete!");
}
