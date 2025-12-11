fn unnecessary_repeat(s: &str, times: u8) -> String {
    let mut output = String::with_capacity(s.len() * times as usize);
    for _ in 0..times {
        output.push_str(s)
    }
    output
}

type RepeatFunction = fn(&str, u8) -> String;

fn does_something_with_numbers(f: RepeatFunction) -> String {
    f("Bye", 2)
}

fn main() {

    // --

    let repeat_pointer: RepeatFunction = unnecessary_repeat;

    let output = repeat_pointer("Hello", 2);
    assert_eq!(output, "HelloHello".to_string());

    // --

    let output = does_something_with_numbers(repeat_pointer);
    assert_eq!(output, "ByeBye".to_string());

    // --
}
