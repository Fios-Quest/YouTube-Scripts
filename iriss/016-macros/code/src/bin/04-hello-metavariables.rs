macro_rules! hello {
    ($name:literal) => {{
        let mut output = String::from("Hello, ");
        output.push_str($name);
        output
    }};
}

fn main() {
    assert_eq!(
        {
            let mut output = String::from("Hello, ");
            output.push_str("Yuki");
            output
        },
        "Hello, Yuki".to_string()
    );
}
