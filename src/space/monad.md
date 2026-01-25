Monads
======

### intro 1

A quick intro, I'm going to explain Monads, but don't panic.

After this sentence we won't use the words monads, monodic, monoid, functor, endofunctor or even category.

### intro 2

That's not because these words aren't important, or learning about them isn't interesting and enriching...

... it's because you don't need to know these words to understand M... this pattern!

I think the terminology can be off-putting which is a deep shame because this pattern is *mwah* chefs kiss.

### intro 3

When programming, we regularly take a value and perform some operation to get a new value.

In fact its pretty much all we do, right?

### intro 4

Let's say we have the value `5` which is an integer.

We could square the value which gives us `25` which is still an integer...

or we could turn the value into a string which is still `"5"` but is now a different type.

### intro 5

If we turn these operations into function like this...

we can think about these functions as taking something of type T and returning something of type U (even if T and U are
the same type).

### intro 6

I'm going to briefly swap to using TypeScript to explain this next bit because Rust simply won't let me write code this
way, but let's say we have a function that returns either a number or nothing represented by "null".

The function isn't important, it just represents that some functions, by their nature, may need to return different
types depending on what happened when they were called.

### intro 7

But our functions (reproduced in typescript here) only take numbers.

This means we can't just pass a variable that might be null into these functions.

### intro 8

We need to check if the value is null and only pass it in if it's not (this is a process called "narrowing").

We can do this with branching, TypeScript is smart enough to know that it's impossible for the value to be
null in this branch so this works...

### intro 9

but it obviously reads terribly, and adds a lot of boilerplate if we have to do this often.

The next logical step is, if we're just checking for null, we could abstract this to a generic function, which in
typescript would look like this.

### intro 10

Here we're passing our function that takes T and returns U into another function that takes T or null, and returns U or
null.

If the input isn't null, we run the passed function, otherwise we return null.

### intro 11

Now we can reuse this code a lot more easily but there's still a lot of to-ing and fro-ing.

Also, none of this works in Rust.

### intro 12

There's another logical step we can take though, one that lets us get back to Rust.

One that's also a little more... "fluent" if you will.

Mmmmmmo.... Containers
----------------------

### containers 1

At the moment we're dealing with a value that could be type T, or it could be the concept of nothing.

These are two separate types.

### containers 2

What if we wrapped these types in a container that could hold either T or nothing.

We'll call our container `M` for Mon... reasons.

### containers 3

So we have our container, `M<T or nothing>`, and our functions f(T) returning U

We can give our container a way to take that function and return a new container, `M<U or nothing>`.

### containers 4

Rust actually has a built-in type that already does this, but to explain the pattern we're going to reproduce it.

We'll start by creating an Enum called Maybe because maybe it contains a value, or maybe it contains nothing.

### containers 5

We'll then give it a method that will map from `Maybe<T>` to `Maybe<U>` using the provided function, `f(T) -> U`.

This method is functionally identical to our if_not_null function we wrote in TypeScript but operates directly on an
owned version of the Maybe.

### containers 6

Now when we call our Rust equivalent function to divide two numbers, we'll get a `Maybe<i64>` back.

We can map the `Maybe<i64>` using square.

### containers 7

Because square takes an i64 and returns an i64, using it on `Maybe<i64>` returns another `Maybe<i64>`.

When we use to_string which takes an i64 and returns a String, our `Maybe<i64>` turns into a `Maybe<String>`.

### containers 8

Finally, we can print this to the screen by examining the Maybe directly.

That's all well and good, but it seems like this hasn't saved us much over what we were doing with if_not_null in
TypeScript.

### containers 9

Where this pattern shines is that in TypeScript we were dealing with a value that could have two types.

By encapsulating it in a Mmmmm... Container, we're only dealing with one type, which means we've built ourselves a
fluent interface for managing our data.

### containers 10

This means we can inline all of our maps, and it becomes far more readable than if we'd inlined our function calls.

For what its worth, this pattern works perfectly fine in TypeScript too.

Variations
----------

### variations 1

By now, you've probably worked out that our Maybe type is us creating a less fantastic Rust Option.

Our containers can be more flexible than just T or nothing though.

### variations 2

It's perfectly reasonable for your container to potentially hold multiple things.

Let's say we have a container that could hold types T or V.

### variations 3

This container should not only be able to take `f(T) -> U` which will give us a container potentially holding U or V

but it should also be able to take `f(V) -> W` which will give us a container potentially holding T or W.

### variations 4

A good example of this in Rust is the Result enum which holds an Ok variant of T and an Error variant of E

However, we can't just hand over `f(T) -> U` or `f(V) -> W` and hope for the best because what if both variants hold the
same type.

### variations 5

It's not unheard of for both variants of a Result to be the same type... either a unit type or a string

I mean, it's not ideal, but it's not impossible.

### variations 6

Rust's result type has separate methods for changing the Ok variant or the Error variant, the former still being "map"
and the later being "map_err".

Rules / Guidelines
------------------

### rules 1

I'm sure I'll have made some hackles rise in my description of this pattern, so I do want to explain there are some
specific rules that make this pattern what it is, it's not just any old container.

### rules 2

Firstly, for a given container, a value representable as a type inside that container, should be convertable to that
container without changing its value.

In our Maybe example (as well as in Option and Result), we simply pass the value to one of the enum variants, so the
inner value doesn't change.

### rules 3

Second, there needs to be a way to apply a function to the inner types such that if you apply `f(T) -> U`  to a
container potentially holding a value of T then you get a container potentially holding a value of U.

### rules 4

In our Maybe, applying `f(T) -> U` to `Maybe<T>` returns `Maybe<U>` regardless of whether it contained a value of T or
not.

### rules 5

In some languages the rules are a little stricter and the pattern works very slightly differently, but there are plenty
of videos that can explain the specifics for those languages in more detail.

### rules 6

I just wanted to explain the basic pattern in an easy to digest way because I think it's an extremely cool pattern, one
that's surprisingly crucial to how Rust works.

Summary
-------

### summary 1

This pattern can significantly reduce complexity, removing a lot of cognitive overhead when reading and writing code.

Rust is filled with this pattern, and in fact wouldn't work as a language without it, we just don't use that name
because it feels overly complex.

### summary 2

I hope I've aptly explained the pattern without all the scary words, but if you do want to learn more about this pattern
warts and all, there's a lot of great YouTube videos out there, I'll recommend some in the description of this one.

### summary 3

If you enjoyed this video, don't forget to like and subscribe.

If you really liked the video, you can become a member of the channel or join the Patreon, see the description for more.

### summary 4

Next time we're going to talk about dependency injection, which is crucial to building complex yet maintainable
software.

I hope I'll see you then.
