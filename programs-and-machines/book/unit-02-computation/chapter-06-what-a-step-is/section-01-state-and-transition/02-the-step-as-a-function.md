# The Step as a Function

We have a state. Now the rule that moves us to the next one.

## The step

A **transition rule** is a function. It takes a state and produces a state:

```
next : State → State
```

That is the entire mechanism. Everything a computer does is repeated application
of a function of this shape.

The reason to insist on the word *function* is that functions have properties we
can use. A function is deterministic: same input, same output. It is total or
partial in a way we can specify. It can be composed. Most usefully, it can be
*reasoned about* — if we know something true of every state the function can
produce, we know something true of the computation, without running it.

## A computation is an orbit

Start at some state and apply the rule repeatedly:

```
s₀  →  s₁  →  s₂  →  s₃  →  …
```

with each arrow one application. The sequence is the computation, and it is
completely determined by the starting state and the rule. Nothing else
participates.

Three things can happen.

**It reaches a state designated as final** and stops. This is a program that
terminates, and it is what we usually want.

**It cycles**, returning to a state it has been in before. Since the rule is
deterministic, from that point it must repeat the same sequence forever. This is
an infinite loop, and note that it is not the machine getting stuck — it is the
machine working perfectly, forever, on a sequence with no end.

**It continues without repeating**, generating new states indefinitely. Possible
only if the state space is unbounded, which for real machines means "until memory
runs out".

That is a complete account of what a computation can do. There is no fourth
option. In Unit VIII we will find that deciding *which* of these will happen, in
general, is impossible — which is a strange and genuinely deep fact, and it lands
much harder once you have this model.

## Being careful about "changes"

We say a step "changes" the state, and the word is doing something worth
examining.

The functional picture is that the step *produces a new state*, leaving the old
one alone. Nothing is modified; there is a sequence of distinct states, each
computed from the last.

The imperative picture — the one Java uses — is that there is one state, sitting
in memory, and each step *overwrites part of it*.

These describe the same computation, and for most purposes it does not matter
which you hold. But they differ in one respect that will matter enormously in
Unit V: in the functional picture the old state still exists, and in the
imperative picture it is gone.

That difference is the source of an entire family of bugs. If two parts of your
program hold what they believe are two separate pieces of state, and it turns out
they are two names for the same memory, then a change made by one is seen by the
other. This is called **aliasing**, and Chapter 20 is largely about it. The model
we are building has no aliasing, because it has no names — which is precisely why
introducing names in the next chapter is a bigger step than it looks.

## Locality

One more property of real transition rules, and it is what makes machines
buildable.

In principle `next` could depend on the entire state and rewrite all of it. In
practice it never does. A real step examines a tiny part of the state and changes
a tiny part:

- add the contents of two registers, put the result in a third
- if this value is zero, set the program counter to that address
- copy one word from memory into a register

Everything else is untouched, and the untouched part is nearly all of it.

This is the same locality we met in Chapter 2, where adding two numbers worked
one column at a time and passed a single carry sideways. It is not a coincidence.
Locality is what allows a fixed, finite piece of hardware to operate on states far
larger than itself — the machine does not have to comprehend the state, only
inspect a small window of it.

And it is why a step can be made small enough to build out of the switching
circuits of Chapter 1. A rule that had to consider everything at once could not be
built at all.

Next, we build one.
