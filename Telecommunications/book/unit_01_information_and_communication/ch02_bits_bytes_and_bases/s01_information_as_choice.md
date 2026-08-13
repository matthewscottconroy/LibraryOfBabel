# 2.1 Information as Choice

Ask most people what information is and they will say something about facts, or
knowledge, or meaning. Ask an engineer and, if they have absorbed the twentieth
century properly, they will say something much stranger and much more useful:

> **Information is the resolution of uncertainty.**

The amount of information in a message is not a property of the message. It is a
property of *the set of messages that might have been sent instead*, and of how
likely each of them was. This is genuinely counterintuitive on first contact, and
it is the single idea from which everything else in Unit I follows.

## The twenty-questions argument

Play the game. I am thinking of a whole number from 1 to 8. You may ask yes/no
questions. How many do you need, in the worst case, if you play well?

Three. *Is it greater than 4?* halves the field to four. *Is it greater than 6?*
(or 2, depending) halves it to two. One more question and you are done. Note that
a *bad* strategy — "is it 1?", "is it 2?" — can take seven, but the question is
what a good strategy costs, because that is what the physics has to support.

Now 1 to 16. Four questions. 1 to 256? Eight. 1 to 4,294,967,296? Thirty-two,
which is not a coincidence and is why an IPv4 address is 32 bits.

The pattern: to identify one possibility out of *n* equally likely ones costs

$$\log_2 n \ \text{bits}$$

Each yes/no answer is one bit. The logarithm is base 2 because each question
halves the field, and it is a logarithm at all because **choices multiply while
information adds**. Two independent selections from an 8-element set give
8 × 8 = 64 combinations, and it costs 3 + 3 = 6 bits, not 3 × 3. That additivity
is the property Hartley identified in 1928 as the reason a logarithmic measure is
the right one, and it is why we can say a link carries "a million bits per second"
and have the number behave sensibly under composition.

## Why a bit, and not something else

Nothing forces base 2 mathematically. Hartley originally used base 10, and the
resulting unit — the **hartley**, or **dit** — is still occasionally seen. Base
*e* gives the **nat**, which is the natural unit for a lot of theory and appears
throughout thermodynamics.

What forces base 2 is *physics*, and it is worth being explicit because students
often suspect binary is an arbitrary computing convention that we are stuck with
for historical reasons.

It is not. The argument runs: any physical system that stores or transmits
information must have distinguishable states. Distinguishing states costs energy
and is defeated by noise. The **fewest** states you can have and still convey
anything at all is two, and two-state systems are the ones that survive noise
best, because the "distance" between the states is maximal for a given energy
budget. A voltage that is either 0 V or 5 V can tolerate two and a half volts of
noise before a receiver mistakes one for the other. Sixteen levels spread over the
same 0–5 V range tolerate about 167 millivolts.

This is not a small effect and it is not going away. When we get to Chapter 7 and
meet PAM-4 signalling — four voltage levels instead of two, used in 400 Gigabit
Ethernet — you will see engineers accepting exactly this tradeoff deliberately:
twice the bits per symbol, at the cost of a signal-to-noise requirement roughly
9.5 dB higher. They do it because at those speeds the channel's bandwidth is the
binding constraint and its noise is not. The tradeoff is real, quantifiable, and
made differently in different circumstances. But the *default*, the thing you do
when you have no special reason to do otherwise, is two states, and that is why
information is measured in bits.

## Unequal probabilities, and the first hint of compression

The formula log₂ *n* assumes all *n* possibilities are equally likely. They
usually are not, and the moment they are not, something useful happens.

Consider a sensor that reports one of four states: NORMAL, WARNING, ERROR,
CRITICAL. Naively that is log₂ 4 = 2 bits per report. But suppose NORMAL occurs
99% of the time. Then most reports carry almost no information — you already knew
what it was going to say. And on the rare occasion it says CRITICAL, that report
carries a great deal.

Shannon's measure handles this. The information content of a single outcome with
probability *p* is

$$I = \log_2 \frac{1}{p} = -\log_2 p \ \text{bits}$$

so NORMAL at *p* = 0.99 carries −log₂(0.99) ≈ 0.0145 bits, while CRITICAL at, say,
*p* = 0.001 carries −log₂(0.001) ≈ 9.97 bits. Rare events are informative;
that is what "informative" means. The *average* over many reports is the
**entropy**, which Chapter 4 develops properly.

The engineering consequence, visible already: if the average is well below 2 bits,
we are wasting capacity by spending 2 bits on every report, and a cleverer
encoding — short codes for common messages, long codes for rare ones — will do
better. That is exactly what Vail did in 1838 by counting the type in a printer's
font case and giving `E` a single dot. It is what every compression algorithm
does. And it is why, in Chapter 7, the 8B/10B line code's overhead is not simply
waste: it buys DC balance and clock recovery with bits that the raw data rate
could have used.

## Bits as a physical commitment

One more framing, because it will matter in Chapter 4 and again in Chapter 42.

A bit is not just an abstraction. Landauer's principle, published by Rolf Landauer
at IBM in 1961, states that *erasing* one bit of information necessarily
dissipates at least *kT* ln 2 joules of energy as heat — about 2.9 × 10⁻²¹ J at
room temperature. This is a very small number, but it is not zero, and it ties
information irreversibly to thermodynamics. The noise that limits every channel in
this book is thermal noise, whose magnitude is set by the same *kT*.

So when Chapter 4 tells you that a channel's capacity depends on temperature, that
is not a metaphor or an approximation. Information and heat are the same currency,
and the exchange rate is Boltzmann's constant.

> **Network+ note.** None of this section is examinable, and it is the most
> important section in the chapter. What N10-009 *will* test is the arithmetic
> downstream of it: given a prefix length, how many hosts; given a host count,
> what prefix. Both are the log₂ *n* question in disguise, and students who
> understand the disguise stop needing the chart.
