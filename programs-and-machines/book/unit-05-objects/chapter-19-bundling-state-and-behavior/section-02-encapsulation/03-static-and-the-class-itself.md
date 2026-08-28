# static and the Class Itself

Everything so far has belonged to an *object*. Each account has its own balance;
each point has its own coordinates.

But some things plainly belong to the kind rather than to any one instance. How
many accounts have ever been created is not a fact about one account. Pi is not a
fact about one circle. A method converting Fahrenheit to Celsius should not need a
temperature to exist before it can run.

Java spells all three with a single keyword, which is unfortunate, because that
keyword means something different in each case.

`static` shares its syntax with everything else in this chapter and means
something different. It is where the Chapter 5 debts finally come due.

## The distinction

A normal field belongs to an **object**. A `static` field belongs to the
**class** — there is exactly one, shared by every instance and existing whether
any instance exists or not.

```java
class Counter {
    private static int totalCreated = 0;   // one, shared
    private int myCount = 0;               // one per object

    Counter() { totalCreated++; }
    void bump() { myCount++; }
    int mine() { return myCount; }
    static int total() { return totalCreated; }
}
```

```java
Counter a = new Counter(), b = new Counter();
a.bump(); a.bump(); b.bump();
```

```
a=2 b=1 created=2
```

`a` and `b` have their own `myCount`. They share `totalCreated`, which counted
both constructions.

A `static` method likewise belongs to the class and is called on it:

```java
Counter.total();        // on the class
a.mine();               // on an object
Math.max(3, 9);         // Math has no instances at all
```

## Why a static method cannot see fields

```java
public class BadStat {
    private int n = 5;
    public static void main(String[] args) { System.out.println(n); }
}
```

```
error: non-static variable n cannot be referenced from a static context
```

This error confuses every beginner, and the reason is exact: `n` belongs to an
object, a static method is not called on any object, so there is **no object whose
`n` it could mean.** The question has no answer, so the compiler refuses.

The fix is either to make the field static — if it really is one per class — or to
create an object and ask that:

```java
public static void main(String[] args) {
    BadStat b = new BadStat();
    System.out.println(b.n);
}
```

## When static is right

**Utility methods that depend on nothing.** `Math.max`, `Integer.parseInt`. They
take input and return output, with no state involved. Chapter 11 called these
pure, and a pure method has nothing to be an instance of.

**Constants.**

```java
public static final int MAX_ATTEMPTS = 3;
```

`static` because one is enough, `final` because it does not change. This is the
standard idiom and the naming convention — UPPER_SNAKE_CASE — signals it.

**Factory methods** that create instances:

```java
public static Account empty(String owner) {
    return new Account(owner, 0);
}
```

Useful because a factory can have a name where a constructor cannot, and can
return a cached instance rather than a new one. `Integer.valueOf` from Chapter 16
does exactly this.

## When static is wrong

**Mutable static state is shared by the whole program**, which makes it a global
variable in everything but name. Every part of the program can change it, nothing
records who did, and Chapter 31 will show that concurrent access to it is a
minefield. If you find yourself reaching for a mutable static field, ask whether
the thing it tracks belongs to an object instead.

**Static methods cannot be overridden**, so a class full of them cannot be
substituted for testing or varied by subclassing. Chapter 14's point again: hard
to test is a design signal.

The rough guide: `static` for things that genuinely belong to the kind rather than
to any particular one, and for pure functions. Not as a way to avoid creating an
object.

## Paying the Chapter 5 debts

Here is the line, fully explained at last:

```java
public static void main(String[] args)
```

**`public`** — visible from anywhere. The JVM is outside your class and must be
able to call it. Section 19.2.1's four levels, and this one needs the widest.

**`static`** — belongs to the class, not to an object. This is the essential one:
when the JVM starts, **no objects exist yet**. Something has to be callable before
anything has been created, and a static method is a method you can call without
first making something.

**`void`** — returns nothing. There is no caller in your program waiting for a
value; the JVM does not use one. (A process does return an exit status, and
`System.exit(1)` is how you set it.)

**`main`** — the name the JVM looks for, fixed by specification.

**`String[] args`** — the command-line arguments, as an array of strings. Chapter
11 covered parameters and Chapter 15 arrays, so both halves are now familiar.

Chapter 5's exercise 5.18 asked you to write down, in one sentence each, what you
believed those four words did, and to keep the page. **This is the moment to go
and compare.**

The comparison is the exercise, not the answers. Whatever you wrote, you wrote it
without classes, without access control, without the object–class distinction, and
without the heap. The interesting thing is not whether you were right but what you
now have that you did not have then — which is a reasonable measure of what
fourteen chapters bought.

## Closing the chapter

A class puts state and the operations on it inside one unit and makes the state
unreachable from outside. That is what it is for. The constructor establishes the
invariant, so a badly-formed object cannot exist; every method preserves it, which
is Chapter 16's obligation with somewhere to live; and `private` makes the set of
code that could break it small enough to check.

The public surface is a set of promises, and it should be the smallest set that
lets callers do their job — named in the domain, exposing behavior rather than
fields, and never handing out the internals. Getters and setters written
mechanically for every field undo the whole arrangement.

And `static` means *belongs to the class rather than to any object*, which is why
`main` is static, why a static method cannot see instance fields, and why mutable
static state is a global variable wearing a hat.

Next: the question this chapter has quietly avoided. Two `Account` objects with
the same owner and the same balance — are they the same account?
