# Common Traits

The Rust standard library _itself_ provides a huge number of traits.

We're going to discuss some of what I think are the most important to be aware of.

Whether that's because:
- you'll want to implement them yourself
- you'll want to consume types that implement them
- or they have interesting knock on effects you should be aware of

As ever, this series is accompanied by a free book, check the description for a link straight to this chapter.

My name is Daniel, welcome to iris.

---

As it happens, there are a _lot_ of really great Traits in the Rust standard library, so many that I'm splitting this video into three.

This time we'll discuss: Markers, Derivables and Error Handling traits

Next time we'll discuss: Converters, Referencing and Dereferencing traits and one other trait that didn't quite fit into any other category.

Then we're going to have a break to talk about collections, before coming back to talk about the Iterator traits.

Before we dive in to today's traits, I want to quickly cover something we brushed over in the last video.

## Required and Provided Methods

Traits can not only define the header for methods you need to provide yourself, but they can also define methods with default behaviour that you can optionally override.

We call the methods you need to write _Required_ and the ones with default behaviour _Provided_.

🦀 For example, in the last video we defined the trait `Animal` like this

🦀 In this case, `get name` doesn't have a body, so anyone implementing `Animal` for their type must write it themselves. 

🦀 This is a _Required_ method.

🦀 If, instead, we were to write some default functionality, this becomes a _Provided_ method which implementors of the Animal trait can choose whether they want to override or to use as is

It's up to you to decide when it makes sense to provide default behaviour.

🦀 In the case of `Animal get_name`, this default behaviour isn't really "providing" anything meaningful, I think keeping it a Required method, with no default behaviour, is the right way to go.

## Markers

Markers are special traits that describe intrinsic properties of types, that is, they relate to what you might call the core essence of the type.

That might not make much sense right now but don't worry, it will.

### Sized

We're starting with a weird one here... well... all markers are a little weird but this one doubly so. 

You never need to implement `Sized` yourself, in fact, you can't, but you may choose to manually opt out of it, and it does have a use.

Anything that is of known size at compile time is consider to be `Sized` and you don't need to specify this yourself, it just is.

For example, a `u8` has a size of 8 bits*, therefore it is sized. 

"cough" *and point*

In fact all primitives are sized, except for string slices, which you can't use outside their reference form anyway, and references are also sized.

Any compound type you create from only `Sized` types is also automatically `Sized`.

So, if you don't need to implement `Sized` yourself, why am I talking about it?

One place you will see `Sized` a lot is that due to a quirk in Rusts design, generic types are always assumed to be `Sized`.

For this reason you will regularly see the trait bound "question mark Sized" which means that the concrete type (the one used to fill in the generic) may or may not be "Sized`.

While trait bounds usually restrict what types can be used in the concrete implementation, this has a widening effect.

🦀 For example, in the last video, I mentioned that I was printing a simplified version of the `To String` implementation for all types that implement `Display`.

🦀 This was because I left out the "question mark Sized" trait bound, so the `To String` generic implementation actually looks more like this:

🦀 The plus means the type `T` must abide both trait bounds so `T` must implement `Display` but also does not need to be `Sized`.

You can (for complex but sensible reasons) opt out of Sized by implementing `!Sized` for your type.

This is a way beyond today's tutorial, but thought I'd mention it, cos its weird and fun. 

### Copy

The `Copy` marker trait means that the data the type contains can be copied, however, "copy" has a very specific meaning in Rust.

It means that all the data can be copied _exactly_ as is.

This only works, however, for types that exist purely on the Stack.

Let's take `String` as an example of something that can't be copied in this way.

`String` is a smart pointer that points to memory on the heap.

The value inside the `String`, the raw string slice, can of course be duplicated, but the `String` type itself is actually just a pointer to a location in memory.

If we were to copy that location data, we'd have two pointers pointing to the same location.

As soon as one of them is cleaned up, the data on the stack would also be cleaned up, and we'd be left with a `String` pointing at memory we no longer own.

We'll talk more about how we can duplicate things like Strings and other smart pointers later in the video.

Types that can exist on the Stack though can be `Copy`. 

All primitives (again, excluding string slices, but including immutable reference) are `Copy` and compound types built from those types can choose to implement `Copy`.

