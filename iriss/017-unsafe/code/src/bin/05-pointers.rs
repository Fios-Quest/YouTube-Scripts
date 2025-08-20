fn main() {
    let the_answer = 42;
    let pointer = &raw const the_answer;

    println!("The variable at {pointer:p} contains the data '{the_answer}'");

    // ---

    let hello = String::from("Hello, world!");
    let pointer_to_variable = &raw const hello;
    let pointer_to_data = hello.as_ptr();

    println!(
        "The variable at {pointer_to_variable:p}, \
        points to {pointer_to_data:p} \
        which contains the data '{hello}'",
    );

    // ---

    let the_answer = 42;
    let pointer = &raw const the_answer;

    // SAFETY: We own `the_answer` which in scope and is not being used elsewhere
    unsafe {
        let data_at_pointer = *pointer;
        assert_eq!(data_at_pointer, 42);
    }
}
