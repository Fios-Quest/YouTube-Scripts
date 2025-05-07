// fn main() {

//     let arr: [u32; 5] = [0, 1, 2, 3, 4];
//     let slice: &[u32] = &arr;

//     assert_eq!(slice, &[0, 1, 2, 3, 4]);

//     let array: [u32; 5] = [0, 1, 2, 3, 4];

//     assert_eq!(&array[1..3],  &[1, 2]);
//     assert_eq!(&array[1..=3], &[1, 2, 3]);
//     assert_eq!(&array[..3],   &[0, 1, 2]);
//     assert_eq!(&array[1..],   &[1, 2, 3, 4]);

// }

fn split<'a>(input: &'a str, sub_string: &str) -> (&'a str, &'a str) {
    if let Some(found_at) = input.find(sub_string) {
        (&input[..found_at], &input[found_at + sub_string.len()..])
    } else {
        (&input[..], &input[input.len()..])
    }
}

fn main() {
    let yuki = String::from("Yuki");
    let sub_string = "uk";

    let (left, right) = split(&yuki, sub_string);

    println!("{left}{right}")
}
