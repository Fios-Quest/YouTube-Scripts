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

Rustc is quite small, so in the worst case you could make a simple compile and run wrapper...

...but the standard library is enormous, so that's not practical for something as portable as node or python.

## Official async Runtime

## Dynamic Libraries

## Automatic Atomic Referencing with Copy on Write

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
