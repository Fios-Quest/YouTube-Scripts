Higher Order Function
=====================

Intro
-----

### intro 1

Higher Order Functions are functions that either take one or more _other_ functions as parameters or return another
function when called themselves.

Easy right?

But... what's a function?

### intro 2

On Fio's Quest we usually talk about data and types, but functions aren't data, they're just a set of instructions, so
how can we pass them around?

And while types can be involved in defining functions, are functions themselves typed?

Kinda...

Function Pointers
-----------------

### pointers 1

Functions aren't data, but they do exist as a set of instructions in memory when your program runs

And if it's in memory, that means it has an address, and if it has an address we can create a pointer to it.

### pointers 2

Fun fact, unlike raw pointers, function pointers are safe in Rust.

But in Rust pointers are typed, so what does a function pointer type look like.

### pointers 3

Functions have signatures.

In this function, we have two parameters, a string slice and a u8.

The return parameter is a String.

### pointers 4

The "type" of a function pointer is actually just this information written like this.

This is basically the function signature written without any names.

### pointers 5

We can set a variable of that type to point at our function...

Though, we wouldn't normally do this inline because Rust can infer the type of the function pointer.

### pointers 6

Where this is actually useful is when defining another function that takes a function this shape as a parameter.

Using the `RepeatFunction` type alias here makes the function header a lot easier to read, and, if we need to, allows us
to reuse the alias in multiple places.

### pointers 7

Referencing a function using a function pointer can be especially useful for things like filters, maps, or folds.

In this example we have a range of numbers 1 to 10.

We use regular functions to filter out odd numbers, map each number to a string, and then combine what is now an 
iterator of strings into a single string. 

### pointers 8

It's worth noting though that `is_even` has the quirk of needing to explicitly take a reference to a usize because
filter gives us a reference to T even if T is copy.

### pointers 9

We can also return function pointers from other functions.

Here we have two functions that take a reference to a usize and return a bool...

a type alias that matches those function signatures...

and a function that returns a pointer to one of the other two functions when called

### pointers 10

We can then call `create_even_filter` to get a pointer to one of those two functions. 

But I think at this point we have to stop and wonder... how useful is this really?

### pointers 11

We can defer the choice of what code to execute until runtime, but you can literally do that with if statements and
other branches.

Using function pointers could be a little bit tidier, depending on the circumstance, but honestly, most of the time you
probably won't use them in higher order functions all that often.

### pointers 12

Higher Order Functions are more useful, and frankly more interesting, when you can configure the functions you send them
at runtime...

...but you can't do that with function pointers.

Closures
--------

### closures 1

Functions are simply a set of instructions.

If we want to configure how they behave later we need to apply a little sprinkle of data...

### closures 2

We can provide functions with a scope that contains some data, and then pass that bundle of instructions and data
around.

This is where closure's come in.

### closures 3

Rust has three types of closure, well, six technically, we'll talk about the beardo weridos from the mirrorverse later.

I'll give you a few seconds to think about why Rust, specifically, needs 3 closure types when you bring stored data into
the mix.

### closures 4

Let's take a look at a very simple example.

We'll create a string, then we'll create a closure.

Closures look a little different from functions.

### closures 5

They aren't named, but they can be stored in variables.

They use pipes to surround parameters, and are followed by an expression, that can be a block expression, which is run
when the closure is called.

### closures 6

For clarity, we're going to go through three closure examples and each one is going to store the returned data in a
variable called response before returning it

This isn't necessary, its more verbose than we'd really need, I just wanted to make this consistent and clear

### closures 7

This closure references the greeting...

The reference to that String will be bundled into the closure.

Because of this, behind the scenes, Rust will make sure that the closure doesn't outlive the thing its referencing.

### closures 8

When we run the closure, it gives us back a string using the reference to our greeting and the provided name.

Running it again with a different name gives us a different string.

Greeting is still available and hasn't changed.

### closures 9

We can also create closures that modify the data they're given.

In this version, we need to make sure both the data we're referencing, greeting, and the closure itself are marked as mutable.

This may initially seem odd, but there are two things to bear in mind.

### closures 10

First, conceptually, calling the closure multiple times will produce different results as if we're modifying the closure
itself.

Second, technically, we are modifying the closure itself...

the closure is storing a mutable reference to some data so just like almost any other data type that wants to let you 
mutate something its holding, it too must be marked as mutable.

### closures 11

This closure will capture a mutable reference to greeting.

We didn't need to tell it to do that specifically, Rust just knows that the push_str method on Strings requires a
mutable reference so that's what it stores.

