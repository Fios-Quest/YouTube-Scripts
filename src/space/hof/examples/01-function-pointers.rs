fn unnecessary_repeat(s: &str, times: u8) -> String {
    let mut output = String::with_capacity(s.len() * times as usize);
    for _ in 0..times {
        output.push_str(s)
    }
    output
}

fn main() {
    type RepeatFunction = fn(&str, u8) -> String;

    let repeat_pointer: RepeatFunction = unnecessary_repeat;

    let output = repeat_pointer("Hello", 2);
    assert_eq!(output, "HelloHello".to_string());
}
