# 3 things others languages should steal from Rust

Over the last 30 years I've used, C, C++, Java, Perl, PHP, GDScript, JavaScript, and TypeScript all fairly extensively at different times.

And I've also used QBasic, Visual Basic, C#, Groovy, Python, Ruby, and Go but somewhat less so.

I love programming and programming languages but my favourite, so far, is Rust.

Rust is amazing because of what its learned (and arguably stolen) from the languages that came before it.

So what could those languages steal right back?

## Testing

Hi, I'm Daniel, and if we've met you know I'm obsessed with testing!

I tend not to believe it when people tell me nice things about my work, but when I write a test, and it goes green, ohh that feels good.

There are testing frameworks in every language, and if one doesn't take your fancy, creating basic assertions is extremely trivial.

So what's so special about Rust?

Rust provides almost everything you need for testing out of the box.

You get all the tooling you need to write unit tests, integration tests and one other kind of test we'll cover later, all without needing addition libraries or configuration.

Is it perfect, no.

You'll need additional libraries for providing things like fake data or mo... mo... ugh, mocking.

You may also need a little extra tooling to capture coverage, though some IDEs handle that for you.

Nonetheless, in Rust, testing is a first class citizen which as far as programming goes, is surprisingly unusual... and awesome!

Everyone should do this!

## Documentation

Good documentation is really important, though it tends to be the sort of thing you only notice when faced with _bad_ documentation.

There are documentation tools for most languages.

Javadoc really set the standard here, and it's something that has been adopted and adapted by most languages with things like JSDoc, PhpDoc, and even rustdoc.

Being able to document your code in your code with these tools is incredibly powerful.

You end up with not only searchable, publishable documentation, but something people can read in place while working on the code and even something your IDE can pick up. 

Rust has taken this idea and absolutely turbocharged it in two crucial ways.

First, rustdoc is part of the base tooling.

You don't need to install and configure it separately, and everyone uses the same tool.

This means when working on your code locally, you can not only document your code, but you can access the documentation of all your dependencies right in the same place, locally, with no fuss.

Everyone documenting the same way also means documentation is easier follow, there's fewer surprises.

Second, tests.

No, I haven't jumped back a section, and yes I really am obsessed.

Its one thing to write documentation, but another to keep it up to date.

Having documentation that's wrong can be worse than having none at all, something I expect most of us have experienced.

In Rust, your code examples can not be wrong because when you run `cargo test` it also runs your code examples.

This is why you'll regularly see code examples in Rust documentation using assertions.

They don't just show you, the reader, what to expect when you call a function, they prove that it's true!

These tests even count towards things like test coverage so writing tests into your documentation kills two birds with one stone.

I have seen this feature in some documentation libraries in other languages, but...

Having documentation and testing all wrapped up neatly with no configuration required, and consistency across the whole ecosystem is a concept absolutely worth stealing.

## No Surprises

In many ways, Rust is an incredibly boring language, and that's a good thing.

There are no surprises... or at least, almost no surprises.

There's several ways Rust achieves this from its strict type system to avoiding out of order execution mechanisms like exceptions.

Perhaps the most impactful way Rust prevents surprises is with Ownership.

I know, I know, if one thing is called out as uniquely difficult in Rust; its ownership.

But, lets look at some simple code.

Here's some TypeScript.

We define and create an example struct with some initial data.

We'll call a function that modifies the example struct.

Then we'll call a function that modifies the string in that struct.

What do we expect the output of this program to be?

I'll give you three possibilities:

Updated, Updated Again

Updated, Updated

or

Initial Value, Initial Value

Before I answer that question, here's the same code in PHP... C#... C++... C... Go... Java... Python... Ruby... QBasic...

The code is, I would argue, equivalent in each example.

Do they all do the same thing?

I won't make you wait any longer, the answer is Updated, Updated Again... for QBasic because both values are passed by reference.

Updated, Updated for TypeScript, PHP, Java, C#, and Ruby because they all pass the object by reference but the string by copy.

And Initial Value, Initial Value for C, C++, Go and Python because they all pass both values by copy.

