fn main() {
    let greet = "Hello, World!".to_string();

    let greet_ptr = &raw const greet;
    println!("greet is stored at {greet_ptr:p}");

    example_move(greet);
    println!();
}

fn example_move(greet: String) {
    let greet = &raw const greet;
    println!("greet is stored at {greet:p}");
}
