/// # SAFETY
///
/// To use this safely, you... don't need to do anything because this function
/// just returns true
unsafe fn conceptually_dangerous_function() -> bool {
    true
}

fn safe_abstraction() -> bool {
    // ...do some checks...

    // SAFETY: We confirmed safety by doing the following checks
    // - again, the function does nothing so nothing to really check here
    unsafe { conceptually_dangerous_function() }
}

fn main() {
    // We can safely call the safe abstraction to do unsafe things safely
    let output = safe_abstraction();
    assert!(output);
}
