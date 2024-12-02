# Common Traits

The Rust standard library itself provides a huge number of traits Today we're going to discuss some of what I think are the most important to be aware of

Whether that's because you'll want to implement them yourself, you'll want to consume types that implement them, or they have interesting knock on effects you should be aware of.

As ever, this series is accompanied by a free book, check the description for a link straight to this chapter.

My name is Daniel, welcome to IRISS.

---

As it happens, there are a _lot_ of really great Traits in the Rust standard library, so I'm splitting this video into two.

This time we'll discuss: Markers, Derivables and Error Handling traits

Next time we'll discuss: Converters, Referencing and Dereferncing traits and one other trait that didn't quite fit into any other category.

A notable exception from either of these videos are traits relating to iterators, specifically Iterator and IntoIterator.

These provide such rich a feature set, I wanted them to have their own separate video which will come after we discuss Collections.

Before we dive in to todays traits, I want to quickly cover something we didn't mention in the last chapter.

Required and Provided Methods
-----------------------------

Traits can not only define the header for methods you need to provide yourself, but they can also define methods with default behaviour that you can optionally override.

We call the methods you need to write _Required_ and the ones you can optionally override as _Provided_.

For example, in the last chapter we defined the trait `Animal` like this:

In this case, `get_name` doesn't have a body, so anyone implementing `Animal` for their type must provide it. 

This is a _Required_ method.

If, instead, we were to write some default functionality, this becomes a _Provided_ method which implementors of the Animal trait can choose whether they want to override or to use as is

It's up to you to decide when it makes sense to provide default behaviour.

In the case of `Animal::get_name`, this default behaviour isn't really "providing' anything meaningful, I think keeping it a Required method, with no default behaviour, is the right way to go.

Markers
-------

Markers are special traits that describe intrinsic properties of types, that is they relate to what you might call the core essence of the type.

That might not make much sense right now but don't worry, it will.

### Sized

We're starting with a weird one here... well... all markers are a little weird but this one doubly so. 

You never need to implement `Sized` yourself, but you may choose to manually opt out of it, and it does have a use.

Anything that is of known size at compile time is consider to be `Sized` and you don't need to specify this yourself.

For example, a `u8` has size 8 bits*, therefore it is sized. 

*cough and point*

In fact all primitives are sized, except for string slices, which you can't use outside their reference form anyway.

Any compound type you create from only `Sized` types is also considered to be `Sized`.

So, if you don't need to implement `Sized` yourself, why am I talking about it?

One place you will see `Sized` a lot is that due to a quirk in Rusts design, generic types are always assumed to be `Sized`.

For this reason you will regularly see the trait bound "question mark Sized" which means that the concrete type (the one used to fill in the generic) may or may not be "Sized`.

While trait bounds usually restrict what types can be used in the concrete implementation, this has a widening effect.

For example, in the last chapter, I mentioned that I was printing a simplified version of `ToString` implementation for all type that implement `Display`.

This was because I left out the "question mark Sized" trait bound, so the `ToString` generic implementation actually looks more like this:

```rust,ignore
impl<T: Display + ?Sized> ToString for T {
    // ...
}
```

The `+` means the type `T` must abide both trait bounds so `T` must implement `Display` but also does not need to be `Sized`.

### Copy

The `Copy` marker trait means that the data the type contains can be copied, however, "copy" has a very specific meaning in Rust which means that all the data can be exactly copied as is.

This only works, however, for types that exist on the Stack.

Let's take `String` as an example.

`String` is a smart pointer that points to memory on the heap.

The value inside the `String`, the raw string slice, can of course be duplicated, but the `String` type itself is actually just a pointer to a location in memory.

If we were to copy that location data, we'd have two pointers pointing to the same location.

As soon as one of them is cleaned up, the data on the stack would also be cleaned up, and we'd be left with a `String` pointing at memory we no longer own.

We'll talk more about how we can duplicate things like Strings and other smart pointers later in the video.

Types that can exist on the Stack though can be `Copy`. 

All primitives (again, excluding string slices) are `Copy` and compound types built from those types can choose to implement `Copy`.

One awesome thing `Copy` does is it changes how the language itself works.

To re-iterate, `Copy` can only apply to things on the Stack, so the memory for a copied value doesn't need to be requested from the operating system like it would with the heap, and the actual copying part is very cheap.

Because of this, Rust will use what are called "Copy Semantics" instead of "Move Semantics".

This means, unlike normal, when you reassign a variable, or pass it to a function, if the variable has the `Copy` trait, you can still use the original variable after.

So ordinarily we can't do something like this, you'll get a compile time error.

```rust,compile_fail
let x = "String is not Copy".to_string();
let y = x;
print!("y owns the str {y}"); 
print!("x no longer owns the str {x}");
```

However, for types that do implement `Copy` that code does work thanks to Copy Semantics:

```rust
let mut x = 42;
let y = x;
print!("x still owns the value {x}, and so does y {y}");
// As its a copy, we can change one without changing the other
x += 10;
print!("we've added 10 to x, which is now {x}, but y is still {y}");
```

You can implement `Copy` directly, though you must also implement a trait called `Clone` which we'll discuss later, but since both traits are derivable, its very rare you'd ever do it manually.

I'll show you that shortly.

### Send / Sync

We haven't talked about concurrent programming yet, however, you might have heard that Rust is extremely safe and efficient compared to many other languages/

Much of that safety comes from the marker traits, `Send` and `Sync`.

`Send` is used when data can be safely "sent" between threads.

Again, we'll talk about this more in the future, so don't worry what this means just yet, however, when something is "sent" from one thread to another, it moves ownership, like when you pass a variable to another function.

`Sync` is used when a _reference_ to data can be safely sent from one thread to another

So `T` is `Send` if `T` can be SENT safely from one thread to another
And `T` is `Sync` if a reference to `T` can be safely used across multiple threads SYNCHRONOUSLY

We'll talk a lot more about threaded programming later in the book so don't worry if this doesn't make sense yet, in fact, `Send` and `Sync`, like `Sized`, are automatically derived. 

