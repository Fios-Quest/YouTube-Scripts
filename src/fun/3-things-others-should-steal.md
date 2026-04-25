# 3 things others languages should steal from Rust

Throughout my life I've used, C, C++, Java, Perl, PHP, GDScript, JavaScript, and TypeScript somewhat extensively

And I've used QBasic, Visual Basic, C#, Groovy, Python, Ruby, and Go somewhat less so.

I love programming and programming languages but my favourite, so far, is Rust.

Rust is amazing because of what its learned (and arguably stolen) from the languages that came before it.

So what could those languages steal right back?

## Testing

Hi, I'm Daniel, and if we've met you people know one thing about me its that I'm obsessed with testing!

I tend not to believe it when people tell me nice things about my work, but when I write a test and it goes green, ohh that feels good.

There are testing frameworks in every language, and if one doesn't take your fancy, creating basic assertions is extremely trivial.

So what's so special about Rust?

Rust provides a complete test harness out of the box.

You get all the tooling you need to write unit tests, integration tests and one other kind of test we'll cover later, all without needing addition libraries or configuration.

Macro's also let you write complex repetative test code once and reuse it, even if you need slight variations.

Is it perfect, no.

There are libraries for providing things like fake data or mo... mo... ugh, mocking.

You may also need a little extra tooling to capture coverage, though some IDEs handle that for you.

None the less, testing is a first class citizen in Rust which is unusual... and awesome!


## Documentation

Documentating your code is really important though it tends to be the sort of thing you only notice when faces with _bad_ documentation.

There are documentation tools for most languages.

JavaDoc really set the standard here and its something that has been adopted and adapted by most languages with things like TSDoc, JSDoc, PhpDoc, and so on.

Being able to document your code in your code with these tools is incredibly powerful.

You end up with not only searchable publishable documention but something people can read in place while working on the code and even something your IDE can pick up. 

Rust has taken this idea and absolutely turbocharged it.

First, rustdoc is part of the base tooling.

You don't need to install and configure it separately, and everyone uses the same tool.

This means when working on your code locally, you can not only document your code, but you can access the documentation of all your dependencies right in the same place, locally, with no fuss.

Everyone documenting the same way also means documentation is easier to navigate.

It follows consistent standards and it becomes easier for us to learn from each other what good documentation looks like.

Second, tests.

No, I haven't jumped back a section, and yes I realy am obsessed.

Its one thing to write documentation, but keeping it up to date is hard.

Having documentation that's wrong can be worse than having none at all, something I expect most of us have experienced.

In Rust, your code examples can not be wrong because when you run `cargo test` it also runs your code examples.

This is why you'll regularly see code examples in documentation using assertions

They don't just show you, the reader, what to expect when you call a function, they prove that its true!

These tests even count towards things like test coverage so writing tests into your documentation kills two birds with one stone.

I have seen this feature in some documentation libraries in other languages, but...

Having documentation and testing all wrapped up neatly with no configuration required, and consistency across the whole ecosystem is absolutely worth every language stealing.

## No Surprises

In many ways, Rust is an incredibly boring language.

There are no surprises... or at least, almost no surprises.

There's two big ways Rust does this.

First, lets talk about order of excution.

Here's some Rust for getting User data from an endpoint...

...and here's that same code in TypeScript.

I'm using Result types to make sure that errors are explicit, not implicit...

Except...

These can still throw exceptions, and this may not actually create a user.


The other, and perhaps even more impactful way Rust prevents surprises is with Ownership.

I know, I know, if one thing about Rust is called out as uniquely difficult its ownership.

But, lets look at some simple code.

So here'ssome TypeScript where we define and create an example struct with some initial data.

We'll can call a function that modifies the example struct.

Then we'll call a function that modifies the string in that struct.

// Examples implicit of COW in other languges being confusing


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
print example.innerValue

CALL updateString(example.innerValue)
print example.innerValue
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

```java
class Example {
    public String innerValue = "Initial Value";
}


void updateExample(Example example) {
    example.innerValue = "Updated";
}

void updateString(String s) {
    s = "Updated Again";
}


void main( String args[] ) {
    Example example = new Example();
    
    updateExample(example);
    System.out.println( example.innerValue );
    
    updateString(example.innerValue);
    System.out.println( example.innerValue );
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


## Outro
