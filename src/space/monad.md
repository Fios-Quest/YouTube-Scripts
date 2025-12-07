Monads
======

### intro 1

A quick intro, I'm going to explain Monads, but don't panic.

After this sentence we won't use the words monads, monodic, monoid, functor, endofunctor or even category.

That's not because these words aren't important, or learning about them isn't interesting and enriching...

... it's because you don't need to know these words to understand M... this pattern!

I think the terminology can be off-putting which is a deep shame because this pattern is *mwah* chefs kiss. 

### intro 2

When programming, we regularly take a value and perform some operation to get a new value.

In fact its pretty much all we do, right?

Let's say we have the value `5` which is an integer. 

We could square the value which gives us `25` which is still an integer...

or we could turn the value into a string which is still `"5"` but is now a different type.

```rust
# fn main () {
let twenty_five = 5 * 5;
let five = 5.to_string();

assert_eq!(twenty_five, 25);
assert_eq!(five, "5");
}
```

If we turn these operations into function like this...

we can think about these functions as taking something of type T and returning something of type U (even if T and U are
the same type).

```rust
// Don't worry that there are already built in ways to do this, 
// its for explanation only
fn square(input: i32) -> i32 {
    input * input
}

fn to_string(input: i32) -> String {
    format!("{input}")
}
# 
# fn main() {
#     assert_eq!(square(5), 25);
#     assert_eq!(to_string(5), "5".to_string());
# }
```

This only works for things that have one type though. 

I'm going to briefly swap to using TypeScript to explain this next bit because Rust simply won't let me write code this
way, but let's say we have a function that returns either a number or nothing represented by "null".

```typescript
function get_even_time(): number | null {
  const time = Date.now() / 1000;
  return time % 2 === 0 ? time : null;
}
```

The function isn't important, it just represents that some functions, by their nature, may need to return different
types depending on what happened when they were called.

But our functions (reproduced in typescript here) only take numbers.

This means we can't just pass a variable that might be null into these functions.

```typescript
function get_even_time(): number | null {
  const time = Date.now() / 1000;
  return time % 2 === 0 ? time : null;
}

function square(input: number): number {
  return input * input;
}

function to_string(input: number): string {
  return `${input}`;
}

const maybe_time = get_even_time();
// This won't work!
// const squared_time = square(maybe_time);

let maybe_squared_time = null;
if (maybe_time !== null) {
  maybe_squared_time = square(maybe_time);
}
let maybe_squared_string = null;
if (maybe_squared_time !== null) {
  maybe_squared_string = to_string(maybe_squared_time);
}

console.log(maybe_squared_string)

function if_not_null<T, U>(input: T | null, f: (_: T) => U): U | null {
  if (input !== null) {
    return f(input);
  }
  return null;
}

const maybe_squared_time_f = if_not_null(maybe_time, square);
const maybe_squared_time_string_f = if_not_null(maybe_squared_time_f, to_string);

console.log(maybe_squared_time_string_f);

console.log(maybe_squared_time == maybe_squared_time_f);
console.log(maybe_squared_string == maybe_squared_time_string_f);
```

We need to check if the value is null and only pass it in if it's not (this is a process called "narrowing").

We can do this with branching, typescript is smart enough to know that the "if" makes it impossible for the value to be
null in this branch so this works...

but it obviously reads terribly, and adds a lot of boilerplate if we have to do this a lot.

The next logical step is, if we're just checking for null, we could abstract this to a generic function, which in
typescript would look like this.

Here we're passing our function that takes T and returns U into another function that takes T or null, and returns U or
null.

If the input isn't null, we run the passed function, otherwise we return null.

Now we can reuse this code a lot more easily but there's still a lot of to-ing and fro-ing.

Also, none of this works in Rust.

There's another logical step we can take though, one that also lets us get back to Rust.

One that's a little more "fluent" if you will.

Mmmmmmo.... Containers
----------------------

At the moment we're dealing with a value that could be type T, or it could be the concept of nothing.

There are two separate types.

What if we wrapped these types in a container that could hold either T or nothing.

We'll call our container `M` for Mmmm reasons.

So we have our container, `M<T or nothing>`, and our functions f(T) returning U

We can give our container a way to take that function and return a new container, M<U or nothing>.

Rust actually has a built-in type that already does this, but to explain the pattern we're going to reproduce it.

```rust
enum Maybe<T> {
    Value(T),
    Nothing,
}

impl<T> Maybe<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Value(inner) => Maybe::Value(f(inner)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

fn square(input: i64) -> i64 {
    input * input
}

fn to_string(input: i64) -> String {
    format!("{input}")
}

fn get_even_time() -> Maybe<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};
  
    let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards!")
            .as_secs() as i64;
  
    match time % 2 {
        0 => Maybe::Value(time),
        _ => Maybe::Nothing,
    }
}

fn main() {
    let maybe_time = get_even_time();
    let maybe_squared_time = maybe_time.map(square);
    let maybe_squared_time_string = maybe_squared_time.map(to_string);
  
    match maybe_squared_time_string {
        Maybe::Value(s) => println!("{s}"),
        Maybe::Nothing => println!("Nothing to show"),
    }
    
    // ---
  
    let maybe_squared_time_string = get_even_time().map(square).map(to_string);
    
    match maybe_squared_time_string {
        Maybe::Value(s) => println!("{s}"),
        Maybe::Nothing => println!("Nothing to show"),
    }
}
```

