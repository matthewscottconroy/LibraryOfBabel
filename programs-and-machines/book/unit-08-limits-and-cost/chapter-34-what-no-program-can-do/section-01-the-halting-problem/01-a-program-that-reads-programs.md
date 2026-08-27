# A Program That Reads Programs

The halting problem asks for a program `halts(P, x)` that returns `true` if
program `P` eventually stops on input `x`, and `false` if it runs forever.

Before proving that cannot exist, two things need to be in place: that a program
can take a program as input at all, and that "halts" is a real question with
observable stakes.

Chapter 25 settled the first. An interpreter takes a program as data, and
`halts(P, x)` would take `P` as data the same way.

## Programs that halt, and one that does not

A minimal language: one register, and five instructions — increment, decrement,
jump if zero, jump, stop.

```
countdown:      DEC ; JZ 3 ; JMP 0 ; STOP
inc-forever:    INC ; JMP 0
halts-if-zero:  JZ 3 ; DEC ; JMP 0 ; STOP
```

Run each on several inputs, with a step budget, reporting steps taken or `?` if
the budget ran out. Verified:

```
  x=0    countdown      ?   inc-forever      ?   halts-if-zero      2
  x=3    countdown      9   inc-forever      ?   halts-if-zero     11
  x=10   countdown     30   inc-forever      ?   halts-if-zero     32
```

Three behaviors, and they are the three that exist.

`inc-forever` never halts, on any input. Obvious from reading it.

`halts-if-zero` always halts, in a number of steps that depends on the input.

`countdown` halts for positive inputs and **not** for zero — it decrements first,
so `x` goes to $-1$ and never reaches zero again. That is not a bug in the
demonstration; it is the interesting case. **Halting is a property of a program
and an input together**, not of a program alone, which is why `halts` takes two
arguments.

## Why a budget does not work

The obvious approach: run it and see.

```
countdown from 500 within budget 100 : ?
countdown from 500 within budget 2000: 1500
```

With a budget of 100 the answer was `?`. Not "it does not halt" — *unknown*. The
program was still running, and running it longer produced 1500 steps and a halt.

That is the whole difficulty. Simulation can confirm halting: if it stops, it
stops, and you know. Simulation can never confirm **non**-halting, because at any
moment the possibilities are "it will stop later" and "it never will", and no
finite observation separates them.

A decision procedure would have to reason about the program rather than watch it.

## Collatz

Chapter 9 introduced a loop and left a question hanging:

```java
while (n != 1) {
    n = (n % 2 == 0) ? n / 2 : 3 * n + 1;
}
```

Halve if even, triple and add one if odd, until you reach 1.

Verified:

```
  n=6      halts after      8 steps
  n=7      halts after     16 steps
  n=27     halts after    111 steps
  n=97     halts after    118 steps
  n=871    halts after    178 steps
  n=6171   halts after    261 steps
  longest under 100,000: n=77031 takes 350 steps
```

The irregularity is the point. Starting from 27 — a small number — takes 111
steps, and along the way the value climbs above 9,000. There is no pattern in the
step counts, and the largest under a hundred thousand is not near the largest
input.

Every value ever tested halts. It has been checked to about $2^{68}$. **Nobody has
proved it always does.** Collatz posed it in 1937 and it is open.

Which answers Chapter 9's question. "This loop terminates" is a claim requiring
proof, the proof can be arbitrarily hard, and for this six-line loop it has
defeated the field for nearly ninety years. Termination is not a formality you
check after establishing correctness; it is a separate question and it can be the
harder one.

Note also what a `halts` procedure would give you. Ask it about the Collatz loop
for every starting value, and you settle a famous open problem by running a
program. That is a strong hint that no such procedure exists — the same hint you
should feel about anything that would make hard mathematics automatic.

## What is being asked for

Precision matters here, because the proof turns on it.

We want a program `halts(P, x)` that:

- **always terminates.** It must give an answer. A procedure that loops forever on
  the hard cases is no use, and is exactly what "run it and see" already gives.
- **is always correct.** `true` when `P` halts on `x`, `false` when it does not.
- **works for every** `P` and `x`. Not for a restricted class.

All three matter. Relax any one and the impossibility goes away, which is Section
34.1.3's subject and is what every real tool does.

## Programs are countable

One more piece, and it is Chapter 24's promise.

A program is a finite string over a finite alphabet. So programs can be listed:
all of length 0, then all of length 1, then length 2, and so on. Verified, for a
two-symbol alphabet:

```
  strings of length <=  5 over {0,1}: 63
  strings of length <= 10 over {0,1}: 2,047
  strings of length <= 20 over {0,1}: 2,097,151
```

Finite at every length, so the whole set is **countably infinite** — every program
has a finite index in the list.

That is what makes the next lesson's argument possible: you can put the programs
in a row and number them.

It also proves something immediately. There are countably many programs. Chapter
24 observed there are uncountably many languages over an alphabet, because they
are all the subsets of a countably infinite set. So **most languages have no
recognizer**, by counting alone, before any specific one is exhibited.

That argument is Chapter 33's counting technique with infinite sets: count what
exists, count what the mechanism can produce, observe the second is smaller. It
tells you the impossible cases are the overwhelming majority, and it tells you
nothing about which. The next lesson exhibits one.

Next: the contradiction.
