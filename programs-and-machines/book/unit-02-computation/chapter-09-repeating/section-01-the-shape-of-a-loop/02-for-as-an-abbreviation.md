# for as an Abbreviation

The three parts of a loop — initialization, condition, progress — are scattered in
a `while`:

```java
int i = 1;              // up here
while (i <= 5) {        // here
    total += i;
    i++;                // and down here, easy to lose
}
```

The `for` loop gathers them onto one line:

```java
for (int i = 1; i <= 5; i++) {
    total += i;
}
```

```
for (initialization; condition; progress) {
    body
}
```

Same loop, same behavior. What changed is that the three parts a reader must
check are adjacent, so checking them is one glance rather than three searches.

## Two real benefits

**The parts cannot be lost.** The commonest cause of an infinite loop is
forgetting the progress step. In a `for`, its absence is visible in the header.

**The counter's scope is the loop.** Declaring `int i` in the header means `i`
does not exist afterwards:

```java
for (int i = 0; i < 3; i++) { }
System.out.println(i);      // error: cannot find symbol
```

Chapter 7 argued for the smallest workable scope. `for` gives you the smallest
possible one for free, and it means you can use `i` again in the next loop
without wondering what it holds.

## When to use which

Not a matter of taste; the two express different things.

**Use `for` when you are counting** — a known number of iterations, or a walk over
a range of indices. The header states the range, and a reader learns the extent of
the loop before reading its body.

**Use `while` when you are waiting for a condition** and there is no counter:

```java
while (!queue.isEmpty()) {
    process(queue.remove());
}
```

Forcing that into a `for` would leave two of three slots empty and communicate
nothing.

The test I use: if the header would have blanks in it, use `while`.

## The counting conventions

Two idioms dominate, and the difference is worth being deliberate about.

```java
for (int i = 0; i < n; i++)      // n iterations, i from 0 to n-1
for (int i = 1; i <= n; i++)     // n iterations, i from 1 to n
```

Both run *n* times. The first is used for anything index-based, because arrays in
Java are indexed from 0 — Chapter 15's subject, and the reason is the address
arithmetic of Chapter 1. The second is used when the numbers are quantities
rather than positions, as in "sum the numbers 1 to n".

Mixing them up is the most common off-by-one in the language. Some counts to keep
in mind:

```java
for (int k = 0; k < 5;  k++)   // 5 iterations: 0,1,2,3,4
for (int k = 0; k <= 5; k++)   // 6 iterations: 0,1,2,3,4,5
for (int k = 1; k < 5;  k++)   // 4 iterations: 1,2,3,4
```

The second is almost always a mistake when the 5 came from a length. The next
section gives you a way to be sure rather than to remember.

## Some things for allows and you should mostly not do

The header's slots accept more than they need to:

```java
for (int i = 0, j = n; i < j; i++, j--)     // two counters
for (;;) { }                                 // no parts at all: infinite
```

The two-counter form is occasionally the clearest way to walk inward from both
ends, and reversing an array is the standard example. Beyond that, a header doing
several things is a header nobody reads carefully.

`for (;;)` is an infinite loop, equivalent to `while (true)` and less obvious.
Prefer `while (true)`, which says so in a word.

You can also leave the body empty, putting all the work in the header. Do not.
The semicolon that ends such a loop is invisible:

```java
for (int i = 0; i < 5; i++);      // note the semicolon
    System.out.println(i);        // not in the loop; also will not compile
```

## The enhanced for

There is a fourth form, which we mention now and use from Chapter 15:

```java
for (String name : names) {
    System.out.println(name);
}
```

Read as "for each name in names". It has no counter, no condition, and no
progress — the language handles them — which means it cannot have an off-by-one
error at all. That is a strong argument, and the rule that follows is: **when you
do not need the index, do not have one.**

It requires something to iterate over, so it waits for arrays and collections.

Next: what happens when loops contain loops.
