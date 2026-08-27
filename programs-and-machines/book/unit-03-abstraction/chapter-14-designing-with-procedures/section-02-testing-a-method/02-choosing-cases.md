# Choosing Cases

A test suite is only as good as its cases. Ten tests of ordinary input give
confidence without warrant; three well-chosen ones can establish a great deal.

This lesson is about choosing.

## Equivalence classes

The idea: **inputs fall into groups whose members are handled the same way.** Test
one from each group, and adding more from that group tells you little.

For `largest`:

- arrays with one element
- arrays with the maximum first
- arrays with the maximum last
- arrays with the maximum in the middle
- arrays with duplicates
- arrays that are all negative

Six classes. Testing `{1,2,3}` and `{1,2,4}` and `{1,2,5}` gives you one class
three times.

This is why a suite's *size* means less than people assume. What matters is how
many distinct classes it covers.

## Boundary values

Within each class, the interesting values are at the edges.

Bugs cluster at boundaries because that is where the code's decisions change —
Chapter 9's four range conventions differ exactly at the ends, and a `<` that
should be `<=` produces correct behavior everywhere except at one value.

So test:

- the smallest acceptable input, and one below it
- the largest, and one above it
- zero, where it is meaningful
- negative values, where they are accepted
- the empty case
- the one-element case

The last two find more bugs than any others, in my experience. `largest` on an
empty array; a loop over an empty list; a `substring` of an empty string. Code is
usually written with a typical case in mind and the degenerate cases are an
afterthought.

## Some standard traps

Cases worth having in mind, because each catches a recurring mistake.

**Empty.** Empty array, empty string, empty collection.

**One.** A single element frequently exercises a different path than several,
because loops that run once do not reveal iteration bugs.

**Two.** The smallest input where order matters.

**Duplicates.** `{4, 4, 4}` breaks code that assumes distinctness, and code that
counts "how many are larger" gets it wrong.

**Already sorted, and reverse sorted.** For anything involving order, these are the
extremes and are often the best and worst cases for performance too.

**The extreme values of the type.** `Integer.MAX_VALUE` and `MIN_VALUE`. Chapter 2
showed what arithmetic near them does, and Chapter 13 showed `Math.abs` returning
a negative number.

**Null**, wherever a reference is accepted. Chapter 12 introduced it; Chapter 16
discusses it properly. If the contract says null is not allowed, test that the
method rejects it as promised.

## Testing the contract's failure

If a method promises to throw on bad input, that promise deserves a test:

```java
static void checkThrows(String name, Runnable r) {
    try {
        r.run();
        failed++; System.out.println("FAIL " + name + ": expected an exception");
    } catch (IllegalArgumentException e) {
        passed++;
    }
}

checkThrows("empty array rejected", () -> largest(new int[]{}));
```

The `() ->` is a lambda, which is Chapter 26. For now read it as "a piece of code
to run later".

This matters because failure behavior is part of the contract. A method that
quietly returns garbage instead of throwing has broken its promise as surely as
one that returns the wrong maximum.

## Coverage, and its limits

You will meet the term **code coverage**: the fraction of lines or branches your
tests execute. Tools report it, and it is genuinely useful for finding code that
is tested by *nothing*.

It is a poor target, and it is worth understanding why.

```java
static int divide(int a, int b) {
    return a / b;
}
```

One test, `divide(6, 2)`, gives 100% line coverage. It also misses the only
interesting case — `b` being zero, which throws.

Coverage tells you which lines *ran*. It says nothing about whether the assertions
were meaningful, whether the interesting inputs were tried, or whether the expected
values were right. **High coverage is necessary and nowhere near sufficient**, and
teams that make it a target reliably get tests that execute everything and check
nothing.

Use it as Chapter 10 used bisection: to find the region you have not looked at.
Not as a score.

## How many tests

A question with no numerical answer, and a useful reframing:

**Enough that you would be surprised if a change broke something without a test
failing.**

That is a statement about your own confidence, which is the thing actually at
stake. If you would change `largest` and then run the program by hand to check —
your tests are not doing their job.

Next: what tests tell a reader.
