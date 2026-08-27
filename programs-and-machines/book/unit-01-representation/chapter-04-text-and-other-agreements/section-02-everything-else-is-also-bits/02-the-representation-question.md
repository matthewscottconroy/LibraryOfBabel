# The Representation Question

This is the last lesson before Java arrives, and its job is to state plainly what
the unit has been circling.

## The question

Whenever you put something into a computer, you answer four questions. Usually
without noticing, which is the problem.

**What am I keeping?** Of everything true about this thing, which properties go
in? A color keeps three numbers and discards the spectrum. A `char` keeps the
identity of a character and discards its shape, its size, the handwriting it was
written in. A timestamp usually keeps the instant and discards the time zone —
and that discarding is the source of an entire genre of bug.

**How finely?** How many bits, how many samples, how many decimal places. This is
where the smallest distinguishable difference gets decided.

**What is the smallest and largest I can hold?** Every finite representation has
edges, and something must happen at them. Chapter 2's answer was wrapping.
Chapter 3's was rounding, with infinity beyond. Sound clips; color saturates.

**What happens to everything that does not fit?** Not *whether* things fail to
fit — they always do — but what the policy is. And crucially: **is the policy
loud or silent?**

Almost every policy in this unit has been silent. Integer overflow wraps without
a word. Floating point rounds without a word. An encoding mismatch produces
mojibake without a word. Color banding, audio quantization noise, image
compression artifacts — all silent.

That is not carelessness on anyone's part. Checking is expensive, and for the
overwhelming majority of operations the loss is exactly what was intended. But it
puts the burden on you. **The machine will not tell you when it discards
something. Knowing what you asked it to discard is your job, and it is a job you
can only do if you know what the representation keeps.**

## Why this is the right first idea

I have put this unit first, ahead of loops and methods and objects, because in my
experience it is what separates programmers who can debug from programmers who
can only rewrite.

When a program behaves inexplicably, the novice's instinct is to change something
and rerun. Sometimes that works. When it does not, they have no next move,
because they have no model of what is happening underneath — only a set of rules
about what to type.

The question "what is actually stored here, and what did that representation
throw away" is a next move. It applies when a total is off by a penny, when a
name displays as `Ã©`, when a comparison that should be true is false, when a
value that was 2 billion is suddenly negative. All four of those are the same
question with different furniture, and you can now answer all four.

## The thread from here

Everything in the rest of the book is this question at a larger scale.

**Unit II** asks it about process rather than data. A machine's state is a
representation of "where the computation has got to", and a loop invariant is the
statement of what that representation is supposed to mean.

**Unit III** asks it about procedures. A method signature is a claim about what
information crosses a boundary — and what does not.

**Unit IV** asks it about structure. An abstract data type is exactly an
agreement about what a pile of values represents, enforced by hiding the pile.

**Unit V** asks it about identity. Deciding whether two objects are "the same" is
deciding which differences your representation is entitled to ignore.

**Unit VI** asks it about programs. An interpreter treats a data structure as
code, which is only possible because "being a program" is a matter of how
something is read.

**Unit VIII** asks how much can be represented at all — Shannon's answer — and
what no representation can capture.

Same question, eight times.

## One last look back

We started with a wire holding 3.2 volts and the observation that the wire does
not hold a number. Everything since has been layers of agreement stacked on that:
a bit is an agreement about voltage, an integer is an agreement about bits, a
character is an agreement about integers, a string is an agreement about
characters, a document is an agreement about strings.

Each layer is real and useful and lets you forget the one below. That forgetting
is what abstraction is *for*, and there is nothing wrong with it — you cannot
think about voltages while writing a payroll system.

But every so often a lower layer makes itself felt. A number wraps. A tenth is
not a tenth. A letter arrives as two. When that happens, the ability to descend a
level and ask what is actually there is the difference between fixing the problem
and guessing at it.

You can descend now. That was the point of the unit.

Next chapter, Java finally arrives — and its first job will be to show you the
bits we have spent four chapters talking about.
