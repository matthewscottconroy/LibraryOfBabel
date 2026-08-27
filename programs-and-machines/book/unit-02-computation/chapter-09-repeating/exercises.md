# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Reading loops

**9.1.** For each, say how many times the body runs and what the first and last
values of the counter are:
```java
for (int i = 0; i < 5;  i++)
for (int i = 0; i <= 5; i++)
for (int i = 1; i < 5;  i++)
for (int i = 5; i > 0;  i--)
for (int i = 0; i < 10; i += 3)
```

**9.2.** Rewrite as a `while` loop, preserving the counter's scope:
```java
for (int i = 1; i <= 5; i++) { total += i; }
```

**9.3.** Predict the output:
```java
int n = 0;
do { n++; } while (n < 0);
System.out.println(n);
```
Then say when `do`-`while` is the right choice.

**9.4.** Trace the nested loop in Section 9.1.3 and give the twelve values
printed, in order.

## Invariants

**9.5.** State the loop invariant:
```java
int count = 0;
for (int i = 0; i < a.length; i++)
    if (a[i] < 0) count++;
```

**9.6. [carries forward]** For this loop, state the invariant and then carry out
all three obligations — establishment, preservation, termination — in writing:
```java
int product = 1;
for (int k = 1; k <= n; k++) product *= k;
```
What does the loop compute?

**9.7.** State the invariant, then find the bug:
```java
int max = a[0];
for (int i = 0; i <= a.length; i++)
    if (a[i] > max) max = a[i];
```
Which of the three obligations fails, and what happens at run time?

**9.8.** This loop reverses an array. State its invariant. (Hint: describe what
is true of the two ends.)
```java
for (int i = 0, j = a.length - 1; i < j; i++, j--) {
    int tmp = a[i]; a[i] = a[j]; a[j] = tmp;
}
```

**9.9. [carries forward]** Write a loop that computes the sum of the *even*
numbers from 1 to *n*, then state and check its invariant.

## Termination

**9.10.** Give a variant for each and say how many iterations it bounds:
```java
for (int i = 0; i < n; i++)
while (x > 1) x = x / 2;
while (!queue.isEmpty()) queue.remove();
```

**9.11.** Show that this loop's invariant holds and that it never terminates.
Explain why both facts can be true at once.
```java
int sum = 0;
for (int k = 1; k <= 5; k = k) sum += k;
```

**9.12.** Trace the Collatz loop from Section 9.2.2 starting at 6, listing every
value until it reaches 1. How many steps?

**9.13.** Explain in your own words why "the invariant holds" and "the loop
terminates" are separate obligations, and what each one alone would leave you
without.

## Off-by-one

**9.14.** A fence 100 metres long has a post every 10 metres, including both
ends. How many posts? Explain the connection to `<` versus `<=`.

**9.15.** Section 9.2.3 gives three reasons Java uses half-open ranges. Restate
each in your own words, with an example.

**9.16.** For an array of length 5, which of these are valid and what does each
do?
```java
for (int i = 0; i <  a.length;     i++)
for (int i = 0; i <= a.length;     i++)
for (int i = 0; i <  a.length - 1; i++)
for (int i = 1; i <= a.length;     i++)
```

**9.17.** Rewrite using the enhanced `for`, and say which class of bug that
removes:
```java
for (int i = 0; i < names.length; i++)
    System.out.println(names[i]);
```

## Going further

**9.18.** Section 9.2.1 claims that writing the invariant *first* and deriving the
loop from it produces correct code by construction. Try it: without writing any
loop yet, state an invariant for "find the index of the first negative number, or
−1 if there is none". Then write initialization that establishes it, a body that
preserves it, and a condition whose failure gives the answer.

**9.19.** The chapter says a loop invariant is mathematical induction applied to a
program. Write out the correspondence explicitly: what is the base case, what is
the inductive step, and what plays the role of "for all n"?

**9.20.** The Collatz conjecture is unproved. Explain what that means for the
claim "this loop terminates", and connect it to what Chapter 34 will say about
the halting problem in general.
