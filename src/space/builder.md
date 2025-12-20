Builder
=======

Intro
-----

The builder pattern allows us to construct complex types in simple steps.

For example, in our previous video, you don't have to have watched it, we had a 'User' struct that had a username, an
email and a date of birth.

In that video, we allowed the date of birth to be set later, but a user also had to be older than 21 (which, I said at
the time, was silly to check after the user was created).

### Intro 2

If we were to make all of these fields required, creating the struct gets a bit unwieldy.

Do we instantiate it manually or write a constructor?

If we manually instantiate it, we can't validate the fields.

If we write a single constructor, the parameter names are slightly obfuscated, but worse than that, we have to cram all
our validation into a single function.

### Intro 3

This isn't so bad in our example User struct...

but what if you're working with a more complicated structure with numerous fields many of which require validation?

This is where the Builder Pattern comes in.

### Intro 4

Instead of constructing the object directly, we use another type to manage the construction.

This Builder type collects data about the struct we want to create, then has a finalizer that creates the struct from
the data we've given it.

There are arguably two versions of this pattern:

a simpler version that will work in just about any language that I'm going to refer to as "Builder Lite", 

and a more complex version that only works in languages with strict generic typing, the "Typestate Builder".

Each has their own pros and cons, so lets go over them.

Builder Lite
------------

The traditional builder pattern uses a type that mirrors the type you want to build but everything is optional.

We use methods to update each field, and then have a finalizer that takes the data we've stored in the builder and
attempts to convert it into the target type.

Let's build one for our `User` example.

Before we start on the Builder though, we need to set up an Error type as there are a lot of ways this could go wrong.

The UserBuilder itself looks just like the User, except the fields are all optional.

Each of our "withers" will set the relevant value and return a new version of the UserBuilder with the new value.

### lite 2

Am I trying to make "withers" a thing? Yes.

Will I succeed? No, not a chance.

### lite 3

Our first potential error is when setting the date of birth, where we test if the user is at least 21.

This doesn't matter to our Fluent Interface in Rust so long as you're using the Builder in a context where you can
bubble the Error.

### lite 4

Our finalizer method is called "build" and simply checks to see if all the Optional parameters have been set.

If we missed any values we'll return an appropriate error or if everything is ok, we'll return the built User type.

Here's an example of where the User will be successfully built.

### lite 5

And, here's an example where we'll get an error.

The biggest pro of this pattern is its really easy to understand at a glance and pretty easy to maintain.

The biggest con though is that we should know, at compile time, that that second usage was never going to succeed.

Typestate Builder
-----------------

### typestate 1

Let's use the TypeState pattern to make that problem go away.

If you're not already familiar with this pattern we have a video on it, but the TL;DW is that we can bake the state of
something into its Type, 

We can then decide what you're allowed to do with the data by restricting behaviour with its TypeState.

### typestate 2

Let's make a TypeState Builder for our User.

First we can edit our Error down to a single event now, the others won't be possible when we're done.

Next, we're going to use a generic type for each of the properties in our builder.

### typestate 3

These generics can either be the type they're supposed to be, or something that represents the fact we haven't set them
yet.

To achieve this, I've created an "Unset" unit struct, and a trait to restrict each generic

(this may not be necessary, but it is tidy).

### typestate 4

We'll implement each trait for Unset as well as the type we _actually_ want it to become in User.

Now we can create our Builder and the properties are all generics restricted by the relevant traits

### typestate 5

So U can be Username or Unset, E can be EmailAddress or Unset and D can be DateOfBirth or Unset

We're going to implement UserBuilder three times, each with different generic variants.

### typestate 6

The first one we'll implement is when all of our generics are Unset, this allows us to have a constructor for the
Builder that returns a version where everything starts off Unset.

Next we want to implement the UserBuilder where we don't know what the generic types are.

### typestate 7

Each method here consumes the Builder and returns a new Builder with the data we wanted set.

This updates the type, setting the generic we care about to that particular type but keeping the others as whatever type
they were when the method was called.

For example, the "with_username" method will return a UserBuilder with the first generic parameter set to Username, but
the other two will just be whatever they were when that method was called.

### typestate 8

Same for the "with_email" method.

The "with_date_of_birth" method is a touch different because it returns a Result, but the Ok variant will still inherit
whatever concrete types U and E were set to previously.

Finally, we'll implement UserBuilder where all of our generics are now the types we want them to be.

### typestate 9

Here's where we put our finaliser method.

Because this method is only implemented for the concrete version of UserBuilder where all of our generics match the
types in our User struct, we can just copy them straight over.

When we come to use this pattern, we need to provide all the necessary data or the user can not be built.

### typestate 10

The build method simply doesn't exist unless we've done that, so this code won't compile, specifically telling us
that build does not exist for `UserBuilder<Username, Unset, Unset>`.

This is objectively... awesome.

There are two big pros to this pattern, the obvious one is that we can not muck up building our user type, but its also
worth pointing out that we've removed all but one branch in our code, which will provide a significant runtime benefit.

The downside though is this code is just more complex to look at, particularly for newer engineers where all those
triangle brackets might be intimidating.

Conclusion
----------

### conclusion 1

Should you just use the more complex but safer and faster TypeState Builder?

That probably depends on your team and your project requirements, but my advice is to trust yourself and your team.

It might require talking through a couple of times, especially if you're building something particularly complex with
your Builder, but getting out of our comfort zones helps us develop our skills.

### conclusion 2

If you enjoyed this video, don't forget to like and subscribe.

If you really liked the video, you can become a member of the channel or join the Patreon, see the description for more.

Next time we're going to talk about higher order functions, and do our best to demystify the fact Rust has numerous
closure types.

So join me next time for that.
