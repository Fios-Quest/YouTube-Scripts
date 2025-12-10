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

fn square(input: i64) -> i64 {
    input * input
}

fn to_string(input: i64) -> String {
    format!("{input}")
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
    let five = 5.to_string();

    assert_eq!(twenty_five, 25);
    assert_eq!(five, "5");

    // ---

    assert_eq!(square(5), 25);
    assert_eq!(to_string(5), "5".to_string());

    // ---

    let maybe_result = divide(4, 2);
    let maybe_squared_result = maybe_result.map(square);
    let maybe_string_result = maybe_squared_result.map(to_string);

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
}
