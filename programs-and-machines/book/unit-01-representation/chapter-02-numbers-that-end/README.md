# Numbers That End

In the last chapter we established that a pattern of bits means whatever we have
agreed it means. Now we make the most important agreement in computing: the one
that turns patterns into whole numbers.

I want to warn you about a trap before we start. You already know how to count.
You have known since you were four, and the knowledge is so automatic that it
does not feel like knowledge — it feels like seeing. When you look at `742` you
do not decode anything; you *know* it is seven hundred and forty-two.

That fluency is going to work against you, because to understand binary you have
to notice what you are doing when you read `742`. There is a rule operating,
learned so long ago that it became invisible. This chapter makes it visible
again, and then swaps out one of its parameters. If at some point you find
yourself thinking "this is obvious, why is he belaboring it" — that is the
fluency talking, and it is worth slowing down anyway. The people who struggle
with two's complement in the second half of this chapter are almost always the
ones who skimmed the first half.

By the end you will be able to answer, from first principles, questions like
these: Why does `int` run from −2,147,483,648 to 2,147,483,647, rather than to
2,147,483,648 — and why is the range lopsided? Why does adding 1 to the largest
`int` give the smallest one? Why is there no separate "subtract" circuit in a
processor? Why do programmers write colors as `#FF8800` instead of as three
numbers? None of these will be facts you memorize. Each will be something you can
derive in about thirty seconds with a pencil, which is a much more durable form
of knowing.

The first section, **What a Numeral Means**, recovers the rule you learned at
four: positional notation, in which a digit's contribution depends on where it
sits. Then we change the base from ten to two and count in binary until it stops
being strange. Then hexadecimal, which is not a third system so much as a compact
way of writing the second one.

The second section, **Arithmetic in a Box**, is where the chapter earns its
title. We add binary numbers bit by bit and discover that the procedure you
learned for decimal addition works unchanged. Then the elegant part: two's
complement, a convention for negative numbers so well chosen that subtraction
becomes addition and the processor needs no extra hardware at all. And finally
overflow — not as a bug, but as the exact and predictable consequence of
everything that came before.

A word on doing the work. This chapter has more arithmetic in it than any other
in the book, and that is deliberate and temporary. Do it by hand. Not because
hand-calculation is a skill you will need — you will almost never convert a
number by hand again after this chapter — but because the conversions are how the
ideas get into your fingers. There is a real difference between someone who has
converted twenty numbers to binary and someone who has read about converting
numbers to binary, and the difference shows up three units later in ways neither
of them can trace back. Twenty minutes with a pencil now saves a great deal of
confusion in Unit IV.
