# TypeScript is my favourite dumpster fire

## Dumpster Fire

TypeScript is an amazing language.

Its typed, forgiving, easy to learn,

you can use it for almost anything.

Good TypeScript is even aesthetically pleasing to look at.

I love TypeScript.

It's my favourite dumpster fire.

## Surprise

That's not fair, right?

Every language is flawed, so what?

Why should anyone care how or why TypeScript is flawed?

Surprise.

Surprise!

Surprise.

## Good TypeScript

Surprise is the enemy of good software engineering, 

and TypeScript is esspecially suprising.

Have a look at this code.

This is fairly standard TypeScript.

But, SURPRISE, there are at least three errors in here.

That is to say I created three intentional errors, but I'm not confident there aren't more.

Now that you know they're there, can you spot them?

Would you have spotted them if you just came across this code in a code review?

## TypeScript is JavaScript

Before we get into what's wrong here...

Let's do the classic YouTuber thing, and go back to the beginning.

TypeScript is a superset of JavaScript.

This doesn't just mean that TypeScript is JavaScript with extra bits.

It means that TypeScript contains all of JavaScript, 

and JavaScript is a dumpster fire.

## Gary Bernhardt

This is a less controversial take.

Maybe you've already seen Gary Bernhardt's excellent talk WAT

I'm not going to reiterate Gary's work here, it's great, go watch it.

But the weirdness Gary talks about is specifically Type errors.

TypeScript provides a type system for JavaScript.

Here's the final bit of code from Gary's talk.

TypeScript correctly points out that this code is nonesense.

Still lets you run it, but... hey... it warned us, that's an improvement

And if that was all that was wrong with JavaScript, TypeScript would be fine.

## Three Dogs

This is TypeScript.

These are three dogs.

Currently, all the dogs are named "Fido" 

Let's update the first two.

Surprise!

Dog 3 doesn't actually have name.

JavaScript, and therefore TypeScript, is a prototype based language. 

Because dog3 was created from dog2, it doesn't get its own name.

It actually doesn't get its own anything.

dog3 references dog2

Anything you try to do with dog3 that it doesn't know how to do, it asks dog2 how to do

When we asked it its name, it didn't have a name, so it asked dog2 what its name was instead.

We gave dog 2 its own name so it returned that!

If we hadn't, dog2 would have asked dog1.

## Surprise Cat

Forget our dogs for a moment.

Let's talk Cats

This function that takes a Cat, and prints it's name... and age.

That's right Cat's have ages.

Why does this work?

Surprise, interface merging.

When you have multiple interfaces called the same thing, objects implementing that name of interface have to implement
all of them.

We can prove Cats have an age with hasOwnProperty... 

## null prototypes

or surprise, we could, if our cats were guaranteed to have the method hasOwnProperty.

At this point you might be wondering, is TypeScript even on?

I promise it is, and has been this whole time.

Dogs aren't Cats, this is an error.

Speaking of our dogs, remember how they inherited from each other.

Although we didn't specify it, Dog1's prototype is "Object"

The object called "Object" provides utility methods like hasOwnProperty.

For cat1, I didn't do that though

I created it with no prototype at all.

TypeScript doesn't see it.

It _does_ match the interface, but the interface doesn't say it needs to inherit Object.

TypeScript still let me make this mistake.

Don't trust that an Object has these utility methods.

For properties, we can use `in` keyword instead.

But at least we know describeCat function can only take Cats, we can see that it rejects dogs...

Oh for...

Surprise!

## Any is any

If any of TypeScript's oddities is its Original Sin... other than, you know, basically being JavaScript... this is it.

I may have lied earlier, dog3 isn't a dog.

---

Actually, sidebar, none of the cats or dogs we've created so far are what they're pretending to be...

At best, they're ducks.

Dog1 looks like a dog, quacks like a dog, therefore we can use it as a dog.

Cat1 looks like a cat, quacks like a cat, and therefore we can use it as a cat... 

but, Cat1 also quacks like a dog, so we can totally use it like a dog as well.

Duck typing is its own mess, but not actually the problem here.

---

Dog 2 and 3 don't look like dogs... they look like anything and everything.

In the early days of TypeScript, we needed a way to describe the types on existing JavaScript tools and functionality.

What does `JSON.parse` return? 

What do you get back from `Object.create`?

These methods return the `any` type.

The `any` type, unfortunately doesn't mean, "this _could_ be anything".

It means "this _is_ anything".

Something that's the `any` type will match anything.

Dog2 and Dog3 were created with `Object.create` which returns something of the `any` type.

We can pass them to a function that takes a Cat because they are not Dogs, they are "anything".

If you have a function which returns something that could be anything...

for the love of all that is holy, use `unknown` type instead.

Don't worry though we can cast them to Dog's with `as`

## As is ass

Right... but we didn't actually do anything to check that this `any` type was a dog did we...

We can't... we can't cast something that's definitely a Dog as Cat right?

Surprise! yay 😐 Are we still surprised at this point?

Never use `as`.

If you take one thing from this video.

Never. Use. As.

Make a lint for it.

Ban your teams from using it.

It does not do what folks seem to think.

`as` can get in the bin.

Instead, use type predicates.

## Predicates

I've spent a lot of time moaning about TypeScript

Let's take a break, and I'll show you a reason to love it.

A type predicate is a function that tests whether a value is the type it's supposed to be.

A type predicate for Dog, might look like this.

Let's just take a second to appreciate how aesthetic this is.

TypeScript is, in my opinion, the prettiest of all modern languages, and...

Hang on...

Why do we test if the value is null after we know it's an object?

Surprise!

## Null

Null, in any language, can absolutely get in the bin. 

Sorry, not sorry.

In TypeScript, null is an object that represents nothing.

So, any time you check something is an object you'll also want to check if its null.

Furthermore, null, despite being an Object, doesn't use the Object prototype.

Outside of smug YouTubers creating things will `Object.create` to prove a point...

it's the only time you're likely to find an object that doesn't implement Object.

If that wasn't bad enough though, by default, null can be used instead of values even if they're typed.

You can turn this off in the config.

For the love of the programming gods, turn it off in the config.

## Finally Throw

Let's go back to the code we started with

Now you can see all of the mistakes I made.

email could be null, we can fix that in our config

user could be anything, we can fix that with a type predicate

But what's the third error?

You might notice that I'm attempting to return Result.

Results are great because they force the caller of a function to deal with potential Errors

You can't just accidentally forget about them like Exceptions

Like we forgot to deal with them here, suprise, last one I promise

Exceptions, like nulls, are a problem much bigger than TypeScript.

In fact, excluding HTML/CSS, the top 10 of Stack Overflows most used languages all either use exceptions or something
worse.

There will be a subset of people who feel attacked by this video.

But here's the thing... it's ok.

## But seriously, I love it

TypeScript is a beautiful mess... but the best of us are.

All languages are flawed, learning those flaws helps keep us safe, and lets us make smug content for the internet.

And for that, if nothing else, how can I not love TypeScript.
