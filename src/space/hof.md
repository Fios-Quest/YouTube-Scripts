Higher Order Functions
======================

Intro
-----

### intro 1

Higher Order Functions are functions that either take one or more functions as parameters or return a 
function when called themselves.

Easy right?

But... what's a function?

### intro 2

On Fio's Quest we usually talk about data and types, but functions aren't data, they're just a set of instructions, so
how can we pass them around like data?

And while types can be involved in defining functions, are functions themselves typed?

Kinda...

Function Pointers
-----------------

### pointers 1

Functions aren't data, but they do exist as a set of instructions in memory when your program runs

And if it's in memory, that means it has an address, and if it has an address we can create a pointer to it.

Fun fact, unlike raw pointers, function pointers are safe in Rust.

But in Rust pointers are typed, so what does a function pointer type look like.

### pointers 2

Functions have signatures.

In this function, we have two parameters, a string slice and a u8.

The return parameter is a String.

### pointers 3

The type of a function pointer is actually just this information writen like this.

This is basically th function header written without any names.

We can set a variable of that type to point at our function.

Now, we wouldn't normally need to do this inline like this because Rust can infer the type of the function pointer.

```rust
fn unnecessary_repeat(s: &str, times: u8) -> String {
    let mut output = String::with_capacity(s.len() * times as usize);
    for _ in 0..times {
        output.push_str(s)
    }
    output
}

fn main() {
    type RepeatFunction = fn(&str, u8) -> String;

    let repeat_pointer = unnecessary_repeat;

    let output = repeat_pointer("Hello", 2);
    assert_eq!(output, "HelloHello".to_string());
}
```

Where this is actually useful is when defining another function that takes one that's this shape.


```rust
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

    let repeat_pointer = unnecessary_repeat;

    let output = repeat_pointer("Hello", 2);
    assert_eq!(output, "HelloHello".to_string());

    does_something_with_numbers(does_something_with_numbers(repeat_pointer), "ByeBye".to_string());
}
```

Referencing a function using a function pointer can be esspecially useful for things like maps, filters, or even folds.

```rust
fn is_even(input: &usize) -> bool {
    input % 2 == 0
}

fn to_string(input: usize) -> String {
    format!("{input}")
}

fn combine(left: String, right: String) -> String {
    format!("{left}{right}")
}

fn main() {
    let output = (1..=10)
        .into_iter()
        .filter(is_even)
        .map(to_string)
        .fold(String::new(), combine);
    assert_eq!(output, "246810");
}
```


Closures
--------

