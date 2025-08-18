fn main() {
    // SAFETY: This function is a no-op
    unsafe {
        this_code_is_unsafe();
    }
}

/// # Safety
///
/// This function doesn't do anything, therefore, you don't need to do anything
/// in particular to use it safely.
unsafe fn this_code_is_unsafe() {}
