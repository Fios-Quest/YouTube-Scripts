# Common Traits Part 2

We're going to jump straight back into the top traits, if you didn't catch the last video, linky in the corner.

My name is Daniel, welcome to IRISS.

---

## Converters

By now you're probably beginning to understand how important types are to Rust, but sometimes, you need to take the data from one type, and move it to another type.

There's two pairs of traits that do this for you really well, depending on whether you can guarantee the conversion will work or not. 

### From / Into

`From` and `Into` are the easiest ways to convert between types so long as the conversion is guaranteed to work.

They provide the `from` and `into` methods respectively. 

For an example of how they work, you'll regularly see people turning a string slice into a string in one of these two ways:

---

We can create a String from a string slice, or we can turn a string slice into a String, though in this case we need to be specific about what we're converting the string slice into.

---

What's really cool though is you rarely, if ever, have to implement `Into` yourself. 

You might have realised that the functionality of "impl Into String for string slice reference" is probably identical to "impl From string slice reference for String", and the good folks behind Rust realised that too!

There is a generic implementation of `Into` that looks like this

---

We haven't talked about `where` yet, but it's a way of providing type bounds (like when we've used colons in previous examples) that's great for when the type bound is a little more complex. 

This generic implementation simply applies "Into U" for any type where "U" can already be gotten "From T".

Simple, but powerful. 

---

Because of this however, you should only ever implement `Into` if and only if you can not implement `From`.

This rarely comes up outside of crate scoping which we'll discuss much further into this series.

To show you how to implement `From`, I'm going to teach you one of the best and most common uses for it!

#### `From`, `Error` and the `?` Operator

Once you understand `From` and `Error` you have access to another incredibly powerful tool in Rust's arsenal, the question mark operator.

We've discussed previously that in Rust, we don't use exceptions, when a function can fail, it should return a Result type.

---

As you can imagine, when we have deeply nested function calls, all of which can fail, it would get a bit annoying to have to constantly handle each Result, extract the data if it's ok, or handle the error if it's not.  

That's where we use the question mark operator.

When used on a `Result` it will, immediately extract the `Ok` variant if the `Result` is `Ok`, otherwise it will return the `Err` variant as an `Error` of the current function.

Of course, it's unlikely that the calling function returns the exact same `Error` type in its `Result` as a function it's called, however, if there is a `From` implementation that can convert them, then the `?` operator will use that to propagate the error.

This is equivalent to this...

### TryFrom / TryInto

Sometimes, its just not possible to _guarantee_ that a conversion from one type into another will definitely actually work though.

`Try From` and `Try Into` can help you with possible errors using another feature of traits we haven't discussed, associated types.

To oversimplify things a little, let's say you're talking to an external system that thinks about pets like this:

---

but your system _only_ cares about Cats:

---

We can't implement From Pet for Cat because not all Pets are Cats, what happens if it sees a Dog!

Instead, we implement Try From Pet for Cat.

Try From is similar to `From` except that it can fail, therefore returns a Result type where the `Ok` type is the type we're converting to which is the type we're implementing for and the `Error` type is... hmm, there's no error type in the generic or what we're implementing for.

---

It's time to introduce "Associated Types".

The official Rust book has a detailed explanation of Associated Types, but I think the best way to think about them is they are a bit like private static placeholder types.

They're similar to generics in that they are defined once and reused throughout a trait implementation, however, you are not leaving the type up to the person calling the generic code.

This loss of flexibility comes with two significant upsides however;

Firstly, the trait is easier to use for the caller (less or no generics)

Secondly, and relatedly, you are reducing the risk area of making mistakes.

---

So, here's how we would implement Try From for our cat 

And yes, `Try Into` is automatically provided by Rust for any types that already provide the reverse `Try From` implementation.

One thing to note though is, like `into`, you still need to type hint to Rust what the generic parts are, but because they're now inside a result its a little more verbose:

However, we only need to specify the Ok variant of the `Result`, the error type can be inferred from the `TryFrom` associated type, how clever is that! 

To ask Rust to infer a type, we use an underscore.

Referencing and Dereferencing
-----------------------------

We've talked a little about referencing already.

In Rust, we have this concept of ownership: a variable "owns" the data stored in it, when the variable goes out scope, its owned data is cleaned up.

To save having to pass ownership around a lot or creating lots of copies of data, you can borrow the data owned by another variable, when you do this you're given a reference to that data.

In many ways references are like pointers in other languages, except that the compiler will check to make sure a reference never out lives the owner of the data it's borrowing.

When a reference goes out of scope, only the reference is cleaned up, the data it was borrowing isn't.

You can have any number of immutable borrows, or a single mutable borrow.

Rust has other kinds of pointers too though, called "Smart Pointers".

These are similar to references in that they "point at" some location in memory, however, they also provide additional functionality to the data.

The common example of this is `String` which is a smart pointer to a string slice that exists on the heap.

Not only is it the only way to manipulate string data, but it also handles things like how that data is freed when the `String` is dropped.

A simpler smart pointer we haven't covered yet is `Box T` which _only_ provides the facilities for requesting the memory allocation, and freeing it once the `Box` value goes out of scope and is dropped.

In Rust, it is possible to write code that manipulates memory directly, but it's actually very rare to _need_ to do this.

We'll cover this later in the `unsafe` chapter which I promise is less scary than it sounds!

These next few traits deal specifically with traits that make references and smart pointers more malleable.

### Borrow / BorrowMut

`Borrow` allows you to borrow the data of one type as another if its another type, and `BorrowMut` allows you to borrow that data mutably.

All types implement borrow: if you borrow type `T` you get a reference to `T`. If you mutably borrow `T` you get a mutable reference to `T`.

So, for any type `T` you know that `Borrow<T>` and `BorrowMut<T>` are already implemented, meaning that any value of type `T` can be borrowed as itself, i.e. `&T`, and any mutable value of type `T` can be mutably borrowed as itself, `&mut T`.

However, you can also borrow a type as a different type. For example, you can borrow String as if it were a string slice! 

As I mentioned above, `String` is a smart pointer to a string slice stored on the heap, and it implements `Borrow<str>` to allow us to borrow the data as if it were a string slice type.

---

In this case, instead of borrowing the `String` as `&String` like it normally would, Rust sees that it's being passed into a function that accepts a reference to a string slice _and_ that `String` implements `Borrow<str>` so it can use that to provide a reference to a string slice, `&str`.

You can also provide further implementations of `Borrow` yourself allowing you to borrow the same data as if it were a variety of types, however there are some important restrictions that effectively mean you should only implement borrow for types where the internal representation remains the same. 

This means you should never implement borrow that only returns part of the underlying data. 

One way to check this is to be sure the hash of a value `v` must be the same as the hash `v.borrow()`.

A common pattern in Rust is to use wrappers around other types, this is the "new type" (often written as "newtype") pattern. 

Imagine you have a type that represents an email address. Obviously a sensible type to store that data in might be `String`, however, there's no validation on creating a `String`, so how do we know if any given string contains a valid email. For that we can wrap the string in a "new type".

---

`BorrowMut` does exactly the same thing but gives you a mutable reference instead. In our `Email` example we _could_ implement it to get a mutable reference to the underlying `String`... but in this case, should we?

Allowing mutation of the data inside the email would bypass the logic of our `Email` type that guarantees the email address is valid.

Now, there's some important caveats to `Borrow` and `BorrowMut`.

- If a type implements borrow, where it's true that `x == y`, then it must also be true that `x.borrow() == y.borrow()`
- If your type implements `Eq` then your borrowed type must also implement `Eq`
- Furthermore, if `x > y` then `x.borrow() > y.borrow()`, if `x < y` then `x.borrow() < y.borrow()`, etc
- Finally, if we have a hashing function, if `hash(x) == hash(y)` then `hash(x.borrow()) == hash(y.borrow())`

There are no compiler checks for these caveats, you need to be sure that its true when you implement `Borrow` and, as you can probably guess, `Borrow` really only works for changing the exact binary representation of a value from one type to another, making it less useful for compound types.

### AsRef / AsMut

So we now have a way to borrow an entire type as a different type, but we won't be able to do that with more complex compound types. 

If we have a more complex object and want to internally reference

Remember earlier we had our `Cat` type which only had a name. 

We could, if we wanted, implement `AsRef<str>` so that it can be used in the place of a `&str`:

---

Arguably, we could make this code even friendly by changing the `cuddle` to take a generic, and then calling `.as_ref()` in the function itself. 

This code looks a little scarier, but once you get used to seeing code like this, you can write far more flexible and easy to use code.

---

`AsMut` is essentially the same as `AsRef` but for mutable references instead!

### Deref / DerefMut

We mentioned above that when referencing, you create a pointer to another piece of data and when dereferencing you're moving back from the reference to whatever is underneath. 

This can be especially useful when working with references to references:

We also talked a bit about smart pointers which are not references but a way to wrap data with additional functionality.

To get inside a smart pointer, we use the `Deref` trait, this is why `String` can be used as if it were a `str`.

---

When a smart pointer wraps a mutable type (remember `str` is not itself mutable) then you can also implement `DerefMut` _but_ you need to think carefully about when it's appropriate to do this.

Let's return to our `Email` type, it makes sense to allow our `Email` to be used as an immutable String, so lets implement `Deref` for it:

---

Let's think about this differently though, what if instead of dereferencing to a str, we dereferenced to a `String`, _and_ we allowed mutability.

Our `Email` type here does some rudimentary validation, if we allowed mutability of the inner `String`, we allow people to change the email to be invalid, and lose the benefit of the `Email` type.

---

Other
-----

### Drop

Rust is _very_ good at cleaning up after itself, especially when you use the standard library:
- If your variable allocate heap memory, that memory is released when the variable that owns it goes out of scope
- If you open a file to read or write, it's closed when the file handler goes out of scope
- If you start a TCP connection, its ended when the handler goes our of scope

The Rust standard library is achieving this with the `Drop` trait.

You can implement the drop trait yourself:

---

When a variable goes out of scope, if it implements the `Drop` trait, then the functionality on that trait is called,
which allows you to write cleanup code for the type implementing the trait.

Depending on the type of programming you do you may not need to think about this trait very much... _except_, there is one thing worth considering.

Each of the examples I gave above is "blocking".

That means that the program will have to wait until whatever the `drop` method of the `Drop` trait needs to do is complete before continuing.

You may want to be mindful of this when you allow things to go out of scope, and be aware of what any library code you're consuming might be doing.

Most of the time this isn't worth worrying too much about, however, if you do find you want to very precisely control _when_ variables are dropped, then let me introduce you to my all-time favourite function `std::mem::drop`.

Here it is in full:

---

Yeah, that's not a mistake.

It has one generic variable and no function body. Remember that variables in Rust are owned by the function they exist in, and when they leave that function they're dropped. 

The intention of this function is that at the exact moment you want to cause a variable to be cleaned up, you pass ownership of that variable into this function, the function immediately ends, and, if the variable has a `Drop` implementation, then that code is run then and there.

Official Documentation: [`Drop`][Drop], [`drop`][drop()]

Next Chapter
------------

There's still, I think, two big traits to talk about Iterator and Into Iterator, but, before we get to them, we're going to need some things to iterate through

So, next time we'll look at collections!

If you've enjoyed this, don't forget to drop me a like, and if you want to see more, hit subscribe, and I'll see you next time! 
