# Iterators

I've been foreshadowing it for a while, but today we finally cover Iterators!

Iterators are a way to produce, and perform operations on, a sequence of values.

We often use them with collections (which we covered in our last video) so that we can perform the same operation on
each item in a collection, or reduce a collection to a single value.

They're also often used implicitly in some forms of loop.

As ever, this series is accompanied by a free book, check the description for a link straight to this chapter.

My name is Daniel, welcome to iris.

## The Iterator Trait

The Iterator trait can be applied to any type and has a single required method:

![iter-rust-doc.png](014-iterators/iter-rust-doc.png)

📕 `.next()` returns an `Option` telling the caller there either is another item (Some), or there isn't (None).

📕 Iterator also has 75 provided methods, which I think goes to show how incredibly versatile this trait is.

📕 We'll be talking about some of these methods, but it's well worth checking out the documentation to see what else is
possible.

While you'll usually get iterators from things like collections, it's possible to "Iterate" through anything.

To show the power of Iterators, we're going to start by building an Iterator from scratch, one that produces the
Fibonacci sequence.

The Fibonacci sequence is a sequence of numbers where each number is the sum of the previous two.

Depending on whom you ask, the sequence either starts, `1, 2`, `1, 1`, or `0, 1`.

Purists will say it's the former (as "Leonardo Bonacci" intended), software engineers usually use the latter... I'm
sticking mine in the middle, but it really doesn't matter.

🦀 Anyway, let's start by making a Struct to store the state of the Iterator.

![fib-base.png](014-iterators/fib-base.png)

🦀 We'll need to know the previous number and the next number, and to keep it simple we'll use a `u8` so we'll only get
numbers zero to two-five-five.

🦀 Let's make a constructor so that the Iterator always starts in a valid state.

🦀 So, let's move on to implementing Iterator itself.

![fib-impl-iterator.png](014-iterators/fib-impl-iterator.png)

🦀 The trait has an associated type `Item` that describes the type returned by each call of `.next()` on the iterator.

🦀 It's an associated type rather than a generic type as it needs to be referenced a _lot_ but, it's type is dictated by
the process used to create the Iterator, so it isn't generic.

🦀 To implement the `next` method, we'll temporarily store the current value of the value `next`.

🦀 Because `self.next` is an Option, and the `next` method returns an Option, we can use the question mark operator
to either unwrap a `Some` variant, or immediate return a `None` variant depending on what's in there.

🦀 If there is a `None` at this point, then we've reached the end of our sequence.

🦀 Next we'll update our internal state, our new next is the current value plus the previous, and our previous becomes
the old current.

🦀 By using `checked_add` here we get an `Option` that is `None` if the result is out of bounds.

🦀 Finally, we return the stored "current" value, though we'll have to re-wrap it in an Option after unwrapping it
earlier.

🦀 So now we have our iterator type!

![fib-next-next-next.png](014-iterators/fib-next-next-next.png)

🦀 We can now get each item off the iterator one at a time by calling the next function:

🦀 You can see that each item in the sequence is wrapped in an `Option`.

🦀 When an Iterator has no more items to provide, it will produce a `None`.

🦀 The final number this Iterator will produce is `233`, after which we would overflow the `u8` we've used.

🦀 Just calling `.next()` is pretty boring, no one wants to iterate through things by hand.

🦀 What if we want to print out all the Fibonacci values that fit inside a `u8`?

![fib-for-in.png](014-iterators/fib-for-in.png)

🦀 You can give an Iterator to a `for ... in ...` loop, and it will automatically unwrap the `Option` for you.

🦀 This code will print out each number on a new line

🦀 Once the loop hits a `None` the loop ends.

That's cool, but on its own, it's still not very interesting.

Iterators are designed to be chained.

Those 75 provided methods I mentioned earlier allow you to do some exceptional tricks.

🦀 For example, a list of Fibonacci numbers might be more useful if we knew what number in the sequence we're on.

