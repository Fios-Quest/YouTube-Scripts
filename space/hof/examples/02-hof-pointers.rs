fn unnecessary_repeat(s: &str, times: u8) -> String {
    let mut output = String::with_capacity(s.len() * times as usize);
    for _ in 0..times {
        output.push_str(s)
    }
    output
}

type RepeatFunction = fn(&str, u8) -> String;

fn inject_bye(f: RepeatFunction) -> String {
    f("Bye", 2)
}

fn main() {
    let repeat_pointer: RepeatFunction = unnecessary_repeat;
    assert_eq!(inject_bye(repeat_pointer), "ByeBye".to_string());
}
