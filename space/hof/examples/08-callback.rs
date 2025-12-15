fn main() {
    let output = (1..=10)
        .into_iter()
        .filter(|i| i % 2 == 0)
        .map(|i| format!("{i}"))
        .fold(String::new(), |left, right| format!("{left}{right}"));
    assert_eq!(output, "246810");
}
