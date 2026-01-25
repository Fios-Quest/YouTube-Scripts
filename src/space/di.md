Dependency Injection
====================

Intro
-----

### intro 1

This video is a bit of a double episode as I want to talk about two patterns that are so tightly coupled, people often (and fairly) treat them like one thing.

Dependency Injection lets us better compartmentalise our code, separating concerns, making the code easier to reason about, and reducing code duplication.

### intro 2

Ports and Adaptors builds on this pattern to make the code more flexible.

It lets you pick the right tool at the right time and is amazing for testing.

Dependency Injection
--------------------

### di 1

Dependency Injection is incredible powerful and is the foundation of a number of other patterns, including the one we'll be discussing later in this video.

### di 2

And yet it's such a simple pattern you might already be using it without realising it.

Dependency Injection, while sounding fancy, does exactly what it says on the tin.

### di 3

Rather than constructing things we're dependent on where we need them, we inject those dependencies from outside.

Let's run through an example.

### di 4 (code)

Lets say we have some user data that we want to store.

We'll keep it fairly simple with just a username and email address.

### di 5 (code)

We want to store our users in a mysql database, so when we construct the user store we'll also connect to the database, collecting the connection settings from the environment.

If you aren't familiar with databases (or if you are and this looks silly) don't worry too much, this is just to show that there is _some_ small amount of work in making that connection.

The point here is that the mysql connection is a dependency for our user store.

### di 6 (code)

Once our user store is instantiated, we can now use it to store our users and our interface for doing so is nice and simple because all our SQL logic is hidden behind method calls.

And if that was it, this would be ok, kinda, I'll get back to this point.

But it's really rare for us to ever make a program that only stores one thing... so let's create a pet.

### di 7 (code)

Again, we'll keep things simple, the pet has a name and a butler.

Next lets create the pet store and...  now our problems are laid bare. 

### di 8 (code)

It's not just that we have to redo all of this logic, which breaks the cardinal rule of "Don't Repeat Yourself"... but there's a problem with this instantiation method that existed even when we just had the one store.

Our constructor is fallible, meaning it returns a Result, but every error it can produce is related not to the store itself, but to either the mysql connection or the environment.
### di 9

We can break this down into three problem domains in our code.

We have some kind of configuration for connecting to the database.

The act of connecting to the database.

And the abstraction around the database that manages the storage of our users or our pets.

Let's rewrite our code separating it into these different domains.

### di 10 (code)

First we have our mysql connection configuration.

We know that we need an address, a port, a username and a password.

We'll create a configuration struct to hold this data.

Next we'll instantiate our configuration from environment variables.

### di 11 (code)

Having a configuration type like this benefits us in a number of ways.

Future engineers have one place to make changes to how this configuration is structured and created.

### di 12 (code)

Similarly, it moves error handling into this one place.

Before, reading environment variables could spew errors out all over our code, now it won't.

### di 13 (code)

Additionally, having a well named type with well named parameters and good controls, makes using this type easier for anyone who needs a mysql configuration in the future.

### di 14 (code)

Finally, while we have this method for getting the settings from the environment, we aren't restricted to that.

Depending on how the application is tested and deployed, there might be a variety of ways we want to construct this type.

### di 15 (code)

Now we can restructure our code so that we inject the configuration dependency into the MySql connection.

Then inject the MySql connection into our user and pet stores.

### di 16

So dependency injection lets us better separate our code into logical domains, reduces code repetitions, and makes our code easier to understand.

Now that we've started talking about logical domains, I want to talk about an old favourite pattern of mine, one that is enabled by dependency injection.

Ports and Adapters
------------------

### p&a 1

So we've created our stores that talk to a MySql database.

However, software rarely sits still.

Business needs change, and we might find ourselves needing to use a different storage mechanism other than MySql.

### p&a 2

I actually went through this recently, working on my little job tracker.

My initial plan was to store the data in RocksDB, using SurrealDB as a communication layer.

That didn't work due to some complexity with the lock file, so I then switched to libsql, a fork of sqlite that also sits on disk.

### p&a 3

I wasn't happy with that either so I ended up storing the data as JSON files.

This was only possible because I was following a pattern called hexagonal architecture, also known as ports and adapters.

### p&a 4

Thinking about logic domains again, we have our software that we control, and external software that we don't.

Whether that's storage such as database servers, logging infrastructure, 3rd party SaaS providers, any external thing our software has to communicate with is an external domain.

### p&a 5

Ports and Adapters allows us to compartmentalise that domain.

We separate how _we_ think about data and logic inside the core of our software, and the code we write to communicate with these external services and how _they_ want to think about data and logic.

### p&a 6

The description of how we want to communicate our needs is called a port...

and the bit of code that matches that description (plugs into that port) and converts it to the way our 3rd party works is called an adapter.

Let's show this separation using our stores from earlier.

### p&a 7

We need to store users, but our user store is actually doing two things.

First, it describes to the rest of our code how to store and retrieve users.

Second, it does the storing and retrieving.

The ports and adapters pattern makes this two different responsibilities.

### p&a 8

We can apply this to our code by creating a UserStore trait, that describes how to store and retrieve users.

And a MySqlUserStore type that implements the trait and acts as the translation layer.

### p&a 9

