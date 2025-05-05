struct Fibonacci {
    previous: u8,
    next: Option<u8>,
}

impl Fibonacci {
    fn new() -> Self {
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

fn main() {
    let mut fib = Fibonacci::new();

    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
    assert_eq!(fib.next(), Some(8));

    let fib = Fibonacci::new();
    assert_eq!(fib.last(), Some(233));

    for f in Fibonacci::new() {
        println!("{f}");
    }

    for (i, f) in Fibonacci::new().enumerate() {
        println!("{i}: {f}");
    }

    for (i, f) in Fibonacci::new().enumerate().take(4) {
        println!("{i}: {f}");
    }
}
