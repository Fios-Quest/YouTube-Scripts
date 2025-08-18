static HELLO_MESSAGE: &str = "Hello!";

fn main() {
    println!("This function can read HELLO_MESSAGE without having ownership: {HELLO_MESSAGE}");
    another_function();
}

fn another_function() {
    println!("So can this one: {HELLO_MESSAGE}");
}
