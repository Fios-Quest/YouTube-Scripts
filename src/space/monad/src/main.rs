#[derive(PartialEq, Debug)]
enum Maybe<T> {
    Value(T),
    Nothing,
}

impl<T> Maybe<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Value(inner) => Maybe::Value(f(inner)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

fn divide(dividend: i64, divisor: i64) -> Maybe<i64> {
    match divisor {
        0 => Maybe::Nothing,
        _ => Maybe::Value(dividend / divisor),
    }
}

fn main() {
    // ---

    let twenty_five = 5 * 5;
    assert_eq!(twenty_five, 25);

    let five = 5.to_string();
    assert_eq!(five, "5");

    // ---

    fn square(input: i64) -> i64 {
        input * input
    }

    fn to_string(input: i64) -> String {
        format!("{input}")
    }

    let twenty_five = square(5);
    assert_eq!(twenty_five, 25);

    let five = to_string(5);
    assert_eq!(five, "5");

    type example = fn(T) -> U;

    // ---

    fn divide(dividend: i64, divisor: i64) -> Maybe<i64> {
        match divisor {
            0 => Maybe::Nothing,
            _ => Maybe::Value(dividend / divisor),
        }
    }

    // Rust
    let maybe_result = divide(4, 2).map(square).map(to_string);

    match maybe_string_result {
        Maybe::Value(s) => println!("{s}"),
        Maybe::Nothing => println!("Nothing to show"),
    }

    // ---

    let maybe_string_result = divide(4, 2).map(square).map(to_string);

    match maybe_string_result {
        Maybe::Value(s) => println!("{s}"),
        Maybe::Nothing => println!("Nothing to show"),
    }

    // ---

    fn bleh() -> Result<&str, &str> {
        let maybe_example = Maybe::Value("Hello");

        let option_example = Some("Hi!");

        let result_example = Ok("Ok!");

        result_example
    }

    let maybe_result = divide(4, 0);

    assert_eq!(maybe_result, Maybe::Nothing);

    let maybe_string = maybe_result.map(square).map(to_string);

    assert_eq!(maybe_string, Maybe::Nothing);
}
