Fluent Interface
================

### intro 1

We often group data together into structs or objects.

> show no-setters

For example, here's a simple User struct that has a username, an email and a date of birth.

### intro 2

Our User requires a Username, so we have a constructor that takes that, but email and date of birth are optional and
can be set later.

If we wanted to add them later, that code would look like this.

This immediately adds some complexity.

### intro 3

Depending on the nature of the data we might want to manage access through getters and setters, particularly if we need
to do any validation, or manipulation, etc.

In our `User` example, it might be good to not have to specify the `Option` when setting email and date of birth.

Let's add some Setters to get around manually setting the Option.

> show intro

This is a bit cleaner, and we now have a good place to put validation.

### intro 4

However, this is still a little bit clunky as we have to refer to the underlying variable (`yuki`) multiple times.

We can make this more "fluent". 

> show fluent

In this version our setters are basically identical, but now we're returning a reference to the object being mutated.

This allows us to chain the setter methods, making the code clearer and easier to read.

Rust Quirks
-----------

### quirks 1

There are two quirks to this pattern that impact Rust specifically.

Firstly, this pattern works in many languages, and particularly in languages that use things like pass by reference and
copy on write by default, you can sort of stop thinking about it there.

Rust has very specific memory management rules to think about though.

This means we need to consider whether our fluent interface will use references or take and return ownership.

### quirks 2

There are pros and cons to each approach.

Going back to our previous example, we need to make sure the owned data exists somewhere before we start modifying it,
and that the data is mutable.

That's why we stored `yuki` first, then modified it, effectively two steps.

### quirks 3

Let's change using references to taking ownership and returning a new version of the object _with_ the data we wanted to
add.

Soooo... I've changed the prefix from "set" to "with" because this better reflects the "take ownership and return 
something new 'with' the extra data" _but_ I don't know if there's an official idiom or pattern like this, so I'm open
to better suggestions.

### quirks 4

Anyway, by passing ownership back and forth instead, we can now move both creation and adding additional data all into
a single chain:

> show fluent-owned

In addition to this giving us a slightly nicer flow, you might notice that `yuki` doesn't need to be mutable, we're
encapsulating the mutability inside the methods where we need it.

### quirks 5

However, there's a downside to this too. If we _did_ want to modify a single value, we'd have to make sure we take
ownership back from the method:

However, a benefit of using the with prefix means you _could_ use both setters and... uh "withers" together.

> show fluent-ownership-issue

So that's the first quirk.

The other quirk is more of a nice touch. 

What if our setter could fail? 

Obviously, it's Rust, we need to return a `Result`.

Luckily, the question mark operator allows us to unwrap an `Ok`, which means we can continue the chain so long as we're
able to bubble the error.

> show fluent-result

Conclusion
----------

In conclusion, using the FluentAPI pattern allows us to write much more readable code!

We can encapsulate validation as well as mundane things like wrapping our types in Options.

Rust requires slightly more thought than some other languages but not having to think about Copy On Write is nice too. 

---

Next time we're going to combine the TypeState pattern and Fluent API pattern into the Builder Pattern.

I hope I'll see you then.
