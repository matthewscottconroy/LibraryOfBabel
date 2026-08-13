# Chapter 2 — Important Concepts

**Information as resolved uncertainty** *(§2.1)* — The amount of information in a
message is a property of the set of messages that might have been sent, not of the
message itself. Formally, identifying one of *n* equally likely possibilities costs
log₂ *n* bits. This framing is why address sizes, prefix lengths, and host counts
are all the same arithmetic.

**Bit** *(§2.1)* — The unit of information: one binary decision, one halving of the
possibility space. Named by John Tukey, adopted by Shannon in 1948. Base 2 is not
a computing convention but a physical optimum: two-state systems maximise the
noise margin available for a given energy budget.

**Self-information** *(§2.1)* — The information content of a single outcome with
probability *p*, equal to −log₂ *p* bits. Rare outcomes carry more information than
common ones; this is the basis of all compression and of the variable-length codes
Morse and Vail found empirically in 1838.

**Landauer's principle** *(§2.1)* — Erasing one bit dissipates at least *kT* ln 2
joules as heat (≈ 2.9 × 10⁻²¹ J at room temperature). Ties information
irreversibly to thermodynamics, and connects to the thermal noise that sets every
channel's capacity in Chapter 4.

**Place value** *(§2.2)* — Positional notation: each digit contributes its value
times a power of the base determined by position. Identical in decimal, binary and
hexadecimal; only the base changes.

**Octet place values** *(§2.2)* — 128, 64, 32, 16, 8, 4, 2, 1. The single most
useful row of numbers in network engineering. Memorise it.

**Contiguous subnet mask** *(§2.2)* — A mask in which all 1 bits precede all 0
bits. Only nine octet values are legal (0, 128, 192, 224, 240, 248, 252, 254,
255). Required by RFC 4632; any other value is a typo.

**Block size rule** *(§2.2)* — Within the octet where a prefix boundary falls,
block size = 256 − mask octet value = 2^(host bits in that octet). Subnet
boundaries occur at multiples of the block size. This one relationship is most of
Chapter 26.

**Bitwise AND** *(§2.2)* — 1 only when both inputs are 1. ANDing an address with
its mask extracts the network address. This is the operation a router performs, in
hardware, on every packet it forwards.

**Bitwise OR / XOR / NOT** *(§2.2)* — OR with the inverted mask produces the
broadcast address. XOR reveals which bits two values differ on, and underlies
longest-prefix match (Chapter 29), CRC (Chapter 15), and stream ciphers
(Chapter 58). NOT of a subnet mask is a **wildcard mask** in Cisco access lists
(Chapter 60).

**Nibble** *(§2.3)* — Four bits; exactly one hexadecimal digit. The reason hex is
the notation of choice for machine data.

**Byte / octet** *(§2.3)* — Eight bits; exactly two hex digits. "Octet" is the
standards-document word, chosen because "byte" historically meant different widths
on different machines.

**Hexadecimal** *(§2.3)* — Base 16. Its entire justification is that 16 = 2⁴, so
digit boundaries coincide with bit boundaries and conversion is a lookup rather
than a calculation.

**Endianness** *(§2.3)* — The order in which the bytes of a multi-byte value are
stored or transmitted. **Big-endian** puts the most significant byte first and is
mandated for all Internet protocol header fields as **network byte order**.
**Little-endian** is used internally by x86 and ARM. Named by Danny Cohen in 1980
after *Gulliver's Travels*.

**Network byte order** *(§2.3)* — Big-endian, required in every protocol header
field in this book. Little-endian hosts byte-swap on every send and receive.

**Parity bit** *(§2.4)* — The simplest error detection: one bit set so the total
count of 1s is even or odd by agreement. Detects any single-bit error, misses any
double-bit error. The ancestor of the CRC in Chapter 15.

**Shift code / stateful encoding** *(§2.4)* — Baudot's mechanism for getting 60
meanings from 32 five-bit combinations, by switching the receiver between modes.
Introduces the failure class of *state divergence*: one corrupted shift character
misinterprets everything after it. The same hazard recurs in TCP sequence numbers
and spanning-tree topology.

**UTF-8** *(§2.4)* — Thompson and Pike's 1992 variable-length Unicode encoding.
ASCII-compatible, self-synchronising, null-free, and order-preserving — a case
study in what good protocol design buys.

**Field = location + agreement** *(§2.4)* — The central principle: a bit pattern
has no intrinsic meaning. Meaning comes from a shared agreement about how to read
it. A mismatch produces confident nonsense rather than an error, which is the
hardest class of fault to diagnose.

**Self-describing framing** *(§2.4)* — A type field that tells the receiver how to
interpret what follows (EtherType 0x0800 = IPv4). The physical implementation of
layering; developed in Chapter 23.
