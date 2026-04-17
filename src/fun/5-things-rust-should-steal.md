# 5 things Rust should steal from other language

## Intro

Rust is an amazing language, ecosystem and community.

In fact, it's easily my favourite language, ecosystem and community.

But nothing is perfect so here's 5 things I think Rust should steal.

## Official Program Interpreter

I recently found myself sitting down to write what I expected to be a small one file script.

This script would be part of a much larger infrasctructure project used by dozens of other engineers.

Unfortunately this meant Rust wasn't an option.

Using Rust would mean building either building executable that would be opaue and I couldn't really store in Git.

Alternatively I could distribute the source code but then anyone using it would have to install Rust.

If Rust came with a light weight interpreter then I could have used that.

`cargo-script` is pretty close to that but you still need all of Rust installed to use it.

Instead I chose my other favourite language, TypeScript, and spent an the hour configuring all the things cargo new gets you for free.

Is an interpreter possible though... unfortunately the answer is, probably not.

Rustc is only about 40 megabytes, so in the worst case you could make a simple compile and run wrapper...

...but the standard library is enormous, 

All in, you're looking at about half a gig which make Rust decidedly less portable than say node which is about 50 megabytes or Python which is barely 10.

## Official async Runtime

JavaScript and C# both have very similar async syntax to Rust but don't require you to set up a complex runtime to actually use it.

Rust doesn't come with a runtime, meaning you have to either write your own, or use something like Tokio or smol.

I've spoken to a few people who find this really off putting, and list it among reasons they've not really given Rust a shot, which is a shame.

So why can't Rust have a built in runtime like other languages?

One benefit of Rust's approach is flexibility.

You can write or choose a runtime that best fits your goals and target hardware.

Meanwhile, JavaScript is a single threaded language which means there's a limited number of ways a runtime can even work.

Using an event loop with external callbacks is not only elegant, it also fits with the mental model JavaScript has had for decades.

C# is a little more complex as programs usually run on a "platform", framework, or host which configures a Synchronization Context for you.

This Syncronization Context is what decides when and where tasks are run.

You can configure or bypass the Syncronization Context but this is rare and could be problematic in rare circumstances.

Still, why doesn't Rust provide a sensible default for your async runtime?

I think its actually fairer to say Rust doesn't have an async runtime _yet_.

While most day to day async code is runtime agnostic, there are a lot of edge cases where the best way to handle a situation remain undecided.

Right now an official asyn runtime would need to make too many assumptions, which is something 3rd party libraries have more tolerance for.

In the future though, I wouldn't rule it out.

## Dynamic Libraries

Dynamic Libraries allow us to compile reusable code that can be shared among many programs.

By separating out these libraries we can reduce the size of our programs _and_ make security updates easier and more impactful.

For example, when Heartbleed happened, we only needed to update the OpenSSL library, we didn't need to update every program that consumed it separately.

If Rustls has a flaw, every Rust library that uses it will need to be updated and redistributed separately.

Because Dynamic Libraries are loaded at runtime, we need to communicate with them through a foreign function interface, abbreviated as FFI.

FFIs are not safe because they make assumptions about how code on the other side of the boundry will work.

Rusts super power is its type system, but a dynamic library is just some pre-compiled code, there's no types, its just pointers.

You cn use both produce and concume dynamic libraries in Rust but only through using the Abstract Binary Interfaces (ABIs) of other languages, such as C.

So the chance of us getting a Rust dynamic library with proper types is unlikely... however.

Rust has possibly the best WebAssembly tooling of any language.

Not only can you compile to WebAssembly but you can import WebAssemply components and run them inside Rust.

WebAssembly does allow you to expose interfaces and types, giving you an easy way to write strictly typed dynamic libraries Rust can consume.

WebAssembly isn't as fast as native code but it is pretty close for single threaded use.

It's still a growing ecosystem too and new features are constantly dropping.

I'm personally pretty hopeful about this as a potential alternative.

## Automatic Atomic Referencing with Copy on Write

Ownership is probably the number one tripping point for new rustaceans.

Sure, it gives us a clean, safe and surprisingly ergonomic way to think about memory... but mastering it is non trivial.

Whats worse is that sometimes... those benefits don't even mean much.

Rust works basically runs on anything

In fact, today, you can run Rust in more places that Java, the write once run anywhere language.

That includes places that use page files instead of giving you heap access, like the browsers or embedded devices.

You could even write your own allocator to provide page files for other systems too.

When you have a page file... ownership starts to matter a lot less.

It would be nice if we could, in these circumstance, just... forget about ownership.

And you sort of can.

Wrapping everything in COWs and wrapping the COWs in ARCs means you don't need to worry about who owns what any more.

It all just requires extra work.

Wouldn't it be nice if we could just have a compiler flag that did that for us, so we didn't need to worry about it?

Well... ok, probably not.

For a start, a compiler flag for your code is something other people, including your consumers if you're writing a crate, then need to worry about.

Its also not a panacea.

Any kind of reference counting adds work to the runtime, using ownership avoids that.

Finally, we've not really side stepped the problem of ownership being hard.

To master efficient Rust, you still need to learn it.

It might be difficult, but once you've got your head around it, you'll be making incredibly fast and efficient programs with the compiler still holding you hand.

## Namespaced packages

You might have guessed my other options were not esspecially sincere, but here'ssomething I do want to see.

Namespaced packages are, frankly, a settled discusion, so I'm not expecting to change minds, just state my case.

Many languages have package or library ecosystems.

In my opinion, two of the best are JavaScripts NPM and PHP's Composer (the latter basically copying the best parts of the former).

Cargo works very similarly.

All three of them allow you to publish and consume libraries of code safely

** cut to all the recent headlines **

ish

One feature that NPM and Composer offer that _does_ help with security is namespaces

In JavaScript, a common example of this might be the Angular framework which is broken down into multiple packages across a single namespace.

In PHP people regularly use packages in the Symfony and Laravel namespaces.

I believe there are two big benefits

First, it allows an organisation fine grain control over their corner of the ecosystem.

Whether the code is being produced by an open source community or a large private corporation, security controls can be set across the namespace.

Second, it helps with discoverability and trust.

If you're looking for plugins and extensions to something you're already using, its easier to trust packages within a namespace.

For some organisations, the lack of control of their ecosystem makes Rust much less desirable.

## Outro

Rust is a fantastic, incredibly well-designed language, with brilliant maintainers and contributors who just keep adding more amazing features!

What else do you think Rust could learn from other languages, let me know in the comments, even if, like me, you know you're being a little disingenuous.

If you enjoyed this video, check out the companion video, 5 things other languages should steal from Rust.
