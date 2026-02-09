# TypeScript is my favourite dumpster fire

## Dumpster Fire

I love TypeScript.

Its typed, forgiving, easy to learn, and you can use it for almost anything.

Good TypeScript is even aesthetically pleasing to look at.

It's an amazing language.

TypeScript is my favourite dumpster fire.

## Surprise

That's not fair, right?

Every language is flawed, so what?

What does it matter if TypeScript is flawed?

That's easy.

Surprise.

Surprise!

Surprise.

## Good TypeScript

Surprise is the enemy of good software engineering, 

and TypeScript is esspecially suprising.

Have a look at this code.

This is fairly standard TypeScript.

But, SURPRISE, there are at least four errors in here.

That is to say I created four intentional errors, but I'm not confident there aren't more.

Can you spot them?

Would you have spotted them if you came across this code organically like in a code review?

## TypeScript is JavaScript

Before we get into what's wrong here...

Let's do the YouTuber thing, and go back to the beginning.

TypeScript is a superset of JavaScript.

This doesn't just mean that TypeScript is JavaScript with extra bits.

It means that TypeScript contains all of JavaScript, 

and JavaScript is a dumpster fire.

## Gary Bernhardt

This is a less controversial take, especially if you've already seen Gary Bernhardt's excellent talk WAT

I'm not going to reiterate Gary's work, it's great, go watch it, 

but the weirdness Gary talks about is specifically Type errors.

TypeScript provides a type system for JavaScript.

Here is the final bit of code from Gary's talk.

TypeScript correctly points out that this code is nonesense.

Still lets you run it, but... hey... it warned us, that's an improvement

And if that was all that was wrong with JavaScript, TypeScript would be fine.

## Three Dogs

This is TypeScript.

These are three dogs.

[//]: # (`Object.create&#40;&#41;` lets me make new dogs based on old dogs.)

Right now, all the dogs are named "Fido" 

Let's update the first two.

Surprise!

To be fair, this isn't a problem you're likely to run into very often.

Dog 3 doesn't actually have name.

TypeScript is JavaScript, and JavaScript is a prototype based language. 

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

We know they have ages because TypeScript says this is fine.

Why does this work?

Surprise, interface merging.

When you have multiple interfaces called the same thing, objects implementing that name of interface have to implement
all of them.

We can prove Cats have an age with hasOwnProperty... 

or surprise, we could, if our cats were guaranteed to have the method hasOwnProperty.

Now you might think at this point, is TypeScript even on?

hasOwnProperty isn't part of Cat, surely we should see an error

But look, Dogs aren't Cats, this is an error.

Remember our dogs, and how they inherited from each other.

Dog1 also has a prototype from "Object"

The object called "Object" provides utility methods like hasOwnProperty.

For cat1, I didn't do that though, I actually created it with no prototype at all.

TypeScript doesn't see it.

It _does_ match the interface, but the interface doesn't say it needs to inherit Object.

TypeScript still let me make this mistake.

Never trust an Object has these utility methods.

Use the `in` keyword to see if an object has a property.

But at least our describeCat function can only take Cats, you couldn't say pass in a Dog...

Oh for fu

Surprise!

## Any is any

If any of TypeScript's oddities is its Original Sin... other than, you know, basically being JavaScript... this is it.

I may have lied earlier, dog3 isn't a dog.

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

Make a lint for it.

Ban your teams from using it.

It does not do what you think.

`as` can get in the bin.

Instead, use type predicates.

## Predicates

I've spent a lot of time moaning about TypeScript, lets take a reprieve while I show you a reason to love it.

A type predicate is a function that tests whether a type is what it's supposed to be.

Let's create one for our simplest type, Dog.

Our function name just says what the predicate does, it tests `isDog`.

It takes one parameter that's maybe a dog, but we don't know that yet so the type is `unknown`.

The predicate will return a boolean, but we annotate it with a special return type.

Predicates (type or otherwise) can usually be one line so we don't need curly brackets or a return, we can just
have the statement.

For the body of the function, we know that Dog's have names so we'll check if maybeDog has one

And here's where things start getting good.

TypeScript now alerts us that, we don't even know if maybeDog is an object, so lets test that first.

Now we have a new warning, that maybeDog could be null, because null is an object

We'll... come back to this in a moment

Once we add a check for null, that's all there is to it.

We leaned into the TypeScript compiler, and we made something I genuinely think is rather beauitiful.

## Finally Throw



## But seriously, I love it

TypeScript is a beautiful mess... but the best of us are.

All languages are flawed, learning those flaws helps keep us safe, and lets us make smug content for the internet.

And for that, if nothing else, how can I not love TypeScript.
