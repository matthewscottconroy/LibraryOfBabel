# A Method You Can Trust

The precondition says `values` must be non-empty. A caller passes an empty array.
Now what?

Java gives four answers, and choosing between them is a real design decision.

## The four options

**1. Undefined behavior.** Do nothing about it; whatever happens, happens. Here,
`values[0]` throws `ArrayIndexOutOfBoundsException` from inside the method.

Cheapest, and the failure is confusing: the caller sees an exception about array
indices from a method they asked for a maximum. The message describes the
*symptom* at the point of failure rather than the *mistake* at the point of the
call.

**2. Check and throw.** Detect the violation and report it clearly:

```java
static int largest(int[] values) {
    if (values == null)       throw new IllegalArgumentException("values must not be null");
    if (values.length == 0)   throw new IllegalArgumentException("values must not be empty");
    ...
}
```

Now the caller gets a message naming their mistake. Chapter 28 covers exceptions
properly; for now, `throw` stops the method and reports.

**3. Return a sentinel.** Hand back a value meaning "no answer" — `-1`, or `null`,
or `Integer.MIN_VALUE`.

Convenient and dangerous, because the caller may not check. A sentinel that gets
used as if it were a real answer produces a wrong result with no error, which is
the worst outcome available. `-1` is a plausible maximum.

**4. Widen the contract.** Decide what the empty case *should* return and
document it. For a maximum there is no defensible answer, which is why the
precondition exists. For a sum, zero is exactly right and the precondition is
unnecessary.

## Choosing

The guidance that holds up:

**For public methods called by code you do not control: check and throw.** You
cannot rely on strangers reading documentation, and a clear exception at the point
of the mistake is worth its cost.

**For private helpers within one class: rely on the precondition.** The callers are
in the same file and you can see them. Checking is noise.

**Never return a sentinel that could be mistaken for a valid answer.** If you must
signal absence, use something that cannot be confused — `Optional` in modern Java,
or an exception. `-1` is fine for an index, because indices cannot be negative; it
is not fine for a temperature.

**Fail as early as possible.** A bad value detected at the call is a two-minute
fix. The same value stored, passed through four layers, and detected when a report
comes out wrong is an afternoon. This principle is called *failing fast*, and it
is one of the highest-value habits available.

## Why failing fast matters

Worth an example, because the reasoning is not obvious.

Suppose a method accepts a negative age and stores it. Nothing fails. Later a
report divides by it, or sorts by it, or displays it, and something looks wrong
three subsystems away from where the bad value entered.

Debugging that means working backwards through everything the value touched —
Chapter 10's bisection, over a much larger space than necessary. The information
that would have identified the mistake instantly, namely *who passed a negative
age*, is long gone.

An exception at the point of entry would have carried the whole diagnosis in its
stack trace.

The general principle: **the distance between a mistake and its symptom is the
cost of the bug.** Everything that shortens that distance is worth doing, and
checking preconditions is the cheapest way to shorten it.

## Assertions

Java has a construct for checking things you believe are true:

```java
assert values.length > 0 : "largest requires a non-empty array";
```

If the condition is false, an `AssertionError` is thrown with that message.

The catch: **assertions are disabled by default** and must be enabled with `-ea`
on the command line. That makes them unsuitable for validating input from
outside — a check that does not run in production is not a check.

What they are good for is stating and verifying your *internal* beliefs: an
invariant you expect to hold, a case you think is impossible. During development
they catch broken assumptions early; in production they cost nothing.

The division is worth remembering:

| situation | tool |
|---|---|
| input from outside your control | `if` + `throw` |
| a belief about your own code | `assert` |

## Trust

Pulling the chapter together.

A method is worth having when you can call it **without reading it**. That
requires the name to say what it does, the signature to say what goes in and out,
and the contract to state what is required and what is guaranteed. When all three
hold, the method is a genuine unit of thought and your attention is freed.

When any fails — a name that misleads, a precondition that is not stated, a
method that quietly does something extra — the abstraction leaks. You have to
remember the method's peculiarities, which is more to carry than the code it
replaced.

Trustworthiness is not a nicety. It is the property that makes the abstraction
work at all.

Next, the mechanism underneath: what the machine actually does when a method is
called, and why Java's parameter passing is more subtle than it first appears.
