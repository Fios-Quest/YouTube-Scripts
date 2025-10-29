What next
=========

### intro 1

We did it!

We finished Idiomatic Rust in Simple Steps.

So what next?

Well, my name is Daniel... wait this is no longer IRISS.

And what's with this camera angle?

*clap*

New set!

Editor Daniel is going to hate magic masking that.

Other Learning Resources
------------------------

### resources 1

First, if you'd like to dive into _other_ learning resources, there's a lot of really amazing stuff out there.

I'll put links to everything I mention in the description of this video.

### resources 2

The official book, "The Rust Programming Language", is available in both "living digital" and "dead tree" versions.

If you got all the way through IRISS and didn't look at the official book yet... how and why? It's amazing!

### resources 3

The official book goes into far more detail than IRISS, and makes fewer pains to avoid non-idiomatic examples, or crates, so you can argue, gets to the point more directly.

Particularly if you've gone through IRISS already, the official book will be a sinch to get through and give you a much broader view of the language.

### resources 4

For the more hands on, I can't recommend Rustlings enough.

It's a collection of short exorcises that don't just tell you how Rust works, but get you to write it.

You run it locally, and you can only pass a section when your code works!

It mirrors the official book pretty closely so working through both at the same time can really support your learning.

### resources 5

Similarly, Tour of Rust is another guide which embeds Rust Playground examples so that you can play with the things you've just learned about.

### resources 6

If you're more into visual learning, there are some great YouTube channels out there (including this one, obviously).

Here are some of my favourites:

### resources 7

No Boilerplate offers what Tris, the creator, describes as "Fast technical videos".

His video's cover everything from high level concepts such as why Rust's design makes certain choices and how they help you be a better developer, down to how to use specific language features.

Tris' video on "async isn't real and can't hurt you" is why I added a reminder that you don't _have_ to write async Rust to out async video.

### resources 8

Let's Get Rusty has a lower level focus than No Boilerplate.

Bogdan, who runs the channel, goes into more detail on common idioms and specific crates you might need to learn for specific tasks.

### resources 9

Chris Biscardi makes great guides on all things [Bevy], showcases new Bevy games, tools and framework features.
  
Even if game dev isn't your thing, Chris routinely runs through [Advent of Code] problems, solving them with Rust, which is a great place to see smart algorithms written succinctly in idiomatic code. 

Chris is where I learned about [nom], and how to do better parsing, discussion of which might have helped land my latest job!

### resources 10

If you know of any other good guides or resources, drop a comment below!

Cargo, Crates, and docs.rs
--------------------------

### tools 1

Very little of what we build is actually built from scratch.

My goal with Idiomatic Rust in Simple Steps was to teach Rust without getting distracted with third party libraries which, even the official book gets a little distracted with sometimes.

### tools 2

But now we're past that and ready to get really stuck in to everything the ecosystem has to offer.

### tools 3

Like most modern languages, Rust has a default library collection called [crates.io].

(In Rust parlance, we call external libraries "crates".)

Here you'll find a wealth of libraries for just about every use you can imagine. 

Whether you're building for tiny embedded microcontrollers or data center scale, distributed, GPU-powered, AI tools. 

### tools 4

When you create a project in Cargo, it will create a `Cargo.toml` manifest file.

By adding libraries from crates.io to your `[dependencies]` section in your manifest file, Cargo will automatically download them for you, and you'll be able to access them in your software. 

### tools 5

