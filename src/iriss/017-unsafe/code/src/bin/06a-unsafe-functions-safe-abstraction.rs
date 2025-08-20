fn main() {
    let mut left = "Left".to_string();
    let mut right = "Right".to_string();

    println!("Left is {left}");
    println!("Right is {right}");

    std::mem::swap(&mut left, &mut right);

    assert_eq!(left, "Right".to_string());
    assert_eq!(right, "Left".to_string());

    println!("Left is {left}");
    println!("Right is {right}");
}
