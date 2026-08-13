# Chapter 2 — Bits, Bytes, and Bases

In 1937, a twenty-one-year-old master's student at MIT submitted a thesis arguing
that the algebra George Boole had invented in 1854 to describe the laws of thought
could be used to describe relay switching circuits. Claude Shannon's *A Symbolic
Analysis of Relay and Switching Circuits* is sixty-nine pages long and it is,
by a wide margin, the most consequential master's thesis ever written. It said
that a switch which is either open or closed, and a proposition which is either
false or true, obey the same algebra — and therefore that circuits could be
designed by calculation rather than by intuition and rework.

Eleven years later, in the 1948 paper, Shannon needed a name for the unit of the
quantity he was measuring. He proposed one that his Bell Labs colleague John Tukey
had coined in an internal memo: *bit*, a contraction of **bi**nary digi**t**.
Shannon's footnote is characteristically dry.

We are going to spend this chapter on that unit, and on the number systems built
from it, and it is worth saying at the outset why a networking book cares.

It cares because **an IP address is a 32-bit integer that has been dressed up in a
costume**, and every confusion students have about subnetting — every single one
— comes from believing the costume. `192.168.10.70` looks like four numbers. It is
one number. The four-numbers-separated-by-dots presentation is a nineteen-seventies
concession to human legibility, and it actively obstructs the one operation that
matters, which is masking off a prefix. Students who learn subnetting as a set of
decimal recipes can subnet on /24 boundaries and fall apart on /27. Students who
can see the thirty-two bits can subnet anywhere, in their heads, forever.

So this chapter is the foundation under Chapter 26, and Chapter 26 is the
foundation under most of the professional practice of networking. It is also the
foundation under MAC addresses, hexadecimal packet dumps, subnet masks, TCP flag
fields, VLAN tags, DSCP markings, IPv6 notation, CIDR aggregation, and the ability
to read a Wireshark hex pane and see something other than a wall of characters.

## What this chapter does

We begin by asking what information *is*, in the specific sense that permits it to
be counted: information as resolved uncertainty, measured by how many yes/no
questions it takes to identify one possibility out of many. From that, binary
falls out as inevitable rather than conventional. We then build binary properly —
place value, conversion in both directions, the powers of two you will use for the
rest of your career, and the bitwise operations that Chapter 26 runs on.

Next, hexadecimal: why a base nobody has fingers for is nonetheless the right
notation for machine data, and why the answer is entirely about the number four.
We cover byte boundaries, nibbles, endianness, and the reason a MAC address is
written in hex and an IPv4 address in decimal even though both are just integers.

Finally, we ask how bit patterns come to *mean* anything: character encodings from
Baudot through ASCII to UTF-8, and the general principle — which recurs at every
layer of every protocol in this book — that a bit pattern's meaning is supplied
entirely by an agreement about how to read it, and that a mismatch in that
agreement produces not an error but confident garbage.

## By the end you will be able to

- Convert between binary, decimal, and hexadecimal in both directions without a
  calculator, and know the powers of two up to 2³² by recognition.
- Explain why information is measured logarithmically, and compute the number of
  bits needed to distinguish one of *n* possibilities.
- Perform bitwise AND, OR, XOR and NOT on binary strings, and explain what
  masking an address with a subnet mask actually does at the bit level.
- Read a hexadecimal packet dump and locate byte boundaries, fields, and
  addresses within it.
- Explain endianness, and predict what happens when two systems disagree about it.
- Distinguish a bit pattern from its interpretation, and give three examples of
  the same 32 bits meaning entirely different things.

## Where this sits in the argument

Chapter 1 said information must be moved. This chapter makes it countable, which
is the precondition for Chapter 3 measuring the rate at which it moves and
Chapter 4 proving there is a ceiling on that rate.

Nothing in this chapter is networking. Everything in this book depends on it. If
you are already fluent in binary and hex, read §2.1 for the framing and §2.4 for
the principle about interpretation, and skim the rest — but do check that you can
do §2.2's exercises without hesitation, because Chapter 26 will assume it.
