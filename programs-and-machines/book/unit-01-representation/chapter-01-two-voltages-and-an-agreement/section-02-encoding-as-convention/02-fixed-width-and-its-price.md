# Fixed Width and Its Price

Write down the number seven hundred and forty-two. You wrote three digits. Write
down five. You wrote one. On paper, a number takes as much room as it needs, and
if it needs more you use more.

A machine cannot work that way, and the reason it cannot is the source of a
great deal of what will otherwise seem arbitrary about programming.

## Why the width has to be decided

Memory is not paper. It is a vast row of numbered locations, each holding a fixed
number of bits — eight, in nearly every machine you will ever use. That group of
eight is called a **byte**, and the numbering of locations is called
**addressing**.

Now consider what it means to find something. If a value can occupy any number of
bytes, then to locate the thousandth value you must first find out how long the
first value is, then the second, and so on — a thousand steps to reach the
thousandth item. But if every value occupies exactly four bytes, the thousandth
one starts at byte 4000, and you can go straight there in a single operation.

That is not a small difference. It is the difference between a program that
finishes and one that does not, and it is the reason arrays work the way they do
— a fact we will collect in Chapter 15, where you will see that indexing is
literally multiplication.

There is a second reason, at least as important. The circuitry that adds two
numbers has to be built out of physical gates, wired to specific positions. An
adder for 32-bit numbers is a specific arrangement of hardware with 32 positions
in it. It cannot stretch. You could build a machine that handles arbitrary widths
in software, and languages do exactly that — Java's `BigInteger` is one — but
underneath, that software is doing many fixed-width operations and stitching the
results together.

So: the width is fixed in advance because fixed width buys constant-time access
and finite hardware. That is the bargain.

## What the bargain costs

Fix the width and you have fixed the number of distinct patterns, and therefore
the number of distinct values. Eight bits give 256 patterns. Not 257. Whatever
you decide those patterns mean, there are 256 meanings available and no more.

Some things you might want to store do not fit in 256 meanings. So what should
happen when a value goes out of range?

There are only three possible answers, and every system you will ever use picks
one of them.

**Refuse.** Detect the overflow, stop, report an error. Safe, and slow — every
arithmetic operation now needs a check, and arithmetic is the most common thing a
processor does.

**Saturate.** Clamp to the largest representable value and stay there. This is
what audio and image processing usually do: brighten an already-white pixel and
it stays white, which is what you wanted anyway.

**Wrap.** Keep the bits that fit, discard the rest, and let the value roll over
like an odometer. This is fast — in fact it is free, because it is what the
hardware does when the extra circuitry is not built at all.

Java chose to wrap, for `int` and `long` arithmetic. So did C, and most
languages descended from it. That decision is why `2147483647 + 1` is
`-2147483648` and why no exception is raised: nothing exceptional happened. The
adder added, the top bit fell off the end, and the result was read back under the
same agreement as always.

We will do the arithmetic of that in Chapter 2, and you will find it is not
mysterious once you have seen where the discarded bit goes.

## The widths Java gives you

For reference — and this table will make more sense after Chapter 2, so treat it
as a landmark rather than something to memorize now:

| Type | Bits | Distinct values | Range |
|---|---:|---:|---|
| `byte` | 8 | 256 | -128 to 127 |
| `short` | 16 | 65,536 | -32,768 to 32,767 |
| `int` | 32 | 4,294,967,296 | -2,147,483,648 to 2,147,483,647 |
| `long` | 64 | about 1.8 × 10^19 | about ±9.22 × 10^18 |
| `char` | 16 | 65,536 | 0 to 65,535 |
| `boolean` | 1 in principle | 2 | false, true |

Two observations worth making now.

The ranges are not symmetric. `byte` runs from -128 to 127 — one more negative
value than positive. That asymmetry is not a mistake or a rounding; it falls
directly out of two's complement, and Chapter 2 will derive it.

And `char` is unsigned while everything else is signed. That is a historical
decision with consequences, and Chapter 4 will explain both the reason and the
trouble it causes.

## The width you cannot see

One more thing, because it catches people.

Fixed width is not only about how big a number can be. It is also about how
*precisely* a number can be described — and that is where Chapter 3 lives.

A 64-bit `double` can hold numbers up to about 10 to the 308th power, which is
vastly more than the number of atoms in the observable universe. It sounds like
range is a solved problem. But 64 bits still gives only about 1.8 × 10^19
distinct patterns, and there are infinitely many real numbers between 0 and 1
alone. So the enormous range is achieved by spacing the representable values out
— finely near zero, coarsely far from it.

Which means a `double` cannot hold most numbers. It holds a specific finite set
of them, and when you write something not in that set, you silently get the
nearest one that is.

That is the same bargain as before, in a different currency. Fixed width, finite
patterns, and a choice about what to do with everything that does not fit.

Next, we take the idea to its conclusion: one pattern, read every way we know
how.
