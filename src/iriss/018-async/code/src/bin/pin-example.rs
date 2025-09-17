use std::pin::Pin;

struct ExampleOfSelfReference {
    value: usize,
    pointer_to_value: Option<*const usize>,
}

impl ExampleOfSelfReference {
    fn new(value: usize) -> Self {
        Self {
            value,
            pointer_to_value: None,
        }
    }

    fn set_pointer(&mut self) {
        self.pointer_to_value = Some(&raw const self.value);
    }

    fn get_value(&self) -> usize {
        // SAFETY: Do NOT use this! Do NOT copy this! It is purposefully NOT SAFE!
        unsafe { *self.pointer_to_value.expect("Did not .set_pointer()") }
    }
}

fn main() {
    let mut example = ExampleOfSelfReference::new(1);
    example.set_pointer();

    let mut pinned_example = Pin::new(&mut example);

    assert_eq!(pinned_example.get_value(), 1);

    // example.value = 2;
    // let example = example;

    pinned_example.value = 2;
    assert_eq!(pinned_example.get_value(), 2);

    let mut pinned_example = pinned_example;
    pinned_example.value = 3;
    assert_eq!(pinned_example.get_value(), 3);
}
