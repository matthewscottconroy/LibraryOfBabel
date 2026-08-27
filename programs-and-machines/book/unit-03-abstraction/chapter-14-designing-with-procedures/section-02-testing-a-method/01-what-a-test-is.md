# What a Test Is

A **test** is a piece of code that runs your code and checks the result against
what it should be.

That is all. It is not a special kind of artifact and it needs no framework to
begin with:

```java
public class TestLargest {

    static int largest(int[] a) {
        int max = a[0];
        for (int i = 1; i < a.length; i++)
            if (a[i] > max) max = a[i];
        return max;
    }

    static int passed = 0, failed = 0;

    static void check(String name, int expected, int actual) {
        if (expected == actual) { passed++; }
        else { failed++; System.out.println("FAIL " + name + ": expected " + expected + ", got " + actual); }
    }

    public static void main(String[] args) {
        check("single element",  5, largest(new int[]{5}));
        check("ascending",       3, largest(new int[]{1, 2, 3}));
        check("descending",      3, largest(new int[]{3, 2, 1}));
        check("all negative",   -2, largest(new int[]{-5, -2, -9}));
        check("duplicates",      4, largest(new int[]{4, 4, 4}));
        check("max at end",      9, largest(new int[]{1, 9}));
        System.out.println(passed + " passed, " + failed + " failed");
    }
}
```

```
6 passed, 0 failed
```

Real tests, doing the real job. In practice you would use JUnit — Appendix B
covers it — which supplies the `check` method, finds your tests automatically, and
reports nicely. It does not change what a test *is*.

## Why bother

Four reasons, and the first is the least important despite being the one usually
given.

**It catches bugs now.** True, and you would probably have caught these by running
the program.

**It catches bugs later.** This is the real value. In six months you will change
`largest` for some reason — to handle a new case, to make it faster — and the
tests will tell you immediately if you broke something. Without them you will not
know until something downstream misbehaves, possibly much later, and Chapter 10
showed what that distance costs.

**It lets you change code without fear.** This is the same point stated as an
enabling condition rather than a safety net, and it is worth separating.
Untested code becomes code nobody dares touch, and code nobody dares touch
accumulates workarounds instead of fixes. A test suite is what makes improvement
affordable.

**It forces you to use your own interface.** Writing a test means calling the
method as a client. Awkwardness shows up immediately — too many parameters, an
unclear return, a dependency that makes the method impossible to call in
isolation. Several design problems are easier to notice from the outside.

## The contract, executed

Here is the connection to Chapter 11.

The contract for `largest` was:

> **Requires:** a non-null array with at least one element.
> **Ensures:** returns the largest element; the array is unmodified.

A test is that, made executable. Each case supplies something satisfying the
precondition and checks the postcondition. The comment says what should be true;
the test checks whether it is.

Which means the two are not separate practices. **A test is a contract you can
run**, and a contract is a test you have not automated yet. When the code changes
and the contract no longer holds, a comment stays silently wrong and a test fails
loudly.

That difference — failing loudly rather than being silently wrong — is the same
distinction this book has been drawing since Unit I, and it is the argument for
tests in one line.

## What to test

For a single method, three categories:

**The normal cases.** What it is for. `largest` of `{1, 2, 3}`.

**The boundaries.** Chapter 9 and Chapter 10 both said this: the smallest input,
the largest, the one-element case, the values at the edges of ranges. Bugs live
here because this is where the four range conventions differ.

**The contract's edges.** What happens at the precondition's boundary — the
one-element array, which is the smallest thing `largest` accepts.

And a category worth naming separately: **the bugs you have already found.** When
you fix a defect, add a test that would have caught it. It cannot come back
silently, and over time these accumulate into a suite that reflects the actual
history of what goes wrong in your code, which is more valuable than any suite
designed in advance.

## When not to test

Not everything deserves a test, and pretending otherwise makes the practice feel
oppressive.

A three-line private helper whose behavior is plain and which is covered
thoroughly by the tests of the method that uses it does not need its own. A
throwaway script does not need a suite.

The judgment is about **consequence and volatility**. Code that matters, or that
will change, or that others depend on, earns tests. Code that is trivial, stable,
and private earns less.

What is not a good reason to skip: "it is clearly correct". Chapter 10's
`largest` looked clearly correct and returned 0 for all-negative input.

Next: choosing the cases.
