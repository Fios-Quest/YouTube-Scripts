Not Why You Think
=================

Why should you try Rust. Or maybe you already use Rust and want to encourage other people to try it out.

There are a lot of great languages out there, but when you think of Rust, what comes to mind as its big selling points?

My name is Daniel, this is Fio's Quest, and I'm here to tell you the reasons you should try Rust are not why you think.

---

Let's get the obvious reasons out of the way.

Rust is fast, that's true.

But Zig and C++ are just as fast, C is a little bit faster and if speed is all that matters you should use assembly.

So "fast" is hardly a unique selling point.

---

Rust is memory safety.

Memory safety is important, supposedly around 70% of all security vulnerabilities are related to memory unsafety.

But pretty much all popular modern languages are memory safe; Java, Go, JavaScript, PHP, Perl, Python and so on are all
as safer or, arguably, safer than Rust.

So that's not a great reason either.

---

Rust is both fast and memory safe.

Ok of the languages I just mentioned it's the only language that's both fast _and_ safe but to that I say... who cares.

I mean some people care of course, particularly anyone building safety critical realtime systems, but I'm a web dev.

I have built web servers in Rust on the pretense of it being faster and more memory efficient than TypeScript.

But reducing my response times from 8ms to 2ms and my memory footprint from 20MB to 4MB while making a project my
colleagues didn't want to touch made me realize I'd mucked up.

For most people, speed and memory safety are not actually good reasons to try Rust.

---

But I still think you should try Rust... why?

Well, first of all... it's boring.

Or perhaps I should say, it's unsurprising.

As I mentioned, I'm a web dev, and I really love TypeScript.

TypeScript was probably the fourth programming language I used for web.

It was easily my favourite language before Rust, so trust me, I'm picking on it here because I love it.

Here's some TypeScript that gets a Pokémon object from an API by its id. 

It does a fetch, parses the data, verifies the data, and even returns a Result to avoid dealing with exceptions.

This code is valid... and it contains 3 mistakes. 4 if you count the duplicate. More if I've missed any.

Can you spot them? pause the video now if you need more time.

---

First one is here, `number`. In TypeScript a `number` is always a double precision floating point number.

The problem with this is that `numbers` can be negative, fractions, Infinite or NaN, `Not A Number`.

So this function will take a lot of values that are not fit for purpose and will still attempt to call the API with them

And that leads us to our second mistake.

What happens if the fetch fails?

---

Well, without going too deep into it, it actually depends on what specifically failed and where, but awaiting a fetch
can cause an Exception to be thrown.

But we've told anyone calling this code we're going to return a Result, which is then misleading as to how this code
actually works.

We see this again on the next line, the duplicate I mentioned, where if the data returned isn't JSON, this will also 
throw an exception.

The last mistake is the worst because its very subtle, and it won't surface in this code, it'll surface somewhere else
in the program, and you'll have to trace it back here.

---

This is actually a Type problem. You see TypeScript has two different types for dealing with ambiguity and, for historic
reasons, parsing has always used the wrong one.

In TypeScript 3, the `unknown` type was introduced. This type doesn't match with any other type and its up to you to
prove that an unknown type is actually the type you expect it to by writing a special validation function called a type
predicate.

Before TypeScript 3, all that existed was `any` and unfortunately `any` doesn't mean this _could_ be anything, it means
this _is_ anything.

No matter what the API returns, if it's JSON, and it has an id, this function will tell the rest of your code that it's
a valid Pokémon.

If we didn't have the id check it'd even work for an empty object... or arrays... or just the word "true".

---

So, in these few lines we have; a data problem, two execution flow problems, and a type problem, each increasing in
severity, and all, I think, fairly well hidden.

Let's look at the Rust.

Now I'll confess I'm very slightly cheating here, and I'll explain that in a moment, but overall this looks almost
identical to the TypeScript with some superficial cosmetic changes.

Let's go over the fixes

---

First we've changed the id type to be a bit more appropriate. An unsigned 16bit integer can still be zero or too big,
but it can't be any of the other options we talked about.

In both languages, you could go further and make a newtype (or value class) to wrap the ID, but this at least isn't
_as_ terrible, so we'll call that a low effort win.

A slightly higher effort win, and where I've cheated slightly, is these question marks.

Rust doesn't have exceptions. All functions that can fail (in a recoverable way) should return a Result type.

The question mark here does something really cool. If the function succeeded, this unwraps the expected data and lets
you carry on. 

If it returned an Error though, it'll automatically return the Error it received from this function, calling code to
transform it if the Error is not the same type.

The reason I've cheated a little here is that I'm going to get an Error from fetching on this line, and an Error from
parsing on this line and both will need transformation code to turn them into the GetPokemonError I'm not showing here.

So you do need to write a bit more code, not a lot, but in return you get this code that's succinct, easy to understand
and nearly impossible to muck up.

---

Finally, that Type error from the TypeScript version of this code is, as you'd imagine, also prevented by Rusts robust 
type system.

In this case the parsing is managed by a library called serde, but Rust is actually generally a great language to write
parsers in, and pro tip, libraries like `nom` come in clutch for things like Advent of Code.

---

That was a big focus on Rust being boring, but there are other languages that do that sort of thing well too, so what
else have we got.

How about Speed?

