Fluent Interface
================

### intro 1

We often group data together into structs or objects.

> slide 1

For example, here's a simple User struct that has a username, an email and a date of birth.

### intro 2

Our User requires a Username, so we have a constructor that takes that, but email and date of birth are optional and
can be set later.

> slide 2

If we wanted to add them later, that code would look like this.

This immediately adds some complexity.

### intro 3

Depending on the nature of the data we might want to manage access through getters and setters, particularly if we need
to do any validation, or manipulation, etc.

> slide 3

In our `User` example, it might be good to not have to specify the `Option` when setting email and date of birth, which
can be done inside a setter.

> slide 4

This is a bit cleaner, and we now have a good place to put validation if we need it too.

### intro 4

However, this is still a little bit clunky as we have to refer to the underlying variable (`yuki`) multiple times.

We can make this more "fluent". 

> slide 5

In this version our setters are basically identical, but now we're returning a reference to the object being mutated.

> slide 6

This allows us to chain the setter methods, making the code cleaner and easier to read.


Rust Quirks
-----------

> camera

### quirks 1

There are two quirks to this pattern that impact Rust specifically.

Firstly, this pattern works in many languages, and particularly in languages that use things like pass by reference and
copy on write by default, you can sort of stop thinking about it there.

Rust has very specific memory management rules to think about though.

This means we need to consider whether our fluent interface will use references or take and return ownership.

### quirks 2

There are pros and cons to each approach.

> slide 6 still

Going back to our previous example, we need to make sure the owned data exists somewhere before we start modifying it,
and that the data is mutable.

That's why we stored `yuki` first, then modified it, effectively two steps.

> slide 7

Instead of taking references lets try taking ownership and returning a new version of the object _with_ the data we
want to add.

### quirks 3

Soooo... I've changed the prefix from "set" to "with" because this better reflects the "take ownership and return 
something new 'with' the extra data" _but_ I don't know if there's an official idiom or pattern like this, so I'm open
to better suggestions.

### quirks 4

Anyway, by passing ownership back and forth instead, we can now move both creation and adding additional data all into
a single chain:

> slide 8

In addition to this giving us a slightly nicer flow, you might notice that `yuki` doesn't need to be mutable, we're
encapsulating the mutability inside the methods where we need it.

> slide 9

However, there's a downside to this too. If we _did_ want to modify a single value, we'd have to make sure we take
ownership back from the method.

That said, because we're using the with prefix, you _could_ use both setters and... uh "withers".

### quirks 5

So that's the first quirk.

The other quirk is more of a nice touch. 

What if our setter could fail?

> slide 10

Obviously, it's Rust, we need to return a `Result`.

### quirks 6

In this case we'll check if a user is older than 21... 

No, this doesn't really make sense at this point, let's not worry too much about the domain logic for our demo app though

> slide 11

Luckily, the question mark operator allows us to unwrap an `Ok`, which means we can continue the chain so long as we're
able to bubble the error.

Conclusion
----------

### conclusion 1

In conclusion, using the FluentAPI pattern allows us to write much more readable code!

Not only do setters let us encapsulate validation as well as mundane things like wrapping our types in Options, but 
method chaining reduces code clutter.

Rust requires slightly more thought than some other languages but not having to think about Copy On Write is nice too. 

### conclusion 2

If you enjoyed this video, don't forget to like and subscribe.

If you really liked the video, you can become a member of the channel or join the Patreon, see the description for more.

Next time we're going to combine the TypeState pattern and Fluent API pattern into the Builder Pattern.

I hope I'll see you then.
