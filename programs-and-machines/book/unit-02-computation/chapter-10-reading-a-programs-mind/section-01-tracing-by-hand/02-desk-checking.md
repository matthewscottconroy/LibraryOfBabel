# Desk Checking

Tracing is the mechanism. **Desk checking** is using it deliberately: reading code
with the intention of finding what is wrong before a machine tells you.

The name comes from an era when machine time was scarce and expensive. You got
one run per day, so you checked your program at your desk first, because a
mistake cost twenty-four hours. That constraint is gone and the technique
survived, because it turns out to find things that running does not.

## The method

**1. Write down what you expect, before looking closely.** One sentence: *this
should return the largest element*. Vague is fine; the point is committing before
you are influenced by what the code does.

**2. Choose inputs that probe the boundaries.** Chapter 9's advice, and it applies
here. The empty case, the one-element case, the values at the ends of ranges. Not
a five-element list of ordinary numbers, which almost anything handles.

**3. Trace, and predict each step before writing it.** This is the part people
skip and it is the part that works. Say what the next row will be, then work out
what it actually is. When they differ, stop — you have found the divergence, and
the divergence is the bug.

**4. Compare the result with step 1.**

## Worked

Suppose this is supposed to find the largest element:

```java
static int largest(int[] a) {
    int max = 0;
    for (int i = 0; i < a.length; i++) {
        if (a[i] > max) max = a[i];
    }
    return max;
}
```

Expectation: returns the largest element.

Boundary input: an array of all negatives, `{-5, -2, -9}`.

| `i` | `a[i]` | `a[i] > max`? | `max` |
|---:|---:|---|---:|
| start | — | — | 0 |
| 0 | −5 | −5 > 0? no | 0 |
| 1 | −2 | −2 > 0? no | 0 |
| 2 | −9 | −9 > 0? no | 0 |

Returns 0. There is no 0 in the array.

The bug is `int max = 0`, which assumes the largest element is at least 0. On any
array of ordinary positive numbers it works, which is why it survives casual
testing. On all-negative input it returns a value that was never there.

The fix is to start from a value that *is* in the array:

```java
int max = a[0];
for (int i = 1; i < a.length; i++) { ... }
```

And note that Chapter 9's invariant catches the same bug from the other
direction: *`max` is the largest of `a[0..i-1]`* fails at establishment for the
original, because before the loop `max` is 0 and `a[0..-1]` is empty.

Two methods, one defect. That is normal, and it is why the methods are worth
having both of.

The fixed version has a new problem — `a[0]` fails on an empty array — and
deciding what `largest` should do with no elements is a design question rather
than a coding one. Chapter 11 takes up how a method states what it requires.

## Where desk checking beats running

**Boundary cases you would not think to test.** Choosing inputs deliberately is
part of the method; running whatever you happen to have is not.

**Code you cannot run.** Reviewing someone's change, reading a library to decide
whether it does what you need, reasoning about a crash from a log.

**Bugs that hide.** A defect producing a plausible-looking wrong answer will pass
a run you did not scrutinize. `largest` returning 0 looks like an answer.

**Understanding, as opposed to confirming.** Running tells you the output.
Tracing tells you the mechanism, which is what you need in order to change it
safely.

## Where running beats desk checking

Honesty demands the other list.

Machines are faster and do not get bored. They do not skip the tedious middle,
and they do not unconsciously trace what the code *should* say. That last one is
the real hazard of desk checking: you read what you meant rather than what you
wrote, which is the same reason proofreading your own writing fails.

The defense is to trace mechanically — evaluate the expression as written, not as
intended — and to be most suspicious exactly where you feel most confident.

Use both. Desk check to understand and to probe boundaries; run to confirm and to
cover volume. Chapter 14 makes the running part systematic with automated tests.

Next: what to do when the program has already failed.
