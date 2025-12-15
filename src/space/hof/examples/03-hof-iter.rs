fn is_even(input: &usize) -> bool {
    input % 2 == 0
}

fn to_string(input: usize) -> String {
    format!("{input}")
}

fn combine(left: String, right: String) -> String {
    format!("{left}{right}")
}

fn main() {
    let output = (1..=10)
        .into_iter()
        .filter(is_even)
        .map(to_string)
        .fold(String::new(), combine);
    assert_eq!(output, "246810");
}
