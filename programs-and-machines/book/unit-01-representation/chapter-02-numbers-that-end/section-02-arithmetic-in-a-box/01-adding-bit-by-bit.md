# Adding Bit by Bit

You learned to add in primary school and the procedure has been automatic ever
since. Line the numbers up, work right to left, add each column, carry when a
column exceeds nine.

That procedure works in any base. Only the threshold for carrying changes: carry
when a column exceeds *b* − 1. In base two, that means carry whenever a column
reaches 2.

## The four cases

In decimal, adding one column requires knowing a hundred single-digit sums. In
binary there are four:

```
0 + 0 = 0
0 + 1 = 1
1 + 0 = 1
1 + 1 = 0, carry 1
```

That last line is the only one that does anything interesting. One plus one is
two, and two is not a binary digit, so it is written `10` — a 0 in this column
and a carry into the next.

Four cases. That is the entire addition table. Compare the decimal table you
spent a year of childhood memorizing.

## Adding 13 and 6

```
    00001101      13
  + 00000110       6
  ──────────
```

Right to left:

- Column 0: 1 + 0 = 1. Write 1.
- Column 1: 0 + 1 = 1. Write 1.
- Column 2: 1 + 1 = 0, carry 1. Write 0.
- Column 3: 1 + 0, plus the carry of 1 = 0, carry 1. Write 0.
- Column 4: 0 + 0, plus carry 1 = 1. Write 1.
- Columns 5–7: all zero, no carry. Write 0.

```
    00001101      13
  + 00000110       6
  ──────────
    00010011      19
```

And 13 + 6 is 19. The procedure is the one you already knew.

## Three inputs, two outputs

Look closely at what a single column actually requires. It takes *three* inputs —
the two digits and the carry coming in from the right — and produces *two*
outputs: the digit to write, and the carry going out to the left.

That is a small enough job to build directly out of the switching circuits from
Chapter 1. Such a circuit is called a **full adder**, and it needs about five
logic gates. Chapter 8 will build it properly, once we have the logic; for now
the important observation is structural.

To add 8-bit numbers, you wire eight full adders in a row, the carry-out of each
feeding the carry-in of the next. To add 32-bit numbers, thirty-two of them.

Nothing in that arrangement understands numbers. Each adder handles its own
column and passes a single bit sideways. The number is nowhere; it exists only in
our agreement about what the row of columns means. This is the locality from the
positional-notation section, made physical.

## Where the carry goes

Now the question this chapter has been building toward.

The row of adders is a fixed size. Eight adders, eight columns. The carry out of
column 0 goes into column 1, and the carry out of column 6 goes into column 7.

What happens to the carry out of column 7?

There is no column 8. The wire has nowhere to go. In a real processor it is
routed to a status flag that a program *may* inspect, but as far as the result is
concerned it is discarded. The eight bits you get back are the eight bits that
fit.

Watch it happen with 200 + 100:

```
    11001000     200
  + 01100100     100
  ──────────
  1 00101100     ← the leading 1 has nowhere to live
```

The eight bits that remain are `00101100`, which is 44. Two hundred plus one
hundred is forty-four.

Under an 8-bit unsigned agreement, that is not an error. It is the correct
behavior of the machine we built. Three hundred is not representable in eight
bits — there are only 256 patterns and it is not among them — so the hardware
returned the low eight bits of the true answer, which is 300 − 256 = 44.

That subtraction is worth staring at. The discarded carry was worth 256, the
value of the ninth column. Throwing it away subtracts exactly 256. Which means
8-bit addition is not really addition at all; it is **addition modulo 256** —
addition on a circle of 256 positions, where counting past the top wraps around
to the bottom.

## The odometer

The mental image I would like you to carry is a car odometer with a fixed number
of wheels.

A three-wheel odometer counts 000, 001, up to 999 — and then to 000 again. It
does not break and it does not report an error. It has no wheel in which to
record the thousand, so the thousand is not recorded.

Binary addition in a fixed width is that odometer with two digits per wheel
instead of ten. The wrap is not a defect bolted on; it is what "fixed number of
wheels" *means*.

Everything in the rest of this chapter follows from taking that image seriously.
In particular — and this is the elegant part — if the numbers live on a circle,
then going backwards is the same as going far enough forwards. Which means, if we
are clever about how we represent negative numbers, we may not need subtraction
hardware at all.

That is the next section.
