unsafe trait NotSafe {
    /// # Safety
    ///
    /// This method isn't actually unsafe
    unsafe fn conceptually_dangerous_method(&self) -> bool {
        true
    }

    fn safe_abstraction(&self) -> bool {
        // SAFETY: The method called isn't actually unsafe
        unsafe { self.conceptually_dangerous_method() }
    }
}

struct ExampleUnitType;

unsafe impl NotSafe for ExampleUnitType {}

fn main() {
    let a = ExampleUnitType;
    a.safe_abstraction();
}
