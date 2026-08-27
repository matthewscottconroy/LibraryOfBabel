# The Bit as a Choice

We have a wire that reads as 0 or 1. Now I want to change how you think about
what that *is*, because the usual mental image — a bit is a tiny box holding a
digit — will slow you down later.

A better image: **a bit is one answered question.**

Not a thing. An answer. One yes-or-no that has been settled.

## One bit distinguishes two things

Suppose I am thinking of either a cat or a dog, and you get to ask one yes-or-no
question. Ask "is it a cat?" and my answer — yes or no — tells you exactly which
one I meant. One question, two possibilities, complete resolution.

That is one bit. Not "the letter c" or "the number 1". One binary distinction,
resolved.

Now suppose I am thinking of a cat, a dog, a bird, or a fish. One question is no
longer enough; whatever you ask, some pair of animals stays unresolved. Two
questions will do it:

- Is it a mammal? *Yes* → cat or dog. *No* → bird or fish.
- Then: is it the smaller one? and you are done.

Two bits, four possibilities. Write the answers as 0 for no and 1 for yes and
the four cases become `00`, `01`, `10`, `11` — which is exactly the pattern you
would expect from counting in binary, and we will come back to that in Chapter
2. But notice the order of the reasoning. The counting is not the fundamental
thing here. The *distinguishing* is. The counting falls out of it.

## The doubling

Add a third question and each of the four cases splits in two, giving eight. A
fourth gives sixteen. The pattern is not additive, it is multiplicative, and this
is the single most important arithmetic fact in the whole subject:

| Bits | Distinguishable possibilities |
|---:|---:|
| 1 | 2 |
| 2 | 4 |
| 3 | 8 |
| 4 | 16 |
| 8 | 256 |
| 16 | 65,536 |
| 32 | 4,294,967,296 |
| 64 | about 18.4 quintillion |

With *n* bits you can distinguish 2 to the power of *n* possibilities. Each new
bit doubles what you can tell apart, because it splits every case you already
had into two.

Take a moment with the bottom rows, because the growth is genuinely difficult to
feel. Thirty-two bits is four billion — roughly one for every two people alive.
Sixty-four bits is eighteen quintillion, which is more than the estimated number
of grains of sand on Earth. Both of these fit comfortably in a wire's worth of
hardware repeated a few dozen times.

## The counting question everyone gets wrong once

Here is the mistake, and almost everyone makes it once.

With 8 bits you can represent 256 different things. So what is the largest number
you can store in 8 bits?

The instinct is 256. The answer is 255.

The reason is that one of your 256 patterns has to represent zero. Count the
possibilities: `00000000` is one of them, and if you have agreed that it means
zero, then the remaining 255 patterns cover 1 through 255. Two hundred and
fifty-six *patterns*, largest value 255.

This is the same reason a three-digit odometer reads 000 to 999 rather than 001
to 1000 — a thousand readings, top value 999. If you have ever been off by one at
the end of a loop, you have met this problem's cousin, and you will meet it again
formally in Chapter 9.

I would rather you did not memorize "subtract one". I would rather you count.
When you are unsure how many things fit, list the smallest case — try 2 bits, on
paper, all four patterns — and the general answer will be obvious in a way that
a remembered formula never is.

## Bits do not mean anything yet

One more time, because this is the idea the chapter exists to install.

Four bits give you sixteen distinguishable patterns. That is a fact about
counting; it is true regardless of what the patterns are for. Those sixteen
patterns could stand for:

- the numbers 0 through 15
- the numbers -8 through 7
- sixteen colors in an old display palette
- the sixteen possible outcomes of four coin flips
- a hexadecimal digit
- sixteen instructions in a very small processor
- nothing at all

The bits are the same in every case. What differs is the agreement. And every one
of those readings is correct, because "correct" here means *consistent with the
stated convention*, not *true about the world*.

## Why "bit"

The word is a contraction of "binary digit", and it entered print in Claude
Shannon's 1948 paper *A Mathematical Theory of Communication*, where Shannon
credits the coinage to his Bell Labs colleague John Tukey.

That paper is worth knowing about even now, because Shannon's definition is
subtler than "a thing that is 0 or 1". For Shannon a bit is a *measure of
information* — specifically, the amount of information you gain from learning the
answer to a question whose two outcomes were equally likely. Which means, in
Shannon's sense, a bit that you could have predicted carries no information at
all.

That idea sounds like a curiosity now. In Unit VIII it will turn out to explain
why compression works, and why there is a hard limit on how far it can go. Note
the shape of it and carry on.

## What you should take from this

A bit is one resolved binary distinction. *n* bits resolve 2 to the *n*
possibilities, because each bit halves the remaining uncertainty. The patterns
mean whatever we have agreed they mean, and until we state the agreement the
question "what number is this" is not answerable.

Next we take up a question you may already have been forming: if two states work
so well, would ten not work better? People tried.