You can implement `Copy` directly, though you must also implement a trait called `Clone` which we'll discuss later, but since both traits are derivable, its very rare you'd ever do it manually.

I'll show you that shortly.

Now, one awesome thing `Copy` does is it changes how the language itself works.

To re-iterate, `Copy` can only apply to things on the Stack, so the memory for a copied value doesn't need to be requested from the operating system like it would with the heap

Stack memory is pre-allocated and the actual copying part is very cheap.

Because of this, Rust will use what are called "Copy Semantics" instead of "Move Semantics".

This means, unlike normal, when you reassign a variable, or pass it to a function, if the data in a variable has the `Copy` trait, you can still use the original variable after.

So ordinarily we can't do something like this, you'll get a compile time error.

---

However, for types that do implement `Copy` that code does work thanks to Copy Semantics:

---

### Send / Sync

We haven't talked about concurrent programming yet, however, you might have heard that Rust is extremely safe and efficient compared to many other languages.

Much of that safety comes from the marker traits, `Send` and `Sync`.

We will cover these in more detail in the future but for now.

`Send` is used when data can be safely "sent" between threads. 

When something is "sent" from one thread to another, it moves ownership, like when you pass a variable to another function.

`Sync` is used when a _reference_ to data can be safely sent from one thread to another. 

You have to be more careful here because this allows two threads to read and potentially write to the same location in memory.

So `T` is `Send` if `T` can be SENT safely from one thread to another
And `T` is `Sync` if a reference to `T` can be safely used across multiple threads SYNCHRONOUSLY

We'll talk a lot more about threaded programming later in the series so don't worry if this doesn't make sense yet.

`Send` and `Sync`, like `Sized`, are automatically derived if all parts of your type are also `Send` and/or `Sync`

Of course, you might still want implement `Send` and `Sync` for types that contain types that don't contain compatible types, and you can do this, but this requires us delving into "unsafe" Rust which isn't as scary as it sounds but is beyond the scope of this chapter.

For now, if you need to use non-send or sync types there are built in types to help that we'll cover in a future video.

## Derivables

Apart from `Sized`, `Send` and `Sync`, most traits _need_ to be manually opted in to, however, for some traits, the behaviour is so simplistic that the trait can be derived.

For _most_ derivable Rust traits there is a requirement that each child of your type implements the trait you're attempting to implement yourself.

To derive a trait we use the derive attribute.

Attributes can be defined either inside or outside the item they are for, however, like Documentation, unless the attribute is being applied to a whole file (for example, as a module), we exclusively use external attributes that come before the item they apply to.

And like Documentation, we use an exclamation mark to differentiate the two

The derive attribute itself, looks a bit like a function, and it takes a list of what _looks_ like traits but are actually what we call "Derive Macros"

Not every trait has a Derive Macro meaning not all traits are derivable.

You can write your own Derive Macros too, though this is a very advanced form of meta programming we probably won't cover in this series. 

Many people do write their own though, to provide custom derive macros for traits provided in their own libraries, and we will talk about that when we start talking about the Rust ecosystem.

### Debug

`Debug` is an extremely useful utility Trait that creates a default way to write out types to places like standard out and standard error.

Let's derive Debug for a Cat type like this.

When printing a `Debug` value, we use colon question inside curly brackets for a positional marker

Or you can put it after the name of a variable, like this.

Ironically perhaps, you should try to avoid using `Debug` for debugging, that's what a debugger is for, head back to our getting started video if you need a reminder.

The `Debug` macro though is very useful for logging, though be careful not to leak private information this way, this might be where you want to implement `Debug` manually.

Importantly `Debug` is required for assertion macros like `assert eq`, mainly used in testing.

If you `assert eq` two values, and they're not equivalent, the test suite will want to print the values to the screen. 

We'll show this more when we talk about the equivalence traits in the next section.

We won't go into it deeply here but `Debug` works very similarly to `Display` taking a formater as a parameter.

You might be worried about making sure your implementation of the `Debug` trait behaves similarly to official or derived implementations, well that's where the formatter gets _really_ cool.

It provides a ton of different tools that help you build a well-structured output.

You can read more on this in the official `Debug` documentation.

### PartialEq / Eq

`EQ` and `Partial EQ` are Rust's equivalency traits, that's right, equivalence, not equality.

