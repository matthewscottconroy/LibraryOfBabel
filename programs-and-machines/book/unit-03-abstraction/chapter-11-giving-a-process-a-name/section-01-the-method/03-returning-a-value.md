# Returning a Value

Parameters carry information in. **Return values** carry it out.

```java
static int square(int n) {
    return n * n;
}
```

The `int` before the name is the **return type** — the promise that this method
produces an `int`. The `return` statement supplies it and ends the method
immediately.

## return ends the method

Worth stating plainly, because it is used constantly:

```java
static int firstNegative(int[] a) {
    for (int i = 0; i < a.length; i++) {
        if (a[i] < 0) return i;      // done: leave now
    }
    return -1;                        // got through without finding one
}
```

The `return i` exits the whole method, not just the loop. That is what makes the
guard-clause style of Chapter 8 work, and it is the clean answer to Chapter 9's
problem of breaking out of nested loops.

Any code after a `return` on the same path is unreachable, and Java rejects it at
compile time — Chapter 5's reachability check.

## void

A method that produces no value declares `void`:

```java
static void greet(String who) {
    System.out.println("Hello, " + who);
}
```

`void` is the second debt from Chapter 5 paid: `main` returns nothing, because
there is nothing for it to hand back to the JVM.

A `void` method may still use `return;` with no value, to leave early:

```java
static void process(int[] data) {
    if (data.length == 0) return;      // nothing to do
    // ...
}
```

## The distinction worth making

A method either **computes a value** or **causes an effect**. `square` computes.
`greet` causes — it prints, which changes something outside the method.

Methods that only compute are dramatically easier to reason about. Call `square(7)`
a hundred times and you get 49 a hundred times; nothing else in the program is
different for having called it. You can test it by comparing input to output, move
it, delete a redundant call, or reorder calls freely.

Methods that cause effects can do none of that. Calling `greet` twice prints
twice. Order matters. Testing means capturing output. And whether a call can be
removed depends on whether anyone wanted the effect.

A method with no effects beyond its return value is called **pure**, and the rule
of thumb is: **prefer pure methods, and when a method must have an effect, do not
also make it compute something interesting.** A method that both modifies state
and returns a value is one whose calls cannot be moved or removed without
thought, and every reader has to notice both jobs.

Chapter 26 returns to this when functions become values, and Unit VII is largely
about the parts of a program where effects are unavoidable.

## Returning one thing

Java methods return exactly one value. Sometimes you want two — a minimum and a
maximum, a quotient and a remainder — and the options are all imperfect:

**Return an object holding both.** Unit V's answer, and usually the right one. A
`MinMax` with two fields, or from Chapter 22 a `record`, which exists for exactly
this.

**Return an array.** `return new int[]{min, max};` Works, and the caller has to
remember which index is which, so it is a positional-argument problem in
reverse.

**Use parameters as outputs.** Pass in something the method fills. Common in
older APIs and generally worse, because it makes the method's effect invisible at
the call site.

**Split into two methods.** Frequently best when the two results are independently
useful. Two passes over the data is usually a cost worth paying for clarity, and
Chapter 32 will let you judge when it is not.

## Naming, again

Return type and name should agree, and readers rely on it more than they realize.

```java
int  count(...)         // returns a number
boolean isValid(...)    // returns true or false
String format(...)      // returns text
void save(...)          // does something
```

Two conventions worth adopting. A method returning `boolean` is usually named
`isSomething` or `hasSomething`, so that `if (isValid(x))` reads as English. A
method returning a value is usually named for the value — `largest`, `count`,
`total` — while a method causing an effect is named for the action — `save`,
`print`, `update`.

Get the convention wrong and you mislead. A method called `getBalance` that
silently opens a network connection is a betrayal of the reader, and the code
that uses it will be written on the assumption that calling it is cheap and safe.

## What a signature says

Put it together. The first line of a method is its **signature**, and it is a
summary of the contract:

```java
static int largest(int[] values)
```

*Given an array of `int`, this produces an `int`.* A reader learns what goes in
and what comes out without reading a line of the body — which is the whole
purpose of the abstraction.

What the signature does *not* say is nearly as important. It does not say what
happens for an empty array. It does not say whether `values` is modified. It does
not say whether the result is the largest, the smallest, or the first.

Those are the contract's remaining terms, and the next section is about stating
them.