This means you don't even have to worry about implementing them for your own types: 

So long as your types are entirely constructed from other types that are `Send` and/or `Sync`, the Rust compiler knows that your type is `Send` and/or `Sync` too.


Derivables
----------

Apart from `Sized`, `Send` and `Sync`, most traits _need_ to be manually opted in to, however, for some traits, the behaviour is so simplistic that the trait can be derived.

For _most_ derivable Rust traits there is a requirement that each child of your type implements the trait you're attempting to implement yourself.

To derive a trait we use the derive attribute.

Attributes can be defined either inside or outside the item they are for, however, like Documentation, unless the attribute is being in some way applied to the whole file (for example, as a module), we exclusively use external attributes that come before the item they apply to.

Like Documentation, we use an exclamation mark to differentiate the two

The derive attribute itself, looks a bit like a function, and it takes a list of what _looks_ like traits but are actually what we call "Derive Macros"

Not every trait has a Derive Macro meaning not all traits are derivable.

You can write your own Derive Macros too, though this is a very advanced form of meta programming we probably won't cover in this series. 

Many people do write their own though, to provide custom derive macros for traits provided in their own libraries, and we will talk about that when we start talking about the Rust ecosystem.

### Debug

`Debug` is an extremely useful utility Trait that creates a default way to write out types to places like standard out and standard error.

When printing a `Debug` value, we use colon question inside curly brackets for a positional marker,

Or you can put it after the name of a variable, like this.

Ironically perhaps, you should try to avoid using `Debug` for debugging, that's what a debugger is for, head back to our getting started video if you need a reminder.

The `Debug` macro though is very useful for logging, though be careful not to leak private information this way, this might be where you want to implement `Debug` manually.

`Debug` is needed for assertion macros like `assert_eq!`, mainly used in testing.

If you `assert_eq!` two values, and they're not equivalent, the test suite will want to print the values to the screen. 

We'll show this more when we talk about the equivalence traits in the next section.

`Debug` works very similarly to `Display` taking a formater as a parameter.

You might be worried about making sure your implementation of the `Debug` trait behaves similarly to official or derived implementations, well that's where the formatter gets _really_ cool.

It provides a ton of different tools that help you build a well-structured output.

We won't go into that here, but you can see more in the official `Debug` documentation.

### PartialEq / Eq

`Eq` and `PartialEq` are Rust's equivalency traits, that's right, equivalence, not equality.

What's the difference, what does equivalence mean and why are there two traits?

Allow me to answer those questions with another question:

Is zero equivalent to negative zero?

Inside a floating point number the binary representation is different but Mathematically, zero is neither positive nor negative, so they're equivalent right?

Sticking with the binary representations inside floating points, it's possible to represent something that's Not a Number (NaN).

Should two NaNs, even if they have the same binary representation, be considered as the same value when you can get there in different ways?

Probably not.

For the most part in Rust, we're only concerned with Partial Equivalence, this is what allows us to compare values with the equals-equals operator.

Given what we've just discussed, consider this code, what do you think the output _should_ be?

Let's run it and see!

That seems correct, right?

You can derive `PartialEq` so long as all the parts of your type also implement `PartialEq`, or you can implement it yourself.

Implementing it yourself can be really handy if you have a structure where some fields _can_ be different but still be considered the same overall "thing".

The official Rust book uses ISBNs as an example, but just to be different, lets consider how you might also want this kind of behaviour for aliased user information.

`PartialEq` has two methods, `eq` which is Required and `ne` which is Provided.

Remember from earlier, we need to implement the required method, but can choose if we want to override the provided one.

The default behaviour for `ne` is `not eq` and I'm fine with that, but it's there if you need it.

In this case, I'm going to say two Users are the same so long as their ID is the same.

Let's write a little test for this, if we derive `Debug`, and have implemented `PartialEq` we can use the "assert eq" macro to do that for us.

`PartialEq` has even more utility though!

It's a generic trait where the generic parameter represents the type for the "right hand side" or RHS.

This generic parameter defaults to being the same type, but we can write code that allows us to compare the equivalence of different types too!

Taking that User alias example again, what if we had a "root" user type, and an aliased User type.

Now we can check if the User is equivalent to its Alias.

Bear in mind though that "right hand side" means this equivalence check only works one way around, you'll need to write the implementation for UserAlias too if you want to go the other way around.

So that's `PartialEq`, but what is `Eq`?

`Eq` doesn't actually provide any additional behaviour, it's an empty trait that can only be applied to types that are also `PartialEq`.

It's purpose _isn't_ to provide functionality but to indicate to you, the software engineer, and anyone looking at your code, that types have exact equivalence.

Those points we made about floating points earlier, different binary representations having equality, and the same binary representation not being considered equal, are not `Eq`, which is why `f32` and `f64` do not implement `Eq`.

There's no way for the compiler to guarantee the correct implementation of `Eq` so it's something you need to be mindful of.

Unlike `PartialEq`, `Eq` is not a generic that can be used with other types (since we're talking about exact equivalence, this wouldn't make sense).

Earlier we chose to make that `User` type partially equivalent if the id matched.

If we instead checked the entire object, it could be considered to be exactly equivalent:

Of course, in this case, it'd be far easier _and safer_ to use the derived version, which protects us making mistakes in complex code, or forgetting to check changes we make in our type later

### PartialOrd / Ord

As you can imagine, `PartialOrd` and `Ord` have a similar relationship to each other as `PartialEq` and `Eq`, and indeed:
- `PartialOrd` can only be applied to types with `PartialEq`
- `Ord` can only be applied to types with `Eq` (and `PartialOrd`)

Both `PartialOrd` and `Ord` have one Required method each (`partial_cmp` and `cmp` respectively) as well as some Provided methods with default behaviour. 

The required methods of each trait use the `Ordering` type which looks roughly like this:

```rust
pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1,
}
```

`PartialEq` is what gives us our usual greater than (`>`), less than (`<`), greater or equal to (`>=`)  and less than or equal to (`<=`) behaviour, through the use of the methods `gt`, `lt`, `ge` and `le` respectively,

