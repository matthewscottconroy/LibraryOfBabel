# Reading a Pattern Many Ways

Let us finish the chapter by doing the thing the chapter has been claiming is
possible. One pattern, thirty-two bits, read under six different agreements.

Here is the pattern:

```
01000001 01001000 00000000 00000000
```

I will work each reading through, and I would encourage you to try at least the
first two yourself before reading the answers. You have everything you need.

## Reading 1: four independent bytes

The simplest agreement: each group of eight bits is its own unsigned number, 0
to 255.

- `01000001` is 64 + 1, which is 65
- `01001000` is 64 + 8, which is 72
- `00000000` is 0
- `00000000` is 0

So under this agreement the pattern is the four values **65, 72, 0, 0**.

(If the arithmetic there was not obvious, do not worry — Chapter 2 does
positional notation properly. For now: the bit positions stand for 128, 64, 32,
16, 8, 4, 2, 1 from left to right, and you add up the ones that are set.)

## Reading 2: text

Now agree that each byte is an ASCII character.

Byte 65 is the letter `A`. Byte 72 is the letter `H`. Byte 0 is the NUL
character, which is not a printable letter at all but a control code — in some
languages, notably C, it is the marker that says a piece of text has ended.

So the pattern is **`AH` followed by two NULs**, and depending on whose
convention we are following, that might be the two-character string `AH` stored
in a four-byte space with the rest padded out.

## Reading 3: one 32-bit unsigned number

Now agree that all thirty-two bits form a single number, most significant bit
first. Then the value is:

65 × 16,777,216 + 72 × 65,536 + 0 × 256 + 0

which is 1,090,519,040 + 4,718,592, or **1,095,237,632**.

The pattern did not change. We agreed to read the boundaries differently
— and where the boundaries fall is part of the agreement, exactly as much as
what the symbols mean.

## Reading 4: the same number, other way round

Here is the one that surprises people.

I said "most significant bit first". Not every machine agrees. The order in which
a multi-byte value is laid out in memory is called **endianness**, and there are
two conventions in use.

A **big-endian** machine stores the most significant byte at the lowest address —
the reading we just did. A **little-endian** machine stores the least significant
byte first. Intel and AMD processors, which is to say the overwhelming majority
of desktop and laptop machines, are little-endian.

Read our pattern as little-endian and the byte order reverses: the value becomes

0 × 16,777,216 + 0 × 65,536 + 72 × 256 + 65

which is **18,497**.

Same thirty-two bits. Same "read it as one unsigned number" instruction. Two
answers, differing by a factor of nearly sixty thousand, because the two machines
disagree about which end to start at.

The names, incidentally, come from *Gulliver's Travels*, in which Lilliput and
Blefuscu go to war over which end of a boiled egg to open. Danny Cohen borrowed
the joke in a 1980 note called "On Holy Wars and a Plea for Peace", and it stuck.
The joke's point was that neither convention is better — they merely have to
agree — which is the argument of this entire chapter in one image.

Java, for what it is worth, specifies big-endian for its own data formats
regardless of the hardware underneath, which removes a whole category of problem
and is one of the quiet reasons Java code moves between machines as easily as it
does.

## Reading 5: a floating-point number

Now agree that the thirty-two bits are an IEEE 754 single-precision float. That
agreement carves the bits into three fields: 1 bit of sign, 8 bits of exponent,
23 bits of fraction.

```
0 10000010 10010000000000000000000
^ ^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^
| exponent  fraction
sign
```

The sign bit is 0, so the number is positive. The exponent field is 130, and the
format says to subtract a bias of 127, giving an actual exponent of 3. The
fraction field encodes 1.5625.

So the value is 1.5625 × $2^{3}$ = **12.5**.

Chapter 3 derives all of this properly; I show it here only so you can see how
differently the same bits can be carved. Notice that this reading does not even
respect byte boundaries — the exponent field straddles two bytes. The agreement
gets to decide that too.

## Reading 6: a color

Agree that the four bytes are the red, green, blue, and alpha channels of a
pixel. Then we have red 65, green 72, blue 0, alpha 0: a dark olive green,
completely transparent. Which is to say, on screen, invisible.

## What to take from this

Six readings. **65, 72, 0, 0** — or `AH\0\0` — or **1,095,237,632** — or
**18,497** — or **12.5** — or an invisible olive pixel.

All six are correct. None is what the pattern "really" is, because the pattern is
not really anything. It is thirty-two settled binary distinctions, and meaning
arrives from outside.

I would like you to notice one more thing before we move on. In every reading,
the machine would happily perform operations. It would add 1 to the olive pixel
and get a slightly redder one. It would add 1 to the 12.5 and get 13.5 — a
completely different bit pattern from the one you get by adding 1 to
1,095,237,632. The *operations* belong to the agreement too. There is no neutral
"adding one" that happens beneath the interpretations.

This is why the types you declare in Java matter so much, and why the compiler
is so insistent about them. When you write `int`, you are not describing what is
in the memory. You are declaring which agreement is in force — and therefore
which operations mean what.

In the next chapter we take the most important of these agreements, the one for
whole numbers, and build it from the ground up.
