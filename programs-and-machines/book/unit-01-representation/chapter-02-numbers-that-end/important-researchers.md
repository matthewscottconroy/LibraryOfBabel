# People

## Brahmagupta (598 – c. 668)

An Indian mathematician and astronomer whose *Brāhmasphuṭasiddhānta*, written in
628, contains the earliest surviving systematic treatment of zero as a number in
its own right, and rules for arithmetic with negative quantities — including that
a debt minus zero is a debt, and that the product of two debts is a fortune.

It is worth registering how recent this is by mathematical standards, and how
contested. Negative numbers were treated with suspicion in Europe for another
thousand years; various mathematicians into the eighteenth century regarded them
as absurd or merely a bookkeeping convenience. The idea that −5 is as legitimate
a number as 5 had to be argued for, and this chapter's problem — how do we
represent a negative in a machine that only has patterns — is a distant echo of
that argument.

## Muhammad ibn Musa al-Khwārizmī (c. 780 – c. 850)

A scholar at the House of Wisdom in Baghdad. His treatise on calculation with
Hindu numerals, written around 825, was the vehicle by which positional notation
and the decimal digits reached the Islamic world and subsequently Europe. The
Latin rendering of his name, *Algoritmi*, gives us the word "algorithm"; the
title of another of his works, on *al-jabr*, gives us "algebra".

He appears in this chapter because positional notation is its foundation. The
rule you apply without thinking when you read `742` is the one his book
transmitted.

## Leonardo of Pisa, called Fibonacci (c. 1170 – c. 1250)

*Liber Abaci* (1202) argued for Hindu-Arabic numerals in Europe, largely on
practical commercial grounds: the new notation made calculation with the pen
faster than calculation on a counting board, and made the working auditable
afterwards.

The resistance he met is instructive. Roman numerals were entrenched, and some
authorities distrusted the new numerals partly because they were easier to
falsify — a 0 can be made into a 6 or 9 with a stroke of a pen, whereas altering
a Roman numeral is conspicuous. Two centuries passed before the change was
complete. Notation is never only notation.

## John von Neumann (1903 – 1957)

The *First Draft of a Report on the EDVAC*, circulated in 1945, set out the
architecture that essentially all subsequent computers have followed — and argued
explicitly for binary rather than decimal representation inside the machine, on
grounds close to those in Chapter 1: binary elements are simpler and more
reliable, and the conversion cost at the boundary is small compared to the saving
throughout.

The document's authorship is genuinely contested — it drew on the work of
J. Presper Eckert and John Mauchly among others, and its circulation under von
Neumann's name alone caused lasting bitterness. We return to the architecture
itself in Chapter 6.

## Joshua Bloch (born 1961)

A Java engineer responsible for much of the Java Collections Framework, and
author of *Effective Java*.

He appears here for a blog post: in June 2006 he described a bug in the binary
search he had himself written for the Java standard library, where
`(low + high) / 2` overflows for sufficiently large arrays. The bug had been
present for around nine years, in code that had been read by a great many
competent people, and was derived from a published and formally verified
algorithm.

The reason to include it is not the bug but its survival. It is the clearest
available evidence that fixed-width arithmetic does not behave the way trained
intuition expects.

## Jacques-Louis Lions (1928 – 2001)

A French mathematician, principally known for work in partial differential
equations and numerical analysis, who chaired the inquiry board into the loss of
Ariane 5 Flight 501 in June 1996.

The board's report is a model of clear technical writing and is worth reading in
full — it is short. The immediate cause was a 64-bit floating-point horizontal
velocity value converted to a 16-bit signed integer, which overflowed because
Ariane 5 flew a steeper and faster trajectory than the Ariane 4 the code was
written and validated for. The board's deeper findings were about testing and
about the reuse of components outside their validated envelope, which are
arguably the more useful lessons.
