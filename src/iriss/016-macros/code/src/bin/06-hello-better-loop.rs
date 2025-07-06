macro_rules! hello {
    () => { hello!("world") };
    ($name:literal $(, $rest:literal)*) => {{
        let mut output = String::from("Hello, ");
        output.push_str($name);
        $(
            output.push_str(" and ");
            output.push_str($rest);
        )*
        output
    }}
}

fn main() {
    assert_eq!(hello!(), "Hello, world".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki".to_string());
    assert_eq!(
        hello!("Yuki", "Daniel", "Indra"),
        "Hello, Yuki and Daniel and Indra".to_string()
    );
}