What's the difference, what does equivalence mean and why are there two traits?

Allow me to answer those questions with another question:

Is zero equivalent to negative zero?

Inside a floating point number the binary representation is different but Mathematically, zero is neither positive nor negative, so they're equivalent right?

Sticking with the binary representations inside of floating points, it's possible to represent something that's Not a Number (NaN).

Should two NaNs, even if they have the same binary representation, be considered as the same value when you can get there in different ways?

Probably not.

For the most part in Rust, we're only concerned with Partial Equivalence, this is what allows us to compare values with the equals-equals operator.

Given what we've just discussed, consider this code, what do you think the output _should_ be?

Let's run it and see!

That seems correct, right?

You can derive `Partial EQ` so long as all the parts of your type also implement `Partial EQ`, or you can implement it yourself.

Implementing it yourself can be really handy if you have a structure where some fields _can_ be different but still be considered the same overall "thing".

The official Rust book uses books with ISBNs as an example, but just to be different, lets consider how you might also want this kind of behaviour for aliased user information.

`Partial EQ` has two methods, `EQ` which is Required and `NE` which is Provided.

Remember from earlier, we need to implement the required method, but can choose if we want to override the provided one.

The default behaviour for `NE` is `not EQ` and I'm fine with that, but it's there if you need it.

In this case, I'm going to say two Users are the same so long as their ID is the same.

Let's write a little test for this, if we derive `Debug`, and have implemented `Partial EQ` we can use the "assert EQ" macro to do that for us.

`Partial EQ` has even more utility though!

It's a generic trait where the generic parameter represents the type for the "right hand side" or RHS.

This generic parameter defaults to being the same type, but we can write code that allows us to compare the equivalence of different types too!

Taking that User aliases example again, what if we had a "root" user type, and an aliased User type.

Now we can check if the User is equivalent to its Alias.

Bear in mind though that "right hand side" means this equivalence check only works one way around, you'll need to write the implementation for UserAlias too if you want to go the other way around.

So that's `Partial EQ`, but what is `EQ`?

`EQ` doesn't actually provide any additional behaviour, it's an empty trait that can only be applied to types that are also `Partial EQ`.

It's purpose _isn't_ to provide functionality but to indicate to you, the software engineer, and anyone looking at your code, that types have exact equivalence.

Remember those points I made about floating points earlier, different binary representations being equivalent, and the same binary representation not being considered equivalent?

This is not exact equivalence, which is why `f32` and `f64` do not implement `EQ`.

There's no way for the compiler to guarantee the correct implementation of `EQ` so it's something you need to be mindful of.

Unlike `Partial EQ`, `EQ` is not a generic that can be used with other types (since we're talking about exact equivalence, this wouldn't make sense).

Earlier we chose to make that `User` type partially equivalent if the id matched.

If we instead checked the entire object, it could be considered to be exactly equivalent:

Of course, in this case, it'd be far easier _and safer_ to use the derived version, which protects us making mistakes in complex code, or forgetting to check changes we make in our type later

### PartialOrd / Ord

As you can imagine, `PartialOrd` and `Ord` have a similar relationship to each other as `Partial EQ` and `EQ`, and indeed:
- `PartialOrd` can only be applied to types with `Partial EQ`
- and `Ord` can only be applied to types with `EQ` (and `Partial Ord`)

Both `PartialOrd` and `Ord` have one Required method each (`partial comp` and `comp` respectively) as well as some Provided methods with default behaviour. 

The required methods of each trait use the `Ordering` enum which looks roughly like this:

`Partial EQ` is what gives us our usual greater than (`>`), less than (`<`), greater or equal to (`>=`)  and less than or equal to (`<=`) behaviour, through the use of the methods `gt`, `lt`, `ge` and `le` respectively,

Unless these methods are implemented, their default behaviour relies on `partial_cmp`, which returns an `Option` of `Ordering`.

Again, using floating point numbers, it's easy to see why we use an `Option` on our comparisons.

When comparing `NaN`, is it greater than, less than, or equal to `NaN`?

We can't determine the answer, so we use the `None` variant to represent that.

One important thing to bear in mind when deriving `Partial Ord` is that although, yes, you can do it if all parts of your type implement `Partial Ord`, there's a catch!

When derived on structs, it will first check the ordering of the first field, and only move on to the next field if the first fields are equal.

