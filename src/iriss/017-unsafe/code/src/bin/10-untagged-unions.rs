union CharOrNumber {
    number: u32,
    character: char,
}

fn main() {
    let mut h = CharOrNumber { character: 'O' };

    // SAFETY: We only set the character variant meaning both variants are valid.
    unsafe {
        println!(
            "The numeric value of the character {} is 0x{:x}",
            h.character, h.number
        );
    }

    h.character = 'o';

    // SAFETY: We only set the character variant meaning both variants are valid.
    unsafe {
        println!(
            "The numeric value of the character {} is 0x{:x}",
            h.character, h.number
        );
    }
}