Again, it'll work out if we're using this safely without us needing to do much at all.

### closures 12

The first time we call this closure, we'll get the same result, but the second time we call the closure, we get a
difference.

This is because we mutated the greeting String itself, which we can see if we inspect it.

### closures 13

Finally, we can actually just give the closure ownership of the data being used.

In this example, I'm using the Add trait to combine a string with string slice references.

### closures 14

For types that aren't copy (and String is not Copy), Add consumes the data, meaning after we call this closure we can
never call it again.

Luckily Rust knows this and will prevent us compiling code that attempts to do so.

If we uncomment either of these lines we get a "value borrowed after move" error.

### closures 15

Earlier I mentioned that Rust has three basic types of closure, did you spot them?

Just like anything to do with Data, Rust has three ways to think about it; immutably referenced, mutably referenced and
owned.

### closures 16

In reverse order, our final function took ownership of the data and therefore could only be run a single time.

This type of closure is known by the trait `FnOnce`.

### closures 17

Our middle closure captured a mutable reference to the data, and therefore could be run as many times as we liked.

This type of closure is known by the trait `FnMut`.

Anything that implements `FnMut` also implements `FnOnce`.

### closures 18

Our first example only captured an immutable reference.

This type of closure is known by the trait `Fn`.

Anything that implements `Fn` also implements `FnMut` and therefore `FnOnce`.

### closures 19

And if you're wondering, function pointers, which do not capture any state, implement all three traits...

_and_ closures that don't capture any state can be treated like function pointers.

This is why we could use function pointers in filter, map and fold earlier which accept anything of type `FnMut`

### closures 20

If we go back to that example, we could write these closures in place.

If your closure is very simple, like these, this may be preferable to writing a named function somewhere

### closures 21

Now, why was I just drawing a load of weird circles?

This is my mental model for working out when to use which closure, not just for writing closures but for consuming them.

### closures 22

You can imagine these types being blocks that fit into slots.

When you're writing something that consumes the closure, you want to use the largest possible slot to allow the most
number of blocks to fit.

### closures 23

If you're only going to run the closure once, you can accept `FnOnce` and you'll be able to take any kind of closure or
a function pointer.

Often you will need to run a closure multiple times though, in which case `FnMut` is the next best option, but you
won't be able to accept `FnOnce` closures.

### closures 24

If you're writing a closure that needs to be passed to something else, then using the smallest possible block you can get
away with will make your closure more portable.

If it doesn't need state then a function pointer or stateless closure will give you the most flexibility.

Async Closures
--------------

### async 1

So, those beirdo weirdo mirrorverse closures...

Async closures are relatively new and so you'll see a mixture of ways to encapsulate futures in closures.

### async 2

More recently AsyncFn, AsyncMut and AsyncOnce were stabalised, allowing you to write async in front of a closure.

Because this is a relatively new feature, in older code you'll likely see closures with async blocks, which feels
backwards, but makes sense.

### async 3

The async keyword, whether applied to a function, code block, or now, a closure, is syntactic sugar that allows two
things.

First of all, the ability to defer a future with await...

and second, and more importantly to what's happening here, it says "this thing produces a Future".

### async 4

If you create a standard closure, and immediately return an async block, then you're returning future from the closure
too.

So in a way, Rust has had async closures for a while... but in a way, it also hasn't.

### async 5

Using this older method causes all sorts of problems with lifetimes.

So actually, you'll almost always see an explicit move with closures that return an async block making the closure an
`FnOnce` that happens to return a Future.

### async 6

One day Rust may introduce even more closure types.

One that I'm personally excited by is that Rust contributors have been thinking how to add generator functions which
implies both generator closures and async generator closures.

Summary
-------

### summary 1

Closures and higher order functions are conceptually complex, but as we start writing them, they're really clean and
incredibly powerful.

In Rust specifically the lifetime juggle can make things a bit more complex, and can be a bit of a headache to work
around.

Been there, believe me.

### summary 2

However, we can see that the amazing people contributing to the Rust language continue to improve usability and
ergonomics.

### summary 3

Just as a reminder, if you enjoyed this video, don't forget hit the like button and letting people know about the
channel is the best way to support it.

### summary 4

Next time, now that we've seen how functions can be passed to other functions, we're going to get a little wild.

Our next video is going to explain... Monads... don't worry though.

### summary 5

The pattern itself is actually super cool and much easier than it sounds.

I'm going to explain the pattern while avoiding the complex language surrounding it.

### summary 6

This will undoubtedly frustrate the purists but its such an awesome pattern its worth discussing anyway!

Hope to see you then.