If we wanted to talk to a different backend, we can easily create a new implementation of the trait for Postgres, Redis, SurrealDB, our options are literally limitless so long as we follow the trait.

This is why it was so easy for me to change between RocksDb, libsql and eventually settle on JSON files.

Integration Tests
-----------------

### integration 1

I'm not going to get into the weeds of integration tests for this video (if you'd like a video on that, let me know in the comments).

What I want to show is that if your program is using ports and adapters, and has multiple adapters for the same ports, you need to make sure that the adapters all behave identically when used.

### integration 2

To achieve this, I've found the best thing to do is write integration tests for ports (not adapters), then pass in adapters to those tests.

For example, a simple test for us might be can we get a user by their username.

### integration 3 (code)

To test that this works, we simply insert the user into the store, then try to get them back again.

So let's write that test only using the trait.

Next we can configure any number of our adapters, and pass them into the test.

### integration 4

As we build up our suite, we likely will spot minor differences in how our backends work, that require bespoke code to work around.

Little things like, what order are lists returned in, or are our fields case-sensitive.

I have seen production bugs caused by things this minor, so be thorough when you test.

### integration 5

Now, outside my example of a personal project that was less well architected than anything I've done strictly professionally, its actually quite rare for something like a database to end up being completely changed.

In my 15 years of professional software engineering, this has only happened to me maybe twice.

### integration 6

However, even if it had never happened, even if you are sure you will never change your database backed, this pattern is still incredibly powerful.

Let me explain why.

Stub Adapters
-------------

### stubs 1

I've actually already talked about Ports and Adapters on this channel before, in my most disliked video.

Don't worry, you don't need to have seen it, I'll cover all the points here.

### stubs 2

While I don't stand by a number of things in that video; the video quality, the audio quality, the presentation style, how many times I said "um"... I do stand by the point I was trying to make.

Mocks. Are. Bad. Don't use them.

### stubs 3

Through that video I showed that mocks provide a behavioural abstraction in your test code, which is bad, and that the behavioural abstraction might be incorrect which is even worse.

If you use mocks in your tests, even though they are not the thing being tested, your test might be wrong because of them, and you won't know.

### stubs 4

A stub adapter is an adapter that doesn't reach out to anything external.

When I was taught this pattern, we called them "In Memory" adapters because we were using them as stand-ins for databases, and they simply stored the information in memory.

### stubs 5

But you can use the same pattern to stub out all kinds of external interactions.

For now though, lets make an in memory stub adapter for our database.

### stubs 6 (code)

We already have a trait for our store type, so the next step is to implement that trait in a way where data is stored and recalled internally.

The easiest way to achieve this is to use a Vector.

### stubs 7 (code)

We can push new users on to the vector, and recall existing users by searching through it.

That's not quite enough though.

### stubs 8

If our other user stores enforce rules, such as "username must be unique", then this stub adaptor need to too.

This means we need to add it to our shared integration test from earlier.

### stubs 9 (code)

Because we already wrote the tests though, this is trivial, we just construct our stub adapter, pass it into the tests and bam, we're good!

### stubs 10

Now that we've created a stub adapter, and proved it's a perfectly good stand-in for our other user stores, we can use it in unit tests any time we'd otherwise need to talk to a database or use a *yuck* mock.

But that's not all.

### stubs 11

If you leave this video thinking stub adapters are only for testing, I've failed to do my job... again.

Almost every time I've created a stub adapter for testing, I've ended up using it in production code later.

### stubs 12

When I was working on hotel advertising, we used our stub adapters for processing reports in cloud functions.

When I was working on an authentication service, we provided a full stub of our library for people to use in their tests and that wrapped the same stub adapters we wrote for our tests.

### stubs 13

And when I finally moved to JSON files for the Job Tracker, I used the stub adapters I made back with the initial RocksDB implementation as a cache inside the JSON implementation.

These things, are seriously underrated.

They're trivial to write, and insanely flexible.

Conclusion
----------

### conclusion 1

I love these patterns... in case you couldn't tell.

They're often sold as being good to make sure your code is flexible, that if you need to you could change database or SaaS provider with little effort, but I think this is a minor benefit.

### conclusion 2

You probably won't drastically change the physical architecture of your solutions so often for this pattern to be worthwhile on its own.

But even if you never change your architecture, these patterns have strong benefits when it comes to maintainability.

### conclusion 3

First, by modularising our code, we make each part of it do less, and only do things related to one specific domain.

This makes it much easier to reason about the code, so when you come back to the code later you can more quickly understand it.

### conclusion 4

It's also easier to write tests for sections of your code that only do one thing.

Second, by using a stub adaptor which is tested against integration tests, we can be _very_ confident that our tests are testing code in the exact way that it will be used.

### conclusion 5

Finally, it's easy to get distracted by the word "maintainability" and feel that this is something that _only_ matters in the future.

While most of the benefit is in the future, writing well tested, maintainable code is invariably faster than rushing something out, even in the short term, because fewer things go wrong while developing, let alone once something's in production.

### conclusion 5

Next time we're going to talk about Event Driven Architecture, which, full disclosure is something I've always been fascinated in but have yet to have a good reason to use at the software level.

We'll talk about its pros and cons, and when it's a good choice to use.

I hope to see you then.
