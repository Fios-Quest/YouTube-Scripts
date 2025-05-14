use std::iter::repeat;

fn char_to_code(c: char) -> Option<char> {
    match c {
        'b' | 'f' | 'p' | 'v' => Some('1'),
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => Some('2'),
        'd' | 't' => Some('3'),
        'l' => Some('4'),
        'm' | 'n' => Some('5'),
        'r' => Some('6'),
        _ => None,
    }
}

fn bad_soundex(input: &str) -> String {
    let mut iter = input.chars();
    let first_char = iter.next().expect("empty input");

    let numbers = iter
        .filter_map(char_to_code)
        .chain(repeat('0'))
        .take(3)
        .collect::<String>();

    format!("{first_char}{numbers}")
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::bad_soundex;

    #[test]
    fn test_robert() {
        assert_eq!(&bad_soundex("Robert"), "R163");
    }

    #[test]
    fn test_rupert() {
        assert_eq!(&bad_soundex("Rupert"), "R163");
    }

    #[test]
    fn test_rubin() {
        assert_eq!(&bad_soundex("Rubin"), "R150");
    }

    #[test]
    fn test_ashcraft() {
        assert_eq!(&bad_soundex("Ashcraft"), "A261");
    }

    #[test]
    fn test_ashcroft() {
        assert_eq!(&bad_soundex("Ashcroft"), "A261");
    }

    #[test]
    fn test_tymczak() {
        assert_eq!(&bad_soundex("Tymczak"), "T522");
    }

    #[test]
    fn test_pfister() {
        assert_eq!(&bad_soundex("Pfister"), "P236");
    }

    #[test]
    fn test_honeyman() {
        assert_eq!(&bad_soundex("Honeyman"), "H555");
    }
}