Now, the fact all these languages do it differently isn't actually the problem.

It is confusing if you have to jump languages a lot but once you know the behaviour you know the behaviour.

What I personally don't think is OK, is that in all but one of those languages we set a value and never use it in at least one function... but we don't even get a warning for behaviour I think is obviously wrong.

So what does Rust do?

Well... this code doesn't work in Rust.

When we pass Example into the first function, we transfer ownership of the value to that function.

This means `main` no longer has access to the variable, so we can't use it any more.

We can hold on to ownership by explicitly cloning the values... but that still doesn't work as we haven't explicitly described the parameters of the function as mutable.

So let's do that, and now we get "Initial value, Initial value" output (because we're changing copies of the data), but look we also got warnings...

...but only on the `update_string` function, so I _am_ going to call Rust out for not picking up the same problem in `update_example`.

Look, I did say _almost_ no surprises.

Anyway, lets clear out that warning as we do want to change our example data.

We need to make our functions _explicitly_ take mutable reference, 

we need to _explicitly_ pass mutable reference into those functions, 

and we need to make example _explicitly_ mutable

For some reason we also need to dereference the string... but not the struct.

I'm not sure about exactly why that is but the compiler does tell you about it, so I don't think it counts as a surprise

Now we get the objectively correct, warning free code that only the highly professional QBasic could give us before.

I know, the ownership model can be a bit of a brain melter to get your head around when you first start using it...

but I think this objectively leads to fewer nasty surprises than the other examples we saw.

Unfortunately... existing languages can't exactly steal the ownership model, it's just too different from what they already do.

But I do think all _new_ languages should seriously consider it.

The speed of low level languages, with the confidence of high level languages and less surprising than either, it's a no-brainer.

And remember at the start I said, "my language favourite, so far, is Rust"?

I'm genuinely excited for the next generation of languages that continue to build on every language that came before them.

### Updated, Updated Again

```basic
TYPE Example
    innerValue AS STRING
END TYPE

SUB updateExample(example AS Example)
    example.innerValue = "Updated"
END SUB

SUB updateString(s AS STRING)
    s = "Updated Again"
END SUB

DIM example as Example
example.innerValue = "Initial Value"

updateExample(example)
print example.innerValue

updateString(example.innerValue)
print example.innerValue
```


```qbasic
TYPE Example
    innerValue AS STRING
END TYPE

SUB updateExample(example AS Example)
    example.innerValue = "Updated"
END SUB

SUB updateString(s AS STRING)
    s = "Updated Again"
END SUB


DIM example as Example
example.innerValue = "Initial Value"

CALL updateExample(example)
PRINT example.innerValue

CALL updateString(example.innerValue)
PRINT example.innerValue
```

### Updated, Updated

```java
class Example {
    public String innerValue = "Initial Value";
}


public class Main {
    static void updateExample(Example example) {
        example.innerValue = "Updated";
    }
    
    static void updateString(String s) {
        s = "Updated Again";
    }
    
    
    public static void main( String args[] ) {
        Example example = new Example();
        
        updateExample(example);
        System.out.println( example.innerValue );
        
        updateString(example.innerValue);
        System.out.println( example.innerValue );
    }
}
```


```typescript
class ExampleClass {
	public innerValue: string = 'Some initial value';
}

const example = new ExampleClass();

function editExample(example: ExampleClass) {
	example.innerValue = 'Updated';
}

editExample(example);
console.log(example.innerValue);

function editString(s: string) {
    s = 'Updated Again'
}

editString(example.innerValue);
console.log(example.innerValue);
```


```php
<?php

class ExampleClass {
	public string $innerValue = 'Some initial value';
}

$example = new ExampleClass();

function editExample(ExampleClass $example) {
	$example->innerValue = 'Updated';
}

editExample($example);
echo $example->innerValue;

function editString(string $s) {
	$s = "Updted again";
}

editString($example->innerValue);
echo $example->innerValue;
```


