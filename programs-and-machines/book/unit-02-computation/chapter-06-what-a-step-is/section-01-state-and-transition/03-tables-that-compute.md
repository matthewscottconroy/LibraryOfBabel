# Tables That Compute

Enough theory. Let us build a machine.

It will be small enough to hold in your head entirely, and it will genuinely
compute something. The point is to see that there is nothing in it but a table.

## The problem

Read a string of `0`s and `1`s, one character at a time, left to right. Decide
whether the string contains an even number of `1`s.

Try it on `1011`: three ones, so odd. On `1001`: two ones, even. On the empty
string: zero ones, and zero is even.

## What state is needed?

Here is the important question, and it is worth stopping on.

To answer "even or odd", how much do you need to remember as you scan?

The instinct is to count the `1`s. But you do not need the count — you only need
to know whether the count *so far* is even or odd. When you meet another `1`, an
even count becomes odd and an odd count becomes even. The actual number never
matters.

So the state is one bit:

```
EVEN    the ones seen so far are even in number
ODD     the ones seen so far are odd in number
```

Two states. That is the whole machine's memory, regardless of whether the input
is four characters or four billion.

This is worth dwelling on, because it is the first real design decision in the
book. A machine that counted would need memory growing with the input. A machine
that tracks only parity needs one bit forever. Both answer the question; one
scales and one does not. **Choosing what to remember is choosing what your
program can afford** — Unit I's representation question, arriving in the theory
of computation.

## The table

Start in `EVEN`, because zero ones have been seen. Then:

| current state | input `0` | input `1` |
|---|---|---|
| **EVEN** | EVEN | ODD |
| **ODD** | ODD | EVEN |

Read the table as instructions: *if you are in this state and you see this
character, go to that state.* A `0` never changes the parity, so it leads back
where it was. A `1` flips it.

When the input runs out, accept if the state is `EVEN`.

That is a complete, working computing machine. Six pieces of information — two
states, four transitions — plus a start state and an accepting state.

## Running it

On `1011`:

```
             state    read    →  next
start        EVEN
step 1       EVEN      1         ODD
step 2       ODD       0         ODD
step 3       ODD       1         EVEN
step 4       EVEN      1         ODD
end          ODD    (input exhausted)
```

Final state `ODD`, which is not accepting, so `1011` is rejected. Three ones —
correct.

On `1001`:

```
start        EVEN
step 1       EVEN      1         ODD
step 2       ODD       0         ODD
step 3       ODD       0         ODD
step 4       ODD       1         EVEN
end          EVEN
```

Accepted. Two ones — correct.

Do `110` yourself before reading on. You should end in `EVEN` and accept.

## What we have built

This is a **finite state machine** — finite because the number of states is fixed
in advance and does not grow with the input.

Look at what it does not have. No arithmetic. No memory beyond the current state.
No ability to look back at characters it has already read, or ahead at ones it
has not. It cannot count. It has, in a real sense, no idea what it is doing.

And it computes the answer correctly for inputs of any length whatsoever.

This is the first genuinely surprising thing in the theory of computation, and I
would like you to feel it rather than nod at it. **Behavior that looks like it
requires understanding is produced by a table with four entries.** Nothing in the
machine knows what parity is. The knowledge is in the arrangement, not in any
part.

You will meet this pattern repeatedly, at every scale. It is why Chapter 34 can
argue about what programs can and cannot do without ever needing to ask whether
they understand anything.

## Where finite states run out

Finite state machines are not all-powerful, and seeing where they fail is as
instructive as seeing where they work.

Try to build one that accepts strings of the form *n* `0`s followed by *n* `1`s —
`01`, `0011`, `000111`, and so on. To do it you must remember how many `0`s you
saw, and that count is unbounded, so no fixed number of states suffices. Whatever
finite machine you build, there is an input long enough to overflow it.

So the model has a limit. Adding unbounded memory removes it, and the next
section is about the two most famous ways of doing that — one abstract, from
1936, and one that became the architecture of every computer you have ever used.

But keep this machine in mind. When we get to Java's `if` and `while`, they are
going to look like a table of transitions with better notation, and that is
exactly what they are.
