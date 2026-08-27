# Negative Numbers and Two's Complement

We have eight bits and 256 patterns. So far every pattern has meant a
non-negative number, 0 through 255. Now we want negatives, and there are no spare
patterns — we have to give some of the existing ones a new meaning.

This is a design problem, and it is worth trying to solve it yourself before
seeing the answer. How would you represent −5 in eight bits?

## The obvious idea, and why it fails

Most people invent the same thing: use the leftmost bit as a sign. 0 means
positive, 1 means negative, and the remaining seven bits give the magnitude.

So 5 is `00000101` and −5 is `10000101`. This is called **sign-magnitude**, it is
how we write numbers on paper, and it has two problems.

The first is cosmetic but irritating: there are two zeros. `00000000` is +0 and
`10000000` is −0. They are different patterns, so a naive comparison says they
are different, but they denote the same quantity. Every piece of code that
compares numbers now needs a special case.

The second problem is fatal. Try adding 5 and −5 with the ordinary column
procedure:

```
    00000101      5
  + 10000101     −5
  ──────────
    10001010     −10
```

The answer should be zero. The adder does not know that the leftmost bit is
special, so it added it like any other column, and produced nonsense. To make
sign-magnitude work, the processor needs a *different* circuit that inspects the
signs, decides whether this is really an addition or a subtraction, compares
magnitudes to find which is larger, subtracts the smaller from the larger, and
works out the sign of the result.

That is a great deal of hardware to add two numbers. There is a better way.

## The circle

Go back to the odometer. Eight bits give 256 positions arranged in a circle:
count up past 255 and you arrive at 0.

Now ask: on a circle, what does subtracting 5 mean?

It means moving five positions backwards. But on a circle of 256 positions,
moving 5 backwards lands in exactly the same place as moving 251 forwards. The
two operations are indistinguishable — not approximately, but exactly, because
251 = 256 − 5.

So if we *agree* that the pattern for 251 also means −5, then subtraction becomes
addition, and we need no new hardware whatsoever.

That agreement is **two's complement**, and it is one of the most satisfying
ideas in this part of the subject.

## Making it concrete

251 in binary is `11111011`. Let us test the claim by computing 7 + (−5), which
should be 2:

```
    00000111       7
  + 11111011      −5  (the pattern for 251)
  ──────────
  1 00000010
```

The carry out of the top falls off the end, as it always does. The eight bits
remaining are `00000010`, which is 2.

Seven minus five is two. The ordinary adder — the same one from the last section,
with no modifications and no knowledge of signs — produced the right answer.

That is the whole trick. The discarded carry is worth 256, and throwing it away
is precisely what converts "add 251" into "subtract 5".

## Which patterns are negative?

We now split the 256 patterns. The convention is to give half to each sign, with
one adjustment for zero:

- `00000000` through `01111111` (0 to 127) mean themselves.
- `10000000` through `11111111` (128 to 255 read as unsigned) mean −128 to −1.

So the range is **−128 to +127**.

Notice the asymmetry the last chapter promised to explain. There is one more
negative value than positive, and now you can see exactly why: one pattern must
be spent on zero. That leaves 255 patterns for non-zero values, and 255 is odd,
so it cannot split evenly. Zero is grouped with the positives (its leading bit is
0), leaving 127 positives and 128 negatives.

The asymmetry is not a wart. It is what happens when you divide an odd number in
half.

Notice too that the leftmost bit still tells you the sign — it is 1 for exactly
the negative values. This is convenient and it is why people call it "the sign
bit", but do not let the name mislead you: it is not a sign *flag* that is
inspected separately. It is an ordinary bit carrying an ordinary place value.
Which value? In eight-bit two's complement, the leftmost position is worth
**−128** rather than +128. Every other position keeps its usual positive value.

Check `11111011` with that rule:

−128 + 64 + 32 + 16 + 8 + 0 + 2 + 1 = −5.

Which is what we wanted. Two's complement is ordinary positional notation with
one position's weight made negative.

## Negating a number

To find the pattern for −*n*, there is a mechanical recipe: **flip every bit,
then add one.**

For −5, start from 5:

```
5           00000101
flip        11111010
add 1       11111011      ← −5
```

Which matches. The recipe works in reverse too — apply it to `11111011` and you
get `00000101` back.

Why does it work? Flipping every bit of *n* gives 255 − *n*, because each bit
contributes its place value when set and the complement fills in exactly the
positions *n* left empty. Adding one gives 256 − *n*, which is the position five
steps back from zero on our circle. That is exactly the pattern we wanted.

## The one that has no partner

Try the recipe on −128, whose pattern is `10000000`:

```
flip        01111111
add 1       10000000
```

You get back where you started. −128 is its own negation, which is impossible for
a number.

It is not a bug in the recipe; it is the asymmetry showing up. +128 is not
representable in eight bits, so `-(-128)` has no correct answer, and the hardware
returns −128 because that is the pattern arithmetic produces.

This is real, and it is in Java. `Math.abs(Integer.MIN_VALUE)` returns
`Integer.MIN_VALUE` — a negative absolute value. The documentation says so
explicitly. It is not a defect in the library; there is no 32-bit pattern for
+2,147,483,648, so no correct answer exists to return.

If you ever find code that computes an absolute value and then assumes the result
is non-negative, you have found a real bug, and it will fire exactly once every
four billion inputs.

## Why everyone uses this

Two's complement won completely — every processor you are likely to meet uses it,
and Java mandates it — for reasons that are now visible:

- One representation of zero.
- Addition, subtraction, and comparison of signed values use the *same* circuits
  as unsigned. No sign inspection, no special cases.
- The ordinary "did the last column carry" logic works unchanged.

The cost is that the encoding is not intuitive to read by eye, and that the range
is lopsided. Both are small prices for eliminating an entire class of hardware.

And notice what has happened, one more time. The bits `11111011` are 251 or −5
depending on which agreement is in force. The pattern did not change. Java's
`int` and a hypothetical unsigned type would share the identical bits and differ
only in what they claim the bits mean — and, consequently, in what `>` means and
what printing does.

Next: what happens when we ask this arithmetic for an answer it does not have.
