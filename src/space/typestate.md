Typestate
=========

The typestate pattern encodes state into type information.

This is useful because, as we showed last time, this information exists when you write the program but not when you run 
it!

This lets us enforce domain logic, that is, how we map our code to real world problems, with minimal overhead.

State
-----

### state 1

> slide 1

Let's imagine a simplified pull request process.

Once a pull request ^ is open, it can ^ be approved ^ or rejected.

Multiple people ^ can approve it, but if anyone rejects it ^ (even after its approved) it can no longer be merged.

So long as the pull request is in the approved state, then ^ it can be merged.

### state 2

Let's try to model this in a "traditional" *cough* bad *cough* way.

> slide 2

First we'll create a status enum that can be Open, Approved, Rejected or Merged

> click

For the purpose of our example the PullRequest type will have status, but we'll ignore any other data you might want to
put on there for now.

### state 3

> slide 3

Our PR struct has three methods, approve, reject and merge.

### state 4

> slide 4

A PR can be approved if the status is Open or Approved.

If the status was not one of those, then we'll need to return an error to say that the status could not be changed.

> slide 5

Similarly, our reject method will work if the status is open, or approved, but not if the PR has been merged.

In this case I've also allowed you to Reject a Rejected PR _because_ it didn't make sense to me for that to be an error
even though it's not part of our process.

> slide 6

Finally, our merge method only works if the PR is in the approved status, otherwise it will return an error.

### state 5

We've written up our three state change operations, but we've had to write a lot of branching logic into each method to
check that the operation is only being run on a PR in a valid state.

> slide 8

To show how complex this gets, I'm listing here the tests for every valid state change to make sure they are successful,
and, ^ the tests for every invalid state change to make sure they are not.

Typestate
---------

> camera, transition to slide 8

### typestate 1

But what if instead of a PullRequest with a status, we represent that status in the Pull Requests type.

> slide 9

In this case we have a separate pull request struct for each state the Pull Request can be in; Open, Approved, Rejected,
Merged.

### typestate 2

Now we can implement each struct separately.

> slide 10

In PullRequestOpen, we can add the "approve" and "reject" methods and there's no need for branching or errors because we
already know the PullRequest is in a valid state.

> slide 11

Similarly, we can implement the `PullRequestApproved` methods and again, there's no need for branching given our simple
 logic.

### typestate 3

What's really cool about this is that we don't need to check the state and return a Result because it's simply not
possible to make an invalid change.

The merge method, for example, _only_ exists on `PullRequestApproved`.

You can't merge a rejected pull request because `PullRequestRejected` doesn't have the necessary method.

### typestate 4

> camera

This also means we don't need to write tests for invalid state changes because those methods do not exist on invalid
types.

Advanced Typestate
------------------

### advanced 1

One downside to this method of doing typestate is that if our Pull Request structs have lots of data associated with
them (data we've conveniently been skipping over for these examples), you have to repeatedly define that data for every 
type.

Heaven forbid you want to change something like the author value from a String to an Author newtype, you'll have to 
change it _everywhere_.

### advanced 2

Luckily, we can improve this pattern with generics.

> slide 12

First, we'll create a trait for our state, this is just to restrict our Pull Request generic later.

> slide 13

We'll use unit structs to represent our states, ^ and implement the trait for each of them.

> slide 15

We're going back to having a single type for our Pull Request on which we can keep all the relevant data, but it's a
generic.

The generic type S is restricted to PullRequestState.

### advanced 3

Now, importantly, when we create a generic type, we need to use the generic parameter somewhere inside it.

### advanced 5

What's cool though is that we could, if we needed to, attach state specific fields to the state structs instead of to
the pull request.

> slide 16

Moving on, I've created a constructor to open a new PullRequest, and so long as this is the only method called Open, ^ 
there is no ambiguity, so we don't need to specify the generic to use it.

### advanced 6

We can now implement our state specific state changes on only the PullRequests that are currently in the right state.

> slide 17

For the Open variant of our PullRequest we can approve or reject them.

As with the previous version there is no branching, we already know we're in the correct state so we can move our data
and return the new type.

Now you might be thinking, can't we just spread the old PullRequest into the new one, they're both PullRequest types?

Unfortunately, the generic means that the concrete types are different and Rust won't allow us to just spread one into
the other.

### advanced 7

One way to mitigate the extra work this might create is to group properties into subtypes that could be moved more
easily.

> slide 18

Our Approved variant can be approved by more people, be rejected, or be merged.

As with our previous implementation of the pattern, Rejected and Merged pull requests can not have their status changed
so we don't need to add any methods to these types.

Conclusion
----------

### conclusion 1

The obvious downside to the pattern, at least in Rust, is having to copy each property across manually.

However, you only have to write these methods once, but the benefits just keep giving.

Utilising the typestate pattern requires writing a lot more code initially _but_ that code has fewer branches, fewer (or
in this case no) potential error states, and everything is logically subdivided making maintaining the code more trivial.

It's almost always going to be better to take a small upfront cost for cheaper easier to use code in the long run.

### conclusion 2

If you enjoyed this video, don't forget to like and subscribe.

If you really liked the video, you can now become a member of the channel, or join the Patreon which I dropped the price
on to match YouTube.

Next time we're going to look at how to write fluent APIs. 

Hopefully I'll see you then!
