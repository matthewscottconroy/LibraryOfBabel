# 2.2 Binary and the Powers of Two

This section is the one piece of pure arithmetic drill in the book. It is here
because Chapter 26 cannot be understood without it, and because every hour spent
here saves five later.

## Place value, stated once

Decimal notation is positional: each digit's contribution is the digit multiplied
by a power of ten determined by its position. `4,096` means

$$4 \times 10^3 + 0 \times 10^2 + 9 \times 10^1 + 6 \times 10^0$$

Binary is the same construction with base 2 and only two digit values. The binary
string `1101` means

$$1 \times 2^3 + 1 \times 2^2 + 0 \times 2^1 + 1 \times 2^0 = 8 + 4 + 0 + 1 = 13$$

That is the entire theory. Everything else is fluency.

## The table to know by recognition

You should be able to produce any entry in this table without computing it. Network
engineers use the left column daily and the right column when sizing address space.

| *n* | 2ⁿ | | *n* | 2ⁿ |
|---|---|---|---|---|
| 0 | 1 | | 12 | 4,096 |
| 1 | 2 | | 13 | 8,192 |
| 2 | 4 | | 14 | 16,384 |
| 3 | 8 | | 15 | 32,768 |
| 4 | 16 | | 16 | 65,536 |
| 5 | 32 | | 17 | 131,072 |
| 6 | 64 | | 18 | 262,144 |
| 7 | 128 | | 19 | 524,288 |
| 8 | 256 | | 20 | 1,048,576 |
| 9 | 512 | | 24 | 16,777,216 |
| 10 | 1,024 | | 32 | 4,294,967,296 |
| 11 | 2,048 | | 48 | 281,474,976,710,656 |

The last two rows are why IPv4 has about 4.3 billion addresses and why a MAC
address space of 2⁴⁸ was, in 1980, considered so absurdly generous that nobody
would ever need to worry about exhaustion. (They were right for forty-five years
and counting, which is a better record than IPv4 managed.)

Within a single octet — the eight bits that IPv4 notation groups — the place
values are:

```
 128   64   32   16    8    4    2    1
  │    │    │    │     │    │    │    │
  b7   b6   b5   b4    b3   b2   b1   b0
```

Memorise that row. It is the row you will use to convert `.192` to `11000000` in
your head in Chapter 26, and it is the row that makes subnet masks legible.

## Binary to decimal

Add the place values where there is a 1.

`10110101` → 128 + 32 + 16 + 4 + 1 = **181**

Work right to left or left to right; it makes no difference. With practice this
takes about two seconds per octet.

## Decimal to binary: two methods

**Method A — subtract the largest power.** Take 181. Largest power of two not
exceeding it: 128. Write 1, subtract: 53. Next place is 64; 64 > 53, write 0.
Next is 32; fits, write 1, remainder 21. Next 16; fits, write 1, remainder 5.
Next 8; doesn't fit, write 0. Next 4; fits, write 1, remainder 1. Next 2; doesn't
fit, write 0. Next 1; fits, write 1, remainder 0.

Result: `10110101`. This is the method to use for network work, because it walks
the place values in the same order you read them.

**Method B — repeated division by 2.** Divide by 2, record the remainder, repeat
with the quotient, then read the remainders bottom to top.

```
181 ÷ 2 = 90 r 1
 90 ÷ 2 = 45 r 0
 45 ÷ 2 = 22 r 1
 22 ÷ 2 = 11 r 0
 11 ÷ 2 =  5 r 1
  5 ÷ 2 =  2 r 1
  2 ÷ 2 =  1 r 0
  1 ÷ 2 =  0 r 1
```

Reading upward: `10110101`. Same answer. Method B generalises to any base;
Method A is faster for eight bits.

## The eight masks worth knowing cold

In an IPv4 subnet mask, an octet is always a run of 1s followed by a run of 0s —
never anything else. That constraint means only nine values can legally appear in
a mask octet, and eight of them are interesting:

| Binary | Decimal | Bits set | Block size |
|---|---|---|---|
| `00000000` | 0 | 0 | 256 |
| `10000000` | 128 | 1 | 128 |
| `11000000` | 192 | 2 | 64 |
| `11100000` | 224 | 3 | 32 |
| `11110000` | 240 | 4 | 16 |
| `11111000` | 248 | 5 | 8 |
| `11111100` | 252 | 6 | 4 |
| `11111110` | 254 | 7 | 2 |
| `11111111` | 255 | 8 | 1 |

Note the relationship, which is the whole of subnetting in one line: **block size
= 256 − mask octet value**, and **block size = 2^(bits not set)**. A mask octet of
224 leaves 5 bits free, 2⁵ = 32, and the subnets in that octet start at 0, 32, 64,
96, 128, 160, 192, 224. That is Chapter 26's central trick, available to you
already.

If a mask octet is not in this table — `11010000`, say, decimal 208 — it is not a
valid contiguous mask. Some very old equipment permitted non-contiguous masks;
they caused so much confusion that RFC 4632 formally requires contiguity. If you
ever see one, it is a typo.

## Bitwise operations

Four operations, applied bit by bit between aligned operands.

**AND** yields 1 only if both inputs are 1.

```
   11000000 10101000 00001010 01000110    192.168.10.70
AND 11111111 11111111 11111111 11100000    255.255.255.224  (/27)
  = 11000000 10101000 00001010 01000000    192.168.10.64
```

That calculation — and it is a calculation your router performs, in hardware, for
every packet it forwards — is how a network address is extracted from a host
address. AND with the mask preserves the network bits and zeroes the host bits.
It is *the* fundamental operation of IP networking. Chapter 26 does nothing else.

**OR** yields 1 if either input is 1. ORing an address with the *inverse* of the
mask sets all host bits to 1, producing the broadcast address:

```
    11000000 10101000 00001010 01000000    192.168.10.64
 OR 00000000 00000000 00000000 00011111    0.0.0.31   (inverted mask)
  = 11000000 10101000 00001010 01011111    192.168.10.95
```

**XOR** yields 1 if the inputs differ. It is how you find which bits two addresses
disagree on — useful for computing how much of a prefix two addresses share, which
is what a longest-prefix-match lookup in Chapter 29 is doing. It is also the
core of the CRC in Chapter 15 and of every stream cipher in Chapter 58.

**NOT** inverts. The inverted mask above is NOT of the mask, and Cisco access
lists call it a **wildcard mask**, which is a naming decision that has confused
students for thirty years and which we will meet properly in Chapter 60.

## Practice, and why it is not optional

Convert these to binary, then AND each with `255.255.255.240`, and state the
resulting network address. Do them on paper.

`10.1.1.200` · `172.16.30.99` · `192.168.4.17` · `203.0.113.254`

Then go the other way: for each of these masks, state how many host bits remain
and how many usable host addresses that gives. `/26` · `/30` · `/22` · `/19`

The answers are in the exercises. The point is not the answers. The point is that
when you can do this without thinking, Chapter 26 takes an afternoon instead of a
semester, and subnetting questions on an exam become arithmetic rather than
recall.

> **Network+ note.** N10-009 expects scenario-based IPv4 addressing including
> subnetting, VLSM, and CIDR — objective 1.7. There is no calculator in the exam.
> There is also no need for one; everything the exam asks can be done with the
> table above and the AND operation.