![fib-for-in-enumerate.png](014-iterators/fib-for-in-enumerate.png)

🦀 We can chain a method called `.enumerate` which will take the old iterator and give us a new one where each `next` now
returns a tuple of `(usize, T)` where the `T` is the original item and the `usize` is the position.

🦀 What's brilliant about this though is that when I say it "takes the old iterator", it doesn't try to process every
item in the iterator (a process in Rust we refer to as "consuming" the iterator), it merely takes ownership of it.

🦀 When we call `.next()` on the iterator returned by `.enumerate()`, _it_ calls next on the iterator being enumerated.

Rust iterators are "lazy" meaning that they try to avoid doing any unnecessary work.

## Getting Iterators

Having built our own, hopefully you now have a _vague_ understanding of how Iterators work, but usually you'll get an
Iterator from a collection.

As with most things in Rust, Iterators (or specifically, the items being iterated) can be thought of in three groups.

All the collections we discussed in the last video can give you an iterator in any of the following forms.

Firstly, referenced data

Often we don't need to _own_ the data we're iterating over, it can be enough to just read it.

All built in collections have a method called `.iter()` which returns an Iterator type where the items are references to
the data held in the collection.

🦀 Here we have two variables that _own_ the Strings inside them.

![iter-iter.png](014-iterators/iter-iter.png)

🦀 We move ownership of the variables inside the Vector

🦀 And then we create an Iterator with the Iter method which will iterate over references that point to the data now
owned by the vector

🦀 So calling `.next()` on the iterator gives as a reference, not the original data.

🦀 This means the vector still owns the data

🦀 One thing to bear in mind is that if the collection contains references, then `.iter()` will give you an Iterator that
produces references to references.

![iter-iter-ref.png](014-iterators/iter-iter-ref.png)

🦀 So if we change the original vector to reference the Strings rather than take ownership

🦀 When we call `.next()` we get a reference to a reference

Sometimes, you need to edit things while iterating through them; our second option lets us do that.

`.iter_mut()` can give you a mutable iterator, and all of Rust's built-in collections support it (so long as the
underlying collection is mutable).

🦀 In this example, we'll use a Vector of numbers, as I mentioned, we need this Vector to be mutable.

![iter-iter-mut.png](014-iterators/iter-iter-mut.png)

🦀 We'll use a for ... in ... loop like earlier, using `.iter_mut()`.

🦀 So here, n, is a mutable reference to the value stored in the vector.

🦀 By dereferencing n, we can add 10 to the original value.

🦀 If we check the original Vector now, we can see all values have increased by 10.

Finally, you may want to take ownership of the underlying data, and that's where our third option comes in.

`.into_iter()` takes ownership of the collection and the data inside (meaning that the collection will no longer be available).

One place this is particularly useful is when converting between types, either converting the items themselves or for the entire collection.

There is a trait called `FromIterator<A>` that is implemented for types that can consume an iterator and populate themselves.

This is almost always used with the `.collect()` iterator method, though you need to be explicit about what you're collecting into.

You can do this either by typing the variable you're collecting into, or by using the turbofish operator that allows you to be explicit about the concrete types to fill in generics.

🦀 In this example, we'll go back to our Strings as they aren't `Copy`.

![iter-into-iter.png](014-iterators/iter-into-iter.png)

🦀 Instead of using a Vector this time, we'll use a LinkedList, and push our strings onto the end.

🦀 This moves ownership from the variables into the LinkedList.

🦀 To turn the LinkedList into a Vector, we'll first convert the LinkedList into an Iterator that owns the original data with `.into_iter()`.

🦀 Then we'll "collect" that Iterator into a collection.

🦀 Because `v` is explicitly typed, Rust knows to use the `FromIterator` implementation of `Vector` when calling `.collect()`.

Honestly, even after years of using Rust, this backwards way of writing code that makes it super modular still makes me think: "wow"

🦀 In this example, we've created a variable _basically_ just to provide type information.

