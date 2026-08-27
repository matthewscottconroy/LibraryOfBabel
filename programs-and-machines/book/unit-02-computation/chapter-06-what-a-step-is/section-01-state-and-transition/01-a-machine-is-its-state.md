# A Machine Is Its State

Here is a question that seems too easy. You are halfway through adding 4,829 and
1,376 by hand. What do you need to write down so that someone else could finish
it?

Not the answer — you do not have it yet. Not just the two numbers, either,
because that would put them back at the start. You would need something like:

```
the two numbers          4829, 1376
digits produced so far   05
which column is next     the hundreds
is there a carry         yes
```

Those four things are the **state**. They are everything that distinguishes
"halfway through this addition" from any other moment. Hand them over and the
work resumes exactly; withhold any one of them and it cannot.

## The definition

**The state of a computation is everything that must be known to continue it.**

That phrasing is worth taking literally, because it gives you a test. Faced with
some piece of information, ask: if I lost this, could the computation still
proceed correctly? If not, it is part of the state.

Notice what the test excludes. The *history* of how you got here is not state,
unless you need it. In our addition, the fact that you started with 4,829 rather
than arriving at this position some other way makes no difference to what happens
next. A machine that has to remember its whole history is a machine whose state
is growing, which is usually a design mistake and always a cost.

Notice also that state includes things that are not data. "Which column is next"
is not part of either number. It is part of the *process* — a record of position
within a procedure. Machines hold that too, and in a real processor it has a
name: the **program counter**, which is nothing but a number saying which
instruction comes next.

## The state space

If the state is a collection of values, then the set of all possible states is a
space you can, in principle, enumerate.

Take a very small machine: a light switch. One bit of state; two possible states,
`off` and `on`. The whole state space is:

```
{ off, on }
```

Now a machine with three switches. Each is independent, so the states multiply:
2 × 2 × 2 = 8. Which is Chapter 1's doubling rule, and it is the doubling rule
for the same reason — each bit of state doubles the number of distinguishable
situations.

Scale that up. A program using a single `int` variable has $2^{32}$ possible states
for that variable alone. Two `int`s give $2^{64}$. A modest program with a few hundred
bytes of live data has a state space larger than the number of atoms in the
observable universe.

I raise this because it explains something you will feel later. **You cannot test
a program by trying all its states.** There are too many, by margins that make
"too many" an understatement. Testing samples the space; it does not cover it.
This is why the loop invariants of Chapter 9 matter — a proof about all states is
achievable where an enumeration of them is not.

## Determinism

One more property, and it is the one that makes machines useful.

Our model says the next state is determined by the current state. Same state,
same next state, every time. This is **determinism**, and it is why a program
that worked yesterday works today.

When a program appears to violate this — works sometimes, fails others — the
explanation is never that the machine became unpredictable. It is always that
something is in the state which you did not know was in the state. The system
clock. A file's contents. The order in which two threads happened to run. An
uninitialized value that held whatever the previous program left there.

That reframing is practically useful, so I will put it as a rule:

> An intermittent bug is not a machine behaving randomly. It is a machine
> behaving deterministically on a state larger than the one you were thinking
> about.

Finding such a bug is the work of finding the hidden state. Unit VII, on
concurrency and the outside world, is largely about the places it hides.

## Why this is the right foundation

Every construct you will meet in Java can be described in these terms, and it is
worth previewing how, because it turns a list of features into one idea.

A **variable** is a named piece of state. An **assignment** is a transition that
changes it. A **conditional** is a transition rule that consults the state to
decide which of two continuations applies. A **loop** is a transition that can
lead back to a state it has visited before, with something changed. A **method
call** pushes new state and remembers where to return to. An **object** is a
bundle of state with the transitions that are allowed to touch it.

Six constructs, one model. Next: what a single step actually is.
