# What It Does and Does Not Mean

Undecidability is the most over-claimed result in this book's subject, and the
overclaiming runs in both directions — people conclude both far too much and far
too little from it. This lesson is the careful version.

## What it does not mean

**It does not mean you cannot tell whether a particular program halts.**

```java
System.out.println("hi");
```

That halts, and nobody needs a theory to see it. And:

```java
while (true) { }
```

does not. Also obvious. The result says no **single procedure** works for **all**
programs — not that every case is hard. Most cases in practice are easy.

**It does not mean the halting question is unanswerable for large classes of
programs.** Any program without loops or recursion halts, trivially, and that is
decidable. Any program whose loops all have constant bounds halts. Whole languages
are designed to be **total** — every program halts by construction — at the cost of
not being Turing complete. Coq's and Agda's termination checkers, and SQL without
recursive queries, are examples.

**It does not mean software verification is pointless.** Enormous programs have
been formally verified: the seL4 microkernel, the CompCert C compiler,
cryptographic implementations. Undecidability says no procedure works
automatically for everything; it says nothing against proving a specific program
correct with human guidance.

**It does not mean computers are limited relative to people.** No human can decide
halting either. The proof does not mention machines, only the impossibility of a
consistent decision procedure, and a person following one would face the same
contradiction. Undecidability is a fact about the problem, not about silicon.

## What it does mean

**No tool can be simultaneously sound, complete, and terminating.** That is the
practical statement, and everything below follows from it.

Given a behavioral property, a tool can have at most two of:

- **Sound** — it never says the property holds when it does not.
- **Complete** — it always says the property holds when it does.
- **Terminating** — it always gives an answer.

Every static analysis tool you use has chosen two, and knowing which explains its
behavior.

## What real tools do

Four strategies, and the same four appear everywhere.

**Be conservative.** Answer "yes", "no", or "cannot determine", and be right
whenever you commit. Java's definite-assignment analysis does this, which is why

```java
int x;
if (complicatedCondition()) x = 1;
System.out.println(x);        // error: x might not have been initialized
```

is rejected even when the condition is always true. The compiler cannot know, so
it refuses. That is a **false positive**, it is deliberate, and it is the price of
soundness.

**Accept false positives.** Static analysers report possible null dereferences that
cannot actually occur. Annoying, and better than missing real ones.

**Accept false negatives.** Most linters and type checkers miss real problems in
order to stay quiet enough to use. A tool nobody runs finds nothing.

**Bound the effort.** Model checkers explore up to a depth and report "no
counterexample found within bound", which is a weaker and honest claim. Section
34.1.1's step budget is the smallest version of this.

Every real tool is one of those four, and none of them is a failure of engineering.
They are the available options.

## Where it shows up

**Compiler warnings** are unreachable-code and uninitialized-variable analyses
being conservative, which is why a compiler warns rather than proves.

**Type systems** decide a decidable approximation of correctness. A type checker
proves "no type errors", which is a weaker and checkable property than "no errors".
That is the deal Chapter 17 made, now with its reason: type systems reject some
correct programs precisely so that they can terminate.

**Antivirus software** cannot decide whether a program is malicious, so it matches
known signatures and heuristics. This is not laziness; a complete virus detector
is provably impossible, and the arms race follows from that.

**Optimizing compilers** must prove transformations safe, and where they cannot,
they do not optimize. Chapter 21's devirtualization happens only when the JIT can
establish monomorphism, and the fallback path exists because sometimes it cannot.

**Termination proofs** in Coq and Agda require the programmer to supply a decreasing
measure. The system checks the argument rather than finding it — human insight
supplying what no algorithm can.

## The connection to Gödel

Turing's result is the computational form of something older.

Gödel's first incompleteness theorem, 1931: any consistent formal system strong
enough for arithmetic contains true statements it cannot prove.

The arguments are the same shape. Gödel constructs a statement asserting its own
unprovability; Turing constructs a program contradicting its own halting
prediction. Both use self-reference and negation, and both are Cantor's diagonal
in a new setting.

Turing's version is arguably the clearer one, and the historical order is worth
noting: Turing's 1936 paper was about a problem in logic — Hilbert's
*Entscheidungsproblem*, the question whether an effective procedure could decide
mathematical truth. The machine was invented as a way of making "effective
procedure" precise. The computer is a side effect of a proof about the limits of
mathematics.

## The honest summary

Chapter 6 said a very small set of operations suffices for anything computable.
This chapter says there is a definite boundary to what "computable" contains, and
that some entirely natural questions fall outside it.

Both are results about the *same* model, and together they say something better
than either alone: **the boundary is sharp, it is knowable, and it is not where
intuition puts it.** Chapter 33's incompressible strings were the first sign;
almost everything is outside the reach of a finite description, and the reachable
part is a small, structured, useful corner.

That is not a limitation to be worked around. It is a map, and having it is what
lets you recognize the difference between a hard problem and an impossible one —
which is worth a great deal of time not spent.

Next: measuring information by the length of the shortest program that produces
it.