![iter-into-iter-turbofish.png](014-iterators/iter-into-iter-turbofish.png)

🦀 You can skip this step using the turbofish operator which looks like this.


## Copying and cloning Items

Using what we've learned above, what if we want to use owned data, but we need to keep the original collection, so `.into_iter()` is out of the question?

There are two methods on `Iterator` for this purpose: `.copied()` and `.cloned()`.

Each one takes the old iterator and returns a new iterator that applies the appropriate action lazily as its called.

`.copied()` only works on Iterators where the item is `Copy` and will take the iterator and return a new iterator which returns each Item copied.

![copied.png](014-iterators/copied.png)

`.cloned()` does the same for types that are `Clone`.

![cloned.png](014-iterators/cloned.png)

## Other Ways to get Iterators

Beyond collections, there are other things that can be iterated through.

🦀 Ranges _are_ iterators, it's why you often see them used in for loops:

![other-iter-range.png](014-iterators/other-iter-range.png)

🦀 But they implement all the same methods so we can collect them like any other iterator.

🦀 You can make an infinitely repeating iterator of anything so long as it implements `Clone`, using `std::iter::repeat`.

![other-iter-repeat.png](014-iterators/other-iter-repeat.png)

🦀 Now, because immutable references implement `Clone` we could use this function to repeat a reference to string slice like "hi".

// FIXME

🦀 Um, I said "hi".

🦀 Wait!

🦀 It literally never ends, every time you call `.next()` it just clones a new value

🦀 Ok enough of that.

🦀 We can also repeat existing iterators infinitely using a method on the iterator called `.cycle()`

![other-iter-cycle.png](014-iterators/other-iter-cycle.png)

🦀 Once the iterator has run out of items, instead of returning a `None`, it simply starts again.

You can also create iterators by combining other iterators with `.chain()`, although they have to be of the same type:

![other-iter-chain.png](014-iterators/other-iter-chain.png)

Many other Types in Rust can also be broken down into Iterators.

The script for this video, for example, can be represented as one large `str`, which you can break the data down by `.lines()`, `.chars()` or `.bytes()` all of which produce iterators.

## Cool ways to use Iterators

### Mathematics

One common thing we might want to do is consume an iterator of numeric values and get some new value from it.

A quick warning though!

Unlike some other operations we might perform on iterators which add some step that _will_ be performed when we call `.next`, methods that "consume" the Iterator will actually try to process everything in the Iterator.

This means if you call them on an infinite iterator, like those created by `.repeat()` or `.cycle()`, your code will enter an infinite loop and never end.

However, when that's not the case there's some useful mechanisms we can use:

🦀 For iterators of items that implement the `Sum` trait (for example numbers) `.sum()` will add all the items in the iterator:

![maths-sum.png](014-iterators/maths-sum.png)

🦀 You'll notice we have the turbofish operator again as we need to know what type to sum to

🦀 For iterators of items that implement the `Product` trait (eg, again, numbers) `.product()` will multiply all the items in the iterator

![maths-product.png](014-iterators/maths-product.png)

🦀 Its worth noting that some surprising things implement `Sum` and `Product`, including blanket implementations for `Option<T>` and `Result<T, E>` where `T` already implements the trait.

![maths-sum-option.png](014-iterators/maths-sum-option.png)

🦀 Doing it this way, for some reason, the Option _needs_ to be owned, so we can `.into_iter()` on the collection, if we don't need to use the collection afterward.

🦀 Or, if we need to keep the collection, because i32 is Copy, we could chain `.iter()` with `.copied()`

![maths-sum-option-copied.png](014-iterators/maths-sum-option-copied.png)

🦀 I want to add another quick warning here, `.sum()` and `.product()` use the basic `add` and `multiply` operators respectively.

🦀 This can be problematic because there's no check to see if the result still fits inside the numeric type.

🦀 For example, this is fine

![maths-product-safe.png](014-iterators/maths-product-safe.png)

// FIXME

🦀 But this will panic

![maths-product-panic.png](014-iterators/maths-product-panic.png)

