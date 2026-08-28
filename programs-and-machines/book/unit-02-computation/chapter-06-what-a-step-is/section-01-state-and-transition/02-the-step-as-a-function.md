# The Step as a Function

You have a state. Now you need the rule that moves you to the next one.

I want to warn you that this lesson is going to look thin. The central idea is one
line long, and the first time you read it you may wonder whether you have missed a
page. You have not. The line is doing more work than it appears to, and the rest of
the lesson is spent unpacking it.

## The whole mechanism

A **transition rule** is a function. It takes a state and produces a state:

```
next : State → State
```

That is it. Everything a computer does — every program you have ever run, every
program anyone will ever write — is that function applied over and over again.

The insistence on the word *function* is not decoration. Functions come with
properties, and we are about to spend them. A function is deterministic: the same
input yields the same output, every time, with no exceptions. It is total or
partial in a way we can state precisely. It composes. And most valuable of all, it
can be **reasoned about** — if you know something that holds of every state the
function can produce, then you know something about the computation without having
to run it.

Hold on to that last one. It is the seed of everything in Chapter 9, and of most of
what makes a program tractable to a human being.

## A computation is an orbit

Start somewhere. Apply the rule. Apply it again.

```
s₀  →  s₁  →  s₂  →  s₃  →  …
```

Each arrow is one application. That sequence is the computation — and here is the
part worth pausing on: it is **completely determined** by the starting state and
the rule. Nothing else takes part. There is no other ingredient, no outside
influence, nowhere for anything else to enter.

Given that, ask yourself what such a sequence could possibly do. Take a moment
before reading on; the answer is shorter than you expect.

There are exactly three possibilities.

**It reaches a state marked final and stops.** This is a program that terminates,
and it is usually what you were hoping for.

**It cycles** — arrives back at a state it has occupied before. And since the rule
is deterministic, from that moment it is committed: it must repeat the identical
sequence forever, with no possibility of escape. This is an infinite loop.

That deserves a second look, because the usual mental image is wrong. The machine
is not stuck, and it is not broken, and it has not failed. It is working perfectly,
forever, on a sequence that has no end in it.

**It runs on without ever repeating**, generating fresh states indefinitely. This
requires an unbounded state space, which on a real machine means "until the memory
runs out".

Three options. That is the complete account of what any computation can do, and
there is no fourth.

Which sets up something you should file away now and collect in Unit VIII: deciding
*which* of those three will happen, in general, for an arbitrary program, is
impossible. Not difficult. Impossible, provably, forever. It is one of the
strangest facts in this book, and it will land considerably harder because you
already have this model to hang it on.

## One word doing quiet work

We say a step "changes" the state. That word is worth examining, because there are
two ways to read it and the difference matters later.

The **functional** reading: the step *produces a new state* and leaves the old one
alone. Nothing is modified. There is a sequence of distinct states, each computed
from the one before it.

The **imperative** reading, which is the one Java uses: there is one state, sitting
in memory, and each step *overwrites part of it*.

For most purposes these describe the same computation and you can hold whichever
you prefer. They part company on exactly one point, and it is the point that will
matter enormously in Unit V: **in the functional picture the old state still
exists, and in the imperative picture it is gone.**

An entire family of bugs lives in that gap. Suppose two parts of your program each
hold what they believe to be their own private state — and it turns out those are
two names for the same memory. Now a change made by one is silently visible to the
other, and neither was written expecting that. This is **aliasing**, and Chapter 20
is largely about living with it.

The model we are building right now has no aliasing at all, because it has no
names. Which is worth knowing before the next chapter, because it means introducing
names is a much bigger step than it will look.

## Why a machine can be built at all

One last property of real transition rules, and it is the one that makes hardware
possible.

In principle `next` could depend on the whole state and rewrite all of it. In
practice it never does. A real step looks at a tiny piece of the state and changes
a tiny piece:

- add the contents of two registers, put the result in a third
- if this value is zero, set the program counter to that address
- copy one word from memory into a register

Everything else is left alone — and "everything else" is very nearly all of it.

This is the same locality you met in Chapter 2, where adding two numbers proceeded
one column at a time and passed a single carry sideways. That it turns up twice is
not a coincidence. **Locality is what lets a fixed, finite piece of hardware
operate on states enormously larger than itself.** The machine never has to
comprehend the state. It only has to look through a small window at it.

And it is why a step can be made small enough to build out of the switching
circuits of Chapter 1. A rule obliged to consider everything at once could not be
built out of anything.

Next, we build one.
