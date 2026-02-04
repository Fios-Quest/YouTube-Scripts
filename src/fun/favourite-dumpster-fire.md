# TypeScript is my favourite dumpster fire

## Dumpster Fire

TypeScript is amazing.

Though it is geared mostly towards full stack web development, you can run TypeScript almost anywhere and for almost
any purpose.

Its typed, forgiving and easy to learn.

I even think well written TypeScript is aesthetically pleasing to look at.

I love TypeScript.

It's my favourite dumpster fire.

## Surprise

That's not fair, right?

Every language is flawed, so what?

What does it matter if TypeScript is flawed?

That's easy.

Surprise.

Surprise!

Surprise.

## Good TypeScript

Surprise is the enemy of good software engineering, and I think TypeScript is esspecially suprising.

Have a look at this code.

I think this is fairly standard TypeScript.

But, SURPRISE, there are at least four errors in here.

That is to say I created four intentional errors (without double counting the same error multiple times), but I'm not
confident there aren't more.

Can you spot them?

Would you have spotted them if you came across this code organically in your application or in a code review?

## TypeScript is JavaScript

Before we get into what's wrong...

Let's do the YouTuber thing, and go back to the beginning.

TypeScript is a superset of JavaScript.

This doesn't just mean that TypeScript is JavaScript with extra features.

It means that TypeScript contains all of JavaScript, and JavaScript is a dumpster fire.

## Gary Bernhardt

This I think is a less controversial take, especially if you've already seen Gary Bernhardt's excellent talk WAT

I'm not going to reiterate Gary's work, it's great, go watch it, but all the weirdness Gary talks about stems from Type
errors.

TypeScript provides a type system for JavaScript so it prevents you from making those kinds of mistakes.

Here is the final bit of code from Gary's talk, TypeScript correctly points out that this code is nonesense.

It still lets you run it, but... hey... it warned us, that's an improvement

And if that was all that was wrong with JavaScript, TypeScript would be fine.

## Three Dogs

Here I've got three dogs.

`Object.create()` lets me make new dogs based on old dogs.

Right now, all the dogs are named "Fido", so lets update the first two.

Surprise!

Ok, this isn't a problem you're likely to run into very often.

The giveaway is when we ask each dog if it has a name.

Dog's one and two do, but dog three does not.

TypeScript is JavaScript, and JavaScript is a prototype based language. 

Because dog3 was created from dog2, it doesn't get its own name.

It actually doesn't get its own anything.

Instead, dog3 references dog2, and anything you try to do with it that it doesn't know how to do it asks dog2 how to do

So when we asked it its name, it didn't have a name, so it asked dog2 what its name was instead.

We gave dog 2 its own name so it returned that!

Had we not done that, dog2 would have asked dog1.

## Surprise Cat

OK, lets forget our dogs for now.

Let's talk Cats

Here I've got a function that takes a Cat, and prints it's name... and age.

That's right Cat's have ages.

We know they have ages because TypeScript says this is fine.

Why does this work?

Surprise, interface merging.

When you have multiple interfaces called the same thing, objects implementing that name of interface have to implement
all of them.

And we can prove that Cats have an age with hasOwnProperty... or surprise, we could, if our cats were guaranteed to have
the method hasOwnProperty.

Now you might think at this point, TypeScript can not be working here.

hasOwnProperty isn't part of Cat, so surely we should see an error

But to prove that it is, I can show you that Cats have no bark.

Normally when you create an object, like dog1, it inherits from the object "Object".

The object called "Object" provides utility methods like hasOwnProperty.

For cat1, I didn't do that though, I actually created it with no prototype at all.

It _does_ match the interface, but the interface doesn't say it needs to inherit Object, yet TypeScript still let me
make this mistake.

So never trust an Object has these utility methods.

You can invoke the method from the Object prototype itself like this, but that's weird and verbose.

Instead, you can use the `in` keyword.

Ok, but at least our describeCat function can only take Cats, you couldn't say pass in a Dog...

Oh for fu

Surprise!

## Any is any

If any of TypeScript's oddities can be called its Original Sin... other than, you know, basically being JavaScript, this
is it.

I may have lied earlier, dog3 isn't a dog.

Actually, none of the cats or dogs we've created so far are what they're pretending to be...

At best, they're ducks.

Dog1 looks like a dog, quacks like a dog, therefore we can use it as a dog.

Cat1 looks like a cat, quacks like a cat, and therefore we can use it as a cat... though, it also quacks like a dog,
so we can totally use it like a dog as well.

Duck typing is its own mess, but not actually the problem here.

Dog 2 and 3, they don't look like dogs... they look like anything and everything.

In the early days of TypeScript, pre-1.0, before we had generics, we needed a way to describe the types on existing
JavaScript tools and functionality.

What does `JSON.parse` return? What do you get back from `Object.create`? 

It could be anything right.

These methods return the `any` type.

The `any` type, unfortunately doesn't mean, "this could be anything".

It means "this _is_ anything".

Something that's the `any` type will match anything.

Dog2 and Dog3 were created with `Object.create` which returns something of the `any` type.

This means, we can pass them to a function that takes a Cat because they are not Dogs, they are "anything".

Today, if you have a function which returns something that could be anything, use the `unknown` type instead, you can
narrow it down later.

Don't worry though we can cast them to Dog's with `as`

## As is ass

Right... but we didn't actually do anything to check that this `any` type was a dog did we...

We can't... we can't cast something that's definitely a Dog as Cat right?

Surprise! yay 😐 Are we still surprised at this point?

Never use `as`.

Make a lint for it.

Ban your teams from using it.

It does not do what you think.

Instead, use type predicates.

## Predicates

I've spent a lot of time moaning about TypeScript, and now I'm going to show you why I love it, and why you should too.

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

> Aside, I'm not critiquing this, null is garbage, but lots of languages make Null or Nill an Object that can match
against other types but not be used as them. 
> 
> JavaScript wasn't the first or last to follow the "billion-dollar mistake" that's a whole other topic

Once we add a check for null, that's all there is to it.

We leaned into the TypeScript compiler, and we made something I genuinely think is rather beauitiful.

It's not perfect, but who amongst us is.

## But seriously, I love it

TypeScript is a beautiful mess... but the best of us are.

All languages are flawed, learning those flaws helps keep us safe, and lets us make smug content for the internet.

And for that, if nothing else, how can I not love TypeScript.