To understand why that might not give us the behaviour we expect, lets imagine a rectangle with width and height.

By deriving the behaviour for `Partial Ord`, Rust will first compare the width, and only if those are equal compare the height.

This means that if the width is larger in one rectangle, then according to the derived behaviour, it's the larger rectangle, even if the height of the other rectangle is significantly larger.

For this reason, it's quite likely that you'd want to implement `Partial Ord` yourself, depending on how you think types should be compared.

Instead of using the derived behaviour, lets create an area method for our rectangles, and use that to compare them to each other.

I think, in this case, this makes more sense.

Finally `Ord` isn't quite the same as `EQ` because it _does_ have methods:
- `comp` which is like `partial comp` but returns `Ordering` without the `Option`
- `max` which returns the greater of the two values
- `min` which returns the lesser
- and `clamp` which will return a value so long as its between two other values, or the closest value that is

Like with `Partial Ord`, `Ord` can be derived but has the same ordering quirk. 

If we want to implement it ourselves, we only need to implement `comp`, and the other methods can use that for their default behaviour.

However, you may still find some weird behaviour with `clamp` even if you do this so its anotther one you might want to override.

Importantly, when implementing both `Partial Ord` _and_ `Ord`, the result of `partial comp` _must_ match `comp`, though the compiler has no way of confirming this for you. 

The easiest way to handle this is you need to manually implement `Partial Ord` is to simply call `comp` and wrap it in an `Option`.

Let's update our Rectangle

Unlike `Partial EQ`, neither `Partial Ord` nor `Ord` are generic, they can only be implemented where both the left hand side and the right hand side are the same type.

### Clone (and Copy)

`Clone` is a bit like `Copy` in that it allows you to duplicate values, however, where `Copy` is implicitly very cheap, `Clone` can get away with doing a bit more work.

With `Copy`, we can make a copy of data on that is purely on the stack, however, this restricts us to `Sized` data. 

This means, for example, `String` which is a smart pointer to data on the heap, can not implement `Copy`. 

In order to duplicate `String` we'd need to request new memory on the Heap to place the data into, then copy the data to the new location, and create a new smart pointer on the stack to point to it.

Requesting heap memory is considered expensive as you have to wait for the operating system to provide you a location you can use, so it's really handy to differentiate `Clone` from `Copy`.

Luckily, you don't have to do all of this memory allocation stuff yourself. 

For any type that is built from other types that already implement `Clone` you can derive `Clone`.

If you need to implement `Clone` yourself (rare and only required in very specific and advanced circumstances), then you can.

In order to derive `Copy`, not only must your type be made from only other types that implement `Copy`, but your type must also implement `Clone`.

### Default

Many types could be considered to have an obvious default state

Defaults for numbers are typically zero, while `String`s and collections default to being empty.

If your type is built from only types that implement `Default` then you can derive `Default`.

The default behaviour is essentially the instantiation of your type with all values set to _their_ default.

Obviously, this may not always be the desired result, so you can obviously implement the trait directly:

You might be wondering if you can derive `Default` for Enums, or if you have to implement it directly, and you actually can, using an additional attribute that you apply to the value you want to be the default.

Unfortunately the `default` attribute only works when deriving `Default` for unit enums, which means if your enum contains nested types, you _will_ have to implement `Default` manually:

### Hash

Hashing is the process of taking a (usually) arbitrary amount of information and distilling it into a fixed size of data.

This is a one way process ✳️ (asterisk), but giving the same input will always give you the same output, and _that_ is pretty useful!

There are lots of different ways to hash that are suitable for lots of different purposes.

In Rust there is a trait that describes a type that is `Hash` which means that it can be "hashed", and another trait called `Hasher` which does the hashing.

These traits aren't for general hashing, though, in Rust they have a specific use, can you guess what it is?

The `Hash` trait only works with the `Hasher` trait and always results in a `u64`, this is rubbish for cryptography but great for generating numbers for lookup tables.

You _generally_ don't need to worry too much about either trait, but `Hash` is useful if you want your type to work as a key in a `Hash Map` or similar data structure.

So long as your type is constructed only of other types that implement `Hash`, then you can derive it, though if you need more control than that, then you can of course implement the trait methods yourself. 

