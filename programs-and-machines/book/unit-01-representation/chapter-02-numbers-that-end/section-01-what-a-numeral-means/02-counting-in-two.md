# Counting in Two

Take the rule from the last section and set the base to 2.

The positions, from the right, are worth 1, 2, 4, 8, 16, 32, 64, 128 — the powers
of two. The digits run from 0 to *b* − 1, which here means 0 to 1. So every digit
is a bit, and a binary numeral is exactly a pattern of bits under an agreement
about what the positions are worth.

That is the whole of it. Everything else in this section is practice.

## Reading a binary numeral

Evaluate `1011`:

```
1  0  1  1
│  │  │  └── 1 × 1 = 1
│  │  └───── 1 × 2 = 2
│  └──────── 0 × 4 = 0
└─────────── 1 × 8 = 8
                     ──
                     11
```

So `1011` in binary is eleven.

Notice that the multiplication is trivial. Each digit is 0 or 1, so the only
question at each position is *include this position's value or do not*. Reading
binary is adding up the positions where a 1 appears. There is no multiplication
table to know — which is a large part of why machines find base two congenial.

Try `11010110`, which appeared in Chapter 1's exercises. The positions are 128,
64, 32, 16, 8, 4, 2, 1:

```
1  1  0  1  0  1  1  0
128 64  -  16  -  4  2  -
```

128 + 64 + 16 + 4 + 2 = 214.

## Writing a number in binary

Repeated division by 2, exactly as before. Take 214:

```
214 ÷ 2 = 107 remainder 0     ← rightmost
107 ÷ 2 =  53 remainder 1
 53 ÷ 2 =  26 remainder 1
 26 ÷ 2 =  13 remainder 0
 13 ÷ 2 =   6 remainder 1
  6 ÷ 2 =   3 remainder 0
  3 ÷ 2 =   1 remainder 1
  1 ÷ 2 =   0 remainder 1     ← leftmost
```

Bottom to top: `11010110`. Which is where we started, so the two procedures are
inverses, as they should be.

There is a second method many people find faster. Take the largest power of two
that fits, subtract it, repeat:

- 214: the largest power of two not exceeding it is 128. Put a 1 in the 128
  place. 214 − 128 = 86.
- 86: largest is 64. 1 in the 64 place. 86 − 64 = 22.
- 22: 32 is too big, so 0 in the 32 place. 16 fits. 1 there. 22 − 16 = 6.
- 6: 8 too big, 0. 4 fits, 1. 6 − 4 = 2.
- 2: 1 in the 2 place. 2 − 2 = 0.
- 1 place: nothing left. 0.

Reading off: 1, 1, 0, 1, 0, 1, 1, 0. Same answer.

Use whichever suits you. The division method is mechanical and never requires
judgment, which makes it better when you are tired; the subtraction method is
faster once the powers of two are memorized, which they will be shortly whether
you intend it or not.

## The powers of two

You will end up knowing these. Not because anyone made you, but because they
recur constantly:

| power | value | | power | value |
|---|---:|---|---|---:|
| $2^{0}$ | 1 | | $2^{8}$ | 256 |
| $2^{1}$ | 2 | | $2^{9}$ | 512 |
| $2^{2}$ | 4 | | $2^{10}$ | 1,024 |
| $2^{3}$ | 8 | | $2^{11}$ | 2,048 |
| $2^{4}$ | 16 | | $2^{12}$ | 4,096 |
| $2^{5}$ | 32 | | $2^{16}$ | 65,536 |
| $2^{6}$ | 64 | | $2^{20}$ | 1,048,576 |
| $2^{7}$ | 128 | | $2^{32}$ | 4,294,967,296 |

Two of these deserve comment.

$2^{10}$ = 1,024, which is close to a thousand. This coincidence is why "kilobyte"
came to mean 1,024 bytes rather than 1,000 in much early usage, and why your
1 TB drive appears to hold rather less than a terabyte when the operating system
reports it. The drive manufacturer counted in powers of ten; the operating
system in powers of two; both were being honest.

$2^{32}$ = 4,294,967,296, a little over four billion. Remember roughly "four billion"
and you will be able to sanity-check a great deal.

## Counting up

Write out the first sixteen numbers in four-bit binary. I would like you to do
this yourself before looking, because the pattern that emerges is one you should
discover rather than be shown.

```
 0  0000      8  1000
 1  0001      9  1001
 2  0010     10  1010
 3  0011     11  1011
 4  0100     12  1100
 5  0101     13  1101
 6  0110     14  1110
 7  0111     15  1111
```

Look at the rightmost column: it alternates every row — 0, 1, 0, 1. The next
column alternates every two rows. The next every four. The leftmost every eight.

That is the doubling from Chapter 1, seen from a different angle. Each bit you
add doubles the count, so each bit changes half as often as its neighbor to the
right.

Notice also the last row: `1111` is 15, not 16. Four bits, sixteen patterns,
largest value fifteen. The $2^{n}$ − 1 boundary again, and now you can see exactly
where it comes from: the largest numeral is all 1s, which is 8 + 4 + 2 + 1 = 15,
one short of 16.

That is not a coincidence about four. All 1s in *n* positions is
1 + 2 + 4 + … + $2^{n-1}$, and that sum is always exactly $2^{n}$ − 1. If you want to see
why without doing algebra: add 1 to a numeral of all 1s and watch every column
carry, all the way off the left end, leaving a 1 followed by *n* zeros — which is
$2^{n}$. So all-1s must be one less.

That parenthetical is going to matter enormously in two sections' time, when the
carry that runs off the left end has nowhere to go.

## Binary is verbose

One honest observation before we move on. Binary numerals are long. 214 takes
three characters in decimal and eight in binary. A 32-bit value takes 32
characters, and if I printed one here you would not be able to tell at a glance
whether it had 31 digits or 33.

This is fine for machines and miserable for people. Which is why, in practice,
almost nobody writes binary — they write hexadecimal, which is the subject of the
next section and is best understood not as a third number system but as a
shorthand for this one.
