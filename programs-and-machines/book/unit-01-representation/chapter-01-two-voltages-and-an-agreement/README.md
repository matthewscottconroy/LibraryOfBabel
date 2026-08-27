# Two Voltages and an Agreement

There is a sentence you have heard many times, probably without examining it:
*the computer stores the number*.

It does not. Nothing in a computer stores a number, and getting clear about that
is the first real work of this book.

What a computer stores is voltage — a physical quantity, measurable with a
meter, sitting in a wire or a capacitor or a transistor gate. High or low.
Charged or drained. That is the entire inventory. Everything you have ever seen
a computer do has been done with that and nothing else. The number is something
*we* supply. At some point, people agreed that certain patterns of high and low
would be read as numbers, and built machinery that respects the agreement. The
agreement is not in the silicon. It is in the design, in the documentation, and
in your head.

I am aware this sounds like the kind of distinction professors enjoy and students
endure, so let me say what it buys you. When you eventually write
`2147483647 + 1` in Java and Java hands you back `-2147483648`, you will be
tempted to file it under "computers are weird". If you do, you will have learned
nothing, and you will be surprised again by the same thing wearing a different
costume six months later. But if you have understood that an `int` is a *pattern
under an agreement* rather than a number, the result is not weird at all. It is
what the agreement says should happen. You will be able to predict it, explain
it, and — this is the part that matters — recognize the same shape of problem in
situations that look nothing like this one. That is the trade this chapter
offers: a little patience now, in exchange for a category of surprise permanently
removed.

The chapter has two halves. The first, **The Only Thing a Machine Has**, starts
with the physical situation: why machines use two states rather than ten, and why
that is an engineering decision rather than a law of nature. The answer has to do
with noise, and it is a good answer — worth understanding rather than accepting.
Then we look at the bit as a *choice* rather than a thing, and find that the
reason bits compose the way they do is combinatorial, not electrical.

The second half, **Encoding as Convention**, is the important one. We establish
that an encoding is a convention, that the same pattern can be read many
different ways, and that all of those ways can be correct at once — because
correctness here means *conforming to a stated agreement* rather than *matching
reality*. Then we look at what it costs to fix the width of a pattern in advance,
which is the seed of nearly everything in Chapter 2.

No Java yet. Chapter 5 is where the language arrives, and by then you will have
something for it to be about.

You will get more out of this chapter with something to write with. Several times
I am going to ask you to work out a pattern before reading on, and the difference
between doing that and reading past it is most of the value.

One more thing. If you already know some of this — if you know what binary is, if
you have seen the word "bit" defined — read it anyway, and read it for the *why*
rather than the *what*. In my experience the people who struggle most in the
third and fourth units are not the ones who arrived knowing nothing. They are the
ones who arrived knowing the vocabulary and assumed that meant they knew the
ideas.
