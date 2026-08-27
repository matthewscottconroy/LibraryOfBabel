# Termination

The invariant proves that **if** the loop finishes, the answer is right. It says
nothing about whether it finishes.

That is a separate obligation, and it is worth seeing that it really is separate:

```java
int sum = 0;
for (int k = 1; k <= 5; k = k) {    // k never changes
    sum += k;
}
```

The invariant still holds. `sum` is still the total of 1 to `k-1` at the top of
every iteration — forever, because `k` stays 1 and `sum` stays 0, and 0 is
correctly the total of the empty range.

Everything the last lesson asked for is satisfied, and the program hangs.

## Proving termination

The standard technique uses a **variant**: a quantity that is a whole number,
never negative, and **strictly decreases** every iteration.

If such a quantity exists, the loop must stop. A non-negative integer cannot
decrease forever — it would have to pass below zero, which it cannot. So the loop
can only run as many times as the variant's starting value.

For our summing loop, take `5 - k`:

- It is a whole number.
- It starts at 4 and the loop stops when it would go below 0.
- Each iteration runs `k++`, so `5 - k` decreases by exactly 1.

Therefore the loop terminates, in at most 5 iterations. Not "seems to" — must.

For the broken version, the variant is `5 - k` again, and it does not decrease,
because `k = k` changes nothing. The proof fails, and it fails at exactly the
line that is wrong.

## The two obligations

Put them together, because this is the complete picture:

| obligation | tool | establishes |
|---|---|---|
| correctness | invariant | if it stops, the answer is right |
| termination | variant | it stops |

Both are needed. An invariant with no termination proof describes a loop that may
hang. A termination proof with no invariant describes a loop that stops and may
produce nonsense.

The pair together is called **total correctness**, and it is the strongest thing
you can say about a loop.

## Where termination gets hard

For counting loops the variant is obvious and nobody writes it down. It becomes
interesting when progress is less direct:

```java
int n = readNumber();
while (n != 1) {
    if (n % 2 == 0) n = n / 2;
    else            n = 3 * n + 1;
}
```

Even numbers are halved; odd numbers are tripled and incremented. Does it stop?

For any value you try, yes. 7 goes to 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5,
16, 8, 4, 2, 1 — sixteen steps. Try any starting value and it reaches 1.

**Nobody knows whether it always does.** This is the Collatz conjecture, posed in
1937, verified by computer for every starting value up to about $2^{68}$, and
unproved. Paul Erdős said of it that mathematics is not yet ready for such
problems.

I include it because it makes the point that termination is genuinely a separate
question, and can be genuinely hard. That six-line loop's termination is an open
problem in mathematics.

It also previews Chapter 34. If deciding whether *this* loop halts is beyond us,
you might guess that deciding it for arbitrary programs is beyond any method at
all. That guess is correct, and Turing proved it in 1936.

## In practice

Most loops you write will terminate for reasons that are immediate, and you will
not think about it. The habit worth having is narrower:

**When you write a loop whose progress is not a simple counter, ask what
decreases.**

If you can name the quantity, the loop terminates and you know its bound. If you
cannot, that is worth a moment's attention — not because you have necessarily
written a bug, but because you have written something you do not fully understand,
and those are the ones that surprise you.

Loops that consume input, walk a structure, or halve a range are usually fine and
the variant is easy to name. Loops that adjust a value by an amount that depends
on the value are the ones to look at twice.

Next: the errors that live at the boundary.
