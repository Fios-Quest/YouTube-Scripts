Dependency Injection
====================

Intro
-----

### intro 1

This video is a bit of a double episode as I want to talk about two patterns that are so tightly coupled, people often
(and fairly) treat them like one thing.

Dependency Injection lets us better compartmentalise our code, separating concerns, making the code easier to reason
about, and reducing code duplication.

### intro 2

Ports and Adaptors build on this pattern to make the code more flexible.

It lets you pick the right tool at the right time and is amazing for testing.

Dependency Injection
--------------------

### di 1

Dependency Injection is incredible powerful and is the foundation of a number of other patterns, including the one
we'll be discussing later in this video.

### di 2

And yet it's such a simple pattern you might already be using it without realising it.

Dependency Injection, while sounding fancy, does exactly what it says on the tin.

### di 3

Rather than constructing things we're dependent on where we need them, we inject those dependencies from outside.

Let's run through an example.

### di 4 (code)

Lets saw we have some users we want to store.

We'll keep it fairly simple with just a username and email address.

### di 5 (code)

We want to store our users in a mysql database, so when we construct the user store we'll also connect to the database,
collecting the connection settings from the environment.

If you aren't familiar with databases (or if you are and this looks silly) don't worry too much, this is just to show
that there is _some_ small amount of work in making that connection.

The point here is that the mysql connection is a dependency for our user store.

### di 6 (code)

Once our user store is instantiated, we can now use it to store our users and our interface for doing so is nice and
simple because all our sql logic is hidden behind method calls.

And if that was it, this would be ok, kinda, I'll get back to this point.

But it's really rare for us to ever make a program that only stores one thing... so let's create a pet.

### di 7 (code)

Again, we'll keep things simple, the pet has a name and a carer *cough* I don't like the word owner, Yuki owns himself,
I just look after him.

Next lets create the pet store and...  now our problems are laid bare.

### di 8 (code)

It's not just that we have to redo all of this logic, which breaks the cardinal rule of "Don't Repeat Yourself"...
but there's a problem with this instantiation method that existed even when we just had the one store.

The new method is fallible, meaning it returns a Result, but every error it can produce is related not to the store 
itself, but to the mysql connection... or the environment.

Let's fix that

### di 9 (code)

First we'll 

Ports and Adapters
------------------

<!-- Discuss architecture -->
<!-- Show variants of user store -->


Stub Adapters
-------------


<!-- Reminder of previous video, what hasn't aged well, what I stand by -->
<!-- Show UserStore test -->

Conclusion
----------

I love these patterns.

They're often sold as being good to make sure your code is flexible, that if you need to you could change database or
SaaS provider with little effort but I think this is a minor benefit.

---

You probably won't drastically change the physical architecture of your solutions so often for this pattern to be worth
while on its own.

But even if you never change your architecture, these patterns have strong benefits when it comes to maintainablity.

---

First, by modularising our code, we make each part of it do less, and only do things related to one specific domain.

This makes it much easier to reason about the code, so when you com back to the code later you can more quickly
understand it.

---

It also makes it easier to write tests for sections of your code that only do one thing.

Second, by using a stub adaptor which is tested against integration tests, we can be _very_ confident that our tests
are testing code in the exact way that it will be used.

---

Finally, it's easy to get distracted by the word "maintainability" and feel that this is something that _only_ matters
in the future.

While most of the benefit is in the future, writing well tested maintainable code is invariably faster than rushing
something out, even in the short term, because less things go wrong while developing.

---

Next time we're going to talk about Event Driven Architecture, which, full disclosure is something I've always been
fascinated in but have yet to have a good reason to use at the software level.

We'll talk about its pros and cons, and when its a good choice to use.

I hope to see you then.
