fn main() {
    let mut greeting = "Hello".to_string();

    let mut greet = |name: &str| {
        greeting.push_str(", ");
        greeting.push_str(name);
        let response = greeting.clone();
        response
    };

    assert_eq!(greet("Daniel"), "Hello, Daniel".to_string());
    assert_eq!(greet("Yuki"), "Hello, Daniel, Yuki".to_string());
    assert_eq!(greeting, "Hello, Daniel, Yuki".to_string());
}
