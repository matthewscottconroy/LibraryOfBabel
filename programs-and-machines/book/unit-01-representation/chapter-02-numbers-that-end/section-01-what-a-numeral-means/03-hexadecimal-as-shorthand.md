# Hexadecimal as Shorthand

Here is a 32-bit pattern:

```
11010110101101011010110100001111
```

Now answer a question about it: is bit 19 set?

You cannot, not without counting along with your finger, and you will probably
miscount. The numeral is accurate and completely unusable.

Hexadecimal exists to fix this, and the way it fixes it is worth understanding,
because it is not a matter of taste. There is a mathematical reason base sixteen
works as a shorthand for base two, and the reason is that 16 is $2^{4}$.

## The correspondence

Because 16 = $2^{4}$, exactly four bits are needed to express one hexadecimal digit,
and every group of four bits corresponds to exactly one hex digit — no
overlaps, no remainders, no carrying between groups.

| Binary | Hex | Decimal | | Binary | Hex | Decimal |
|---|---|---|---|---|---|---|
| `0000` | 0 | 0 | | `1000` | 8 | 8 |
| `0001` | 1 | 1 | | `1001` | 9 | 9 |
| `0010` | 2 | 2 | | `1010` | A | 10 |
| `0011` | 3 | 3 | | `1011` | B | 11 |
| `0100` | 4 | 4 | | `1100` | C | 12 |
| `0101` | 5 | 5 | | `1101` | D | 13 |
| `0110` | 6 | 6 | | `1110` | E | 14 |
| `0111` | 7 | 7 | | `1111` | F | 15 |

Sixteen rows, because four bits give sixteen patterns. Ten of them borrow the
decimal digits; the remaining six borrow letters, because we ran out of digits.

The letters are the part beginners dislike, and I understand why — `F` does not
look like a number. But `F` is not a number any more than `9` is a number. Both
are symbols denoting quantities, and we needed six more symbols than
base ten supplies.

## Converting is regrouping

Take that unreadable 32-bit pattern and split it into groups of four, from the
right:

```
1101 0110 1011 0101 1010 1101 0000 1111
  D    6    B    5    A    D    0    F
```

So the pattern is `D6B5AD0F`.

Eight characters instead of thirty-two, and — this is the important part — no
information was lost and no arithmetic was performed. Each group of four bits was
looked up in the table. That is all.

Going back is the same operation reversed. Each hex digit expands to its four
bits. `D6B5AD0F` becomes `1101`, `0110`, `1011`, … and you have the original
pattern, exactly.

Compare this with decimal. To write that pattern in decimal you must actually
compute — divide repeatedly, or sum thirty-two position values — and the answer,
3,602,230,543, tells you nothing at all about which bits are set. You cannot look
at 3,602,230,543 and see that its lowest four bits are all 1s. You can look at
`D6B5AD0F` and see it immediately, because the final `F` is exactly those four
bits.

That is the entire argument for hexadecimal. **It is a lossless, arithmetic-free
compression of binary that preserves the bit structure.** Decimal is neither
arithmetic-free nor structure-preserving.

## Now answer the question

Is bit 19 set?

Number the bits from 0 at the right. Each hex digit covers four bits, so the
rightmost digit holds bits 0–3, the next holds bits 4–7, and so on. Bit 19 falls
in the group covering bits 16–19 — the fifth digit from the right, which is `5`.
Within that group bit 19 is the highest of the four, and `5` is `0101`, whose
highest bit is 0.

So no, bit 19 is not set. That took a few seconds and no finger-counting, which
is what the notation was for. The answer came out "no", and you should be no less
confident in it than if it had come out "yes" — the procedure does not care which
way it lands.

## Where you will meet it

**Colors.** `#FF8800` is three bytes: red `FF` (255, maximum), green `88` (136,
just over half), blue `00` (none). An orange. Written in decimal as
"255, 136, 0" it is no less accurate but the channel boundaries are no longer
visible in the notation itself, and you cannot see at a glance that red is
saturated.

**Memory addresses.** A debugger reports `0x7FFE3A2C` rather than 2,147,367,468.
The hex form makes alignment visible: an address ending in `0` or `8` is
eight-byte aligned, which matters, and which the decimal form hides.

**Byte values.** Any single byte is exactly two hex digits, `00` through `FF`.
This is why hex dumps of files are laid out in pairs — each pair is one byte,
always, with no ambiguity about where one ends.

**Unicode.** Code points are written `U+00E9` for é. Chapter 4 will lean on this.

The `0x` prefix, which you will see constantly, is a convention marking a numeral
as hexadecimal. Java uses it: `0xFF` is 255. Without it, `10` is ambiguous — it
could be ten, or sixteen, or two.

## Octal, briefly

You will occasionally meet base eight, since 8 = $2^{3}$ and three bits make one octal
digit. It was common when machines had word sizes divisible by three, and it
survives mainly in Unix file permissions, where `chmod 755` sets three groups of
three permission bits.

Java writes octal with a leading zero, which is a genuine trap: `010` in Java
source is 8, not 10. If you have ever seen a program mishandle a zero-padded
number, this may be why.

## The thing to hold on to

Hexadecimal is not a third way of writing numbers to be learned alongside binary
and decimal. It is *binary*, regrouped four bits at a time so that human eyes can
cope with it.

When you see `D6B5AD0F`, the useful reflex is not "what decimal number is that".
It is "that is thirty-two bits, and I can see any of them I want".
