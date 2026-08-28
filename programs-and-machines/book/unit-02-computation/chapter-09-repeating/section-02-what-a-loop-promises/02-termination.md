# Termination

The invariant proves that **if** the loop finishes, the answer is right.

Read that sentence again with an eye on the word *if*, because there is a hole in
it large enough to hide a hanging program.

Here is the hole:

```java
int sum = 0;
for (int k = 1; k <= 5; k = k) {    // k never changes
    sum += k;
}
```

Check the invariant. Genuinely check it, do not take my word for it: at the top of
every iteration, is `sum` the total of 1 through `k-1`?

It is. `k` stays at 1 and `sum` stays at 0, and 0 is exactly the correct total of
the empty range from 1 to 0. The invariant holds on entry, it is preserved by the
body, and it holds forever.

Every single thing the last lesson asked of you is satisfied. And the program
hangs.

So correctness and termination are two separate obligations, and you have only met
one of them.

## Proving that a loop stops

The tool is a **variant**: a quantity that is a whole number, is never negative,
and **strictly decreases** on every iteration.

The argument for why that is enough is almost too short to feel like a proof. A
non-negative integer cannot decrease forever. To do so it would have to eventually
pass below zero, and it cannot. Therefore the loop can run at most as many times as
the variant's starting value, and then it must stop.

That is the whole thing. Take a second with it, because you have just been handed a
method for proving that something never happens, and those are rarer than they
should be.

For the summing loop, take `5 - k`:

- It is a whole number.
- It starts at 4, and the loop ends when it would go below 0.
- Each iteration runs `k++`, so `5 - k` goes down by exactly 1.

Therefore the loop terminates within 5 iterations. Not "appears to". Not "in every
case I tried". Must.

Now try the same variant on the broken version. It is `5 - k` again — and it does
not decrease, because `k = k` changes nothing. The proof fails, and notice *where*
it fails: at exactly the line that is wrong. The technique did not merely detect
the bug, it pointed at it.

## The two obligations together

| obligation | tool | establishes |
|---|---|---|
| correctness | invariant | if it stops, the answer is right |
| termination | variant | it stops |

You need both, and it is worth seeing what each failure looks like on its own. An
invariant with no termination proof describes a loop that may hang forever while
being perfectly correct about nothing. A termination proof with no invariant
describes a loop that reliably stops and may hand you nonsense.

The pair together is called **total correctness**, and it is the strongest claim
anyone can make about a loop.

## Where this stops being easy

For a counting loop the variant is so obvious that nobody writes it down. Things
get interesting when the progress is less direct. Read this one:

```java
int n = readNumber();
while (n != 1) {
    if (n % 2 == 0) n = n / 2;
    else            n = 3 * n + 1;
}
```

Even numbers get halved. Odd numbers get tripled and incremented. Does it stop?

Try 7 by hand if you like — it goes 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16,
8, 4, 2, 1. Sixteen steps and it lands. Try any starting value you can think of and
it reaches 1.

**Nobody knows whether it always does.**

This is the Collatz conjecture. It was posed in 1937. It has been verified by
computer for every starting value up to roughly $2^{68}$, and it is unproved. Paul
Erdős, who was not a man easily intimidated by a problem, said of it that
mathematics is not yet ready for such questions.

I put it in front of you because it makes the point better than any argument could.
Termination is genuinely a separate question from correctness, and it can be
genuinely hard. The termination of that six-line loop is an open problem in
mathematics.

It also quietly sets up Chapter 34. If deciding whether *that particular* loop
halts has defeated everyone for ninety years, you might reasonably guess that
deciding it for arbitrary programs is beyond any method whatsoever.

Your guess would be right. Turing proved it in 1936, a year before Collatz asked
his question.

## What to actually do

Most loops you write will terminate for reasons so immediate that thinking about it
would be a waste of your attention. The habit worth forming is narrower than "prove
your loops terminate".

**When a loop's progress is not a simple counter, ask what decreases.**

If you can name the quantity, you are done: the loop terminates, and you also know
its bound, which you got for free. If you cannot name it, that is worth a moment —
not because you have definitely written a bug, but because you have written
something you do not fully understand, and those are the ones that surprise you
later.

Loops that consume input, walk a structure, or halve a range are almost always
fine, and their variant is easy to say out loud. The ones to look at twice are
loops that adjust a value by an amount that depends on the value — which, as you
have just seen, is where the mathematicians got stuck too.

Next: the errors that live at the boundary.
