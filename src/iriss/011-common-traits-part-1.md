# Common Traits

The Rust standard library itself provides a huge number of traits.

Today we're going to discuss some of what I think are the most important to be aware of.

Whether that's because you'll want to implement them yourself, you'll want to consume types that implement them, or they have interesting knock on effects you should be aware of.

As ever, this series is accompanied by a free book, check the description for a link straight to this chapter.

My name is Daniel, welcome to IRISS.

---

As it happens, there are a _lot_ of really great Traits in the Rust standard library, so I'm splitting this video into three.

This time we'll discuss: Markers, Derivables and Error Handling traits

Next time we'll discuss: Converters, Referencing and Dereferncing traits and one other trait that didn't quite fit into any other category.

Then we're going to have a break to talk about collections, before coming back to talk about the Iterator traits.

Before we dive in to todays traits, I want to quickly cover something we didn't mention in the last chapter.

## Required and Provided Methods

Traits can not only define the header for methods you need to provide yourself, but they can also define methods with default behaviour that you can optionally override.

We call the methods you need to write _Required_ and the ones you can optionally override _Provided_.

For example, in the last chapter we defined the trait `Animal` like this:

In this case, `get_name` doesn't have a body, so anyone implementing `Animal` for their type must write it themselves. 

This is a _Required_ method.

If, instead, we were to write some default functionality, this becomes a _Provided_ method which implementors of the Animal trait can choose whether they want to override or to use as is

It's up to you to decide when it makes sense to provide default behaviour.

In the case of `Animal::get_name`, this default behaviour isn't really "providing' anything meaningful, I think keeping it a Required method, with no default behaviour, is the right way to go.

## Markers

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

For example, in the last chapter, I mentioned that I was printing a simplified version of the `ToString` implementation for all types that implement `Display`.

This was because I left out the "question mark Sized" trait bound, so the `ToString` generic implementation actually looks more like this:

The `+` means the type `T` must abide both trait bounds so `T` must implement `Display` but also does not need to be `Sized`.

You can (for complex but sensible reasons) opt out of Sized by implementing `!Sized` for your type.

This is a bit beyond today's tutorial, but thought I'd mention it, cos its weird and fun. 

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

However, for types that do implement `Copy` that code does work thanks to Copy Semantics:

You can implement `Copy` directly, though you must also implement a trait called `Clone` which we'll discuss later, but since both traits are derivable, its very rare you'd ever do it manually.

I'll show you that shortly.

### Send / Sync

We haven't talked about concurrent programming yet, however, you might have heard that Rust is extremely safe and efficient compared to many other languages

Much of that safety comes from the marker traits, `Send` and `Sync`.

`Send` is used when data can be safely "sent" between threads.

Again, we'll talk about this more in the future, so don't worry what this means just yet, however, when something is "sent" from one thread to another, it moves ownership, like when you pass a variable to another function.

`Sync` is used when a _reference_ to data can be safely sent from one thread to another

So `T` is `Send` if `T` can be SENT safely from one thread to another
And `T` is `Sync` if a reference to `T` can be safely used across multiple threads SYNCHRONOUSLY

We'll talk a lot more about threaded programming later in the series so don't worry if this doesn't make sense yet, in fact, `Send` and `Sync`, like `Sized`, are automatically derived. 

This means you don't even have to worry about implementing them for your own types: 

So long as your types are entirely constructed from other types that are `Send` and/or `Sync`, the Rust compiler knows that your type is `Send` and/or `Sync` too.


## Derivables

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

Those points we made about floating points earlier, different binary representations being equivalent, and the same binary representation not being considered equivalent, are not `Eq`, which is why `f32` and `f64` do not implement `Eq`.

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

`PartialEq` is what gives us our usual greater than (`>`), less than (`<`), greater or equal to (`>=`)  and less than or equal to (`<=`) behaviour, through the use of the methods `gt`, `lt`, `ge` and `le` respectively,

Unless these methods are implemented, their default behaviour relies on `partial_cmp`, which returns `Option<Ordering>`.

Again, using floating point numbers, it's easy to see why we use an `Option` on our comparisons.

When comparing `NaN`, is it greater than, less than, or equal to `NaN`?

We can't determine that, so we use the `None` variant to represent that.

One important thing to bear in mind when deriving `PartialOrd` is that although, yes you can do it if all parts of your type implement `PartialOrd`, when derived on structs, it will first check the ordering of the first field, and only move on to the next field if the first fields are equal.

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

The easiest way to handle this is you need to manually implement `PartialOrd` is to simply call `cmp` and wrap it in an `Option`.

Let's update our Rectangle

Unlike `PartialEq`, neither `PartialOrd` nor `Ord` are generic, they can only be implemented where both the left hand side and the right hand side are the same type.