🦀 But you won't know that until runtime.

🦀 Usually this won't be a problem as you'll likely be using number types with a lot of space, not `u8`s, but we'll touch on slower (but safer) ways to get around this later.

🦀 Anyway, carrying on:

🦀 For iterators of items that implement `Ord` you can use `.min()` and `.max()` to find the largest and smallest values respectively.

![maths-min-max.png](014-iterators/maths-min-max.png)

🦀 Chars are ordered by their numeric value so capitals come first, making "H" the smallest value and "w" the "largest" value

🦀 If you just want to know how many items there are, you can use `.count()` which merely tells us how many items are in an iterator.

// Count consumes iterator

![maths-count.png](014-iterators/maths-count.png)

🦀 However, if the iterator implements `ExactSizeIterator`, and many iterators do, then you can use `.len()` instead.

![maths-count.png](014-iterators/maths-len.png)

🦀 Not only does this not consume the iterator, it's almost always faster to get the same result:

### Applying a Process over each item

One of the most common uses for Iterators is to process a set of Items one at a time.

There are a number of methods on the Iterator trait (that themselves return new Iterators) that are really helpful for this.

In fact, it's common to chain multiple iterators together in this way.

Let's start with one of the simplest.

You can take an iterator and exclude Items based on the result of a predicate using the `.filter()`.

🦀 For example, we could take a range of numbers, and filter out all odd numbers like this:

![process-filter-range.png](014-iterators/process-filter-range.png)

🦀 If we were to look at the count of the iterators before and after the filter, you'll see they've changed!

🦀 Now, I was going to point out that ideally we should use `.len()` but that `.filter()` can not return an `ExactSizeIterator`.

![process-filter-range-inclusive.png](014-iterators/process-filter-range-inclusive.png)

🦀 But, to my surprise, `ExactSizeIterator` isn't implemented for `RangeInclusive` either... but it is for `Range`.

🦀 So, here's a quick edit to the code that does the same thing.

🦀 Now we can use `.len()` on the range, much better

🦀 For the obvious reasons we don't know if an Item is included or not before an item is processed by the filter, so we don't get an `ExactSizeIterator` back from the `.filter()` method, so we have to count each item.

🦀 I don't know why `RangeInclusive` isn't `ExactSizeIterator`, if you do, let me know in the comments, but I think it's a sign to prefer `Range` over `RangeInclusive`.

Anyway, another great way to process Iterators one Item at a time is to take that Item and transform it in some way.

We can pass a function into the `.map()` method that receives the item and returns a new value.

If that value is of a different type, the Iterator you get back will also be of that new type:

🦀 If we start off with a Range like this, this is an Iterator where the Item is of type `i32`.

![process-map.png](014-iterators/process-map.png)

🦀 We can then use `.map()` to take the number and return the result of the format macro.

🦀 And now we have an Iterator where the Item is of type `String`.

🦀 Sometimes the process you apply to an item might itself result in an `Option`, and rather than having an iterator of `Options` you may want to discard `None`s and unwrap `Ok`s, this is where `.filter_map()` is really handy.

![process-filter-map.png](014-iterators/process-filter-map.png)

🦀 This range gives us every valid `u8` number in sequence from smallest, 0, to largest, 255 

🦀 (Despite what we just discussed this time it has to be `RangeInclusive`).

🦀 If we add 250 to each number, most of them will no longer fit inside a `u8`.

🦀 The `.checked_add()` method found on most numeric types in Rust will return an Option, where the `None` variant represents an overflow.

🦀 If we combine this with a `.filter_map()` all numbers that overflow will automatically be excluded (because of the `None`) and all numbers in the `Some` variant are automatically unwrapped

🦀 That leaves us with these six numbers.

🦀 This not only saves us from having to deal with doubly wrapped options from `next` (for example `Some(Some(255))`) but entirely removes the items from the iterator meaning anything else we chain doesn't have to deal with it either.

