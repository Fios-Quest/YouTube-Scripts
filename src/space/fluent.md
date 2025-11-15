Fluent Interface
================

We often group data together into structs or objects.

> show no-setters

Depending on the nature of the data we might want to manage access through getters and setters, particularly if we need
to do any validation, or manipulation, etc. In our `User` example above, it might be good to not have to specify the
`Option`.


> show unfluent

However, this is still a little bit clunky as we have to refer to the underlying variable (`yuki`) multiple times.

We can make this more "fluent" simply by returning a reference to that object after each call of a setter. This allows
us to chain the setter methods, making the code clearer and easier to read.

> show fluent

Rust Quirks
-----------

There are two quirks to this pattern that impact Rust specifically.

Firstly, this pattern works in many languages, and particularly in languages that use things like pass by reference and
copy on write by default, you can sort of stop thinking about it there. Rust has very specific memory management rules
to think about though.

This means we need to consider whether our fluent interface will use references or take and return ownership.

There are pros and cons to each approach.

Using references in our previous example, we need to make sure the owned data exists somewhere before we start modifying
it. That's why we stored `yuki` first, then modified it, effectively two steps.

If we passed ownership back and forth instead, we could create a single chain:

> show fluent

In addition to this giving us a slightly nicer flow, you might notice that `yuki` doesn't need to be mutable, we're
encapsulating the mutability inside the methods where we need it.

However, there's a downside to this too. If we _did_ want to modify a single value, we'd have to make sure we take
ownership back from the method:

> show fluent-ownership-issue

Which version you use will be up to you and likely depend on your circumstances. You could even mix and match (though
you will need to give the methods different names).

The other quirk is more of a nice touch. What if our setter could fail? Obviously, it's Rust, we need to return a
`Result`. Luckily, the question mark operator allows us to unwrap an `Ok`, which means we can continue the chain so
long as we're able to bubble the error.

> show fluent-result

Using this pattern allows us to write much more readable code!
