macro_rules! hello {
    () => {
        hello!("world")
    };
    ($name:literal) => {{
        let mut output = String::from("Hello, ");
        output.push_str($name);
        output
    }};
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki".to_string());
}
