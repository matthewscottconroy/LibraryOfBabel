# The Contradiction

Here is the whole proof. It is about fifteen lines, most of them a program you
could write this afternoon if the thing it depends on existed.

Read it slowly the first time — it is short enough to feel like a trick, and it is
not one. Everything it does is ordinary programming, which is exactly what makes
the conclusion unavoidable.

Suppose `halts(P, x)` exists: always terminates, always correct, works for every
program and input.

Then write this:

```java
void trouble(Program P) {
    if (halts(P, P)) {
        while (true) { }          // loop forever
    } else {
        return;                   // halt
    }
}
```

`trouble` takes a program `P`, asks whether `P` halts when given *itself* as
input, and then does the opposite.

Passing a program its own text is legal: a program is a string, and `trouble`
takes a string. Chapter 25's interpreter could be given its own source.

Now ask the question the whole proof turns on.

**Does `trouble` halt when given `trouble`?**

**Case 1: it halts.** Then `halts(trouble, trouble)` returned `true`. But the `if`
branch runs when `halts` says `true`, and that branch loops forever. So `trouble`
does not halt. Contradiction.

**Case 2: it does not halt.** Then `halts(trouble, trouble)` returned `false`. But
the `else` branch runs when `halts` says `false`, and that branch returns. So
`trouble` halts. Contradiction.

Both cases are impossible. `trouble` is built from `halts` by ordinary
programming — a call, an `if`, a loop — so the only assumption that can be wrong
is that `halts` exists.

**No such program exists.** The halting problem is **undecidable**.

## What happened

So what actually did the work there?

**Self-reference.** `trouble` is applied to itself. That is only possible because
a program is data, which is Unit VI's entire thesis and the reason this proof
comes after Chapter 25 rather than after Chapter 6.

**Negation.** `trouble` does the *opposite* of what it is told. A predictor that
is always correct is defeated by something that always contradicts it.

**Ordinary construction.** Nothing exotic was used. If `halts` existed, `trouble`
would be an easy program to write.

That combination is the classic shape of a diagonal argument, and it is the same
one Cantor used in 1891 to show the real numbers are uncountable, and Gödel used
in 1931 for incompleteness. Turing's is the computational version and he was
explicit about the debt.

## The diagonal, made visible

Why "diagonal". Imagine a table: rows are programs, columns are inputs, and the
entry is 1 if that program halts on that input.

Verified, on a finite example:

```
        in0  in1  in2  in3  in4  in5
  P0    1    0    0    1    0    1       <- diagonal entry 1
  P1    0    1    0    0    1    1       <- diagonal entry 1
  P2    0    0    1    1    0    0       <- diagonal entry 1
  P3    1    1    1    0    0    0       <- diagonal entry 0
  P4    1    0    0    0    0    1       <- diagonal entry 0
  P5    1    1    1    0    0    1       <- diagonal entry 1

  D:    0    0    0    1    1    0       <- differs from P_i at input i, for every i
```

`D` is built by walking the diagonal and flipping each entry. So `D` disagrees with
$P_0$ at input 0, with $P_1$ at input 1, and in general with $P_i$ at input $i$.

Therefore **`D` is not any row of the table.**

In the real argument the table is infinite — the rows are all programs, which
Section 34.1.1 showed is a countable list — and `D` is `trouble`. Since every
program is a row, and `D` is not a row, `D` is not a program. But `D` was
constructed from `halts` by ordinary means, so `halts` is not a program either.

The finite table above is not the proof; it is the picture. The proof needs the
rows to be *all* programs, which requires the countability from the last lesson.

## Once one thing is undecidable

The halting problem is not an isolated curiosity. Almost everything interesting
about a program's behavior reduces to it.

**Does this program ever print "hello"?** Suppose you could decide that. Then given
any `P` and `x`, build `P'` that runs `P` on `x` and then prints "hello". `P'`
prints "hello" exactly when `P` halts. So deciding printing would decide halting,
which is impossible. Therefore deciding printing is impossible.

That technique is **reduction**, and it is how undecidability spreads. Show that
solving your problem would solve the halting problem, and you have shown yours is
unsolvable — without any diagonal argument of your own.

By reduction, all of these are undecidable:

- does this program ever reach line 42?
- do these two programs compute the same function?
- does this program ever dereference null?
- is this variable ever unused?
- does this program contain a virus, for any behavioral definition of virus?
- is this the shortest program computing this function?

## Rice's theorem

In 1951 Henry Rice proved the general version, and it is worth meeting properly,
because it is enormously stronger than any list of examples could be.

> Every non-trivial property of a program's **behavior** is undecidable.

"Non-trivial" means some programs have it and some do not. "Behavior" means what
the program computes, not what its text looks like.

So there is no clever special case. Not "does it halt", not "is it correct", not
"is it secure" — any question about what a program *does*, unless the answer is the
same for all programs, has no decision procedure.

The escape clause is the word *behavior*. Questions about a program's **text** are
perfectly decidable: how many lines it has, whether it uses `goto`, whether a
variable is declared before use, whether the types check. That is why compilers
work.

**Compilers analyse syntax, which is decidable. They cannot analyse behavior,
which is not.** Every static analysis tool lives on that line, and Section 34.1.3
is about what they do at it.

Next: what this actually rules out.
