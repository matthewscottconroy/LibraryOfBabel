# Computation

Unit I settled what a machine can hold. This unit asks what it means for a
machine to *do* something.

That sounds like it should be the easier question. It is not, and the reason is
worth stating at the outset: doing something is not one act but a sequence, and a
sequence requires memory of where you are. A machine that computes must hold, in
addition to the data, a representation of *how far it has got*. That
representation is called **state**, and almost everything difficult in
programming — for the rest of this book and for the rest of your career — is
difficulty about state.

Consider what you do when you add two multi-digit numbers by hand. You work
right to left, one column at a time. Between columns you are holding something:
which column you are in, and whether there is a carry. Neither of those is part
of the numbers. They are part of *where you are in the process*, and if someone
interrupted you and asked you to resume an hour later, they are exactly what you
would need written down.

That is the whole idea of this unit. A computation is a sequence of small,
mechanical changes to a state, and the state is what makes the next step
determinable from the current position.

Five chapters.

**Chapter 6 — What a Step Is** builds the model. State, transition, and the
observation that a table of "if you see this, do that" is already a computer. We
meet two famous machines: Turing's, which strips computation to the absolute
minimum, and von Neumann's, which put the program in the same memory as the data
and thereby made programmable computers possible.

**Chapter 7 — Names and Boxes** introduces variables, which are Java's way of
letting you name a piece of state. The distinction between a name and a value is
one of the two or three genuinely important distinctions in the book, and this is
where it starts.

**Chapter 8 — Choosing** is about conditionals, and it goes deeper than `if`
statements. Boolean logic connects directly to the switching circuits of Chapter
1, and Shannon's observation that the two are the same thing is the reason
computers can be designed rather than merely built.

**Chapter 9 — Repeating** is about loops, and it contains the first place in this
book where you will *prove* something rather than test it. A loop invariant is a
statement about state that survives every iteration, and it is the difference
between a loop you hope works and one you know works.

**Chapter 10 — Reading a Program's Mind** is a practical chapter about tracing,
debugging, and the discipline of finding out what a program is actually doing
rather than what you assumed.

A note on how this unit relates to the last one. Unit I was about representation
and I claimed at its close that the same question recurs. Here it is, recurring:
a program's state is a *representation* of where the computation has got to. It
keeps some things and discards others. A loop that tracks only a running total
has discarded the individual values it summed — deliberately, and that discarding
is what lets it run in constant memory.

Same question. Different subject.
