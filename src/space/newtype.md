NewType
=======

What is a type?
---------------

### type 1

Why do we use types?

Forget newtypes, for now, why do we use types at all?

If I show you this binary value, can you tell me what it is?

### type 2

We can rule out a few things.

It's 64 bits, which means it's definitely not a boolean or a character.

It's also too long for an IPv4 address, and too short for an IPv6 address.

More likely it's a number, either an integer or a float.

### type 3

> show program 00_what_is_this

Let's try parsing the data as a few things.

Here's the data.

For reference, this is it as bytes.

So it could be a huge integer

It's probably not a float though.

Ah, well, this looks right

So it's probably a short, UTF-8 encoded string advertising our new collection of videos, but, who's to say really?

Without knowing for sure, we're just guessing.

### type 4

Types turn data into information.

Without knowing whether we're looking at a number or a sequence of ascii characters, we'd have a really hard time
writing code and, more importantly, what we write would be very error-prone.

Without type information, there's nothing stopping us from accidentally passing a boolean to a function that expects a
complex user structure.

### type 5

We start having to depend constantly on runtime checks to make sure any data our functions receive is valid before
trying to process it.

All modern languages (even ones we may not usually think of being "typed") come with their own built-in types that
usually cover, at the very least, floats, strings, lists and objects (or dictionaries).

This helps us reason about the data stored in memory.

### type 6

You'll notice I didn't include integers in the bare minimum types, this is because some languages opt to store _all_
numbers as double precision floating points.

This only really becomes an accuracy issue for integers over 2 to the power of 53 though, which is over 9 million
billion, so I think we can give them a break.

### type 7

But is it enough to consider `"hello@example.com"` to be a string?

It technically is a string, sure, and we could manipulate it as one, but...

### type 8

In the context of our software, it might be that there is a meaningful difference between `"hello@example.com"` and
`"Hello, example.com"`, in the same way as there is a meaningful difference between `"spaceFTW"` and

- <!-- 6 --> six quintillion
- <!-- 292 --> two hundred and ninty two quadrillion
- <!-- 731 --> seven hundred and thirty one trillion
- <!-- 980 --> nine hundred and eighty billion
- <!-- 616 --> six hundred and sixteen million
- <!-- 396 --> three hundred and ninty six thousand
- <!-- 915 --> nine hundred and fifteen

Why normal types aren't enough
------------------------------

> show code 01_dates

### problem 1

Here's some code that represents years, months and days.

For simplicity, I've use `u64`s for everything.

### problem 2

Yes, we should probably use `u8`s for the month and day, but the size is irrelevant to the point, not every language has
differently sized integers... or as we discussed, integers at all... and `u64`s will help me demonstrate a point at the
end of the video.

### problem 3

However, we can immediately identify some problems with this.

First, there's nothing stopping us from using valid `u64`s to represent invalid data.

I can make all of these numbers invalid by changing them to zero

### problem 4

Second, there's nothing stopping us passing any valid number to a function that's supposed to take a month.

> show code 02_get_english_month_name

### problem 3

In this example we've got a function that takes a month and returns the name of the month in English.

However, because you might pass in a `u64` that's not a valid month, that means we need to validate the parameter and
return a Result.

### problem 4

I'm using a unit struct for an error here, ideally, we'd go through the steps to impl Error, but I hope you'll forgive
me if I skip that for this example.

### problem 5

Our function checks that the number given to it is between one and twelve.

Unfortunately, it doesn't stop us passing data to the function that isn't _supposed_ to be a month.

> run code

Rust is happy to let us pass in data that's _supposed_ to represent a year or day, and that's a mistake that may be
missed even at runtime

### problem 5

What I'd like you to take away from this is that concepts of Days, Months and Years are meaningfully different within
the domain of our application here. 

There is contextual information that we're ignoring about the data.

We need to know more about the "type" of data we're dealing with beyond it just being a number!

This is what the `newtype` pattern is for.

Introducing newtype
-------------------

### newtype 1

To oversimplify a touch, Newtypes, are wrappers around existing types that provide the context for the information
stored within.

First, let’s prevent days being passed into functions that take months.

### newtype 2

> show code 03_basic_newtype

Here, I've wrapped our `u64`s in tuple structs, one for each of Year, Month and Day.

I've also modified our function to explicitly take the `Month` type.

### newtype 4

This is immediately more informative to anyone reading your code.

Even better, if we try to pass a Year or Day into the function, we get a compile time error that very clearly tells us
what's wrong.

### newtype 5

Our second issue is that we can still produce invalid values such as `Month(13)`.

We can fix this by controlling access to the inner data.

