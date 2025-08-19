/// # Safety
///
/// This method isn't actually unsafe
unsafe fn conceptually_dangerous_method<T: Safer + ?Sized>(w: &T) -> bool {
    true
}

trait Safer {
    fn safe_abstraction(&self) -> bool {
        // # Safety The method called isn't actually unsafe
        unsafe { conceptually_dangerous_method(self) }
    }
}

struct ExampleUnitType;

impl Safer for ExampleUnitType {}
