# The State Table

The most reliable debugging technique anyone has found needs no debugger, no
tooling, and no cleverness whatsoever. It needs a piece of paper and a willingness
to be bored for ninety seconds.

Almost nobody does it. What people do instead is read the code again, harder — as
though the mistake might confess under pressure. It will not. You already read it
that way when you wrote it, and that is exactly how the mistake got in. Your eyes
skate over what you *meant*; the machine is running what you *typed*.

So we are going to stop reasoning about the program and start writing down what it
does, one line at a time, whether or not that feels beneath us.

A computation is a sequence of states — that was Chapter 6's whole claim. A
**state table** is that sequence written down: one row per step, one column per
variable.

## Doing one

```java
int a = 3;
int b = 4;
int t = a;
a = b;
b = t;
```

| after line | `a` | `b` | `t` |
|---|---:|---:|---:|
| `int a = 3;` | 3 | — | — |
| `int b = 4;` | 3 | 4 | — |
| `int t = a;` | 3 | 4 | 3 |
| `a = b;` | 4 | 4 | 3 |
| `b = t;` | 4 | 3 | 3 |

It swaps them. And the table shows *how* — the third variable exists because the
line `a = b` destroys the old value of `a`, so it has to be saved first.

Try removing `t` and see the table explain the failure:

```java
a = b;      // a is now 4, the old 3 is gone
b = a;      // b is set to 4, not 3
```

| after line | `a` | `b` |
|---|---:|---:|
| start | 3 | 4 |
| `a = b;` | 4 | 4 |
| `b = a;` | 4 | 4 |

Both are 4. The table makes the loss visible at the exact line where it happens,
which prose about "you need a temporary variable" does not.

## Tracing a loop

Loops are where tracing earns its cost. One row per iteration:

```java
int n = 13;
int count = 0;
while (n > 0) {
    if (n % 2 == 1) count++;
    n = n / 2;
}
```

| iteration | `n` at top | `n % 2` | `count` after | `n` after |
|---|---:|---:|---:|---:|
| 1 | 13 | 1 | 1 | 6 |
| 2 | 6 | 0 | 1 | 3 |
| 3 | 3 | 1 | 2 | 1 |
| 4 | 1 | 1 | 3 | 0 |
| — | 0 | — | 3 | — |

The loop ends with `count` equal to 3.

What does it compute? 13 in binary is `1101`, which has three 1s. The loop counts
the set bits — it repeatedly takes the lowest bit and shifts right, which is
Chapter 2's repeated-division conversion with only the remainders counted.

I would not have seen that from reading the code. The table showed it, because
the column of `n` values — 13, 6, 3, 1 — is the division sequence, and once you
notice that the rest follows.

That is the second use of tracing: not only checking whether code is right, but
working out what unfamiliar code *does*.

## How to do it properly

**One row per state change, not per line of source.** A loop's fifth iteration
gets its own row.

**A column for every variable in scope.** Including the ones you think are
irrelevant, because if you were certain which ones mattered you would not be
tracing.

**Write the value after the line executes.** Pick a convention and hold it; half
the errors in hand-tracing come from being unsure whether a cell is before or
after.

**Do not skip ahead.** The temptation, three rows in, is to say "and this
continues for a while" and jump to the end. That is precisely where the bug is
hiding, because if you could reliably predict the middle you would not have a
bug.

**Stop when the pattern is genuinely established.** Not when it is boring — when
you can state the invariant. If you can write down what is true at the top of
every iteration, you have got what tracing was for, and Chapter 9's method takes
over from here.

That last point is the relationship between the two chapters. Tracing is how you
*find* the invariant when it is not obvious. The invariant is how you avoid
tracing the other nine hundred iterations.

## Why bother, when you could print

A fair objection: adding a print statement inside the loop produces the same table
in a second.

Two answers.

**Printing tells you what happened; tracing tells you what you expected.** The
value of hand-tracing is that you commit to a prediction. When the trace and the
run disagree, you have located a false belief precisely — and the false belief is
the bug, not the symptom.

**You will not always be able to run it.** Reading unfamiliar code in a review,
reasoning about a failure you cannot reproduce, working out what a fragment does
before you decide whether to use it — none of these involve a machine.

Trace by hand until the machine holds no surprises. That takes maybe a dozen
serious traces, and afterwards you will do it in your head for short code and on
paper for hard code, which is what experienced programmers are doing when they
appear to be staring at a screen.
