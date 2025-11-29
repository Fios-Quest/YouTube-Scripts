Builder
=======

The builder pattern allows us to construct complex types in steps.

For example, in our previous video we had a 'User' struct that had a username, an email and a date of birth.

In that video, we allowed the date of birth to be set later, but a user also had to be older than 21 (which I mentioned
was silly to do after the user was created).

```rust
use std::str::FromStr;

#[derive(Debug)]
struct Username(String);

impl FromStr for Username {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct EmailAddress(String);

impl FromStr for EmailAddress {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct DateOfBirth(String);

impl FromStr for DateOfBirth {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}


#[derive(Debug)]
struct User {
    username: Username,
    email: Option<EmailAddress>,
    date_of_birth: Option<DateOfBirth>,
}
```

If we were to make all of these fields required, creating the struct gets a bit unweildy. Do we construct it manually or
write a constructor?

If we manually construct it, we can't validate the fields.

If we write a single constructor, the parameter names are slightly obfuscated, but worse than that, we have to cram all
our validation into a single function.

```rust
use std::str::FromStr;

#[derive(Debug)]
struct Username(String);

impl FromStr for Username {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct EmailAddress(String);

impl FromStr for EmailAddress {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct DateOfBirth(String);

impl DateOfBirth {
    fn get_age(&self) -> u8 {
        16
    }
}

impl FromStr for DateOfBirth {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct User {
    username: Username,
    email: EmailAddress,
    date_of_birth: DateOfBirth,
}

impl User {
    fn new(username: Username, email: EmailAddress, date_of_birth: DateOfBirth) -> Result<Self, ()> {
        if date_of_birth.get_age() < 21 {
            return Err(())
        }

        Ok(User {
            username,
            email,
            date_of_birth,
        })
    }
}

fn main() -> Result<(), ()> {
    let user = User {
        username: Username::from_str("Yuki")?,
        email: EmailAddress::from_str("yuki@example.com")?,
        date_of_birth: DateOfBirth::from_str("2009-05-01")?,
    };

    let user = User::new(
        Username::from_str("Yuki")?,
        EmailAddress::from_str("yuki@example.com")?,
        DateOfBirth::from_str("2009-05-01")?,
    );
    Ok(())
}
```

This isn't so bad in our example User struct, but what if you're working with a more complicated stucture with numerous
fields many of which require validation?

This is where the Builder Pattern comes in.

Instead of constructing the object directly, we use another type to manage the construction.

This Builder type collects data about the struct we want to create, then has a finaliser that creates the struct from
the data we've given it.

There are arguably two versions of this pattern: a simpler version that will work in just about any language that I'm
going to refer to as "Builder Lite", and a more complex version that only works in languages with strict generic typing,
the "[Typestate](./typestate.md) Builder".

Each has their own pros and cons, and we'll go over those too.

Builder Lite
------------

The traditional builder pattern uses a type that mirrors the type you want to build but everything is optional.

We use methods to update each field, and then have a finalizer that takes the data we've stored in the builder and
attempts to convert it into the target type.

For our `User` example that might look like this.


```rust
use std::fmt;
use std::str::FromStr;
use std::error::Error;

#[derive(Debug)]
struct ImpossibleError;

impl fmt::Display for ImpossibleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This error should be impossible!")
    }
}

impl Error for ImpossibleError {}

#[derive(Debug)]
struct Username(String);

impl FromStr for Username {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct EmailAddress(String);

impl FromStr for EmailAddress {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct DateOfBirth(String);

impl DateOfBirth {
    fn get_age(&self) -> u8 {
        16
    }
}

impl FromStr for DateOfBirth {
    type Err = ImpossibleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

#[derive(Debug)]
struct User {
    username: Username,
    email: EmailAddress,
    date_of_birth: DateOfBirth,
}

#[derive(Debug)]
enum UserBuilderError {
    NoUsername,
    NoEmailAddress,
    NoDateOfBirth,
    NotOldEnough,
}

impl fmt::Display for UserBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This error should be impossible!")
    }
}

impl Error for UserBuilderError {}

#[derive(Default)]
struct UserBuilder {
    username: Option<Username>,
    email: Option<EmailAddress>,
    date_of_birth: Option<DateOfBirth>,
}

impl UserBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_username(mut self, username: Username) -> Self {
        self.username = Some(username);
        self
    }

    fn with_email(mut self, email: EmailAddress) -> Self {
        self.email = Some(email);
        self
    }

    fn with_date_of_birth(mut self, date_of_birth: DateOfBirth) -> Result<Self, UserBuilderError> {
        if date_of_birth.get_age() < 21 {
            return Err(UserBuilderError::NotOldEnough);
        }
        self.date_of_birth = Some(date_of_birth);
        Ok(self)
    }

    fn build(self) -> Result<User, UserBuilderError> {
        let username = self.username.ok_or(UserBuilderError::NoUsername)?;
        let email = self.email.ok_or(UserBuilderError::NoEmailAddress?);
        let date_of_birth = self.date_of_birth;

        Ok (User {
            username,
            email,
            date_of_birth,            
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // We can successfully build a User if we have all the required information
    let user_result = UserBuilder::new()
        .with_username(Username::from_str("Yuki")?)
        .with_email(EmailAddress::from_str("yuki@example.com")?)
        .with_date_of_birth(DateOfBirth::from_str("2009-05-01")?)?
        .build();
    assert!(user_result.is_err());

    // But if we don't give all the required information we get an error
    let user_result = UserBuilder::new()
        .with_username(Username::from_str("Fio")?)
        .build();
    assert!(user_result.is_err());

    Ok(())
}
```

Typestate Builder
-----------------