We'll start by creating an Enum called Maybe because maybe it contains a value, or maybe it contains nothing.

We'll then give it a method that will map from Maybe<T> to Maybe<U> using the provided function, f(T) -> U.

This function is functionally identical to our if_not_null function we wrote in TypeScript but operates directly on an
owned version of the Maybe.

Now when we call our Rust equivalent function to get even time, we'll get a Maybe<i64> back.

We can map the Maybe<i64> using square.

Because square takes an i64 and returns an i64, using it on Maybe<i64> returns another Maybe<i64>.

When we use to_string which takes an i64 and returns a String, our Maybe<i64> turns into a Maybe<String>.

Finally, we can print this to the screen by examining the Maybe directly.

That's all well and good, but hasn't saved us much over what we were doing with if_not_null in TypeScript.

Where this pattern shines is that in TypeScript we were dealing with a value that could have two types.

By encapsulating it in a Mmmmm... Container, we're only dealing with one type, which means we've built ourselves a
fluent interface for managing our data.

This means we can inline all of our maps, and it becomes far more readable than if we'd inlined our function calls.

For what its worth, this pattern works perfectly fine in TypeScript too.

```typescript
class Maybe<T: NonNullable> {
    inner: T | null,
    
    construct(inner: T | null) {
        this.inner = inner;
    }
    
    map<T, U>(f: (_: T) => U): Maybe<U> {
        if (this.inner !== null) {
            return new Maybe(null);
        }
        return new Maybe(f(this.inner));
    }
}

function get_even_time(): Maybe<number> {
  const time = Date.now() / 1000;
  return new Maybe(time % 2 === 0 ? time : null);
}

function square(input: number): number {
  return input * input;
}

function to_string(input: number): string {
  return `${input}`;
}

const maybe_squared_time_string = get_even_time().map(square).map(to_string);

console.log(maybe_squared_time_string);
```

Variations
----------

By now, you've probably worked out that our Maybe type is us creating a less fantastic Rust Option.

Our containers can be more flexible than just T or nothing though.

It's perfectly reasonable for your container to potentially hold multiple things.

Let's say we have a container that could hold types T or V. 

This container should not only be able to take `f(T) -> U` which will give us a container potentially holding U or V,
but it should also be able to take `f(V) -> W` which will give us a container potentially holding T or W.

A good example of this in Rust is the Result enum which holds an Ok variant of T and an Error variant of E

However, we can't just hand over `f(T) -> U` or `f(V) -> W` and hope for the best because what if both variants hold the
same type.

It's not unheard of for both types to be either a unit type or a string, I mean, it's not ideal, but it's not
impossible.

Rust's result type has separate methods for changing the Ok variant or the Error variant, the former still being "map"
and the later being "map_err".

Rules / Guidelines
------------------

I'm sure I'll have made some hackles rise in my description of this pattern, so I do want to explain there are some
specific rules that make this pattern what it is, it's not just any old container.

Firstly, for a given container, a value representable as a type inside that container, should be convertable to that
container without changing its value. 

In our Maybe example (as well as in Option and Result), we simply pass the value to one of the enum variants, so the
inner value doesn't change.

Second, there needs to be a way to apply a function to the inner types such that if you apply `f(T) -> U`  to a
container potentially holding a value of T then you get a container potentially holding a value of U.

In our Maybe, applying `f(T) -> U` to Maybe<T> returns Maybe<U> regardless of whether it contained a value of T or not.  

In some languages the rules are a little stricter and the pattern works very slightly differently, but there are
plenty of videos that can explain this in more detail.

I just wanted to explain the basic pattern in an easy to digest way because I think it's an extremely cool pattern, one
that's surprisingly crucial to how Rust works.

Summary
-------

This pattern can significantly reduce complexity, removing a lot of cognitive overhead when reading and writing code.

Rust is filled with this pattern, and in fact wouldn't work as a language without it, we just don't use that name
because it feels overly complex.

I hope I've aptly explained the pattern without all the scary words, but if you do want to learn more about this pattern
warts and all, there's a lot of great YouTube videos out there, I'll recommend some in the description of this one.

---

If you enjoyed this video, don't forget to like and subscribe.

If you really liked the video, you can become a member of the channel or join the Patreon, see the description for more.

Next time we're going to talk about dependency injection, which is crucial to building complex yet maintainable 
software.

I hope I'll see you then.