🦀 By comparison, if we just used a map, we'd have 256 Options, 250 of which are `None`.

![process-filter-map-count.png](014-iterators/process-filter-map-count.png)

🦀 Another way to reduce how many items we want to deal with in an iterator is by using `.take(n)` and `.skip(n)`.

![process-take-skip.png](014-iterators/process-take-skip.png)

🦀 We can end an iterator earlier by only taking a certain number of items from it with `.take(n)`

🦀 Or we can skip over a number of items with `.skip(n)` before resuming the iterator from that point.

🦀 `.take()` can be particularly useful when working with infinite iterators too

![process-repeat-take-cycle.png](014-iterators/process-repeat-take-cycle.png)

🦀 I want you to consider the output of the following program.

🦀 Consider it!

🦀 *cough cough*

🦀 An Iterator method we used earlier, `.enumerate()`, allows us to add an index to our Iterator by changing the type of the Item `T` to a tuple of `(usize, T)`.

🦀 This can be really handy in combination with other iterators when the position in an iterator is important.

🦀 Let's say we want to filter every other item out of a `Vec`.

![process-enumerate.png](014-iterators/process-enumerate.png)

🦀 We can do that by chaining together several of the Iterators we've just learned.

🦀 We'll take ownership of the data because we don't need the original Vector after this.

