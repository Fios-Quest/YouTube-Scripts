use std::mem::ManuallyDrop;
use std::ops::Deref;

fn main() {
    let capacity = 100;

    let mut original_string = String::with_capacity(capacity);
    original_string.push_str("This string is a longer string");

    let pointer = original_string.as_mut_ptr();

    // SAFETY: We create a string from the original, but we prevent the new string
    // from being moved or dropped
    unsafe {
        let overlapping_string = String::from_raw_parts(pointer, 15, capacity);
        let mut overlapping_string = ManuallyDrop::new(overlapping_string);

        assert_eq!(overlapping_string.deref(), &"This string is ".to_string());

        overlapping_string.push_str("short");

        assert_eq!(
            overlapping_string.deref(),
            &"This string is short".to_string()
        );
    }

    // Un oh!
    assert_eq!(original_string, "This string is shortger string");
}