In the previous example we need to deal with calling `.build()` on a builder that may not have all the required
information. To manage this potential problem we return a result. What if I told you, we can write this code in such
a way as to be sure that the `.build()` method can only be used once we can guarantee we have everything we need, thus
negating the Result requirement.

This is an advanced application of the [Typestate](./typestate.md) pattern. Instead of migrating between concrete
types representing individual states, we can use generics as markers on top of which we can implement different methods

The only slight trick is that generic types must be "used" _in_ our type. For example, the following won't compile 
because we didn't use "T" in the struct itself, even though our instantiation uses a Unit Struct:

```rust,compile_fail
struct BadExample<T> {
    data: String,
}

struct Marker;

# fn main() {
let example = BadExample::<Marker> { data: "This won't work".to_string() };
# }
```

This is where `PhantomData` comes in. It's a zero-sized marker that "uses" the types in generics, allowing you to use
generics as nothing more than a compile time marker.

```rust
use std::marker::PhantomData;

struct GoodExample<T> {
    data: String,
    marker_for_t: PhantomData<T>,
}

struct Marker;

# fn main() {
let example = GoodExample::<Marker> {
    data: "This will work!".to_string(),
    marker_for_t: PhantomData,
};
# }
```

Let's build our builder again using this method.

```rust
use std::error::Error;
use std::collections::HashSet;
use std::marker::PhantomData;

# // shush shush shush this code is supposed to look good, not be good
# struct Username(String);
# impl<S: ToString> From<S> for Username {
#     fn from(s: S) -> Self  {
#         Self(s.to_string())
#     }
# }
# #[derive(Eq, PartialEq, Hash)]
# struct EmailAddress(String);
# impl<S: ToString> From<S> for EmailAddress {
#     fn from(s: S) -> Self  {
#         Self(s.to_string())
#     }
# }
# struct DateOfBirth(String);
# impl From<String> for DateOfBirth {
#     fn from(s: String) -> Self  {
#         Self(s)
#     }
# }
# 
# struct User {
#     // Required valus
#     username: Username,
#     email: EmailAddress,
#     // Optional values
#     secondary_emails: HashSet<EmailAddress>,
#     date_of_birth: Option<DateOfBirth>,
# }
#
# enum UserBuilderError {
#     NoUsername,
#     NoPrimaryEmail,
# }
# 

// We'll use these unit structs as markers for when values are set
// Deriving Default means we can simplify our constructor
#[derive(Default)]
struct Set;
#[derive(Default)]
struct Unset;

#[derive(Default)]
struct UserBuilder<U, PE> {
    username: Option<Username>,
    email: Option<EmailAddress>,
    secondary_emails: HashSet<EmailAddress>,
    date_of_birth: Option<DateOfBirth>,
    // We need to record the markers somewhere but this won't impact runtime
    username_set: PhantomData<U>,
    email_set: PhantomData<PE>,
}

// The constructor only exists for UserBuilder<Unset, Unset> so that's how our
// builder starts
impl UserBuilder<Unset, Unset> {
    fn new() -> UserBuilder<Unset, Unset> {
        Self::default()
    }
}

impl<U, PE> UserBuilder<U, PE> {
    // When we set the username we have to completely migrate the type and
    // create a new struct for the generics to work.
    fn with_username(mut self, username: Username) -> UserBuilder<Set, PE> {
        UserBuilder {
            username: Some(username),
            email: self.email,
            secondary_emails: self.secondary_emails,
            date_of_birth: self.date_of_birth,
            username_set: PhantomData,
            email_set: PhantomData,
        }
    }

    // Same goes for primary email
    fn with_email(mut self, email: EmailAddress) -> UserBuilder<U, Set> {
        UserBuilder {
            username: self.username,
            email: Some(email),
            secondary_emails: self.secondary_emails,
            date_of_birth: self.date_of_birth,
            username_set: PhantomData,
            email_set: PhantomData,
        }
    }

    // The other setters return the same type including generics, whatever they were before
    fn add_secondary_emaail(mut self, email: EmailAddress) -> UserBuilder<U, PE> {
        self.secondary_emails.insert(email);
        self
    }

    fn with_date_of_birth(mut self, date_of_birth: DateOfBirth) -> UserBuilder<U, PE> {
        self.date_of_birth = Some(date_of_birth);
        self
    }
}

// By only implementing `.build()` on UserBuilder<Set, Set>, the method will only
// be available once both `.with_username()` and `.with_email()` are called.
impl UserBuilder<Set, Set> {
    fn build(self) -> User {
        let username = self.username
            .expect("It should not be possible to call build with username unset");
        let email = self.email
            .expect("It should not be possible to call build with primary email unset");
        let secondary_emails = self.secondary_emails;
        let date_of_birth = self.date_of_birth;

        User {
            username,
            email,
            secondary_emails,
            date_of_birth,
        }
    }
}

fn main () {
    // We can only build a User if we have all the required information
    let user = UserBuilder::new()
        .with_username(Username::from("Fio"))
        .with_email(EmailAddress::from("fio@example.com"))
        .build();

    // This won't compile because .build() only exists on UserBuilder<Set, Set>
    // let user_result = UserBuilder::new()
    //     .with_username(Username::from("Fio"))
    //     .build();
}
```

Using the typestate builder we prevent `.build()` being called unless all required data has been set. Rather than
runtime validation, we get compile time validation!

Pro's and Con's
---------------

I don't think there's one "right" builder to use.

Having compile time validation that you've used the builder correctly is nice, but it adds a lot of complexity through
the type system, and you can arguably be sure you've used the builder correctly through tests. But using the lite
builder adds more complexity to your tests and error handling code.

Its also worth noting that the typestate builder won't work if you can't guarantee each method is called at runtime. 
Methods that set a required parameter change the type of the builder, meaning you can't put a call in a branch (such as
an if) because you can't reconcile the types later.