For example, we could restrict the instantiation of our types to a constructor that validates the input.

### newtype 6

Let's focus on `Month`.

### newtype 7

> show code 04_month_newtype

We need to make sure the interior of the struct can only be instantiated or edited by things we control.

In this code I've moved the Month newtype into a month module

This lets us protect the internal data so only things inside the module can see or change it.

And we can control exposure through marking things explicitly as public.

So the interior of our Unit struct is not public, but our constructor is.

### newtype 8

Now we can validate the month when it's constructed, returning the error if necessary.

Since we can no longer instantiate an invalid month, we can fairly confidently remove the result from any functions like
`get_english_month_name`.

We still technically have to deal with every possible `u64` in the match statement, but theoretically it has to be
between 1 and 12, and if it's not, something has gone badly wrong so I think a panic is now in order.

### newtype 9

I'd say there's still one improvement we can make here, at least for months.

Our program is written in English, so why use numbers to represent the months in our code at all.

Plu, dealing with numbers are why we still need to have that panic in our  `get_english_month_name` function.

### newtype 10

> show code 05_enum_newtype

We can change how we model Month in our code without changing its underlying numeric representation by changing it to
an enum and specifying the enums discriminant type with the repr attribute.

### newtype 10

Depending on your data you obviously may not need to specify this, and again, a `u8` would be more appropriate here, but
I still need to make that point later. :)

Anyway, by using an enum, we can remove the code branch that was theoretically impossible anyway AND I'd say this is a
lot easier for anyone reading th code to understand.

### newtype 11

So, that big point I've been working up to... these types only exist at compile time.

In memory, in our running program, these types are all exactly the same:

### newtype 12

> show code 06_newtype_size

In this program we construct each of the Month types we've discussed (I've had to give them different names here but
don't worry they're otherwise identical)

We've got our `u64`, our tuple struct, and our enum versions.

Each version looks very different but conceptualy represent the same information.

I found this function that can extract the memory of any given value regardless of type.

Passing each version of our month in for September gives us back a byte array.

Exactly the same byte array in fact.

This is why I stuck with `u64`s for what its worth. 

### newtype 13

Types aren't real.

They provide us, the software engineers, information we need to structure our programs better and we can use the type
system to prevent us from making mistakes.

But at runtime, it all dissapears

Tradeoffs?
----------

### tradeoffs 1

There seems to be a lot of extra work going on here.

We need to add more validation, extra error types (and all the extra work they're supposed to involve that we skipped
here), not to mention how verbose the match statements were for the enum version of our month newtype.

That's true.

### tradeoffs 2

But we've been focused on the newtype, not the impact that it has across your program.

By moving our validation code into a single domain type, we're decluttering the rest of our program.

### tradeoffs 3

> show code 07_newtype_size

Let's think about a more complex type, like an email.

Using built-in types, we just create a validator and call it done:

### tradeoffs 4

This code is straightforward, terse and requires little testing.

However, everytime we want to use a string as an email, we will need to run the validator.

This not only could risk the same email needing to be validated multiple times, but adds some significant risk,
particularly as our code evolves.

Any time we _don't_ use the validator for a function that accepts an email string because we perhaps initially create it
only in a context where the string has already been validated, we risk that function being reused with no validation
somewhere else.

### tradeoffs 5

Worse, it's easy to imagine that our software might end up with multiple validation functions, each following slightly
different rules.

In this example we're using my "good enough, no false negatives" approach, but another engineer might write a different
validator, perhaps following emailregex.com which catches more bad emails but has a small chance of flagging a false
negative.

Then we have two models of what an email should be inside our software, and they no longer agree with each other.

### tradeoffs 6

> show code 08_email_newtype

Here's what it might look like to make an email newtype.

We've added an Error type for potentially invalid addresses and, this time I did those extra steps to implement the
Error trait.

Next we've got our a constructor which calls to our validator, though the validator logic is identical.

We've added one new test for the constructor, and the one for our validator is the same.

### tradeoffs 7

Now, though, we only ever need to validate the email when we create the data type, which will usually be when we're
getting that data from an external source, for example, from a user or importing it from a database.

This will also mean we only have to look in one place if we need to change our Email logic, or fix any bugs.

Conclusion
----------

### conclusion 1

For a small amount of extra work, newtypes give us:

- Centralised validation and error handling
- Reduced complexity
- More information to understand the domain we're working in
- And more defensive, safer to use code

### conclusion 2

If you enjoyed this video, don't forget to like and subscribe.

Next time we're going to look at another way the type system can be leveraged to make more rigorous code with the
type-state pattern, if that sounds familiar it might be because we covered it in the IRISS series.

hopefully I'll see you then!
