# Overloading

Java lets several methods share a name, provided their parameters differ:

```java
static void f(int x)    { System.out.println("int"); }
static void f(long x)   { System.out.println("long"); }
static void f(double x) { System.out.println("double"); }
static void f(Object x) { System.out.println("Object"); }
```

This is **overloading**, and the compiler decides which one a call means by
looking at the arguments.

```java
f(1);        // int
f(1L);       // long
f(1.0);      // double
f("s");      // Object
```

## Why have it

Because forcing distinct names for the same operation on different types produces
noise:

```java
printInt(5);
printDouble(3.14);
printString("hi");
```

versus

```java
print(5);
print(3.14);
print("hi");
```

The second says what is meant. `System.out.println` is overloaded about ten times
for exactly this reason, which is why it accepts anything you hand it.

The rule of thumb: **overload when the methods do the same thing to different
kinds of input.** Do not overload when they do different things — that is two
operations wearing one name, and the reader has to work out which.

## What distinguishes an overload

The **signature**: the name plus the parameter types, in order.

```java
static void f(int a, String b)      // distinct
static void f(String a, int b)      // distinct — different order
```

The return type is **not** part of it:

```java
static int  g(int x)
static long g(int x)      // error: g(int) is already defined
```

At first this looks arbitrary. The reason is that a call can be written where the
return value is discarded:

```java
g(5);       // which one?
```

With no context to choose from, the compiler cannot decide. Rather than allowing
it sometimes, Java forbids it always.

## How the compiler chooses

Roughly, in three passes, taking the first that finds a match:

**1. Exact match, or widening.** `f(1)` finds `f(int)` exactly. Given only
`f(long)` and `f(double)`, an `int` argument widens to `long`, which is preferred
over `double` because it is the *smaller* widening.

This is why `f(b)` with a `byte` prints `int`: `byte` widens to `int` before it
widens to `long` or `double`.

**2. Boxing.** If no primitive match works, `int` becomes `Integer` — Chapter 16's
subject.

**3. Varargs.** Last resort, so `f(int...)` is chosen only if nothing else fits.

You do not need to memorize this. What you need is the consequence: **the compiler
picks based on the declared types, and it prefers the most specific match.** When
overload resolution surprises you, that is the rule to consult.

## Where it goes wrong

Overloading interacts badly with a few things, and knowing where saves time.

**Null is ambiguous.** Given `f(String)` and `f(Integer)`, a call `f(null)` is
ambiguous — `null` fits both — and the compiler rejects it. You must cast:
`f((String) null)`.

**Autoboxing produces surprises.** The classic:

```java
List<Integer> list = ...;
list.remove(1);              // removes the element at index 1
list.remove(Integer.valueOf(1));   // removes the value 1
```

`List` has both `remove(int)` and `remove(Object)`. Passing `1` matches
`remove(int)` exactly, so the index version wins, and code that meant to remove
the value 1 removes whatever is second. This is a real bug that catches
experienced people, and Chapter 17 returns to it.

**Overload resolution is static.** The compiler chooses using the *declared* type
of the argument, not what it turns out to be at run time. Chapter 21 contrasts
this with overriding, which is resolved dynamically, and the difference between
the two is one of the more important distinctions in Unit V.

## When not to overload

Three cases where it is better to use different names.

**When the methods do different things.** `open(String)` reading a file and
`open(int)` opening a network port share nothing but a word.

**When the parameter counts are the same and the types are related.** `f(int)` and
`f(long)` are fine because the operation is the same; `f(int)` and `f(Integer)`
doing different things is a trap.

**When a good distinct name exists.** `valueOf`, `parseInt`, and `toString` could
all have been one overloaded name. They are not, and the library is clearer for it.

## Closing the chapter

We started with a question: how does a method call know where to return to?

The answer is a stack of frames. Each frame holds one execution's parameters,
locals, and return address. Frames are pushed on call and popped on return, and
the last-in-first-out discipline is not a design choice but a recognition — calls
nest, so a stack is the shape the problem already has.

From that one mechanism: local variables are fresh on every call and vanish on
return; a stack trace is that structure printed; allocation is one arithmetic
operation, so calls are cheap; and the region is bounded, so runaway recursion
produces `StackOverflowError` at a depth in the tens of thousands.

And the sentence this chapter existed to get right. **Java copies the contents of
the variable into the parameter, always.** For a primitive that is the value; for
an object it is a reference, so the parameter becomes an alias — and modifying
what it points at is visible everywhere, while assigning to the parameter itself
is visible nowhere.

One rule. Three demonstrations that look contradictory. No contradiction.

Next: a method that calls itself, which the stack makes possible and which will
turn out to be Chapter 9's induction in a different notation.
