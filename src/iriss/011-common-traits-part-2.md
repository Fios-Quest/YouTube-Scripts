# Common Traits Part 2

We're going to jump straight back into the top traits 

## Converters

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
