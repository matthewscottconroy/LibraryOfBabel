# Gates and Circuits

A truth table says what an operator does. A **gate** is a physical arrangement
that does it.

## From switches to logic

Chapter 1 left us with a transistor: a switch controlled by a voltage. Wire two
of them in series — so current flows only if *both* are closed — and you have
built AND. Wire them in parallel, so current flows if *either* is closed, and you
have OR.

That is the entire bridge between physics and logic, and it is why Shannon's 1937
observation was so consequential. He did not invent the circuits; engineers had
been building relay networks for decades. What he supplied was the recognition
that Boole's algebra — published in 1854, and concerned with the laws of thought
— describes exactly what those circuits do.

The consequence is that a circuit becomes an object you can calculate with. You
can simplify it algebraically, prove two designs equivalent, and derive an
implementation from a specification, rather than building something and testing
whether it works.

The standard symbols:

```
   A ──┐                 A ──┐                    
       ├─ AND ── out         ├─ OR ── out      A ──○── NOT ── out
   B ──┘                 B ──┘                    
```

## Building an adder

Now we can build the thing Chapter 2 described but did not construct.

Recall the four cases of one-column binary addition:

```
0 + 0 = 0
0 + 1 = 1
1 + 0 = 1
1 + 1 = 0, carry 1
```

Write that as a truth table with 1 for true, taking two input bits and producing
a sum bit and a carry bit:

| A | B | sum | carry |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 1 | 1 | 0 |
| 1 | 0 | 1 | 0 |
| 1 | 1 | 0 | 1 |

Look at the `sum` column. It is 0, 1, 1, 0 — which is exactly **XOR**.

Look at the `carry` column. It is 0, 0, 0, 1 — which is exactly **AND**.

So:

```
sum   = A XOR B
carry = A AND B
```

Two gates. That is a **half adder**, and it is the arithmetic of Chapter 2
implemented in logic.

It is called *half* because it cannot accept a carry coming in from the column to
its right. A **full adder** takes three inputs — A, B, and carry-in — and is built
from two half adders plus an OR:

```
sum       = (A XOR B) XOR carryIn
carryOut  = (A AND B) OR ((A XOR B) AND carryIn)
```

Read the carry line: a carry goes out if both inputs were 1, *or* if exactly one
input was 1 and a carry came in. Which is what you do on paper.

Chain 32 full adders, each one's carry-out feeding the next one's carry-in, and
you have a circuit that adds two 32-bit integers. That is the adder Chapter 2
described in words. It is roughly 160 gates, and it is the reason
`Integer.MAX_VALUE + 1` wraps: the thirty-second carry-out has no thirty-third
adder to go to.

## The point

Take a moment with what just happened.

We started with a physical switch. We wired switches into gates that implement a
small algebra. We wrote down the truth table for adding two bits, noticed it was
two of our gates, and chained the result into an arithmetic unit.

Arithmetic was not built in. It was **derived** — from logic, which was derived
from switching, which was derived from Chapter 1's decision to recognize two
voltage levels.

And nothing in the chain understands numbers. The adder does not know it is
adding. It is a lattice of switches whose steady state happens to encode, under
our agreement, the sum of what we put in.

This is the same observation as the parity machine in Chapter 6, at a lower
level. Behavior that looks like understanding is arrangement.

## Everything else, too

The same construction gives the rest of a processor.

**Comparison.** `A == B` for single bits is NOT XOR. For 32-bit values, XOR each
pair of bits and check that all results are 0 — which is a chain of ORs followed
by a NOT.

**Selection.** A **multiplexer** takes two inputs and a selector bit and outputs
one of them: `out = (S AND B) OR (NOT S AND A)`. This is `if` in hardware, and it
is where Chapter 6's "consult the state to choose a continuation" physically
happens.

**Memory.** Cross-couple two NOR gates so each feeds the other's input, and the
pair has two stable states — it holds a bit. That is a latch, and it is how a
circuit remembers, which is how the state of Chapter 6 exists at all.

Arithmetic, comparison, choice, and memory. All of it from NAND; NAND from
switches; switches from the agreement about voltage in Chapter 1.

Next: the algebra that lets you rewrite conditions without changing them.
