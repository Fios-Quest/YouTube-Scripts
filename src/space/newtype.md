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

[//]: # (show program 00_what_is_this)

Let's try parsing the data as a few things.

Here's the data.

For reference, this is it as bytes.

So it could be a huge integer

It's probably not a float though.

Ah, well, this looks right

So it's probably a short, UTF-8 encoded string advertising our new collection of videos, but, who's to say really?

### type 4

Types turn data into information.

Without knowing whether we're looking at a number or a sequence of ascii characters, we'd have a really hard time writing code and, more importantly, what we write would be very error-prone.

[//]: # (Without type information, there's nothing stopping us from accidentally passing a boolean to a function that expects a complex user structure.)

### type 5

[//]: # (We start having to depend constantly on runtime checks to make sure any data our functions receive is valid before trying to process it.)

All modern languages (even ones we may not usually think of being "typed") come with their own built-in types that usually cover, at the very least, floats, strings, lists and objects (or dictionaries).

This helps us reason about the data stored in memory.

### type 6

You'll notice I didn't include integers in the bare minimum types, this is because some languages opt to store _all_ numbers as double precision floating points.

This only really becomes an accuracy issue for integers over 2 to the power of 53 though, which is over 9 million billion, so I think we can give them a break.

### type 7

But is it enough to consider `"hello@example.com"` to be a string?

It technically is a string, sure, and we could manipulate it as one, but...

### type 8

in the context of our software, it might be that there is a meaningful difference between `"hello@example.com"` and `"Hello, example.com"`, in the same way as there is a meaningful difference between `"spaceFTW"` and

- <!-- 6 --> six quintillion
- <!-- 292 --> two hundred and ninty two quadrillion
- <!-- 731 --> seven hundred and thirty one trillion
- <!-- 980 --> nine hundred and eighty billion
- <!-- 616 --> six hundred and sixteen million
- <!-- 396 --> three hundred and ninty six thousand
- <!-- 915 --> nine hundred and fifteen

Why normal types aren't enough
------------------------------

[//]: # (show code 01_dates)

### problem 1

Let's imagine we wanted to represent years, months and days, we could do so with unsigned integers.

For simplicity, I've use `u64`s for everything.

### problem 2

Yes, we should probably use u8s for the month and day, but the size is irrelevant to the point, not every language has differently sized integers... or integers at all... and u64s will help me demonstrate a point at the end of the video.

However, we can immediately identify some problems with this.

First, there's nothing stopping us from using valid `u64`s to represent invalid data.

I can make all of these numbers invalid by changing them to zero

### problem 4

Second, there's nothing stopping us passing any valid number to a function that's supposed to take a month.

This means we need to validate the data inside every function that uses it.

[//]: # (show code 02_get_english_month_name)

### problem 3

In this example we've got a function that takes a month and returns the name of the month in English.

However, because you might pass in a `u64` that's not a valid month, that means we need to return a Result.

### problem 4

I'm using a unit struct for an error here, ideally, we'd go through the steps to impl Error, but I hope you'll forgive me if I skip that for this example.

Our function checks that the number given to it is between one and twelve.

Unfortunately, it doesn't stop us passing data to the function that isn't _supposed_ to be a month.

[//]: # (run code)

Rust is happy to let us pass in data that's _supposed_ to represent a year or day, and that's a mistake that may be missed even at runtime

### problem 5

What I'd like you to take away from this is that concepts of Days, Months and Years are meaningfully different within the domain of our application here. 

There is contextual information that we're ignoring about the data.

We need to know more about the "type" of data we're dealing with beyond it just being a number!

This is what the `newtype` pattern is for.

Introducing newtype
-------------------

### newtype 1

To oversimplify a touch, Newtypes, are wrappers around existing types that provide the context for the information stored within.

First, let’s prevent days being passed into functions that take months.

### newtype 2

[//]: # (show code 03_basic_newtype)

In this example I've wrapped our `u64`s in tuple structs, one for each of Year, Month and Day.

I've also modified our function to explicitly take the `Month` type.

### newtype 4

This is immediately more informative to anyone reading your code.

Even better, if we try to pass a Day into the function now, we get a compile time error (even if the numeric value _could_ be a month)

[//]: # (run code)

### newtype 5

Our second issue is that we can still produce invalid values such as `Month(13)`.

We can fix this by restricting the instantiation of our types to a constructor and validating the input.

### newtype 6

The question becomes, what we should do when someone attempts to use invalid data; I would argue we should return a Result with a relevant error.

Let's focus on `Month`.

### newtype 7

First, we need to make sure the interior of the struct can only be instantiated or edited by things we control.

In this code I've moved the Month newtype into a month module

This lets us protect the internal data, only things inside the module that can change our data must be explicitly marked as public.

### newtype 8

Now we can validate the month when it's constructed, returning the error if necessary.

Since we can no longer instantiate an invalid month, we can fairly confidently remove the result from any functions like `get_english_month_name`.

We still technically have to deal with the `u64`, but theoretically we can'

I'd say there's still one improvement we can make here, at least for months.

Our program is written in English, so why use numbers to represent the months in our code at all.

In fact, numbers are causing us a specific problem, even beyond readability.

In the `get_english_month_name` function, by matching on the numeric value we still have to show the compiler we're doing something with a number that's not 1-12, even if we're sure it isn't possible to have that as a value.

We can change the code representation of our Month without changing its underlying numeric representation by changing it to an enum.

```rust
mod month {
#     #[derive(Debug, PartialEq)]
    #[repr(u64)]
    pub enum Month {
        January = 1,
        February = 2,
        March = 3,
        April = 4,
        May = 5,
        June = 6,
        July = 7,
        August = 8,
        September = 9,
        October = 10,
        November = 11,
        December = 12,
    }

#     #[derive(Debug)]
    pub struct InvalidMonthNumber;
    
    impl Month {
        pub fn from_number(month: u64) -> Result<Month, InvalidMonthNumber> {
            match month  {
                1 => Ok(Month::January),
                2 => Ok(Month::February),
                3 => Ok(Month::March),
                4 => Ok(Month::April),
                5 => Ok(Month::May),
                6 => Ok(Month::June),
                7 => Ok(Month::July),
                8 => Ok(Month::August),
                9 => Ok(Month::September),
                10 => Ok(Month::October),
                11 => Ok(Month::November),
                12 => Ok(Month::December),
                _ => Err(InvalidMonthNumber),
            }
        }

        pub fn get_english_month_name(&self) -> String {
            match self  {
                Month::January => "January".to_string(),
                Month::February => "February".to_string(),
                Month::March => "March".to_string(),
                Month::April => "April".to_string(),
                Month::May => "May".to_string(),
                Month::June => "June".to_string(),
                Month::July => "July".to_string(),
                Month::August => "August".to_string(),
                Month::September => "September".to_string(),
                Month::October => "October".to_string(),
                Month::November => "November".to_string(),
                Month::December => "December".to_string(),
            }
        }
    }
}

use month::Month;

# fn main() {
let month = Month::from_number(9).expect("Somehow not September?!");
assert_eq!(month, Month::September);
println!("{}", month.get_english_month_name());
# assert_eq!(month.get_english_month_name(), "September".to_string());
# }
```

Now we've even removed the code branch that was theoretically impossible anyway.

###

It's worth pointing out that these types only exist at compile time.

In memory, in our running program, these types are all exactly the same:

```rust
#[repr(u64)]
pub enum MonthEnum {
    // ...snip
#     January = 1,
#     February = 2,
#     March = 3,
#     April = 4,
#     May = 5,
#     June = 6,
#     July = 7,
#     August = 8,
#     September = 9,
#     October = 10,
#     November = 11,
#     December = 12,
}

pub struct MonthStruct(u64);

fn get_memory<'a, T>(input: &'a T) -> &'a [u8] {
    // ...snip
#     // Credit: https://bennett.dev/rust/dump-struct-bytes/
#     unsafe {
#         std::slice::from_raw_parts(
#             input as *const _ as *const u8,
#             std::mem::size_of::<T>()
#         )
#     }
}


let sept_num: u64 = 9;
let sept_struct = MonthStruct(9);
let sept_enum = MonthEnum::September;

let num_bytes = get_memory(&sept_num);
let struct_bytes = get_memory(&sept_struct);
let enum_bytes = get_memory(&sept_enum);

println!("Num bytes: {num_bytes:?}");
println!("Struct bytes: {struct_bytes:?}");
println!("Enum bytes: {enum_bytes:?}");
#
# assert_eq!(num_bytes, struct_bytes);
# assert_eq!(struct_bytes, enum_bytes);
```

Tradeoffs?
----------

### tradeoffs 1

There seems to be a lot of extra work going on here.

We need to add more validation, extra error types (and all the extra work they're supposed to involve that we skipped here), not to mention how verbose the match statements were for the enum version of our month newtype.

That's true.

### tradeoffs 2

But this is because we're focusing on the newtype, not the impact that it has across your program.

By moving our validtion code into a single domain type, we're decluttering the rest of our program.

### tradeoffs 3

Let's think about a more complex type, like an email.

Using built-in types, we just create a validator and call it done:

```rust
fn is_valid_email_address<S>(email: S) -> bool
    where S: AsRef<str>
{
    let s = email.as_ref();
    // Must contain an @ that's not the first or last character
    s.contains('@')
        && s.chars().next() != Some('@')
        && s.chars().last() != Some('@')
}

// Tests
assert!(is_valid_email_address("a@b"));
assert!(!is_valid_email_address("@ab"));
assert!(!is_valid_email_address("ab@"));
```

This code is straightforward, terse and requires little testing.

However, everytime we want to use a string as an email, we will need to run the validator.

This not only could risk the same email needing to be validated multiple times, but adds some significant risk, particularly as our code evolves.

Any time we _don't_ use the validator for a function that accepts an email string because we perhaps initially create it only in a context where the string has already been validated, we risk that function being reused with no validation somewhere else.

> It's worth noting, we're using my "good enough, no false negatives" approach rather than a complex regex or parser, which would be even more computationally expensive!

> See https://emailregex.com/ for a completely compliant regex validation string, but... the only way to really know if an email address is valid is to email it.

Here's a newtype representing an Email:

```rust
mod email_address {
    use std::{fmt, error::Error};

    #[derive(Debug)]
    pub struct InvalidEmailAddressError;

    impl fmt::Display for InvalidEmailAddressError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Invalid Email Address")
        }
    }

    impl Error for InvalidEmailAddressError {}

    pub struct EmailAddress(String);

    impl EmailAddress {
        pub fn from_string<S>(email: S) -> Result<EmailAddress, InvalidEmailAddressError>
            where S: ToString + AsRef<str>
        {
            match (Self::is_valid(&email)) {
                true => Ok(EmailAddress(email.to_string())),
                false => Err(InvalidEmailAddressError),
            }
        }

        pub fn is_valid<S>(email: S) -> bool
            where S: AsRef<str>
        {
            let s = email.as_ref();
            // Must contain an @ that's not the first or last character
            s.contains('@')
                && s.chars().next() != Some('@')
                && s.chars().last() != Some('@')
        }
    }
}

fn main() {
    use email_address::EmailAddress;

    // Tests
    let valid_email = EmailAddress::from_string("hello@example.com");
    let invalid_email = EmailAddress::from_string("Ted");

    assert!(valid_email.is_ok());
    assert!(invalid_email.is_err());

    assert!(EmailAddress::is_valid("a@b"));
    assert!(!EmailAddress::is_valid("@ab"));
    assert!(!EmailAddress::is_valid("ab@"));
}
```

We've added an Error type for potentially invalid addresses and a constructor... but our validator is identical, and we only add two new tests.

Now, though, we only ever need to validate the email when we create the data type, which will usually be when we're getting that data from an external source, for example, from a user or importing it from a database.

This will also likely be where we deal with any potential validation errors, further simplifying our code.

Conclusion
----------

### conclusion 1

For a small amount of extra work, newtypes give us:

- Centralised validation and error handling
- Reduced complexity
- More defensive code

### conclusion 2

If you enjoyed this video, don't forget to like and subscribe.

Next time we're going to look at another way the type system can be leveraged to make more rigorous code with the type-state pattern, hopefully I'll see you then!