Unless these methods are implemented, their default behaviour relies on `partial_cmp`, which returns `Option<Ordering>`.

Again, using floating point numbers, it's easy to see why we use an `Option` on our comparisons.

When comparing `NaN`, is it greater than, less than, or equal to `NaN`?

We can't determine that, so we use the `None` variant to represent that.

One important thing to bear in mind when deriving `PartialOrd` is that although, yes you can do it if all parts of your type implement `PartialOrd`, when derived on structs, it will first check the ordering of the first field, and only move on to the next field if the first field is equal.

To look at why that might not give you the behaviour you expect, lets imagine a rectangle with width and height.

By deriving the behaviour for Partial Ord, Rust will first compare the width, and only if those are equal compare the height, leading to this weird behaviour.

For this reason, it's quite likely that you'd want to implement `PartialOrd` yourself, depending on how you think types should be compared.

Instead of using the derived behaviour, lets create an area method for our rectangles, and use that to compare them to each other.

I think, in this case, this makes more sense.

Finally `Ord` isn't quite the same as `Eq` because it _does_ have methods:
- `cmp` which is like `partial_cmp` but returns `Ordering` without the `Option`
- `max` which returns the greater of the two values
- `min` which returns the lesser
- `clamp` which will return a value so long as its between two other values, or the closest value that is

Like with `PartialOrd`, `Ord` can be derived but has the same ordering quirk. 

If we want to implement it ourselves, we only need to implement `cmp`, and the other methods can use that for their default behaviour.

Importantly, when implementing both `PartialOrd` _and_ `Ord`, the result of `partial_cmp` _must_ match `cmp`, though the compiler has no way of confirming this for you. 

The easiest way to handle this is if you need to manually implement `PartialOrd` is to simply call `cmp` and wrap it in an `Option`.

Let's update our Rectangle

```rust
use std::cmp::Ordering;

// Remember PartialEq is required for PartialOrd, Eq is required for Ord
#[derive(Debug, Eq, PartialEq)]
struct Rect {
    width: u64,
    height: u64,
}

impl Rect {
    pub fn area(&self) -> u64 {
        self.width * self.height
    }
}

impl Ord for Rect {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.area().cmp(&rhs.area())
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

# fn main() {
// --- Example comparison ---
    
let test_one_lhs = Rect { width: 2, height: 1 };
let test_one_rhs = Rect { width: 1, height: 1000 };
# assert_eq!(test_one_lhs.cmp(&test_one_rhs), Ordering::Less);
println!("test one: lhs is {:?} than rhs", test_one_lhs.cmp(&test_one_rhs));

// --- You still need to be careful with default behaviour ---
// --- What do you think happens here? ---

let two_one = Rect { width: 2, height: 1 };
let one_two = Rect { width: 1, height: 2 };
let four_four = Rect { width: 4, height: 4 };
println!("{:?}", four_four.clamp(two_one, one_two));
# }
```

Unlike `PartialEq`, neither `PartialOrd` nor `Ord` are generic, they can only be implemented where both the left hand side and the right hand side are the same type.

### Clone (and Copy)

`Clone` is a bit like `Copy` in that it allows you to duplicate values, however, where `Copy` is implicitly very cheap, `Clone` can get away with doing a bit more work.

With `Copy`, we can make a copy of data on that is purely on the stack, however, this restricts us to `Sized` data. 

This means, for example, `String` which is a smart pointer to data on the heap, can not implement `Copy`. 

In order to duplicate `String` we'd need to request new memory on the Heap to place the data into, then copy the data to the new location, and create a new smart pointer on the stack to point to it.

Requesting heap memory is considered expensive as you have to wait for the operating system to provide you a location you can use, so it's really handy to differentiate `Clone` from `Copy`.

Luckily, you don't have to do all of this memory allocation stuff yourself. 

For any type that is built from other types that already implement `Clone` you can derive `Clone`.

```rust
#[derive(Clone, PartialEq, Debug)]
struct MyNewType(String); // String already implements Clone, PartialEq and Debug

# fn main() {
// --- Testing clone ---

let a = MyNewType("Hello, world!".to_string());
let b = a.clone();
assert_eq!(a, b);
# }
```

If you need to implement `Clone` yourself (rare and only required in very specific and advanced circumstances), then you can do so:

```rust,ignore
struct MyNewType(String);

impl Clone for MyNewType {
    fn clone(&self) -> Self {
        // ...
    }
}
```

In order to derive `Copy`, not only must your type be made from only other types that implement `Copy`, but your type must also implement `Clone`.

```rust
#[derive(Copy, Clone, PartialEq, Debug)]
struct MyNewType(u32); // This tuple struct uses a u32 which implements Copy and Clone

// --- Testing copy ---

# fn main() {
let a = MyNewType(1);
let b = a; // Copy is automatic when its available
assert_eq!(a, b);
# }
```

### Default

Many types could be considered to have an obvious default state: 

Defaults for numbers are typically zero, while `String`s and collections default to being empty.

If your type is built from only types that implement `Default` then you can derive the behaviour of `Default` for your type to be, essentially, the instantiation of your type with all values set to _their_ default.

```rust
#[derive(Default, Debug)]
struct Person {
    name: String,
    age: u8,
}

# fn main() {
// --- Usage ---
    
let person = Person::default();
assert_eq!(&person.name, "");
assert_eq!(person.age, 0);
println!("Default persons name is '{}' and their age is '{}'", person.name, person.age);
# }
```

Obviously, this may not always be the desired result, so you can obviously implement the trait directly:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: "Jane Doe".to_string(),
            age: 30,
        }
    }
}

# fn main() {
// --- Usage ---

let person = Person::default();
assert_eq!(&person.name, "Jane Doe");
assert_eq!(person.age, 30);
println!("Default persons name is '{}' and their age is '{}'", person.name, person.age);
# }
```

You might be wondering if you can derive `Default` for Enums, or if you have to implement it directly, and you actually can, using an additional attribute that you apply to the value you want to be the default.

```rust
#[derive(Default, Debug, PartialEq)]
enum SomeEnum {
    Variant1,
    #[default]
    Variant2,
}

