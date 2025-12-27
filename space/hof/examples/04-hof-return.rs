fn is_even(input: &usize) -> bool {
    input % 2 == 0
}

fn is_odd(input: &usize) -> bool {
    input % 2 == 1
}

type NumericFilter = fn(&usize) -> bool;

fn create_even_filter(invert: bool) -> NumericFilter {
    match invert {
        false => is_even,
        true => is_odd,
    }
}

fn main() {
    let even_filter = create_even_filter(false);
    assert!(even_filter(&4));

    let odd_filter = create_even_filter(true);
    assert!(odd_filter(&5));
}
