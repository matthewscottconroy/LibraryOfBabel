# Choosing Cases

A test suite is worth exactly as much as its cases are, and no more.

That is worth being blunt about, because a passing suite feels like evidence and
often is not. Ten tests of perfectly ordinary input will give you a green tick and
a warm feeling and establish almost nothing. Three well-chosen ones can establish a
great deal.

So the skill here is not writing tests. It is choosing what to test, and this
lesson is about how.

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

Those last two find more bugs than all the others put together, in my experience.
`largest` on an empty array. A loop over an empty list. A `substring` of an empty
string.

The reason is not mysterious, and knowing it will make you a better tester
immediately: **code gets written with a typical case in mind.** Whoever wrote the
method was picturing an array with several things in it. The degenerate cases were
an afterthought, if they were thought about at all — which is precisely why that is
where you should go looking.

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

It is also a poor target, and rather than argue that, let me show you. Read this
method and the test below it, and work out what percentage of its lines the test
covers.

```java
static int divide(int a, int b) {
    return a / b;
}
```

One test, `divide(6, 2)`, and you have 100% line coverage. A perfect score.

It also misses the only interesting thing that can happen in this method — `b`
being zero — which throws.

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
