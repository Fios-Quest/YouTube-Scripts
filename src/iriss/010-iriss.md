# Introduction to Traits

Last time I showed you how to add functionality to types by building a state machine for my cat Yuki.

We identified a couple of issues with the state machine though: 

First, we couldn't access anything on the Cat type from inside our Generic States.

Second, the behaviours didn't seem generally applicable. 

Would `Hangry<Human>` make loud noises and bite someone like a `Hangry<Cat>`? 

Usually not... usually.

Today we're going to solve these problems using Rust's incredible Trait system

As a note, we are going to be building on the code we developed last time so if you missed that, either follow the link appearing above me now,

or see the relevant chapter in the free book that accompanies this series, check the description for a link straight to that chapter.

Let's begin, my name is Daniel, welcome to IRISS.

---

Traits describe common behaviour between types that implement a given trait.

For example, have you noticed that lots of types have a method called `to_string()`, including numbers, string slices (`&str`) and even strings?

This is because there is a trait called `ToString` that describes the function header for a method called `to_string()` and all of these types implement that trait. ⚠️

This is what `ToString` looks like in the Rust standard library, if we remove the comments and annotations you can see how simple it is

Any type can implement this trait to provide the `to_string()` method.

---

We've previously shown how generics allow you to create a kind of template for a function or type where the specific types used within can be decided later.

When we used this with our emotional states, this wasn't actually very handy, as we couldn't make any assumptions about what was inside the state, it could be anything!

However, we can use what are called Trait Bounds to limit what types are allowed to slot into our generics.

For example here's a function called `say_hello` where we're using the generic `S` for the parameter.

If we limit what types `S` can be to only those that implement `ToString`, then we can now use any methods that that trait provides because we know only types that implement that trait can end up in this function.

For example, we could pass in all those types I mentioned before because they all implement `ToString`, however, `Vec` does not implement `ToString`, so this won't compile.

---

We can also implement `ToString` on our own types. ⚠️Huh, whats that.

Imagine we have a **ehm** poorly designed Person type with a first and last name.

Because this `Person` type doesn't yet implement `ToString`, it can't be passed to our function.

To implement `ToString` for our `Person`,  ⚠️ huh, there it is again. I wonder what it means. ... anyway 

To implement `ToString` for our `Person` it's very similar to our Implementation code from the last video but instead of Implementing `Person` we implement `ToString for Person`.

We can use the format macro here to simply return the first and last name with a space between as a new String.

And now we can use `Person` inside of `say_hello`.

---

As an aside, it's worth noting that in order to use methods associated with a trait, the trait must be in scope.

We don't have to do this in this case because `ToString` is part of the Rust prelude, a collection of types and traits that are always available in Rust.

Often when people create libraries they'll make their own prelude module that contains the most commonly used types and traits so that you can import the entire prelude module rather than having to import everything separately.

We'll talk more about this in the future when we start talking about Rust's wider ecosystem. 

`ToString` is one of many traits that are built into the Rust standard library, and we'll talk more about some of the other traits available to you the next video.

For now though, we're going to build our own!

## `Animal`s

The first problem we were having was that we can't access anything related to Cat inside our emotional states.

This is because our emotional states are generics that can take _any_ type inside of them.

What we need to do is limit what can go inside our emotional states in a way that enables us to understand the functionality of the inner types. 

To do this, we'll make an `Animal` trait to represent the common behaviours of any animal.

We'll also do a little reorganising while we're at it.

First lets create an animal module. 

---

In `main.rs` we'll add `mod animal` and then create a `mod.rs` file in an `animal` directory.

Let's move `cat.rs` to the `animal` directory too, so that it's a submodule of `animal`.

Finally, we need to remember to add `pub mod cat` to `animal/mod.rs` and to update our use statement in `main.rs` to `animal::cat::Cat`.

We're now ready to make our trait, which we'll put in `animal`s `mod.rs`

With trait methods, we don't _have_ to define any behaviour (though we can, and we may talk about that another time), we only need to tell Rust how the method will be used.

In this case we define a method called `get_name` which will take a reference to the data this is implemented for, and will return a string slice. 

We also don't need to specify that the method is public as Traits are Rust's equivalent of Interfaces, everything listed is assumed to be public.

Now we need to implement this for `Cat`.

---

In `cat.rs` we'll add the implementation.

Huh... but now we have _two_ methods for Cat called `get_name()`, one in `impl Cat`, and one in `impl Animal for Cat`.

