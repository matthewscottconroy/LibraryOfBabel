# Why Not Just Return -1

A method that parses an integer needs to say something when the text is not a
number. The oldest answer is to return a value that means *failed*.

```java
static int parseOrMinusOne(String s) {
    try { return Integer.parseInt(s); }
    catch (NumberFormatException e) { return -1; }
}
```

Verified:

```
parse("42")   = 42
parse("oops") = -1
parse("-1")   = -1
```

The third line is the problem. `"-1"` is a valid input whose correct answer is
$-1$, and `"oops"` is an invalid input whose failure code is $-1$. The caller
cannot distinguish them.

That is the **sentinel problem**, and it is not a defect of this example. It is
structural: a sentinel steals a value from the result space, and it is a bug
whenever that value is legitimate.

## The four alternatives, and how each fails

**A sentinel value.** As above. Works only when some value is genuinely
impossible, which is rarer than it looks. `String.indexOf` returns $-1$ and gets
away with it because an index cannot be negative — and even there, callers forget
to check, and $-1$ then flows onward as an index and produces a confusing failure
somewhere else.

**Return `null`.** Available for object results, and it has the same shape.
`Map.get` returns `null` for a missing key, which is fine until the map is allowed
to contain `null` values, at which point `get` returning `null` is ambiguous —
which is why `containsKey` exists as a separate method. And an unchecked `null`
produces a `NullPointerException` far from its cause.

**Set a flag.** The C convention: return a status code and put the answer in an
out-parameter, or set a global `errno`. It works and it makes the failure path
easy to ignore, because the flag is a separate value the caller must remember to
consult. Decades of C security vulnerabilities are unchecked return codes.

**Return a result object.** `Result<T>` holding either a value or an error.
Rust and Go do versions of this, and it is genuinely good: the failure is in the
type, so the caller cannot ignore it without saying so. Java's `Optional` is a
degenerate case — it says *absent* without saying why.

All four share one property, and it is the decisive one.

## The propagation problem

Suppose `a` calls `b` calls `c` calls `d`, and `d` fails. With any return-based
scheme, every method in between must check for the failure and pass it on:

```java
int r = d();
if (r < 0) return r;      // in c
```

Written in `c`, in `b`, and in `a`. Three checks that exist only to forward
something none of them can do anything about.

That is bad in three ways. It is repetitive. It obscures the main logic, which is
now interleaved with error plumbing. And it is easy to skip — one missing check
and the failure is silently converted into a wrong answer.

**Exceptions solve exactly this.** A thrown exception passes through the
intermediate frames without their cooperation:

```java
static void c() { d(); }        // no error handling at all
```

If `d` throws, `c` does nothing, and the exception continues to whoever is
prepared to handle it. The methods that have nothing to say about a failure do not
have to say anything.

That is the argument for exceptions in one sentence: **they decouple where a
failure is detected from where it is handled.**

## What it costs

Two real costs, both worth naming since this chapter is trying to be honest about
a feature with critics.

**The control flow is invisible.** Reading `c`, there is no sign that it can fail
or that the call to `d` might not return normally. Every method call becomes a
potential exit point, and a reader who needs to know must go and look. Go's
designers rejected exceptions largely for this reason.

**It is easy to catch too broadly.** `catch (Exception e)` catches everything,
including things you did not anticipate and cannot handle, and the usual result is
that a genuine bug is swallowed. Section 28.2.3 has more.

The position this chapter takes: the propagation problem is real and large, the
costs are real and smaller, and the resulting rule is that exceptions should be
used for what the caller cannot reasonably prevent, not as a general control-flow
device.

## When not to throw

Which gives the boundary.

**Do not throw for an expected outcome.** A lookup that misses is normal.
`Map.get` returning `null`, or better an `Optional`, is right; throwing
`KeyNotFound` for an ordinary absent key would make every caller wrap every
lookup.

**Do not throw for flow control.** Using an exception to exit a loop works and is
both slow and unreadable. Section 28.2.3 measures the slow part.

**Do not throw for what the caller could have checked.** If a caller can ask "is
this valid?" cheaply and reliably, offering that question is better than punishing
them for not knowing.

The test is roughly: **could a careful caller have avoided this?** If yes, give
them the means. If no — the file was deleted between the check and the open, the
network died mid-request — that is what exceptions are for.

Next: the mechanism.