🦀 We'll enumerate the iterator so we get the index from the point we're at in the iterator onwards (here we're at the start, but that might not always be the case)

🦀 The index starts at zero, so by checking the modulo of the index, we can take every other item starting with the first.

🦀 Filter doesn't change the data only decides if it should be kept or not, this means we can ignore the original data inside the predicate

🦀 Since we no longer need the index after this, we can map the tuple and return just the data we care about

🦀 Finally, we'll collect it and test the result

🦀 Any time you see a `filter` and a `map` next to each other though, you might be able to abbreviate your code.

🦀 Booleans can be turned into `Option`s with `.then_some()`, so this works, but...

![process-enumerate-filtermap.png](014-iterators/process-enumerate-filtermap.png)

🦀 in my opinion, you should always go with the code that's easiest to read, and its up to you to decide what that is

Finally, there are three more consuming methods I want to cover for processing data

`.fold()` and `.reduce()` consume iterators and return a single value by modifying that value for each item in the Iterator.

`.fold()` lets you specify the initial value for the returned value, but `.reduce()` uses the first item in the iterator as the initial value and continues processing from the next item.

Earlier I mentioned some risk with `.sum()` and `.product()` and promised a slower but safer way to do the same thing.

🦀 So let's try doing that with `.fold()`

![process-fold.png](014-iterators/process-fold.png)

🦀 it takes two parameters, the first being the initial value, and the second being a closure with two parameters

🦀 We'll use an Option with a 0 for the initial value, which is why we can't use `.reduce()` in this specific case.

🦀 For the closure, I usually stick to calling the parameters `acc` and `cur` representing the "acc-umulated" value which starts as our initial value, and the "cur-rent" value, which is the current Item in the iterator.

🦀 This closure is called for every Item in the iterator and returns the _next_ accumulated value.

🦀 We're simply going to add the values together, our accumulated value is an Option, and we only need to add if it's a `Some` varient

🦀 We can use `.and_then()` to get inside the Option, and we'll use the same `checked_add` to increase the value

🦀 This maps our Option to another Option that comes out of checked_add so we don't need to do anything else before returning from the closure

🦀 That said, there's actually a better way to provide this functionality.

🦀 The way we've built this, once we hit our first `None`, we _know_ the answer is going to be `None` too

🦀 There's a method designed exactly for this, `.try_fold()` 

![process-try-fold.png](014-iterators/process-try-fold.png)

🦀 Not only will stop iterating on it's first `None`, potentially ending very early, but because it knows we're dealing with `Option`s will automatically unwrap our option for us, making the code much simpler!

🦀 The last consumer method I wanted to talk about is `.for_each()`.

🦀 It lets you do something with each item in the iterator without returning anything.

🦀 The simplest example might be, if we went back to our Fibonacci sequence instead of printing the value in a loop, we can use `.for_each()` to print the value.

![process-try-fold.png](014-iterators/process-try-fold.png)

🦀 The lack of return value might _feel_ like it rather limits the usefulness of this function.

🦀 However, it can be useful when doing things like sending data somewhere else, for example, across threads, which we'll be looking at in the next video.

## More Iterator Traits

There are a few more traits you may want to be aware of when making your own iterators, or consuming others.

`IntoIterator` can be implemented on any type that can be turned into. an... `Iterator`.

Ah, I see what they did there.

One place you may find yourself using it is on newtypes, or types where the most important data inside is represented as some kind of collection.

🦀 Let's say we have a newtype Albums, that contains a Vector of Album

![albums.png](014-iterators/albums.png)

🦀 In our domain logic, it might make sense that we can start a new collection of Albums and add an Album to it by buying it.

🦀 So we can build and Albums struct like this

![albums-new.png](014-iterators/albums-new.png)

🦀 If we were to implement `IntoIterator` for `Albums` there are two important associated types that we need to specify.

![albums-into-iterator.png](014-iterators/albums-into-iterator.png)

🦀 `Item` is easy, that'll be our `Album` type.

🦀 `IntoIter` is the name of the type that _is_ the Iterator.

🦀 Remember back at the start, we created a Fibonacci Iterator.

🦀 Our struct `Fibonacci` is the Iterator type.

🦀 What the trait is actually asking us for here is the data type that will manage the iteration process for us.

🦀 We're using a Vec internally, and Vec has a generic Iterator struct it uses when you turn a Vec into an Iterator, so we can use that, filling in the Generic part with our Item type.

🦀 All we need to do in our case then, is return the result of calling Vec `.into_iter()`

🦀 Once we've done that, turning our Albums type into an Iterator of Album is trivial.

![albums-to-artists.png](014-iterators/albums-to-artists.png)

🦀 But what if we want to go back to having our Albums type again.

🦀 `FromIterator` allows you to turn an Iterator into another type, usually through the `.collect()` method on an `Iterator`

🦀 In our case, again, we could really just lean into the existing Vec type's utilities, 

![albums-from-iterator-manual.png](014-iterators/albums-from-iterator-manual.png)

🦀 But you could also rely on the structs own implementation to do the same thing by looping through the iterator.

![albums-from-iterator-vec.png](014-iterators/albums-from-iterator-vec.png)

🦀 Now we can create our Albums struct from any Iterator of Album

![albums-vec-to-albums-struct.png](014-iterators/albums-vec-to-albums-struct.png)

Finally, the last two traits you should be aware of are `DoubleEndedIterator` and `ExactSizeIterator`.

We could implement these for our Album type but we'd really just be wrapping Vec, so I'll give some more direct examples.

🦀 We've spoken about `ExactSizeIterator`, it can tell you the size of the iterator _without_ consuming it, using the `.len()` method 

🦀 `DoubleEndedIterator` allows you to reverse the order of an Iterator with `.rev()`.

![other-traits.png](014-iterators/other-traits.png)

🦀 The Iterators returned from all collections in the standard library are all both of these 

🦀 To my surprise, even the `Iter` structs used for `LinkedList` and `BinaryHeap` are both `DoubleEndedIterator`, I didn't expect that.

![other-traits-ll.png](014-iterators/other-traits-ll.png)

## Next Time

First, I want to say a huge thank-you to my Patreons, your support for the channel so early on really helps!

And if you enjoyed the video, don't forget to like and subscribe, you'd be surprised how much of a positive impact that has for creators! 

We've now covered all of what I'd describe as the core, synchronous language features (at least... I hope, let me know if you think I've missed something crucial).

We're moving on to Threads in the next video, we'll discuss what they are, why you'd use them, and some of the most important and useful tools to use when working with them.

With that, I'll see you next time.
