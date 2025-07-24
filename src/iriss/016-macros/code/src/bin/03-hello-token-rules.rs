macro_rules! hello {
    (world) => { String::from("Hello, world") };
    (yuki) => { String::from("Hello, yuki") };
}

fn main() {
    assert_eq!(hello!(world), "Hello, world".to_string());
    assert_eq!(hello!(yuki), "Hello, yuki".to_string());
}

