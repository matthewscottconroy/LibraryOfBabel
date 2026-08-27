# Repeating

This chapter completes Chapter 6's model and then does something the book has not
done before.

The completion is small. We have transitions that change state and transitions
that consult it; a loop is a transition that returns the program counter to a
place it has already been, with the state changed. That is all a loop is, and the
first section makes it concrete.

The new thing is in the second section. Up to now, when we wanted to know whether
code was right, we ran it and looked. That works, and Chapter 6 explained why it
can never be enough: the state space is astronomically large, so testing samples
it rather than covering it. A loop that works for the twenty inputs you tried may
fail on the twenty-first.

The alternative is to **prove** it. Not informally — actually establish a claim
that holds for every input, by an argument you can check.

The tool is the **loop invariant**: a statement about the state that is true
before the loop, stays true across every iteration, and — combined with the
reason the loop stopped — tells you the answer is right. It is the most powerful
idea in this unit, and it is not difficult. It is mostly a matter of writing down
what you already believe and then checking that you were entitled to believe it.

## Why bother

A fair question, since the loops you will write this month are short enough to
eyeball.

Two reasons. The first is that off-by-one errors are the most common bug in
programming and eyeballing does not catch them, because the mistake is invisible
by construction — the code looks right, which is why you wrote it. An invariant
catches them because it forces you to state the boundary explicitly.

The second is that invariants are how you will understand loops you did not
write. Faced with an unfamiliar loop, "what is true here every time round" is the
question that unlocks it, and it is much faster than tracing.

I will not pretend every loop deserves a written proof. Most do not. But the
*habit* of asking what stays true is cheap, and it is the difference between a
loop you hope works and one you know works.

## What is here

**The Shape of a Loop** covers `while`, `for`, and `do`-`while`, and argues that
`while` is the real construct and the others are abbreviations.

**What a Loop Promises** is the invariant, termination, and a treatment of
off-by-one errors as failed proofs rather than as carelessness.
