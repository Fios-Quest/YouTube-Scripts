# TypeScript is my favourite dumpster fire

## Dumpster Fire

TypeScript is amazing.

Though it is geared mostly towards full stack web development, you can run TypeScript almost anywhere and for almost
any purpose.

Its typed, forgiving and easy to learn.

I love TypeScript.

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

Surprise is the enemy of good software engineering.

Have a look at this code.

I think this is fairly standard TypeScript.

But, surprise, there are at least four errors in here.

That is to say I created four errors (without double counting the same error multiple times), but I'm not confident
there aren't more.

Can you spot them?

Would you have spotted them if you came across this code in naturally in your application or a code review?

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

TypeScript provides a type system for JavaScript so it prevents you from making those kinds of mistakes...

...or at least it still lets you make those mistakes, but now it warns you about them first.

And if that was all that was wrong with JavaScript that would be fine.

## Three Dogs

Here I've got three dogs.

`Object.create()` lets me make new dogs based on the old dogs.

Right now, all the dogs names are Fido, so lets update two of them.

Surprise!

Ok, this isn't a problem you're likely to run into very often.

The giveaway is if we check that each Dog has a name.

Dog's one and two do, but dog three does not.

JavaScript is a prototype based language. 

Because dog3 was created from dog2, it doesn't get its own name. It actually doesn't get its own anything.

Instead, dog3 references dog2, and anything you try to do with it that it doesn't know how to do it asks dog2 how to do

So when we asked it its name, it didn't have a name, so it asked dog2 what its name was instead.

We gave dog 2 its own name so it returned that!

## Surprise Cat

Ok, lets forget our dogs for now.

Let's talk Cats.

Cats have names

Here I've got a function that takes a Cat, and prints it's name and age.

That's right Cat's have ages.

We know they have ages because TypeScript says this is fine.

Why does this work?

Surprise, interface merging.

Got multiple interfaces called the same thing, well now your objects have to implement all of them.

And we can prove that Cats have an age with hasOwnProperty... or surprise, we could, if our cats were guaranteed to have
the method hasOwnProperty.

Normally when you create an object, like dog1, it inherits from the object "Object".

The object "Object" provides utility methods like hasOwnProperty.

For cat1, I didn't do that though, I actually created it with no prototype at all.

It _does_ match the interface, but the interface doesn't say it needs to inherit Object, yet TypeScript still let me
make this mistake.

So never trust an Object has these utility methods, instead you can just invoke the method from the Object prototype
which is weird and verbose, or use the `in` keyword.

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

What does `JSON.parse` return? What do you get back from `Object.create`? It could be anything right.

The `any` type, unfortunately doesn't mean, "this could be anything".

It means "this _is_ anything".

Something that's the `any` type will match anything.

Dog2 and Dog3 were created with `Object.create` which returns something of the `any` type.

This means, we can pass them to a function that takes a Cat because they are not Dogs "anything".

Today, if you have a function which returns something that could be anything, use the `unknown` type instead

Don't worry though we can cast them to Dog's with `as`

## As is ass

Right... but we didn't actually do anything to check that this `any` type was a dog did we...

We can't... we can't cast something that's definitely a Dog as Cat right?

Surprise! yay 😐 If we're still surprised at this point?

Honestly, I push the teams I work with to never use `as`, it just doesn't work the way people naturally expect it to.

Instead, use type predicates.

## Predicates

I've spent a lot of time moaning about TypeScript, and now I'm going to show you why I love it, and why you should too.

A type predicate is a function that tests whether a type is what it's supposed to be.

It returns a boolean, but we annotate it with a special type.

Let's create one for our simplest type, Dog.

Our function name just says what the predicate does

It takes one parameter that's maybe a dog, but we don't know that yet so the type is `unknown`.

The function return type is whether maybeDog is Dog or not.

We can make the predicate one line so rather than using curly brackets and returning something we'll make it a statement.

Now we know that Dog's have names so we'll check if maybeDog has one

TypeScript now alerts us that, we don't know what maybeDog is, so to fix that we'll check if maybeDog is an object.

Now we have a new warning, that maybeDog could be null, because null is an object

(Aside, I'm not critiquing this, null is garbage, but lots of languages make Null or Nill an Object so we'll forgive
TypeScript this quirk)

And that's all there is to it.

We leant into the compiler, and we made something I genuinely think is rather beauitiful.

It's not perfect, but who is.

## But seriously, I love it

TypeScript is a beautiful mess... but aren't we all.

I really do love the language, not despite its flaws but because of them.

All languages are flawed, but learning how to manage those flaws ... <something>
