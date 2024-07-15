# Lifetimes in 60s

Rust lifetimes in 60 seconds

In Rust, all data is owned by something, let's pretend it's a crab.

When the crab goes out of scope, that data ceases to exist.

You can pass the crab up and down the stack, but if we had to pass ownership like this all the time it'd get really messy.

Instead, we can create a reference to data, let's pretend it's a kite.

The crab still owns the data, but the kite (reference) allows access to it.

The kite can now go out of scope, but the crab still keeps the data.

Lifetimes let us trace where the reference goes, like the string that connects the crab to the kite.

The kite can fly up and down the stack, so long as it never flies below the ground (where the crab is standing).

A function can take any number of kites in and return any number of kites out.

Sometimes it's obvious to us, which incoming kites are attached to which outgoing kites, we just have to tell the compiler which kite strings are which.

Sometimes we can't determine that, in which case we tie kite strings together and the returned kites can only go as low as the highest crab on the hill.