# fn main() {
// --- Usage ---

let choice = SomeEnum::default();
assert_eq!(choice, SomeEnum::Variant2);
# }
```

Unfortunately the `default` attribute only works when deriving `Default` for unit enums, which means if your enum contains nested types, you _will_ have to implement `Default` manually:

```rust
// The nested types here mean we can't derive default
#[derive(Debug, PartialEq)]
enum SomeEnum {
    Variant1(u32),
    Variant2(String),
}

impl Default for SomeEnum {
    fn default() -> Self {
        SomeEnum::Variant2("Hello".to_string())
    }
}

# fn main() {
// --- Usage ---

let choice = SomeEnum::default();
assert_eq!(choice, SomeEnum::Variant2("Hello".to_string()));
# }
```

### Hash

Hashing is the process of taking a (usually) arbitrary amount of information and distilling it into a fixed size of data.

This is a one way process (kinda), but giving the same input will always give you the same output, and _that_ is pretty useful!

There are lots of different ways to hash that are suitable for lots of different purposes.

In Rust there is a trait that describes a type that is `Hash` which means that it can be "hashed", and another trait called `Hasher` which does the hashing, but these traits aren't for general hashing, in Rust they have a specific use.

You _generally_ don't need to worry too much about either trait, but `Hash` is useful if you want your type to work as a key in a`HashMap` or similar data structure.

So long as your type is constructed only of other types that implement `Hash`, then you can derive it, though if you need more control than that, then you can of course implement the trait methods yourself. 

This might be useful if you want to skip over some of the types that make up your compound type that can't be hashed _BUT_ when using `Eq`, if `A == B`, then`hash of A must == hash of B` must also be true.

To derive it yourself simply use the derive attribute, and you'll be good to use it in a `HashMap`:

```rust
#[derive(Hash)]
struct Email(String);
```

Error Handling
--------------

### Display

Before we jump straight into the `Error` trait, lets recap on `Display`.

This trait allows us to display information related to the type that implements it. 

Once you implement it, if you pass a value of your type into a macro like `println!`, `eprintln!` or `format!`, then `Display` defines how the type will be rendered.

`Display` has single Required method which takes a reference to `self`, and a mutable pointer to a `Formatter` and it returns a `fmt::Result` which is a type alias for `Result<(), fmt::Error>`. 

The easiest way to implement it is with the `write!` macro which returns this same type, and to `use std::fmt` so that you can reference things in the module namespace rather than contaminating your own.

```rust
use std::fmt;

struct MyUnitStruct;

impl fmt::Display for MyUnitStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "My unit struct")
    }
}
```

### Error

The `Error` trait is applied to types that are specifically used to represent something that went wrong during the execution of code.

Although `Result`s do not _require_ the `Error` trait be implemented for types in their Error variant, it is definitely worth doing as error types with the `Error` trait provide a lot of utility for very little effort.

The trait itself has several "provided" methods but no Required methods. 

You're unlikely to want to alter the provided behaviour of the `Error` trait which means the only thing you need to do is make sure that your error type _also_ implements `Debug` and `Display`. 

As we know, `Debug` is usually derivable, so that just leaves `Display`. 

Let's create a custom Error for a fridge to demonstrate how we _might_ do this.

We'll use an enum to represent the error states which are either, 
- too warm, with a temperature
- too cold with a temperature
- or an alert that the power has failed

We'll derive Debug, and we'll implement Display which will just produce a nice human readable message.

Finally we can implement Error which will provide all the methods we need.

While we've avoided talking about the wider ecosystem so far, it's worth mentioned there are some _extremely_ powerful Error libraries that might change the way you work with errors. 

We will cover these in the Ecosystem part of the book.

Converters
----------

### From / Into

By now you're probably beginning to understand how important types are to Rust, but sometimes, you need to take the data from one type, and move it to another type.

`From` and `Into` are the easiest ways to do this, providing the `from` and `into` methods respectively. 

For example, you'll regularly see people turning a string slice into a string in one of these two ways:

We can create a String from a string slice, or we can turn a string slice into a String, though in this case we need to be specific about what we're converting the string slice into.

```rust
# fn main() {
let first = String::from("first");
let second: String = "second".into();
println!("{first}, {second}");
# let hidden_example = String::from(second); // String also implements `From<String>`
# }
```

What's really cool though is you rarely, if ever, have to implement `Into` yourself. 

You might have realised that the functionality of `impl Into<String> for &str` is probably identical to `impl From<&str> for String`, and the good folks behind Rust realised that too!

There is a generic implementation of `Into` that looks like this:

```rust,ignore
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

