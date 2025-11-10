Typestate
=========

The typestate pattern encodes state into type information.

This is useful because this information exists when you write the program but not when you run it

This lets us enforce domain logic, that is, how we think about how our code maps to real world problems, with minimal
overhead.

State
-----

### state 1

Lets imagine a simplified pull request process.

We can open a new pull request, which then must be approved by at least one person, before it's merged.

Multiple people can approve it, but if anyone rejects it (even after its approved) it can no longer be merged.

### state 2

Let's try to model this in a "traditional way".

First we'll create a status enum that can be Open, Approved, Rejected or Merged

For the purpose of our example the PullRequest type will have status but we'll ignore any other data you might want to
put on there for now.

### state 3

A PR can be approved if the status is ReadyForReview or Approved.

If the status was not one of those, then we'll need to return an error to say that the status could not be changed.

Similarly, we can implement our reject and merge methods by checking to see if the PR is in a valid status.

### state 4

We've written up our three state change operations but we've had to write a lot of branching logic into each method to
check that the operation is only being run on a PR in a valid state.

If it wasn't in a valid state we produce errors that now need to be dealt with.

### state 5

> show typestate code

But what if instead of a PullRequest with a status, we represent that status in the Pull Requests type.

In this case we have a seperate pull request struct for each state the Pull Request can be in.

Now lets implement the `PullRequestOpen` struct.

### state 6

We can implement each state change method but because we already know what state we're in, we don't have to validate
the current state first, we simply return the new struct that represents the changed state.

Similarly we can implement `PullRequestApproved`s state change methods (Note this version of the pull request can also
be merged).

### state 7

What's really cool about this is that we don't need to check the state and return a Result because it's simply not
possible to make an invalid change.

The merge method, for example, _only_ exists on `PullRequestApproved`.

You can't merge a rejected pull request because `PullRequestRejected` doesn't have the necessary method.

Advnaced Typestate
------------------

### advanced 1

One downside to this method of doing typestate is that if our Pull Request structs have lots of data associated with
them (data we've convinently been skipping over for now), you have to repeatedly define that data for every type.

Heaven forbid you want to change something like the author value from a String to an Author type.

### advanced 2

Luckily, we can improve this pattern with generics.

We're going to need a special marker for this called PhantomData, we'll cover that later.

### advanced 3

Next we're going to create a trait for our state, this is just to restrict our Pull Request generic later.

We'll use unit structs to represent our states, and implement th trait for each of them.

We're going back to having a single type for our Pull Request on which we can keep all the relevant data, but it's a
generic.

The generic type S is restricted to PullRequestState.

### advanced 4

Now, importantnly, when we construct a generic type, we need to use the generic parameter somewhere inside the type to
make it a concrete type, but our state types are all zero width meaning they don't actually exist at runtime.

Rust gets a bit funny about this, which is why we use the PhantomData marker.

Weirdly, the PhantomData type is also zero width... if you know how or why this works, let me know in the comments.

### advanced 5

None the less, it does work, so we can use this to mark the State with zero width data that doesn't exist at Runtime.

I've also implemented a constructor to make instatiation of types a little less weird.

We can now implement our state specific state changes on only the PullRequests that are currently in the right state.

### advanced 6

This is our Open state and this is our Approved state.

This, in my opinion, visially challenging code but it works extremely well, and you get used to using it.

Conclusion
----------

### conclusion 1

The obvious downside, even with the more advanced version of the pattern, is the verbosity of having to move each item
from the old state to the new one.

However, you only have to write these methods once, but the benefits just keep giving.

Utilising the typestate pattern requires writing a lot more code _but_ that code has fewer branches, fewer (or in this 
case no) potential error states, and everything is logically subdivided making maintaining the code more trivial.

It's almost always going to be better to take a small upfront cost over for cheaper easier to use code in the long run.

### conclusion 2

If you enjoyed this video, don't forget to like and subscribe.

Next time we're going to look at how to write fluent APIs. 

Hopefully I'll see you then!
