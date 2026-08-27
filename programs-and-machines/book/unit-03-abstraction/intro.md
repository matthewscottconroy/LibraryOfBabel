# Abstraction by Procedure

Unit II left us with a complete model of computation and one unsolved problem.

The model is complete in a strong sense: state, transitions that change it,
transitions that consult it, and transitions that jump backwards. Chapter 6
argued that nothing further is needed, and the Church–Turing thesis says nothing
further is *possible* — a machine with those four things computes everything
computable.

The unsolved problem is that a program written entirely in those terms is
unreadable past a few hundred lines. Not incorrect. Unreadable, which for
practical purposes is worse, because a program nobody can understand is a program
nobody can change.

This unit introduces the first and most important remedy: **giving a process a
name**.

Consider what happens when you name something.

Before: a block of eleven lines that walks an array, compares elements, and keeps
track of a largest-so-far. To understand the code around it, you must read those
eleven lines and work out what they accomplish.

After: `largest(scores)`. To understand the code around it, you read the name.

The eleven lines did not go away, and the machine does exactly what it did
before. What changed is that a reader can now treat the whole thing as **one
step** — can stop tracking its internals and think about it as a unit with a
purpose.

That is abstraction, and it is worth being precise about what it consists of. To
abstract is to **hide detail behind a name in such a way that the detail no
longer needs to be considered.** The last clause is the load-bearing one. Hiding
detail that you still have to think about is not abstraction; it is
concealment, and it makes things worse.

Which means a method is only as good as the promise it keeps. `largest(scores)`
helps only if it returns the largest, always, without doing anything else you
would need to know about. When that is true you can forget the eleven lines.
When it is nearly true — usually returns the largest, except in one case — you
must remember both the name and the exception, which is more to track than
before.

That is the theme of this unit. A method is a **contract**, and the value of the
abstraction is exactly the reliability of the contract.

**Chapter 11 — Giving a Process a Name.** Methods: parameters, return values, and
the discipline of deciding what a method promises. This is also where we start
paying the debts from Chapter 5.

**Chapter 12 — The Stack.** What actually happens when a method is called. The
call frame, the stack discipline, and why Java passes everything by value even
when it appears not to. This chapter contains the most misunderstood sentence in
the language, and we will get it right.

**Chapter 13 — Recursion.** A method that calls itself. This is where Chapter 9's
loop invariants reappear as mathematical induction, and where a certain kind of
problem becomes dramatically easier to solve.

**Chapter 14 — Designing with Procedures.** Decomposition — how to decide what
the methods should be, which is a design skill rather than a syntax one — and
testing, which is how you check a contract without reading its implementation.

Units I and II were about the machine. From here the book is increasingly about
*you*: what you can hold in mind, what a reader can follow, how to divide work so
that each piece is small enough to be got right.

That is not a change of subject. Chapter 6 established that the machine's
capabilities were settled and that everything a language adds is expressiveness.
Expressiveness is a claim about human beings. We are now studying the constraint
that actually binds.
