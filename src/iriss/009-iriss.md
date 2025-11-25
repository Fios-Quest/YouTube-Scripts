# Implementing Types (if held)

Today we're going to learn how to add functionality to data by modelling a common emotional pattern followed by my cat.

As ever, this series is accompanied by a free book, check the description for a link straight to this chapter.

My name is Daniel, his name is Yuki, welcome to IRISS.

# Implementing Types (if alone)

Today we're going to learn how to add functionality to data by modelling a common emotional pattern followed by my cat Yuki.

As ever, this series is accompanied by a free book, check the description for a link straight to this chapter.

My name is Daniel, welcome to IRISS.

---

We'll model 3 emotional states of my Cat, describe behaviours unique to each state, and allow him to transition between
those states.

So what is the behaviour we want to model in these states?

## Yuki States

We'll initialise Yuki in the Mischievous state because... that's how he wakes up.

In this state he makes lots of fun little noises
He'll also try to get up to various naughtiness
But, if we forget to feed him he'll get Hangry

Once hangry, he only really makes one noise, a desperate plea to "pay attention and do your job"
He'll get hyper focused on getting your attention and may choose violence
Once he's fed though, he'll get Eepy

After eating, he'll get sleepy, he won't make any noises he'll just settle down in one of his beds
Once he's slept, he'll get right back to his mischievous state

## Making a Cat

Let's create a new project with `cargo new yuki-state-machine` and open it in our IDE of choice.

We're going to create a separate cat module.

In VSCode put the cursor on cat and open the context menu.

We can now create a cat.rs file from here.

Inside our `cat.rs` file lets create a structure to hold a cat, we'll make it public by putting the word `pub` in front of the struct.

Back in our `main.rs` lets try to create our Cat, we'll use the use keyword to simplify things and...

There's a problem...

`Cat` might be public but `name` is not... so we can't instantiate our object.

We could, of course, simple make `name` public, but this means anything can access it at any time and, if your `cat` is mutable, the name can be changed. 

We don't want anything renaming Yuki, so, we'll manage the property privately.

To create the object then, we'll need a "constructor", a function that is associated with our `Cat` type, that returns an instantiated object. 

To do this we need to use an `impl` block.

## impl

In Rust, code that is associated with specific types is called "implementation" code, and lives inside an `impl` block.

`impl` blocks are incredibly flexible, and we'll see more of this in the next video, for now though, we're just going to cover the basics.

We "implement" code for a type with "impl" and the type name, followed by a block that contains the code for that type.

Functions inside the block are called "methods", and there are two types normal "methods" and "static methods".

Normal methods apply to the data, in our case this is Yuki, and static methods are part of the type, which here is Cat.

To instantiate our `Cat` type, we'll make a static method that instantiates a new Cat for us.

---

We put our constructor inside the impl block, it'll take the name of the cat as a String, and return the instantiated Cat object.

Before we move on there are some nice shortcuts we can use here, first, when we assign name to name, Rust lets you shortcut this to just "name".

Second, inside "impl" blocks, there is a special type called "Self", with a capital S. This is an alias for the Type that is being implemented.

We can use this in place of Cat... for now this might seem pointless, and arguably it is, but as our types get more complicated, this will simplify our code... a lot.

---

Before we move on, lets make our Cat a little bit more useful by creating a "getter" method for its name, this way we can access the name while still preventing modification.

Normal methods, the kind that will be available on our Yuki object, have a first parameter that is one of, `self`, a reference to `self`, or a mutable reference to `self`.

This is another one of those magic shortcuts:
self is short for self Self
ampersand self is short for self colon ampersand Self
ampersand mut self is short for self colon ampersand mut Self

For our getter we don't need to own or modify our object, we just need a reference to it, we can then return a reference to the string slice inside our cats name.

Because we're only taking one reference in, and returning one reference, those references will automatically have their lifetimes tied together internally, so we don't need to think about it.

---

Finally, we can make our program work, lets return to `main`, and use our new implementation:

Great, lets move on to Yuki's state!

## State Machines and Rust

We want to represent three of Yuki's many "states", but in particular, we want to move between them in a very specific way.

