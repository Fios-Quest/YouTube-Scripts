# 5 things others languages should steal from Rust

Throughout my life I've used, C, C++, Java, Perl, Visual Basic, PHP, GDScript, JavaScript, and TypeScript somewhat extensively

And I've used QBasic, Visual Basic, C#, Groovy, Python, Ruby, and Go somewhat less so.

I love programming and programming languages but my favourite, so far, is Rust.

Rust has learned a huge amount from the languages that came before it but here are 5 things I wish other languages would steal right back!

## Consistent Documentation

## Testing

## Ownership

I know, I know, if one thing about Rust is called out as uniquely difficult its ownership.

// Examples implicit of COW in other languges being confusing

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

## No Throw and Strict Type Systems

## Daniel is a liar

## Outro