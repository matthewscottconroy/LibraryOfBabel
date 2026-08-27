# Key Concepts

**Voltage, not value.** A digital circuit holds a physical quantity. Any value it
is said to hold is supplied by a convention applied from outside.

**Threshold and noise margin.** A binary convention designates a band of voltages
as 0 and another as 1, with a forbidden gap between. The width of that gap is the
noise margin, and it is what allows a disturbed signal still to be read
correctly.

**Signal restoration.** Because only two states are legitimate, each stage of a
digital circuit can regenerate a clean signal rather than passing degradation
along. This is why error does not accumulate over a long computation, and it is
the property analogue systems lack.

**Bit.** One resolved binary distinction — the answer to one yes-or-no question.
The word is a contraction of "binary digit", coined by John Tukey and put into
print by Claude Shannon in 1948.

**The doubling rule.** *n* bits distinguish $2^{n}$ possibilities, because each
additional bit splits every existing case in two. Growth is multiplicative, not
additive.

**The $2^{n}$ − 1 boundary.** If one of the $2^{n}$ patterns is spent on zero, the largest
remaining value is $2^{n}$ − 1. Eight bits give 256 patterns and a maximum of 255.

**Encoding.** A rule assigning meanings to patterns — formally, a function from
patterns to values. An encoding must be agreed in advance by writer and reader;
nothing in a pattern can announce which encoding governs it.

**Silent mismatch.** Applying the wrong encoding produces a wrong value, not an
error. Both sides behaved correctly; only the agreement was violated, and the
machine has no access to intent.

**Mojibake.** The visible garbling that results from a character-encoding
mismatch, as when UTF-8 bytes are read under a single-byte encoding and `café`
becomes `cafÃ©`. The bytes are undamaged; only the interpretation was wrong,
which is why such damage is often reversible.

**Byte.** A group of eight bits, and the unit in which memory is addressed on
essentially every machine in current use.

**Fixed width.** Values occupy a size decided in advance. This buys
constant-time addressing and finite arithmetic hardware, and costs a bounded
range of representable values.

**Overflow policy.** When a result exceeds the representable range a system must
refuse, saturate, or wrap. Java wraps for `int` and `long`, which is why
arithmetic near the range boundary rolls over silently rather than raising an
exception.

**Endianness.** The convention for the order in which the bytes of a multi-byte
value are laid out. Big-endian places the most significant byte first;
little-endian places the least significant byte first. Neither is superior; both
ends must agree. Java specifies big-endian for its own data formats regardless
of the underlying hardware.

**Operations belong to the agreement.** There is no neutral arithmetic beneath an
encoding. "Add one" means something different to an integer encoding, a
floating-point encoding, and a color encoding, because each defines its own
operations. This is what a type declaration in Java actually announces.