```c#
using System;

public class Example {
	public String innerValue = "initial Value";
}
					
public class Program
{
	public static void editExample(Example example) {
		example.innerValue = "Updated";
	}
	
	public static void editString(String s) {
		s = "Updated again";
	}
	
	public static void Main()
	{
		Example example = new Example();
		
		Program.editExample(example);
		Console.WriteLine(example.innerValue);
		
		Program.editString(example.innerValue);
		Console.WriteLine(example.innerValue);
	}
}
```


```ruby
class Example
  attr_accessor :inner_value
  
  def initialize
    @inner_value = "Initial Value"
  end
end

def update_example(example)
  example.inner_value = "Updated"
end

def update_string(s)
  s = "Updated again"
end


example = Example.new

update_example(example)
puts example.inner_value

update_string(example.inner_value)
puts example.inner_value
```

### Initial value, initial value

```c11
#include <stdio.h>

struct Example {
    char *innerValue;
};

void updateExample(struct Example example) {
    example.innerValue = "Updated";
}

void updateString(char *s) {
    s = "Updated again";
}

int main() {
    struct Example example = { "Initial value" };
    
    updateExample(example);
    printf("%s\n", example.innerValue);
    
    updateString(example.innerValue);
    printf("%s\n", example.innerValue);
}
```


```c++20
#include <stdio.h>

struct Example {
    char *innerValue;
};

void updateExample(Example example) {
    example.innerValue = (char *)"Updated";
}

void updateString(char *s) {
    s = (char *)"Updated again";
}

int main() {
    Example example = Example { (char *)"Initial value" };
    
    updateExample(example);
    printf("%s", example.innerValue);
    
    updateString(example.innerValue);
    printf("%s", example.innerValue);
}
```


```go
package main

import "fmt"

type Example struct {
	innerValue string
}

func updateExample(example Example) {
	example.innerValue = "Updated"
}

func updateString(s string) {
	s = "Updated Again"
}

func main() {
	example := Example{"Initial Value"}

	updateExample(example)
	fmt.Println(example.innerValue)

	updateString(example.innerValue)
	fmt.Println(example.innerValue)
}
```


```python
class Example:
  innerValue = "Initial Value"

def update_example(example: Example):
  example.inner_value = "Updated"


def update_string(s: str):
  s = "Updated again"


example = Example()

update_example(example)
print(example.innerValue)

update_string(example.innerValue)
print(example.innerValue)
```

### Rust

```rust
#[derive(Clone)]
struct Example {
    inner_value: String,
}

fn update_example(mut example: Example) {
    example.inner_value = "Updated".to_string();
}

fn update_string(mut s: String) {
    s = "Updated Again".to_string();
}

fn main() {
    let example = Example {
        inner_value: "Initial value".to_string(),
    };
    
    update_example(example.clone());
    println!("{}", example.inner_value);
    
    update_string(example.inner_value.clone());
    println!("{}", example.inner_value);
}
```

```Rust
struct Example {
    inner_value: String,
}

fn update_example(example: &mut Example) {
    example.inner_value = "Updated".to_string();
}

fn update_string(s: &mut String) {
    *s = "Updated Again".to_string();
}

fn main() {
    let mut example = Example {
        inner_value: "Initial value".to_string(),
    };
    
    update_example(&mut example);
    println!("{}", example.inner_value);
    
    update_string(&mut example.inner_value);
    println!("{}", example.inner_value);
}
```

## Outro

I hope you enjoyed that and took it for the expression of passion for programming that it was meant to be.

I love, maybe not all, but most programming languages.

I love how people approach problems from different angles and come up with different solution.

As much as I love Rust, I don't want it to be my last language, and I want to keep being able to use my old languages too.

What did I miss? 

What else could we learn from Rusts design?

Let me know in the comments, but please be kind.

The languages you don't like might be someone else's favourite and vice versa, and there's still lots we can learn from each other.

If you need a palette cleanser, there's a companion video to this one on my main channel, that goes the other way

5 things Rust should steal from other languages

I'm trying out a second channel for things I want to talk about that aren't as tightly related to my main content which is more explicitly learning about Rust, so let me know what you think.

I've got some more ideas for here so, I hope you'll be back.

See ya then :)
