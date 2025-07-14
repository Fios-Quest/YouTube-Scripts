macro_rules! hello {
    () => {
        String::from("Hello, world")
    };
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
            output.push_str(" and ");
            output.push_str("Daniel");
            output.push_str(" and ");
            output.push_str("Indra");
            output
        },
        "Hello, Yuki and Daniel and Indra".to_string()
    );
}