Documentation for libraries can almost always be found on docs.rs (usually linked from the crate's page on crates.io).

docs.rs is built from rustdoc, which we covered in IRISS' documentation video.

This means library documentation will always be structured in a familiar way and match the layout of the library, making it easy to use.

Cargo, Crates, and docs.rs
--------------------------

### tools 1

Very little of what we build is actually built from scratch.

My goal with Idiomatic Rust in Simple Steps was to teach Rust without getting distracted with third party libraries which, even the official book gets a little distracted with sometimes.

### tools 2

But now we're past that and ready to get really stuck in to everything the ecosystem has to offer.

### tools 3

Like most modern languages, Rust has a default library collection called [crates.io].

(In Rust parlance, we call external libraries "crates".)

Here you'll find a wealth of libraries for just about every use you can imagine. 

Whether you're building for tiny embedded microcontrollers or data center scale, distributed, GPU-powered, AI tools. 

### tools 4

When you create a project in Cargo, it will create a `Cargo.toml` manifest file.

By adding libraries from crates.io to your `[dependencies]` section in your manifest file, Cargo will automatically download them for you, and you'll be able to access them in your software. 

### tools 5

Documentation for libraries can almost always be found on docs.rs (usually linked from the crate's page on crates.io).

docs.rs is built from rustdoc, which we covered in IRISS' documentation video.

This means library documentation will always be structured in a familiar way and match the layout of the library, making it easy to use.

Start building
--------------

### building 1

The best way to learn Rust is to start building.

Rust is one of the few languages that work in just about any field.

So what do you want to build?

### building 2

To give you some food for thought, you can build command line apps, cross-platform GUI apps, full stack web apps, embedded microcontroller apps, machine learning tools, networking tools, video games, and even libraries that can be consumed by other programs and programming languages. 

The list is essentially endless.

### building 3

Early examples for me were;

### building 4

A CSV to Json converter using SIR DAY... SEER DE... SIRD?

Serde, of no agreed pronunciation, is short for **ser**ialize, **de**serialize.

It's the goto tool for converting string data formatted in a variety of file formats, into data in your application such as structs, and back again.

It's also what lets me store data from my Job Tracking app as JSON files.

### building 3

Another easier tool I built, this time for work, was a web API for reading specific details from a WordPress database using [Actix Web] and [Diesel].

It was 4 times faster and 5 times more memory efficient than a similar tool we'd build in TypeScript.

Actix Web is an incredibly fast web server framework that's surprisingly easy to work with.

Diesel is a Database ORM that not only lets you read and write to databases, but can also manage things like table structure and migrations.

### building 3

Something more fun was a two hundred and fifty thousand cell game of life in [Web Assembly] that ran at 60fps.

Web Assembly isn't a framework or library like other tools I'm listing here, it's a compile target.

You can compile Rust into Web Assembly.

I would even go so far as to argue that, thanks to how Rust works as a language, and especially the supporting tooling, Rust should be everyone's first choice when writing high compute performance code for frontend web.

### building 3b

But, to clarify, I don't want to yuck anyones yum, there's no single right choice for your programming language of choice and there are many factors to consider including personal preference.

WebAssembly 3 now comes with features that make it easier to transpile from other languages.

Rust still offers a good performance to effort ratio in my opinion but, you do you.

### building 3

While learning Japanese, I wrote a command line flash card database using [Clap] and [Sqlx] (SQLX also of no concrete pronounciation).

Clap is a tool for parsing command line arguments as well as providing a common experience across cli applications.

Sqlx is another framework for working with Databases, it supports async out of the box, but doesn't have the ORM or migration features built into Diesel.

### building 4

My advice is to think of something you're missing in your life, ideally something that's limited in scope and achievable.

Plan out how you'd structure the solution, and try building it.

Over to you
-----------

Where you go next is up to you, but I'd honestly love to hear about it. 

Fio's Quest has a Discord server, see the description, on which you'll find a community of wonderful, supportive people.

Let us know what learning resources you're using, or what apps or tools you're building.

What's next for Fio's Quest
---------------------------

### next 1

I've started working on a new collection of videos, Software Patterns and Coding Excellence, SPACE.

SPACE will be less sequential than IRISS and the videos _should_ be shorter so I'm hoping to get my turnaround time down.

### next 2

I'm also going to be making some other changes to the channel.

I'll be filming here from now on and trying to do more stuff while filming to cut down on editing.

### next 3

Also, I'd like to do a bit more streaming (we're so close to having the Job Tracker done).

Finally, I need to decide what I'm actually doing with my Patreon which is admitedly a mess right now and generally decide what I'm going to do when this channel get's monetised which (touch wood) will happen in the next few months.

### next 4

I'd still really like to hear from you though.

What am I missing that you'd like to see.

My goal here is to be helpful and I can't do that blind.

Drop me a comment, or pop by the discord.

### next 5

Other than that, I'll see you next in SPACE.

