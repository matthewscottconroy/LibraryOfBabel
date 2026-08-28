# Information as Surprise

Two messages arrive, each one bit long:

- the coin came up heads
- the sun rose this morning

They are the same size in every encoding, and one of them told you nothing. Whatever
information *is*, it cannot be a property of the message alone.

Two messages, each one bit long.

**"The coin came up heads."** A fair coin, so before the message you thought heads
and tails equally likely. Afterwards you know. Something was learned.

**"The sun rose this morning."** You were already certain. Nothing was learned.

Both messages are the same size in any encoding. They carry different amounts of
information, and the difference is that one resolved an uncertainty and the other
did not.

That is Shannon's insight, and it is the whole foundation:

> **Information is the reduction of uncertainty. A message carries information in
> proportion to how surprising it is.**

Which means information is not a property of a message. It is a property of a
message **and what the receiver already believed**. The same words carry different
information to different people, and that is a feature of the definition rather
than a problem with it.

## Putting a number on it

Shannon's measure of the surprise in an event of probability $p$:

$$I(p) = \log_2 \frac{1}{p} = -\log_2 p \quad \text{bits}$$

Check it against intuition.

A **certain** event, $p = 1$: $\log_2 1 = 0$ bits. The sun rose. Nothing learned.

A **fair coin**, $p = 1/2$: $\log_2 2 = 1$ bit. Exactly the bit of Chapter 1 — one
choice between two equal possibilities.

A **fair die**, $p = 1/6$: $\log_2 6 \approx 2.58$ bits. More than two bits,
because six outcomes need more than two yes-or-no questions on average, and less
than three because three would distinguish eight.

An **unlikely** event, $p = 1/1000$: about 10 bits. Rare news is informative news.

An **impossible** event, $p = 0$: infinite. Which is why it does not happen.

## Why the logarithm

The choice is not arbitrary; it is forced by one requirement.

**Information from independent events should add.** Learning two coin flips should
be two bits, not one and not four.

Probabilities of independent events **multiply** — two heads has probability one
quarter. So the measure must turn multiplication into addition, and the logarithm
is the only function that does:

$$\log(pq) = \log p + \log q$$

Check: $-\log_2(1/4) = 2$ bits. Two flips, two bits.

The base-2 choice is what makes the unit a bit. Base $e$ gives *nats*, base 10
gives *hartleys*, and they differ by a constant factor — Section 32.1.2's point
about logarithms, in a different setting.

## Yes-or-no questions

The most useful way to hold the idea: **the information in a message is the number
of yes-or-no questions needed to determine it**, when the questions are chosen
well.

Guessing a number from 1 to 8, all equally likely. "Is it above 4?" halves it.
Three questions suffice, and $\log_2 8 = 3$.

Guessing from 1 to 1000: about 10 questions, and $\log_2 1000 \approx 9.97$.

Which should feel familiar, because you have now arrived at the same logarithm
from three completely different directions — a search in Chapter 9, a cost in
Chapter 32, and now a count of questions. **Binary search takes $\log_2 n$ steps
because that is how many
bits of information it needs to acquire**, and each comparison yields at most one
bit. The complexity bound and the information bound are the same bound.

Which is a genuinely striking connection: a limit on how fast an algorithm can be
turns out to be a limit on how fast information can be gathered.

## Unequal probabilities

Everything above assumed equal likelihood. When outcomes differ in probability,
the surprise differs per outcome.

English text. The letter `e` occurs in about 12% of positions and `z` in about
0.07%. So:

$$I(\texttt{e}) = -\log_2 0.12 \approx 3.06 \text{ bits}$$
$$I(\texttt{z}) = -\log_2 0.0007 \approx 10.5 \text{ bits}$$

A `z` tells you three and a half times as much as an `e`.

And it follows directly that giving them the same number of bits is wasteful.
Storing English as one byte per character spends eight bits on a symbol carrying
three, and Section 33.2.1 is about spending the right number.

That is Chapter 1's claim — a fixed-width encoding wastes space when symbols are
unequally likely — with the waste quantified.

## What this is not

Two clarifications, because the word "information" is doing specific work here.

**It has nothing to do with meaning.** A million random characters carry more
Shannon information than a million characters of Shakespeare, because they are
less predictable. That sounds wrong and it is the correct consequence of the
definition: Shannon was measuring what a communication channel must carry, and a
channel must carry the random string faithfully too.

Meaning is a separate question, and nobody has a comparable theory of it.

**It is about a source, not a string.** Strictly, the surprise of an event requires
a probability, and a probability requires a model of what was likely. In practice
one estimates the model from the data, which Section 33.1.2 does, and it is worth
knowing that this is an estimate rather than a property of the string itself.

Chapter 34 gives the other definition — Kolmogorov complexity — which *is* a
property of the string, needs no probabilities, and is uncomputable. The trade
between those two is the subject there.

Next: averaging the surprise.
