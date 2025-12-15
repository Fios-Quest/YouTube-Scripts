fn main() {
    let greeting = "Hello".to_string();

    let greet = |name: &str| {
        let response = format!("{}, {}", &greeting, name);
        response
    };

    assert_eq!(greet("Daniel"), "Hello, Daniel".to_string());
    assert_eq!(greet("Yuki"), "Hello, Yuki".to_string());
    assert_eq!(greeting, "Hello".to_string());
}
