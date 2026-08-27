# Variable-Length Codes

Section 33.1.2 said English carries about one bit per character and is stored at
eight. Closing that gap is compression, and the first idea is the oldest one.

**Give frequent symbols short codes and rare symbols long ones.**

Samuel Morse did this in 1838 by counting the type in a printer's case to find out
which letters were common. `E` is one dot; `Q` is dash-dash-dot-dash. It is the
right idea a century before there was a theory saying why.

## The prefix property

Variable-length codes have a problem fixed-length ones do not. Given

```
a -> 0        b -> 1        c -> 01
```

the bits `01` could be `c`, or `a` then `b`. The code is ambiguous.

The fix is the **prefix property**: no code word is a prefix of another. Then a
decoder reading left to right knows a symbol has ended the moment it recognizes
one, with no lookahead and no separators.

```
a -> 0        b -> 10       c -> 110      d -> 111
```

Decode `0101110`: `0` is `a`; `10` is `b`; `111` is `d`; `0` is `a`. Unambiguous.

Such a code is exactly a **binary tree** with symbols at the leaves — left is 0,
right is 1, and a path from the root spells a code word. Nothing is a prefix of
anything else precisely because no leaf is an ancestor of another.

That is the Chapter 24 tree in a third role, and it is why the algorithm below
builds one.

UTF-8 is a prefix code, which is Chapter 4's promise: a byte's leading bits say how
many bytes the character occupies, so a decoder is never ambiguous and can
resynchronize after damage.

## Huffman's algorithm

David Huffman, 1952, as a term-paper alternative to a final exam. The algorithm is
four lines and it is provably optimal.

> Put every symbol in a priority queue keyed by frequency.
> While more than one remains: remove the two smallest, join them under a new node
> whose frequency is their sum, and put it back.
> The remaining node is the root. Left edges are 0, right edges are 1.

The rarest symbols get joined first, so they end up deepest, so they get the
longest codes. Exactly the right thing, and it falls out of the greedy rule.

## Worked and verified

The string `aaaaaaaabbbbccdd` — eight `a`, four `b`, two `c`, two `d`, sixteen
characters.

Verified output:

```
a  freq  8  code 0    (1 bits)
b  freq  4  code 10   (2 bits)
c  freq  2  code 110  (3 bits)
d  freq  2  code 111  (3 bits)

entropy       = 1.7500 bits/char
Huffman avg   = 1.7500 bits/char
fixed 2-bit   = 2.0000 bits/char

total: 28 bits Huffman vs 32 bits fixed vs 128 bits ASCII
```

Three things in that.

**The average matches the entropy exactly.** 1.75 both. That is not luck — it
happens whenever every probability is a power of two, as here: $1/2, 1/4, 1/8,
1/8$. Section 33.1.2 said the entropy is achievable, and here it is achieved.

**Check the arithmetic.** The entropy is

$$H = -\left(\frac{1}{2}\log_2\frac{1}{2} + \frac{1}{4}\log_2\frac{1}{4}
      + 2 \cdot \frac{1}{8}\log_2\frac{1}{8}\right)
    = \frac{1}{2} + \frac{1}{2} + \frac{3}{4} = 1.75$$

and the weighted code length is

$$\frac{8 \cdot 1 + 4 \cdot 2 + 2 \cdot 3 + 2 \cdot 3}{16} = \frac{28}{16} = 1.75$$

**28 bits against 128.** The ASCII version spends eight bits on each of sixteen
characters. Huffman spends 28, a factor of about 4.6.

When the probabilities are not powers of two, Huffman is within one bit per symbol
of the entropy and no better code with integer-length code words exists.
**Arithmetic coding** does better by abandoning the requirement that each symbol
get a whole number of bits, and reaches the entropy in general. It is what modern
compressors use.

## What real compressors add

Huffman handles symbol frequencies. Real data has more structure than that, and
the standard additions are two.

**Run-length encoding** replaces repetitions with a count. `aaaaaaaa` becomes
`8a`. Trivial, and excellent on images with flat regions.

**Dictionary compression** — Lempel and Ziv, 1977 and 1978 — replaces a repeated
sequence with a reference to its earlier occurrence. This is where most of the win
comes from in text, because text repeats words and phrases rather than merely
favouring letters.

`gzip` is LZ77 followed by Huffman: find the repetitions, then encode what remains
by frequency. Almost every general-purpose compressor is some version of that
pair.

## Measured

Three inputs of exactly 9,000 bytes through `gzip`:

```
repeated 'a'       9,000 ->      44 bytes  (0.5%)  H = 0.000 bits/char
english text       9,000 ->     108 bytes  (1.2%)  H = 4.397 bits/char
random letters     9,000 ->   5,636 bytes  (62.6%)  H = 4.699 bits/char
```

The first two are what Section 33.1.2 predicts. Nine thousand identical characters
have zero entropy and compress to 44 bytes, essentially all of it header.

The third is the important one. Random letters have entropy 4.70 bits per
character — the maximum for 26 symbols — and compressed to 62.6%, which is close
to $4.70/8 = 58.75\%$. There is no structure to exploit beyond the fact that only
26 of 256 byte values occur, and `gzip` extracted approximately that and nothing
more.

The English row deserves a note. Its per-character entropy is 4.40 and it
compressed to 1.2%, far below $4.40/8$. The reason is that the text was one
sentence repeated two hundred times, and LZ77 found the repetition — which
per-character entropy cannot see, because it looks at one character at a time. That
is Section 33.1.2's conditional entropy: with context, the real entropy of this
particular source is near zero.

## Lossy compression

Everything above is **lossless** — the original is recovered exactly.

JPEG, MP3 and video codecs are **lossy**: they discard information deliberately,
choosing what a human will not notice. Frequencies outside hearing, colour detail
the eye resolves poorly, small differences between adjacent frames.

The ratios are much larger — ten to a hundred times, against two or three for
lossless text — because they are not bound by Section 33.1.2's theorem. The
theorem bounds encodings that preserve the message, and a lossy codec is sending a
different, simpler message.

Which is why you must never lossily compress data whose exact values matter, and
why repeatedly re-encoding a JPEG degrades it: each pass discards more.

Next: the thing no compressor can do.