We haven't talked about `where` yet, but it's a way of providing type bounds (like when we've used colons in previous examples) that's great for when the type bound is a little more complex. 

This generic implementation simply applies `Into<U>` for any type where `U` can already be gotten `From<T>`.

Simple, but powerful. 

Because of this however, you should only ever implement `Into` if you _can't_ implement `From`, which rarely comes up outside of crate scoping which we'll discuss much further into this series.

To show you how to implement `From`, I'm going to teach you one of the best and most common uses for it!

#### `From`, `Error` and the `?` Operator

Once you understand `From` and `Error` you have access to another incredibly powerful tool in Rust's arsenal, the `?` operator.

We've discussed previously that in Rust, we don't use exceptions, when a function can fail, it should return a `Result<T, E>`.

As you can imagine, when we have deeply nested function calls, all of which can fail, it would get a bit annoying to have to constantly handle each possible error separately. 

That's where we use `?`. When used on a `Result` it will,
immediately extract the `Ok` variant if the `Result` is `Ok`, otherwise it will return the `Err` variant as an `Error`
of the current function.

eg:
```rust,ignore
let value_if_ok = function_that_returns_result()?;
```

Of course, it's unlikely that the calling function returns the exact same `Error` type in its `Result` as a function
it's called, however, if there is a `From` implementation that can convert them, then the `?` operator will use that
to propagate the error.

Here's an example of a job we need to run that will get a user from a store, and then do something with them.

```rust
// A lot of boilerplate code hidden here, use the 👁️ icon if you want to see it. ->

# use std::fmt;
# use std::error::Error;
#
# struct User;
#
# #[derive(Debug)]
# struct DbError;
# #[derive(Debug)]
# struct InvalidUserError;
# #[derive(Debug)]
# struct ProductStoreError;
#
// This Error describes a specific UserStore problem 
#[derive(Debug)]
enum UserStoreError {
    DatabaseError(DbError),
    UnknownEmail(String),
}
# 
# impl fmt::Display for UserStoreError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
#         write!(f, "UserStoreError")
#     }
# }
#
# impl Error for UserStoreError {}

// This Error describes wider problems that might occur in our application
#[derive(Debug)]
enum ApplicationError {
    UserStoreError(UserStoreError),
    ProductStoreError(ProductStoreError),
}
#
# impl fmt::Display for ProductStoreError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
#         write!(f, "ProductStoreError")
#     }
# }
#
# impl Error for ProductStoreError {}

// We Implement From our UserStore error to our general application error
impl From<UserStoreError> for ApplicationError {
    fn from(value: UserStoreError) -> ApplicationError {
        ApplicationError::UserStoreError(value)
    }
}

struct UserStore;

impl UserStore {
    // This function produces our first error
    fn get_user_by_email(email: &str) -> Result<User, UserStoreError> { 
        // ...
        Err(UserStoreError::UnknownEmail(String::from(email)))
    }
}

// This function returns a different error type
fn run_job() -> Result<(), ApplicationError> {
    // If the Result is Ok, then the Ok value is given to `user`  
    // Otherwise it converts the UserStoreError to an ApplicationError and returns it
    let user = UserStore::get_user_by_email("wrong-email@example.com")?;
    // It's equivalent to:
    // ```
    // let get_user_result = UserStore::get_user_by_email("wrong-email@example.com");
    // let user = match get_user_result {
    //     Ok(value) => value,
    //     Err(error) => { return Err(ApplicationError::from(error)); }
    // };
    // ```
    // So you can see it saves a lot of time!
    
    // ... do stuff with the user ...
    
    Ok(())
}

fn main() {
    if let Err(error) = run_job() {
        eprintln!("{error:?}");
    }
}
```

Official Documentation: [`From`][From], [`Into`][Into], [`?`][?]

### TryFrom / TryInto

Sometimes, its just not possible to guarantee that a conversion from one thing into another will actually work.
`TryFrom` and `TryInto` can help you with possible errors using another feature of traits we haven't discussed,
associated types.

To oversimplify things a little, let's say you're talking to an external system that thinks about pets like this:

```rust
enum PetType {
    Cat,
    Dog,
    Rat, //..
}

struct Pet {
    pet_type: PetType,
    name: String,
}
```

but your system _only_ cares about Cats:

```rust
struct Cat {
    name: String
}
```

We can't `impl From<Pet> for Cat` because not all Pets are Cats. We can use `impl TryFrom<Pet> for Cat` to manage this
for us which is similar to `From` except that it can fail, therefore returns a `Result<Ok, Err>` type where the `Ok`
type is the type we're converting to which is the type we're implementing for and the `Err` type is... hmm, there's no
error type in the generic or what we're implementing for.

It's time to introduce "Associated Types".

The official Rust book has a detailed explanation of [Associated Types][Associated Types], but I think the best way
to think about them is they are a bit like private static placeholder types. They're similar to generics in that they
are defined once and reused throughout a trait implementation, however, you are not leaving the type up to the person
calling the generic code.

This loss of flexibility comes with two significant upsides however;

1. The trait is easier to use for the caller (less or no generics)
2. Relatedly, you are reducing the risk area of making mistakes.

```rust
use std::convert::TryFrom; // You do not need to do this since Rust 2021, including for backwards compatability

# #[derive(Debug,PartialEq)]
# enum PetType {
#     Cat,
#     Dog,
#     Rat, //..
# }
# 
# #[derive(Debug)]
# struct Pet {
#     pet_type: PetType,
#     name: String,
# }
# 
# #[derive(Debug)]
# struct Cat {
#     name: String,
# }
#
#[derive(Debug)]
struct NotACatError(Pet);

impl TryFrom<Pet> for Cat {
    type Error = NotACatError;
    
    fn try_from(pet: Pet) -> Result<Cat, Self::Error> {
        if pet.pet_type != PetType::Cat {
            Err(NotACatError(pet))
        } else {
            Ok(Cat { name: pet.name })
        }
    }
}

# fn main() {
// --- Usage ---

let yuki_pet = Pet { pet_type: PetType::Cat, name: "Yuki".into() };
let yuki_cat_result = Cat::try_from(yuki_pet);
// This should display "Result: Ok(Cat { name: "Yuki" })"
println!("Result: {yuki_cat_result:?}");

