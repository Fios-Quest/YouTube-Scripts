macro_rules! hello {
    (this must be present) => {
        String::from("Hello, world")
    };
}

fn main() {
    assert_eq!(hello!(this must be present), "Hello, world".to_string());
    assert_eq!(hello!(this wont compile), "Hello, world".to_string());
}