That's actually ok, but, because they're both doing the same thing, this is indicative of a code smell.

What happens if we want to add more functionality to the getter? We'd have to remember to update both. 

It'd be better to call the underlying `Cat::get_name` from `Animal::get_name`, but `self.get_name()` is the function we're in... how do we call the other one?

Have you noticed that when calling methods with the dot syntax like `yuki.get_name()`, even though the methods first argument is `&self`, we don't actually pass anything in here, the self argument is skipped.

This is because when we call a method with the dot syntax, we call it on a specific instance, so Rust, like many similar languages, can infer the value of `self` to be the instance the method was called on.

But, we can also call the method directly and manually pass in the value of `self`.

For example, in the method `Animal::get_name` we could call the `Cat` method of the same name, manually passing in `self`.

This lets Rust know that it should call the `Cat` implementation of `get_name`.

Now the behaviour of `Animal::get_name` for `Cat` will always be the same as `Cat::get_name` even if we change the Cat version of the method in the future.

---

For each state (`Mischievous`, `Hangry`, `Eepy`), we can add a Trait Bound so that the generic `A` must be a type that has implemented the `Animal` trait.

Now that we know that whatever is in each state's `animal` field must implement the `Animal` trait, we can treat it as such in any implementation code for those states. 

There's a bit of a trick here though, when we add the trait bound to the implementation block, it actually goes on the implementation's generic listing, as the other generics are references back to this one.

We're saying that for this implementation, `A` must have this trait bound, and this is the same `A` that will be used for the structs generic.

Let's update all the other states.

---

So that's our first problem solved! We can now access the `Cat`'s data through the `Animal` trait.

## Making more flexible `Animal`s

Now that we can read details from the underlying `Cat` object, lets start to think about how we can expand this functionality out to other types of animals... starting with the most dangerous of animal.

We can start by adding `pub mod human;` to `animal/mod.rs`. Now we can add our Human type in here, we'll keep it pretty simple for now.

Finally, we'll `implement Animal for Human`

Once we update our main function, we can run the program to make sure everything's working.

Notice that we barely had to change anything to add humans to our code, how cool is that!

But there's still an issue... my mischievous state doesn't tend to have me breaking into wardrobes by pulling on exposed clothing... I have opposable thumbs.

In fact, when I'm in a mischievous mood, I probably don't even behave the same as other humans, you and I probably don't behave the same.

## Optional Homework

Can you change the code so that each states behaviours are defined when the structs are instantiated? 

To do this you will need to:
- modify the `Human` and `Cat` structs to store the behaviours
- add new methods to the `Animal` trait to get the behaviours
- and then implement those methods for each struct

If you get stuck, I've implemented the code in this videos chapter of the IRISS book, just scroll to the blank code block at the bottom and hit the eye icon to reveal it.

Do note though that a limitation of the book means all the code is in one place, you should split your modules into files so that it's easier to manage and work with.

And just to show that I did actually check the homework isn't too hard this time, here it is running *wink*

## Next Time

Next time we'll continue to explore Traits by looking at some of the more commonly used ones available in the Rust standard library.

This will also allow us to cover some Trait features we haven't seen so far, including associated types!

If that sounds interesting to you, don't forget to like and subscribe.

And if I don't see you there, I'll see you next time. 👋🏻

---

Wait! I lied.

Remember when I told you that all of these built in types implement `ToString`... they... they don't.

Well, not directly anyway, and you shouldn't either.

There is another, more flexible Trait called `Display` that allows you to write string data to an arbitrary formatter which can then output that data to somewhere else, whether that's a newly allocated location in heap memory, an I/O stream or something else. 

Numbers, string slices and even Strings implement `Display` but _not_ `ToString`.

But we used `ToString` as our trait bound, how did our function accept those other types if they don't implement `ToString`?

Well, when you implement a trait, you can implement it for a generic, and then use a trait bound to make sure that generic already implements another trait.

In Rusts standard library, there is a generic implementation for `ToString` for type `T` where `T` already implements Display.

We'll cover this technique more in later videos, but for now your takeaway should be, don't implement `ToString`, implement `Display`.

While it looks more intimidating, you can generally pass the formatter to the `write!` macro and the rest of the macro works the same as the format macro but returns a Result instead of a String.

Here's how we can change our `ToString` implementation for `Person` to use `Display` instead.

Anything that has Display automatically has ToString, so we can still use our Person in our `say_hello` function

Ok, now I'll see you next time.