let lassie_pet = Pet { pet_type: PetType::Dog, name: "Lassie".into() };
let lassie_cat_result = Cat::try_from(lassie_pet);
// This should display "Result: Err(NotACatError(Pet { type: Dog, name: "Lassie" }))"
println!("Result: {lassie_cat_result:?}");
# }
```

And yes, `TryInto` is automatically provided by Rust for any types that already provide the reverse `TryFrom`
implementation. One thing to note though is, like `into`, you still need to type hint to Rust what the generic parts
are, but because they're now inside a result its a little more verbose:

```rust
# use std::convert::{TryFrom, TryInto}; // You do not need to do this since Rust 2021, including for backwards compatability
#
# #[derive(Debug,PartialEq)]
# enum PetType {
#     Cat,
#     Dog,
#     Rat, //..
# }
#
# #[derive(Debug)]
# struct Pet {
#     pet_type: PetType,
#     name: String,
# }
#
# #[derive(Debug)]
# struct Cat {
#     name: String,
# }
#
# #[derive(Debug)]
# struct NotACatError(Pet);
# 
# impl TryFrom<Pet> for Cat {
#     type Error = NotACatError;
# 
#     fn try_from(pet: Pet) -> Result<Cat, Self::Error> {
#         if pet.pet_type != PetType::Cat {
#             Err(NotACatError(pet))
#         } else {
#             Ok(Cat { name: pet.name })
#         }
#     }
# }
# fn main() {
let yuki_pet = Pet { pet_type: PetType::Cat, name: "Yuki".into() };
let yuki_cat_result: Result<Cat, _> = yuki_pet.try_into();
println!("Result: {yuki_cat_result:?}");
# }
```

Note: we only need to specify the ok variant of the `Result`, the error type can be inferred from the `TryFrom`
associated type, how clever is that! To ask Rust to infer a type, we use `_`.

Official Documentation: [`TryFrom`][TryFrom], [`TryInto`][TryInto], [Associated Types][Associated Types],
[Inference][Inference]

Referencing and Dereferencing
-----------------------------

We've talked about this a little bit already but in Rust, rather than having to pass around ownership of a value, you
can instead reference it, while leaving ownership wherever it originated (so long as the reference never outlives the
owned data it points to). Reference's in Rust are similar to pointers in other languages that you might have heard of,
in that they are a value which "points at" another location in memory where the actual value is.

Since the reference only points at the data, if you pass it into a function, when the function ends, only the reference
is cleaned up, not the data it was pointing at. Because the reference doesn't own the data it points at, we describe it
as "borrowing" the data. You can have any number of immutable borrows, or a single mutable borrow.

Rust has other kinds of pointers too though, called "Smart Pointers". These are similar to references in that they
"point at" some location in memory, however, they also provide additional functionality to the data. The common example
of this is `String` which is a smart pointer to a string slice that exists on the heap. Not only is it the only way
to manipulate string data, but it also handles things like how that data is freed when the `String` is dropped.


A simpler smart pointer we haven't covered yet is `Box<T>` which _only_ provides the facilities for requesting the
memory allocation, and freeing it once the `Box` value goes out of scope and is dropped.

In Rust, it is possible to write code that manipulates memory directly, but it's actually very rare to _need_ to do
this. We'll cover this later in the `unsafe` chapter which I promise is less scary than it sounds!

The next few traits deal specifically with traits that make references and smart pointers more malleable.

### Borrow / BorrowMut

`Borrow` allows you to borrow the data of one type as another if its another type, and `BorrowMut` allows you to borrow
that data mutably.

One type that already implements `Borrow` is `String`. As I mentioned above, `String` is a smart pointer to a string
slice stored on the heap, and it implements `Borrow<str>` to allow us to borrow the data as if it were a string slice
type.

```rust
// We need to bring the trait in scope
use std::borrow::Borrow;

fn say_hello(who: &str) {
    println!("Hello, {who}!");
}

fn main() {
    let name = "Yuki".to_string();
    say_hello(name.borrow()); // Borrows name as if it were a str
}
```

In this case, instead of borrowing the `String` as `&String` like it normally would, Rust sees that it's being passed
into a function that accepts a `&str` _and_ that `String` implements `Borrow<str>` so it can use that to provide a
reference to a string slice, `&str`.

There are blanket implementations of both traits, so, for any type `T` you know that `Borrow<T>` and `BorrowMut<T>` are
already implemented, meaning that any value of type `T` can be borrowed as itself, i.e. `&T`, and any mutable value of
type `T` can be mutably borrowed as itself, `&mut T`.

You can also provide further implementations of `Borrow` yourself allowing you to borrow the same data as if it were a
variety of types, however there are some important restrictions that effectively mean you should only implement borrow
for types where the internal representation remains the same. This means you should never implement borrow that only
returns part of the underlying data. One way to check this is to be sure the hash of a value `v` must be the same as the
hash `v.borrow()`.

A common pattern in Rust is to use wrappers around other types, this is the "new type" (often written as "newtype")
pattern. Imagine you have a type that represents an email address. Obviously a sensible type to store that data in might
be `String`, however, there's no validation on creating a `String`, so how do we know if any given string contains a
valid email. For that we can wrap the string in a "new type".

```rust
use std::borrow::Borrow;
# use std::error::Error;
# use std::fmt;
# use std::ops::Deref;
# use std::hash::{DefaultHasher, Hash, Hasher};

# #[derive(Debug)]
struct InvalidEmailError; // Use the eye icon to see all the error code ->
# // Arguably the error code isn't necessary for this example but now we've explained 
# // it I think its important to do things properly

# impl fmt::Display for InvalidEmailError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
#         write!(f, "{self:?}")
#     }
# }
# 
# impl Error for InvalidEmailError {}
#
# #[derive(Hash, Debug)]
struct Email(String);

impl Email {
    // We'll validate the email when we create it, returning a Result
    pub fn new(email: String) -> Result<Email, InvalidEmailError> {
        if Email::is_valid(&email) {
            Ok(Email(email))
        } else {
            Err(InvalidEmailError)
        }
    }
    
    // This will validate any &str
    fn is_valid(email: &str) -> bool {
        // Check there is an '@' and its at neither the beginning nor end
        let at_pos = email.find('@');
        if let Some(at_pos) = at_pos {
            return at_pos > 0 && at_pos < email.len() - 1;
        }
        false
    }
}

// --- Our Borrows ---

impl Borrow<String> for Email {
    fn borrow(&self)  -> &String {
        &self.0
    }
}

impl Borrow<str> for Email {
    fn borrow(&self)  -> &str {
        &self.0
    }
}

// --- Functions that accept our borrow types, but not email ---

fn test_str(s: &str) {
    println!("{s} is an &str")
}

fn test_string(s: &String) {
    println!("{s} is an &String")
}

// --- Code that shows it all works: ---

# fn main() {
let good_address = "example@example.com";
let email = Email::new(good_address.to_string())
    .expect("failed to create email, check result");

