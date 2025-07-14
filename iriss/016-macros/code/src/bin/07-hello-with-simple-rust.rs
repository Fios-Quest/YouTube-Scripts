macro_rules! hello {
    ($($names:literal),*) => {{
        let names = [$($names, )*];

        use std::iter::Peekable;
        use std::slice::Iter;
        let mut names_iter: Peekable<Iter<&str>> = names.iter().peekable();

        let mut output = String::from("Hello, ");
        output.push_str(names_iter.next().unwrap_or(&"world"));

        while let Some(next_name) = names_iter.next() {
            match names_iter.peek() {
                Some(_) => output.push_str(", "),
                None => output.push_str(" and "),
            }
            output.push_str(next_name);
        };

        output.push_str("!");
        output
    }}
}

fn main() {
    assert_eq!(hello!(), "Hello, world!".to_string());
    assert_eq!(hello!("Yuki"), "Hello, Yuki!".to_string());
    assert_eq!(
        hello!("Yuki", "Daniel", "Indra"),
        "Hello, Yuki, Daniel and Indra!".to_string()
    );
}
