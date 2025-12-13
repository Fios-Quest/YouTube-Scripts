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

    assert_eq!(does_something_with_numbers(repeat_pointer), "ByeBye".to_string());
}
```

Referencing a function using a function pointer can be esspecially useful for things like filters, maps, or folds.

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

We can also return function pointers from other functions.

Here we are returning a function that takes a reference to a usize and returns a bool.

```rust
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
```

But I think at this point we have to stop and wonder... is this useful?

We can defer the choice of what code to execute until runtime but you can litereally do that with if statements.

Using function pointers could be a little bit tidier, depending on circustance, but honestly, most of the time you won't
need to do this.

Higher Order Functions are more useful, and frankly more intresting when you can configure the function itself at runtime...

...but you can't do that with function pointers.

Closures
--------

Functions are literally just a set of instructions.

If we want to configure how they behave later we need to apply a little sprinkle of data...

some state that can be carried around with them as we pass them back and forth.

This is where closure's come in.

Rust has three types of closure, well, six technically, we'll talk about the beardo weridos from the mirrorverse later.

I'll give you a few seconds to think about why Rust, specifically, needs 3 closure types when you bring stored

Async Closures
--------------

Async closures are relatively new and so you'll see a mixture of ways to encapsulate futures in closures.

More recently AsyncFn, AsyncMut and AsyncOnce were stabalised, allowing you to write async in front of a closure.

Because this is a relatively new feature, in older code you'll likely see closures with async blocks, which feels
backwards, but makes sense.

async, whether on a function, code block, or now, a closure, is syntactic sugar that allows two things.

First of all, the ability to defer a future with await...

and second, and more importantly to whats happening here, it says "this thing produces a Future".

If you create a standard closure, and immediately return an async block, then you're returning future from the closure
too.

So in a way, Rust has had async closures for a while... but in a way, it also hasn't.

Using this older method causes all sorts of problems with lifetimes.

So actually, you'll almost always see an explicit move with closures that return an async block...

Maybe multiple times...

Summary
-------

Closures and higher order functions are conceptually complex, but as we start writting them, they're really clean and
incredibly powerful.

In Rust specifically the lifetime juggle makes things a bit more complex, and can be a bit of a headache to work around.

Been there, believe me.

However, we can see that the amazing people contributing to the Rust language continues to improve usability and
ergonomics.

Just as a reminder, if you enjoyed this video, don't forget hit the like button

YouTubers are a fragile lot and it helps nurse our egos

Next time, now that we've seen how functions can be passed to other functions, we're going to get a little wild.

Our next video is going to explain... Monads... don't worry though.

The pattern itself is actually super cool and much easier than it sounds.

I'm going to explain the pattern while avoiding the complex language surrounding it.

This will no doubt frustratate the purists but its such an awesome pattern its worth discussing anyway!

Hope to see you then.