// We can borrow the string slice inside of email
test_str(email.borrow());
test_string(email.borrow());
# 
#     // Hello curious reader 👋🏻. When using borrow the hash of the borrowed value must be equal to the hash of the 
#     // original value (as I understand it) 
#     let borrowed_email: &str = email.borrow();
#     assert_eq!(hash(&email), hash(borrowed_email));
# 
#     // These just test if the is_valid logic works, I don't want to look like an idiot now do I? 😅
#     assert!(Email::is_valid("a@b"));
#     assert!(!Email::is_valid("@ab"));
#     assert!(!Email::is_valid("ab@"));
#     assert!(Email::is_valid("example@example.com"));
# }
# 
# fn hash<H: Hash + ?Sized>(hashable: &H) -> u64 {
#     let mut hasher = DefaultHasher::new();
#     hashable.hash(&mut hasher);
#     hasher.finish()
# }
```

`BorrowMut` does exactly the same thing but gives you a mutable reference instead. In our `Email` example we _could_
implement it to get a mutable reference to the underlying `String`... but in this case, should we?

Allowing mutation of the data inside the email would bypass the logic of our `Email` type that guarantees the email
address is valid.

Now, there's some important caveats to `Borrow` and `BorrowMut`.

- If a type implements borrow, where it's true that `x == y`, then it must also be true that `x.borrow() == y.borrow()`
- If your type implements `Eq` then your borrowed type must also implement `Eq`
- Furthermore, if `x > y` then `x.borrow() > y.borrow()`, if `x < y` then `x.borrow() < y.borrow()`, etc
- Finally, if we have a hashing function, if `hash(x) == hash(y)` then `hash(x.borrow()) == hash(y.borrow())`

There are no compiler checks for these caveats, you need to be sure that its true when you implement `Borrow` and, as
you can probably guess, `Borrow` really only works for changing the exact binary representation of a value from one type
to another, making it less useful for compound types.

Official Documentation: [`Borrow`][Borrow], [`BorrowMut`][BorrowMut]

### AsRef / AsMut

So we now have a way to borrow an entire type as a different type, but we won't be able to do that with more complex
compound types. If we have a more complex object and want to internally reference

Remember earlier we had our `Cat` type which only had a name. We could, if we wanted, implement `AsRef<str>` so that
it can be used in the place of a `&str`:

```rust
struct Cat {
    name: String,
    age: u8,
}

impl AsRef<str> for Cat {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

// --- Example Use ---

fn cuddle(who: &str) {
    println!("Cuddle {who}");
} 

fn main() {
    let yuki = Cat { name: "Yuki".into(), age: 15 };
    cuddle(yuki.as_ref());
}
```

Arguably, we could make this code even friendly by changing the `cuddle` to take a generic, and then calling `.as_ref()`
in the function itself. This code looks a little scarier, but once you get used to seeing code like this, you can write
far more flexible and easy to use code.

```rust
# struct Cat {
#     name: String,
# }
# 
# impl AsRef<str> for Cat {
#     fn as_ref(&self) -> &str {
#         &self.name
#     }
# }
# 
fn cuddle<S>(who: &S)
where
    S: AsRef<str> + ?Sized 
{
    println!("Cuddle {}", who.as_ref());
} 

fn main() {
    let yuki = Cat { name: "Yuki".into() };
    cuddle(&yuki); // We can now just pass a reference to Cat
}
```

`AsMut` is essentially the same as `AsRef` but for mutable references instead!

Official Documentation: [`AsRef`][AsRef], [`AsMut`][AsMut]

### Deref / DerefMut

We mentioned above that when referencing, you create a pointer to another piece of data and when dereferencing you're
moving back from the reference to whatever is underneath. This can be esspecially useful when working with references
to references:

```text
     ---- reference ---->        ---- reference ---->
Cat                        &Cat                        &&Cat
     <-- dereference ----        <-- dereference ----
```

We also talked a bit about smart pointers which are not references but a way to wrap data with additional functionality.
To get inside a smart pointer, we use the `Deref` trait, this is why `String` can be used as if it were a `str`.

```text
     --- Smart Pointer ----->
str                            String
     <----- dereference -----
```

When a smart pointer wraps a mutable type (remember `str` is not itself mutable) then you can also implement `DerefMut`
_but_ you need to think carefully about when it's appropriate to do this.

Let's return to our `Email` type, it makes sense to allow our `Email` to be used as an immutable String, so lets
implement `Deref` for it:

```rust
use std::ops::Deref;
# use std::error::Error;
# use std::fmt;
# 
# #[derive(Debug)]
struct InvalidEmailError; // Use the eye icon to see all the error code ->
# // Arguably the error code isn't necessary for this example, but now we've explained 
# // it above, I think it's important to do things properly

# impl fmt::Display for InvalidEmailError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
#         write!(f, "{self:?}")
#     }
# }
# 
# impl Error for InvalidEmailError {}
#
# #[derive(Debug)]
struct Email(String);

impl Email {
    // ...
#     pub fn new(email: String) -> Result<Email, InvalidEmailError> {
#         if Email::is_valid(&email) {
#             Ok(Email(email))
#         } else {
#             Err(InvalidEmailError)
#         }
#     }
#     
#     pub fn is_valid(email: &str) -> bool {
#         // Note: this is oversimplified but is less likely to give false negatives than many approaches
#         let at_pos = email.find('@');
#         if let Some(at_pos) = at_pos {
#             return email.len() >= 3 && at_pos > 0 && at_pos < email.len() - 1;
#         }
#         false
#     }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_str(s: &str) {
    println!("{s} is an &str")
}

fn main() {
    let good_address = "example@example.com";
    let email = Email::new(good_address.to_string())
        .expect("failed to create email, check result");

    // We can deref the Email into a string slice simply by referencing it...
    // ...yes, I know, it's weird
    test_str(&email);
    
    // We can even pass the email type to its own validation function now!
    assert!(Email::is_valid(&email));
# 
#     // I kept these tests in case I change anything by mistake 
#     assert!(Email::is_valid("a@b"));
#     assert!(!Email::is_valid("@ab"));
#     assert!(!Email::is_valid("ab@"));
#     assert!(Email::is_valid("example@example.com"));
}
```

Let's think about this differently though, what if instead of dereferencing to a str, we dereferenced to a `String`,
_and_ we allowed mutability. Our `Email` type here does some rudimentary validation, if we allowed mutability of the
inner `String`, we allow people to change the email to be invalid, and lose the benefit of the `Email` type.

```rust
use std::ops::{Deref, DerefMut};
# use std::error::Error;
# use std::fmt;
# use std::hash::{DefaultHasher, Hash, Hasher};
# 
# #[derive(Debug)]
struct InvalidEmailError; // Use the eye icon to see all the error code ->
# // Arguably the error code isn't necessary for this example, but now we've explained 
# // it above, I think it's important to do things properly

# impl fmt::Display for InvalidEmailError {
#     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
#         write!(f, "{self:?}")
#     }
# }
# 
# impl Error for InvalidEmailError {}
#
# #[derive(Debug)]
struct Email(String);

impl Email {
    // ...
#     pub fn new(email: String) -> Result<Email, InvalidEmailError> {
#         if Email::is_valid(&email) {
#             Ok(Email(email))
#         } else {
#             Err(InvalidEmailError)
#         }
#     }
#     
#     pub fn is_valid(email: &str) -> bool {
#         // Note: this is oversimplified but is less likely to give false negatives than many approaches
#         let at_pos = email.find('@');
#         if let Some(at_pos) = at_pos {
#             return email.len() >= 3 && at_pos > 0 && at_pos < email.len() - 1;
#         }
#         false
#     }
}

impl Deref for Email {
    // Note that DerefMut requires Deref and uses _it's_ target
    // This means you can not DerefMut to a different type
    type Target = String;

