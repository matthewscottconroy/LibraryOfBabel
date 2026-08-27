# The Stored-Program Idea

Turing's machine keeps its rules outside the tape — the table is the machine's
fixed nature, and the tape is what it works on. Real early computers were built
the same way, and the consequence was brutal.

ENIAC, from Chapter 1, was programmed by physically rewiring it. Setting up a new
calculation meant moving cables and setting switches across a room-sized machine,
a job that took days and was done largely by six women — Kay McNulty, Betty
Jennings, Betty Snyder, Marlyn Wescoff, Fran Bilas, and Ruth Lichterman — whose
work was uncredited for decades. The machine could compute a trajectory in
seconds and took two days to be told which trajectory.

The problem is structural. The program is in the wiring, and wiring is not
something the machine can change.

## The idea

The *First Draft of a Report on the EDVAC*, circulated in 1945 under von
Neumann's name and drawing on work by Eckert, Mauchly and others, proposed
something different: **put the instructions in the same memory as the data.**

That is it. That is the whole idea, and it is worth being precise about why it
is not merely a convenience.

If instructions live in memory, then:

**Programs load in seconds, not days.** Changing the program is writing to
memory, which the machine already does constantly.

**A program can compute an address.** Instead of "read location 40", you can
write "read the location whose number is in this register", which is how arrays
and loops over data become possible at all.

**A program can read another program**, because a program is a pattern in memory
like any other, and reading memory is something programs do.

**A program can write a program.** And this is the one that changes everything.

## Instructions are data

Look back at Chapter 1, section 1.2.1. I listed five readings of the pattern
`01000001`, and one of them was "a machine instruction, on some processor
somewhere". At the time that was an item in a list. Now it is the point.

In a stored-program machine there is no distinction *in the memory* between
instructions and data. There is no flag on a byte saying "I am code". A given
pattern is an instruction if and only if the processor's program counter points
at it and the processor decodes it — which is to say, **being a program is not a
property of a pattern but of how the pattern is used.**

This is Chapter 1's thesis, arriving in its most consequential form. Meaning
comes from the agreement, and here the agreement is "the program counter points
here now".

## What it made possible

Nearly everything.

**Compilers.** A compiler is a program whose output is a program. `javac` reads
your text and writes bytecode — data going in, data coming out, and the output
happens to be runnable. Without stored programs the idea is incoherent.

**Operating systems.** Loading and running an application is writing a pattern
into memory and jumping to it. Your operating system is a program whose main job
is running other programs.

**Interpreters.** A program that reads a program and does what it says. Unit VI
builds one.

**The JVM.** A `.class` file is data on disk. The JVM reads it, and the bytes
become instructions. Chapter 5's two-step model is stored-program logic all the
way down.

Every one of these is the same move: treat a program as data, then do something
to it.

## And what it costs

Honesty requires the other side.

If instructions and data share memory, then anything that can write data can write
instructions. A program that mishandles input can be persuaded to overwrite the
region holding its own next instructions — which is the mechanism behind an
entire history of security vulnerabilities. Buffer overflow attacks work by
writing past the end of a data area into memory the processor will later execute.

The defenses used now — marking pages non-executable, randomizing layout — are
attempts to re-establish, at some cost, the separation the design deliberately
removed. There is no way to have both properties fully. The flexibility and the
vulnerability are the same feature.

That is a genuinely deep engineering pattern and worth noticing early: the
capability you want and the risk you fear are frequently not two things but one
thing seen from two sides.

## Where the model stands

We now have three machines:

**Finite state machine.** Fixed states, no memory beyond the current state.
Enough for parity; not enough to count.

**Turing machine.** Unbounded tape, fixed rules held outside it. Computes
anything computable. A reference, not a design.

**Stored-program machine.** Instructions and data in one memory. Turing-complete,
and buildable, and the architecture of everything you use.

Next: where Java sits.
