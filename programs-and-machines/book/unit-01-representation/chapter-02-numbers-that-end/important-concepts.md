# Key Concepts

**Number versus numeral.** A number is a quantity; a numeral is a written
pattern denoting it under some agreement. 742, `2110`, and `2E6` are three
numerals for one number.

**Positional notation.** A digit's contribution depends on its position. In base
*b*, positions from the right are worth 1, *b*, $b^{2}$, …, and digits run from 0 to
*b* − 1. The invention that makes arithmetic mechanical.

**Locality of arithmetic.** Positional notation lets a column be handled without
comprehending the whole number, passing only a carry to its neighbour. This is
why arithmetic can be built from circuits.

**Base two.** Positional notation with *b* = 2. Positions are powers of two;
digits are bits. Reading a binary numeral means adding the position values where
a 1 appears — no multiplication table required.

**Powers of two.** $2^{10}$ = 1,024 (hence the kilobyte confusion); $2^{16}$ = 65,536;
$2^{32}$ ≈ 4.29 billion; $2^{64}$ ≈ 1.8 × 1$0^{19}$.

**All-ones is $2^{n}$ − 1.** The largest value in *n* bits, because adding one carries
off the end and leaves $2^{n}$.

**Hexadecimal.** Base sixteen. Because 16 = $2^{4}$, each hex digit corresponds to
exactly four bits, so conversion is regrouping rather than arithmetic. Hex is a
lossless, structure-preserving shorthand for binary; decimal is neither.

**Full adder.** A circuit taking two digits plus a carry-in and producing a digit
plus a carry-out. Chained *n* times, it adds *n*-bit numbers. Nothing in it
understands numbers.

**Modular arithmetic.** Fixed-width addition is addition modulo $2^{n}$. The discarded
carry is worth exactly $2^{n}$, so dropping it subtracts $2^{n}$. Values live on a circle.

**Sign-magnitude.** The intuitive scheme — leading bit as sign, rest as
magnitude. Rejected because it has two zeros and because ordinary addition gives
wrong answers, requiring separate hardware.

**Two's complement.** The convention in universal use. In a *W*-bit word, −*n* is
represented by the pattern for $2^{W}$ − *n*. Because values live on a circle of size
$2^{W}$, subtracting *n* and adding $2^{W}$ − *n* are the same operation — so subtraction
needs no hardware of its own, and one adder serves for both.

**The negative weight.** In *W*-bit two's complement, the leftmost position
carries weight −$2^{W-1}$ instead of +$2^{W-1}$. Everything else is ordinary positional
notation. Reading `11111011` as −128 + 64 + 32 + 16 + 8 + 2 + 1 gives −5.

**Flip and add one.** The mechanical recipe for negation. It works because
flipping gives $2^{W}$ − 1 − *n* and adding one gives $2^{W}$ − *n*.

**The asymmetric range.** *W* bits give −$2^{W-1}$ to $2^{W-1}$ − 1: one more negative than
positive, because one pattern is spent on zero and the remaining odd count cannot
split evenly. Hence `int` runs to −2,147,483,648 but only +2,147,483,647.

**The unpaired minimum.** The most negative value has no positive counterpart,
so negating it returns itself. `Math.abs(Integer.MIN_VALUE)` is negative, by
specification.

**Unsigned versus signed overflow.** Unsigned overflow is the carry falling off
the end. Signed overflow is the carry landing in the sign position and flipping
the result's sign. Signed overflow can only occur when adding operands of like
sign.

**Wrapping as policy.** Java wraps rather than trapping, for speed and because
hashing, checksums, and random number generation depend on it. `Math.addExact`
and its relatives throw on overflow when that is what you need.