(You said Speed wasn't a good reason earlier)

Absolutely right hidden voice.

In my first ever industry job, my boss mentioned that hardware is cheaper than engineers... this was pre AI and
blockchain but for now his point still stands.

His point was, he'd rather just buy a bigger machine than have us spend ages tracking down every little inefficiency.

Our time is expensive, so the less time we spend working on something the better value it'll be.

Rust has a reputation for being slower to write but as we've just seen, what you write is more likely to be correct.

If you deliver something faster but its full of mistakes you have to fix, have you actually delivered it?

As someone who's worked in a lot of languages, I think Rust is just less faff.

If it compiles and my tests pass, then I'm much more confident what I've got works.

There's one more way Rust makes me faster.

It's niche, it's finite, it might just be that I'm an idiot, and again, I'm going to pick on my other favourite 
language.

When I start a new TypeScript project, assuming I have node set up then I have to...

- npm init
- Install and configure TypeScript
- Install and configure a linter
- Install and configure a style checker
- Install and configure a testing framework
- Fiddle with all the configurations
- Start working on the project

I can never remember all the config bits and frameworks constantly change so there's a fair bit of googling every time.

This usually takes around an hour.

It's just an hour, but its every, single, time.

Let's compare that to setting up a Rust project with all the same tooling.

First we run `cargo new` and then, end of steps, it's done, we can start working on our project.

Rust comes with all the tooling we need all configured in sensible ways and the tooling is another great reason to try
Rust.

Let's talk about that tooling.

---

RustC is the compiler and does all your type checking and stuff for you.

While Rust still has a reputation for being a difficult language to learn I don't think that's fair anymore.

Rust is one of the most hand-holdy languages available today.

When something goes wrong, there are no cryptic linker errors or anything like that... I'm looking at you C++,
thats time I'll never get back.

The compiler will always point to exactly where a problem occurs, give you a solid explanation of why something is
wrong, and it'll usually make suggestions on how to fix it.

In fact, the best example of this comes from Tris of No Boilerplate in his video "Rust is Easy".

He writes "Hello World" in JavaScript and by doing nothing other than following RustC's suggestions is able to rewrite
it into Rust.

---

Testing in Rust is also a fantastic experience.

Unit Tests are written right next to the code they're testing, usually wrapped in a module to excluded then from typical
compilation.

When you run the test suite it runs any function marked with a test attribute. 

There's a handful of assertion macros for confirming expectations.

If that's too barebones, then there are a bunch of testing frameworks too, but I personally almost never use
them, this is more than enough for me.

---

Rust's built-in formatter is very boring, again, in a good way.

You run it with cargo fmt, and it formats your code.

Its comes configured out of the box and while you can tweak that configuration, people rarely do, so all Rust code
ends up looking the same.

That means its really easy to jump between Rust projects, even ones you didn't create, there's no wondering which
version of AirBnB's closures styles you need to worry about or what namespacing schema is being used.

What about linting?

---

RustC will prevent you from writing invalid Rust, but valid doesn't always mean Good

Clippy prevents a lot of common mistakes in valid Rust.

Stuff like unnecessary use of references or inefficient use of memory, that sort of thing.

This is actually one thing a lot of people _will_ reconfigure, but only to make it more hardcore.

Clippy comes with several lint packs but some like "nursery" which is more experimental, or "pedantic" which is... 
well... pedantic, are usually turned off.

Many of us will turn them back on just to know we're writing the best Rust we can.

Obviously Pedantic lints _can_ be over the top, for example, here's an array I want to get the average of by dividing 
the sum by the length, but Clippy correctly points out that the length could be so large that it doesn't accurately
fall into the 2^52 bits we have for whole numbers in f64s.

This is valid Rust, it'll work even in that case, it just may not give us the exact right answer.

Clippy picks up on this, tells us where the problem is, and what it is.

In this case we're not likely to have that many elements but for the sake of an average we probably don't need perfect
precision, so we can tell Clippy not to worry about it with an attribute

What's particularly good about turning the lint off like this is we can write a reason down

This lets us immediately see there's a danger here and why we were so confident ignoring it when we come to the code 
later

---

Documentation wasn't part of that TypeScript set up but I want to talk about it anyway

RustDoc is where things get _really_ interesting, and if you only take one reason to try Rust from this video, it should
be this.

Many of you will be used to writing documentation into special comments above the thing being documented but, again,
we don't need another framework here, it's all included.

When we run `cargo doc`, our documentation is automatically generated, and, everyone uses this tool the same way.

This means that in addition to Rust providing a free repository of all documentation for every library released, and
all libraries coming with their documentation which you can access offline using this tooling, all
of it is written in the same way meaning you don't have to think too hard about how to follow it.

We can see the name of the thing being documented, its signature, the description we wrote and even the example we
wrote

In conclusion...

Wait... go back... ah whoops, adding one to one does not in fact equal three...

Luckily, when we run our tests... our documentation examples are also tested, so our tests fail _because_ our 
documentation is wrong.

Like I said, I think this alone is the best reason to try Rust

---

In conclusion 

When using Rust, there's very few surprises compared to most other languages

You can build stuff really fast. That lets you go outside and touch grass or move onto the next project way quicker.

And the tooling lets you focus on the things that actually matter. You focus about your project, not managing
everything around it.

---

So what next.

If you're looking for tutorials, I happen to know a guy, but honestly, you should maybe hop in with the official
learning resources or whatever works best for you

Rust is actually pretty good for whatever thing you want to build; system tools, web servers, even websites. 

In fact there's presentation version of this video I gave at work that is itself written in Rust, here's a slide that
shows the code for itself.

If I've convinced you to try Rust, I'd love to hear where your journey goes next, maybe it doesn't work out, I'd still
like to hear why.

If you do decide to look for tutorial videos, I have a whole series called Idiomatic Rust in Simple Steps, maybe I'll
see you there.

Best of luck!
