# Voltage and Meaning

Imagine I hand you a wire and a voltmeter and ask you to tell me what number the
wire is holding.

You would measure it. Suppose the meter reads 3.2 volts. Now: what number is
that?

The question has no answer, and noticing *why* it has no answer is the whole
point. The wire is holding 3.2 volts. It is not holding a number. If I now tell
you "in this machine, anything above 2 volts counts as a 1", then you can report
back that the wire holds a 1 — but notice what happened. You did not discover
that. I told you. The fact came from the agreement, not from the wire.

Change the agreement and the same wire reports something else. If I had said
"anything above 4 volts counts as a 1", your 3.2-volt wire would be holding a 0.
The physical situation did not change at all. The reading changed because the
convention changed.

This is the first idea in the book and it recurs, in disguise, for the next
seven units.

## The physical picture

Let me make the physical side concrete, because a vague picture here causes
trouble later.

Inside a modern processor the fundamental component is a transistor, which you
can think of as a switch operated by electricity rather than by a finger. A
voltage on one terminal determines whether current can flow between the other
two. Billions of these are etched onto a piece of silicon a couple of centimetres
across.

Memory works on a related principle. In the dynamic RAM that holds your running
programs, each cell is essentially a tiny capacitor — a bucket that holds
electrical charge. Charged bucket, empty bucket. The buckets leak, which is why
this kind of memory has to be refreshed thousands of times per second, and why
your program's memory vanishes when the power goes off. The charge was the only
thing there.

So when we say a computer "has 16 gigabytes of memory", we are saying something
about how many of these buckets it has. We are not saying anything at all about
what is in them, because what is in them is charge, and charge does not have a
meaning until we assign one.

## Why we do not read voltage directly

Here is a reasonable objection. The voltmeter gave us 3.2, a perfectly good
number with lots of detail in it. Why throw that away and collapse it to "1"?
Would we not store more information by keeping the whole reading?

In principle, yes. In practice this is exactly the thing that does not work, and
the reason is noise.

Voltage in a real circuit is never clean. It sags when a nearby component draws
current. It picks up interference from the switching happening a millimetre away.
It drifts with temperature. Measure that 3.2-volt wire again in a microsecond and
you might get 3.14, or 3.3, or — if something large just switched on next door —
2.9.

Now suppose we had agreed that the voltage encodes a number directly: 3.2 volts
means 3.2. Every one of those fluctuations is a corruption. The value you wrote
is not the value you read back. And crucially, the errors *accumulate*: copy the
value from one place to another and each copy adds its own noise, so after a
hundred operations your 3.2 has wandered somewhere unrecognisable.

Now suppose instead we agreed on the two-state convention: below 1 volt is a 0,
above 2 volts is a 1, and we design the circuits never to sit in between. The
3.2-volt wire reads as 1. Noise knocks it to 2.9 — still a 1. Interference drags
it to 2.4 — still a 1. You would need to disturb it by nearly a volt and a half
before the reading changes.

That gap is called **noise margin**, and it is the single most important reason
digital machines work. We deliberately discard almost all of the information in
the voltage in exchange for near-certainty about the little we keep.

## Restoring the signal

There is a second consequence, and it is the one that makes long computations
possible at all.

Because the circuit knows that a legitimate input is either "low" or "high", it
can *clean up* what it receives. A gate handed a slightly degraded 2.9 volts does
not output a slightly degraded signal; it outputs a fresh, full-strength "high"
generated locally. The noise is not passed along. It is thrown away at every
step.

This is why a modern processor can perform billions of operations per second for
years and give exactly the right answer every time. Not because the components
are perfect — they are not, they are noisy analogue devices — but because the
agreement to recognize only two states lets every stage discard the error the
previous stage introduced.

Analogue systems cannot do this. If you have ever copied a cassette tape, or
photocopied a photocopy of a photocopy, you have felt the difference: each
generation is worse, because there was no agreement that let the machine tell
signal from noise.

## Back to the agreement

So here is where we have arrived.

A wire holds voltage. We *agree* to read voltage above some threshold as one
symbol and below another threshold as a different symbol. We call those symbols 0
and 1 — but note that this naming is itself a further agreement. We could call
them "low and high", or "false and true", or "off and on", and in different parts
of this book we will call them all of those things, because in different contexts
different names are clearer.

Nothing about the wire changes when we rename them. What changes is what we are
prepared to do with them. Call them 0 and 1 and arithmetic suggests itself. Call
them false and true and logic suggests itself. Chapter 8 will show you that these
turn out to be the same machinery wearing different clothes, which is a genuinely
surprising fact and one of the deepest things in the early part of this subject.

For now, hold on to this: **the machine has patterns; you supply the meaning.**
When you eventually cannot understand why a program is doing what it does, come
back and ask which of the two you have confused.