    // ...
#     fn deref(&self) -> &Self::Target {
#         &self.0
#     }
}

impl DerefMut for Email {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// --- And here's why we shouldn't implement DerefMut, BorrowMut or AsMut for Email ---

# fn main() {
let good_address = "example@example.com";
let mut email = Email::new(good_address.to_string())
    .expect("failed to create email, check result");

match Email::is_valid(&email) {
    true => println!("{} is a valid email", email.deref()),
    false => println!("{} is NOT a valid email", email.deref()),
}

email.remove(7);

match Email::is_valid(&email) {
    true => println!("{} is a valid email", email.deref()),
    false => println!("{} is NOT a valid email", email.deref()),
}
# 
#     // I kept these tests in case I change anything by mistake 
#     assert!(Email::is_valid("a@b"));
#     assert!(!Email::is_valid("@ab"));
#     assert!(!Email::is_valid("ab@"));
#     assert!(Email::is_valid("example@example.com"));
# }
```

Official Documentation: [`Deref`][Deref], [`DerefMut`][DerefMut]

Other
-----

### Drop

Rust is _very_ good at cleaning up after itself, especially when you use the standard library:
- If your variable allocate heap memory, that memory is released when the variable that owns it goes out of scope
- If you open a file to read or write, it's closed when the file handler goes out of scope
- If you start a TCP connection, its ended when the handler goes our of scope

The Rust standard library is achieving this with the `Drop` trait.

You can implement the drop trait yourself:

```rust
struct UnitStruct;

impl Drop for UnitStruct {
    fn drop(&mut self) {
        println!("Dropping: UnitStruct")
    }
}

fn main() {
    println!("Starting: Outer scope");
    {
        println!("Starting: Inner scope");
        
        println!("Creating: UnitStruct");
        let unit_struct = UnitStruct;
        
        println!("Leaving: Inner scope");
    } // <- Drop happens here
    println!("Leaving: Outer scope");
}
```

When a variable goes out of scope, if it implements the `Drop` trait, then the functionality on that trait is called,
which allows you to write cleanup code for the type implementing the trait. Depending on the type of programming you do
you may not need to think about this trait very much... _except_, there is one thing worth considering.

Each of the examples I gave above is "blocking". That means that the program will have to wait until whatever the `drop`
method of the `Drop` trait needs to do is complete before continuing. You may want to be mindful of this when you allow
things to go out of scope, and be aware of what any library code you're consuming might be doing.

Most of the time this isn't worth worrying too much about, however, if you do find you want to very precisely control
_when_ variables are dropped, then let me introduce you to my all-time favourite function `std::mem::drop`.

Here it is in full:

```rust
pub fn drop<T>(_x: T) {}
```

Yeah, that's not a mistake. It has one generic variable and no function body. Remember that variables in Rust are owned
by the function they exist in, and when they leave that function they're dropped. The intention of this function is that
at the exact moment you want to cause a variable to be cleaned up, you pass ownership of that variable into this
function, the function immediately ends, and, if the variable has a `Drop` implementation, then that code is run then
and there.

Official Documentation: [`Drop`][Drop], [`drop`][drop()]

Next Chapter
------------

I've skipped Iterators in this chapter because, despite them being a very common trait, there is a _lot_ to talk about,
and this chapter already got out of hand. 😉 Before we get to them though, I want to talk about Collections (like `Vec`)
which will make Iterators easier to understand.

[Sized]: https://doc.rust-lang.org/std/marker/trait.Sized.html
[Copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[Send]: https://doc.rust-lang.org/std/marker/trait.Send.html
[Sync]: https://doc.rust-lang.org/std/marker/trait.Sync.html
[Debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[PartialEq]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
[Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[PartialOrd]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[Ord]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
[Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[Default]: https://doc.rust-lang.org/std/default/trait.Default.html
[Hash]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[Error]: https://doc.rust-lang.org/std/error/trait.Error.html
[From]: https://doc.rust-lang.org/std/convert/trait.From.html
[Into]: https://doc.rust-lang.org/std/convert/trait.Into.html
[TryFrom]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[TryInto]: https://doc.rust-lang.org/std/convert/trait.TryInto.html
[Borrow]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[BorrowMut]: https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html
[AsRef]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
[AsMut]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
[Deref]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[DerefMut]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[Drop]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[drop()]: https://doc.rust-lang.org/std/mem/fn.drop.html

[?]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator
[Associated Types]: https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html
[Inference]: https://doc.rust-lang.org/rust-by-example/types/inference.html