A naive approach to this could use an `enum` to represent the states, and then we could have methods that change the state.

Here you can see the methods take a mutable reference to self so that we can modify the state.

But let's look again at the diagram:

A Hangry cat doesn't become Mischievous because it slept. What should happen if we try to call `sleep` on a Hangry cat?
Should it do nothing?
Should it throw an error?
Does it even make sense for this to be an option?

Furthermore, if we look back at the specification, a Hangry cat may choose violence, but Eepy cats and Mischievous cats won't.

Instead, we could use a pattern called a State Machine.

In a State Machine we transition fully between different states, and those states have different functionality.

---

Let's make our states unit structs instead of an enum

We can then apply the functionality to each struct in turn:

This is _much_ more ergonomic as `Hangry` simply does not have the method `sleep`, meaning you get compile time checking that your cat states are being used correctly.

---

At this point, we don't have any data about our cat though.
There are a few ways we could deal with this.

We could put our cat data into the states like this

Now when we change state, we can call the constructor of the new state and pass our data across.

Also, note that the state change functions take ownership of the old state
this both saves us an expensive-ish memory allocation as we can move the data instead of copying it
and, it also prevents old states hanging around, making it unclear what the valid state is.

But this is going to get rather heavy if our Cat has a lot of details.

It's also not very flexible.

What if we put our Cat type inside our States.

---

You can see how this makes it easier to manage the Cat and states more easily,

But Daniel, I imagine hearing you say, that isn't actually any more flexible... and if you are saying that, wherever you are, then you're right!

So now lets swap our Cat out with Generics!

## Generics

Generics exist in many languages and are a way of creating templates that only become "concrete" at build time.

Functions, Structs and Enums can all be made generic by adding triangle brackets after their name, containing a list of generic parameters.

Very over you might see a single letter generic name, particularly you might see something like `T` as you do with `Option<T>`, however, you might want to hint about the purposed of the type like the Error `E` in `Result<T, E>`.

You can even use whole words, though this is seen less.

For our purposes, we'll use the letter `A` to represent "Animal".

Later we can fill in the Generic and make it a "concrete" type, in our case this will eventually be `Mischievous<Cat>`.

---

Next we'll need to update our implementations. 

Implementations that involve Generic parts, need to list those after the `impl` to save confusing implementations on generic types and implementations on concrete variants of generic types

This just means you can implement the concrete version of `Mischievous<Cat>` and that's not a generic implementation, but `impl<A> Michievous<A>` is. 

Once we've specified the generic parameters of the impl block, we don't need to respecify them for each function so long as the type is being used consistently.

So, our `new` functions can use `A` for the animal parameter, and that will be linked to our state's generic `A`, and our state changes can use `A` to show that when the state changes, the generic type will remain the same.

I'm also going to add a quick extra function, `describe`, to each implementation for use at the end.

---

Finally, lets update our Cat implementation to return a concrete `Mischievous<Cat>` type

---

We can now play with all of this in our main function:

## Tidying up

Before we finish the video, our code has gotten a bit messy, we should clean it up.

Now that our states are not Cat specific, they should be moved out of `cat.rs`.

Let's create a new module for states, and some sub modules for each state.

We need to `use` each state in the appropriate file being used, and you need to make your individual state modules public in your general state `mod.rs`.

And there we have it, a neat little project! (no tests or documentation admittedly)!

## Next Chapter

This is cool... but we've got a slight problem.

We can't get Yuki's name from the state, and really his actions should belong to him, not be part of the state.

Personally, I _try_ not to choose violence when I'm hangry, so I wouldn't be comfortable putting myself in this state.

We should move some of this behaviour to the Cat, but how can we access details on our Cat type, when we don't know that our Generic states will contain Cat's until runtime?

In the next chapter we'll discuss Traits which provide a way to resolve this, as well as some other cool things they can do with Generics!

If that sounds interesting to you, don't forget to like and subscribe.

Finally, there's now a Patreon and more importantly a Discord server, so check the description and come join our growing community!

And if I don't see you there, I'll see you next time. 👋🏻