This might be useful if you want to skip over some of the types that make up your compound type that can't be hashed _BUT_ when using `EQ`, if `A is equivalent to B`, then `the hash of A must be equivalent the hash of B`.

## Error Handling

Wait! Before we jump straight into the `Error` trait, lets recap on `Display`.

### Display

This trait allows us to display information related to the type that implements it. 

Once you implement it, if you pass a value of your type into a macro like `println!`, `eprintln!` or `format!`, then `Display` defines how the type will be rendered.

🦀 `Display` has single Required method which takes a reference to `self`, and a mutable pointer to a `Formatter` and it returns a `fmt::Result` which is a type alias for `Result<(), fmt::Error>`. 

🦀 The easiest way to implement it is with the `write!` macro which returns this same type, and to `use std::fmt` so that you can reference things in the module namespace rather than contaminating your own.

### Error

The `Error` trait is applied to types that are _specifically_ used to represent something that went wrong during the execution of code.

Although `Result`s do not _require_ the `Error` trait be implemented for types in their Error variant, it is definitely worth doing as error types with the `Error` trait provide a lot of utility.

The trait itself has several "provided" methods but no "required" methods. 

You're unlikely to want to alter the provided behaviour of the `Error` trait which means the only thing you need to do is make sure that your error type _also_ implements `Debug` and `Display`. 

As we know, `Debug` is usually derivable, so that just leaves `Display` (see why I wanted to bring it up again?). 

Let's create a custom Error for a fridge to demonstrate how we _might_ do this.

🦀 We'll use an enum to represent the error states which are either, 
- too warm, with its current temperature
- too cold, with its current temperature
- or an alert that the power has failed

🦀 We'll derive Debug, and we'll implement Display which will just produce a nice human-readable message.

🦀 Finally, we can implement Error which will provide all the methods we need.

While we've avoided talking about the wider ecosystem so far, it's worth mentioned there are some _extremely_ powerful Error libraries that might change the way you work with errors. 

We will cover these in the Ecosystem part of the book.

## Next time

Next time we're going to continue learning the top traits, but I particularly want you to keep Errors in mind as we're going to learn a very cool trick with them.

If this has been useful to you, don't forget to like and subscribe and I hope to see you... hey... what are you doing here, we'll see about this.

## Bonus

*Big grin*

Ok so I wanted to sneak in a little bonus trait that I discovered while deep diving during a re-write of this script.

I got away without knowing about this Trait for seven years so, so I don't think you need to know about it, but this trait is used in every single Rust program that you write even the pre-built Hello World template that cargo provides in a new binary project.

This thing had been hiding in plain sight from me that whole time, and it blew my mind when I found it.

If you've worked with other programming languages, you might be aware of "exit codes" or "exit status'"

These are effectively a return value from the program itself, usually returned from the main function when it ends, to let the operating system know if the program succeeded or not

Different operating systems use different exit codes so you need to be careful here but _usually_ exiting with a 0 means everything was ok, and anything else indicates the program in some way failed.

In Rust, we don't return numbers from main. 

Usually we don't return anything... right?

Well, actually, we do, we always do... kind of.

📕 There is a trait called `Termination`, and `main` must return something that implements it.

📕 `Termination` will provide an `Exit Code` via a "Required" report method, and this is how we get our programs success of failure status.  

You may have noticed though that we almost never returns something from `main`, _but_, in Rust, when we don't specify a return type, the return type is the Unit Type.

And of course there is an implementation of Termination for the Unit Type which will produce the success exit code, whatever that might be for your target operating system.

This means that, normally, so long as your code runs to completion, and you exit your program by reaching the end of the main function, your program will be considered successful.

You might have seen code examples (particularly hidden in documentation) where main returns a `Result`, usually where the Ok variant is a unit type.

📕 And this is because there's also an implementation for `Result`s where the Ok variant already implements `Termination` and the Error variant implements `Debug` which, as we just discussed, Errors usually will!

I have used both your usual no-return style main function, _and_ main functions that return a Result, and had thought that Rust was doing some magic here for exit codes, but never thought to look it up.

I didn't need to know this, arugably you don't either, but isn't it awesome when your curiosity leads you down a little path and you find something cool like this!

Ok, that's it for real this time, thanks for listening to me ramble about this useless bit of knowledge, I'll see you next time.