### Clone (and Copy)

`Clone` is a bit like `Copy` in that it allows you to duplicate values, however, where `Copy` is implicitly very cheap, `Clone` can get away with doing a bit more work.

With `Copy`, we can make a copy of data on that is purely on the stack, however, this restricts us to `Sized` data. 

This means, for example, `String` which is a smart pointer to data on the heap, can not implement `Copy`. 

In order to duplicate `String` we'd need to request new memory on the Heap to place the data into, then copy the data to the new location, and create a new smart pointer on the stack to point to it.

Requesting heap memory is considered expensive as you have to wait for the operating system to provide you a location you can use, so it's really handy to differentiate `Clone` from `Copy`.

Luckily, you don't have to do all of this memory allocation stuff yourself. 

For any type that is built from other types that already implement `Clone` you can derive `Clone`.

If you need to implement `Clone` yourself (rare and only required in very specific and advanced circumstances), then you can do so:

In order to derive `Copy`, not only must your type be made from only other types that implement `Copy`, but your type must also implement `Clone`.

### Default

Many types could be considered to have an obvious default state: 

Defaults for numbers are typically zero, while `String`s and collections default to being empty.

If your type is built from only types that implement `Default` then you can derive the behaviour of `Default` for your type to be, essentially, the instantiation of your type with all values set to _their_ default.

Obviously, this may not always be the desired result, so you can obviously implement the trait directly:

You might be wondering if you can derive `Default` for Enums, or if you have to implement it directly, and you actually can, using an additional attribute that you apply to the value you want to be the default.

Unfortunately the `default` attribute only works when deriving `Default` for unit enums, which means if your enum contains nested types, you _will_ have to implement `Default` manually:

### Hash

Hashing is the process of taking a (usually) arbitrary amount of information and distilling it into a fixed size of data.

This is a one way process (kinda), but giving the same input will always give you the same output, and _that_ is pretty useful!

There are lots of different ways to hash that are suitable for lots of different purposes.

In Rust there is a trait that describes a type that is `Hash` which means that it can be "hashed", and another trait called `Hasher` which does the hashing, but these traits aren't for general hashing, in Rust they have a specific use.

You _generally_ don't need to worry too much about either trait, but `Hash` is useful if you want your type to work as a key in a`HashMap` or similar data structure.

So long as your type is constructed only of other types that implement `Hash`, then you can derive it, though if you need more control than that, then you can of course implement the trait methods yourself. 

This might be useful if you want to skip over some of the types that make up your compound type that can't be hashed _BUT_ when using `Eq`, if `A == B`, then`hash of A must == hash of B` must also be true.

To derive it yourself simply use the derive attribute, and you'll be good to use it in a `HashMap`:

## Error Handling

### Display

Before we jump straight into the `Error` trait, lets recap on `Display`.

This trait allows us to display information related to the type that implements it. 

Once you implement it, if you pass a value of your type into a macro like `println!`, `eprintln!` or `format!`, then `Display` defines how the type will be rendered.

`Display` has single Required method which takes a reference to `self`, and a mutable pointer to a `Formatter` and it returns a `fmt::Result` which is a type alias for `Result<(), fmt::Error>`. 

The easiest way to implement it is with the `write!` macro which returns this same type, and to `use std::fmt` so that you can reference things in the module namespace rather than contaminating your own.

### Error

The `Error` trait is applied to types that are specifically used to represent something that went wrong during the execution of code.

Although `Result`s do not _require_ the `Error` trait be implemented for types in their Error variant, it is definitely worth doing as error types with the `Error` trait provide a lot of utility for very little effort, such as where the Error occurred.

The trait itself has several "provided" methods but no Required methods. 

You're unlikely to want to alter the provided behaviour of the `Error` trait which means the only thing you need to do is make sure that your error type _also_ implements `Debug` and `Display`. 

As we know, `Debug` is usually derivable, so that just leaves `Display`. 

Let's create a custom Error for a fridge to demonstrate how we _might_ do this.

We'll use an enum to represent the error states which are either, 
- too warm, with a temperature
- too cold, with a temperature
- or an alert that the power has failed

We'll derive Debug, and we'll implement Display which will just produce a nice human-readable message.

Finally, we can implement Error which will provide all the methods we need.

While we've avoided talking about the wider ecosystem so far, it's worth mentioned there are some _extremely_ powerful Error libraries that might change the way you work with errors. 

We will cover these in the Ecosystem part of the book.

## Next time

Next time we're going to continue learning the top traits, but I want you to keep Errors in mind as we're going to learn a very cool trick with them.

If this has been useful to you, don't forget to like and subscribe and I hope to see you next time! 
