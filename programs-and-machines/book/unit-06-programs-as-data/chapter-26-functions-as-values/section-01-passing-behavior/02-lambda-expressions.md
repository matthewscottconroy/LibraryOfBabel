# Lambda Expressions

The syntax, then the interesting part.

## Forms

```java
x -> x * x                        // one parameter, one expression
(a, b) -> a + b                   // two parameters
() -> System.out.println("hi")    // none
(String s) -> s.length()          // explicit type, rarely needed
x -> { int y = x * 2; return y + 1; }    // a block, so return is required
```

The rules, briefly: parentheses are optional for exactly one parameter; types are
inferred from the target interface and are usually omitted; a single expression is
the return value, and a braced block needs an explicit `return`.

The arrow was chosen over other candidates because it reads as *maps to*, which is
what it means. The name comes from Church's lambda calculus, which Chapter 13
introduced — `λx.x*x` is the same expression in the notation of 1936.

## What a lambda can see

```java
int captured = 42;
IntOp usesLocal = x -> x + captured;
usesLocal.apply(8)
```

Verified: `50`.

The lambda used `captured`, a local variable of the enclosing method. It did not
receive it as a parameter; it *captured* it.

This is the closure Chapter 25 defined and said our interpreted language lacked.
The `Procedure` record had no `Env` field, so a procedure could not see where it
was created. A Java lambda can, and the mechanism is the same one: the value is
stored with the code.

A lambda may see:

- its own parameters
- **effectively final** local variables of the enclosing method
- fields of the enclosing object, via `this`
- static fields

## Effectively final

The restriction, and it is worth understanding rather than memorizing:

```java
int count = 0;
Runnable r = () -> System.out.println(count);
count++;                    // does not compile
```

The error names the rule: *local variables referenced from a lambda expression
must be final or effectively final*. "Effectively final" means you never assign to
it after initialization — you need not write `final`, but you must behave as
though you had.

The reason is a lifetime problem. A local variable lives in Chapter 12's stack
frame, and the frame is destroyed when the method returns. A lambda can outlive
the method that created it — stored in a field, handed to a thread, returned. So
the value must be *copied* into the lambda rather than referenced.

And once it is a copy, allowing assignment would be a lie: changing `count`
afterwards could not affect the copy, so the two would silently disagree. Java
forbids the assignment rather than permit the confusion.

Other languages choose differently. JavaScript closures capture the variable
itself, which is why a loop creating closures over the loop counter used to
produce closures that all saw the final value — a famous source of bugs that `let`
was introduced to fix. Java's restriction is stricter and has no such trap.

The rule does not apply to fields. A lambda capturing `this.count` sees the field
through the reference, so changes are visible — and if two threads are involved,
Chapter 31's problems apply in full.

## The escape hatch, and why not to use it

```java
int[] counter = {0};
list.forEach(n -> counter[0]++);
```

The array reference is effectively final; its contents are not. This compiles, and
it is how people get around the restriction.

Verified, over ten elements sequentially: `10`. Correct.

Now the same thing over a million elements in parallel, three runs:

```
parallel count of 1,000,000 -> 97282
parallel count of 1,000,000 -> 78637
parallel count of 1,000,000 -> 906250
```

Three runs, three answers, none of them a million. The increments raced, and
roughly nine in ten were lost.

That is Chapter 31's subject arriving early, and it is here because this specific
trick is how people meet it. The restriction Java placed on captured locals is not
arbitrary tidiness — it is a nudge toward lambdas that do not mutate shared state,
and the array trick removes the nudge without removing the reason for it.

The rule to carry: **a lambda that modifies something outside itself is a lambda
you must think hard about.** Section 26.2.3 gives the general form of this.

## Lambdas are not anonymous classes

They look like shorthand and there are two differences that matter.

**`this` means the enclosing object.** In an anonymous class, `this` refers to the
anonymous instance. In a lambda, it refers to the object whose method contains the
lambda, which is nearly always what you meant.

**No new class file, usually.** An anonymous class compiles to a separate `.class`
file. A lambda compiles to an `invokedynamic` instruction that builds the
implementing object at first use — which means fewer classes to load, and it is
why replacing anonymous classes with lambdas modestly improves startup time in
large programs.

## Method references

When a lambda does nothing but call an existing method, there is a shorter form:

```java
Function<String, Integer> len1 = s -> s.length();
Function<String, Integer> len2 = String::length;    // the same thing
```

Verified: both give 5 for `"hello"`.

Four kinds, and the third is the one that confuses people:

```java
System.out::println      // a method on a particular object
Integer::sum             // a static method
String::length           // an instance method, receiver supplied as the argument
ArrayList::new           // a constructor
```

Verified: `Integer::sum` applied to 3 and 4 gives 7; `ArrayList::new` used as a
`Supplier` produces an empty list.

`String::length` is worth a sentence. It looks like a static reference and it is
not — the receiver becomes the first argument, so a `Function<String, Integer>` is
satisfied by an instance method taking nothing. Java works this out from the target
type, which is convenient and occasionally produces an error message that takes
some reading.

This is Chapter 17's `Integer::sum`, promised there without explanation.

Use a method reference when it names what is happening — `Person::name` reads
better than `p -> p.name()`. Keep the lambda when the parameter name carries
information the method name does not.

Next: methods that take and return behavior.
