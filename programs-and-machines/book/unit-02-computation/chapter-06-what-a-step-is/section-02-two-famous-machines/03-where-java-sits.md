# Where Java Sits

We have a model. Let us locate Java in it, because that placement makes several
of the language's decisions legible.

## The layers

When you run a Java program, four machines are involved, stacked.

**Your program** describes a computation in Java's terms — variables, methods,
objects.

**The JVM** executes it, and is itself a machine in our sense. It has a state:
the operand stack, the local variables, the program counter, the heap. It has a
transition rule: fetch the bytecode instruction at the program counter, do what
it says, advance. Chapter 5 sketched this; now you can see that it is exactly a
stored-program machine with an instruction set chosen by specification rather
than by silicon.

**The physical processor** executes the JVM, and is a stored-program machine with
an instruction set chosen by Intel or ARM.

**The circuits** implement the processor, and are Chapter 1's switching elements.

Four layers, and here is the observation worth having: **each layer is a machine
whose state is data to the layer below.** The JVM's program counter is, to the
physical processor, just a number in a register. Your object is, to the JVM, a
region of the heap. Your program's entire state is data to the machine underneath
it.

That is the stored-program idea applied recursively, and it is how a system of
this complexity is built at all.

## What Java's constructs are

Now the payoff. Every Java construct is a way of describing states and
transitions, and you can name which:

**A variable** is a named cell of state. **Assignment** is a transition that
overwrites it.

**A statement sequence** is transitions applied in order — the program counter
advancing.

**An `if`** is a transition rule that consults the state to choose between two
continuations. In the parity machine, this was the table having different entries
for `0` and `1`.

**A loop** is a transition that can return the program counter to an earlier
position with the state changed. It is the parity machine's ability to be in
`ODD` twice with different amounts of input consumed.

**A method call** creates new state — the call frame of Chapter 12 — and records
where to resume. **Return** discards it and restores the recorded position.

**An object** is a bundle of state, together with the transitions permitted to
touch it. Unit V argues this is a design decision rather than a mechanism, and
now you can see why: mechanically, an object adds nothing. Its value is in what
it forbids.

**An exception** is a transition to a state determined by something other than
the program counter's normal advance — a non-local jump, which is why Chapter 28
treats it as control flow rather than as error reporting.

Nothing in that list adds computational power. Java computes what the
three-rule tape machine computes. Everything the language provides is a way of
writing state and transitions that humans can manage.

## So what is Java for?

If it adds no power, what is the point?

The point is that we are finite. A Turing machine program to sort a list would be
thousands of rules and beyond any human's ability to write correctly. The
constructs above are not about what can be computed but about **what can be
described by a person without making mistakes**, and that is the actual
engineering problem.

This reframes the whole book. Units III through VII are not about extending what
your programs can do. They are about extending what you can *hold in your head*
while writing them. A method lets you name a process and stop thinking about its
insides. A class lets you bundle state so you need not track its pieces
separately. An interface lets you depend on a promise instead of a
representation.

Every one of those is a device for managing your own limitations. That is not a
lesser thing than computational power. It is the entire discipline.

## Closing the chapter

We built a model with three parts — state, transition, and a starting point — and
found it sufficient to describe everything a computer does.

We built a machine from a four-entry table that decides parity for inputs of any
length, and saw that it has no understanding of parity anywhere in it.

We met Turing's machine, which is the reference definition of computability, and
the stored-program design, whose idea that instructions are data is the one this
book keeps returning to.

And we located Java: a notation for describing states and transitions, running on
a virtual machine, running on a physical machine, running on switches.

From here the book gets concrete and stays concrete. Next chapter, variables —
which is to say, names for pieces of state, and the surprisingly large amount of
trouble that naming introduces.
