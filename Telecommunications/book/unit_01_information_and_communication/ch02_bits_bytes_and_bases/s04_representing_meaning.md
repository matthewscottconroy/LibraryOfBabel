# 2.4 Representing Meaning

We now have bits, and we can write them compactly. What we do not yet have is any
reason for a bit pattern to *mean* anything, and this section is about the answer,
which is both simpler and more consequential than students usually expect.

The answer is: **it means whatever the two parties agreed it means, and nothing
else.** There is no fact of the matter about what `01000001` denotes. Given an
agreement that we are exchanging ASCII text, it is the letter `A`. Given an
agreement that we are exchanging unsigned integers, it is 65. Given an agreement
that we are reading an IPv4 header's fifth octet, it is a type-of-service byte
with particular subfields. The bits are identical in all three cases. The
difference is entirely in the reader.

This is the deepest recurring principle in the book, and it is worth stating in
its general form because we will apply it at every layer:

> A protocol field is a **location plus an agreement**. Change either and the
> meaning changes. A mismatch produces not an error but confident nonsense.

## The history of one agreement: text

Watching character encoding evolve is the cheapest way to see the principle in
action, because the same problem was solved four times with progressively better
answers.

**Baudot (1870, revised as ITA2 in 1930).** Émile Baudot's telegraph code used
five bits per character: 2⁵ = 32 combinations, which is not enough for 26 letters
plus 10 digits plus punctuation. The solution was **shift codes** — two reserved
combinations that switched the receiver between "letters mode" and "figures
mode," so each of the remaining 30 combinations meant two different things
depending on state.

This is our first example of a **stateful protocol**, and it demonstrates the
classic hazard of one: if the shift character is corrupted in transit, every
subsequent character is misinterpreted until the next shift. A single lost bit
corrupts an unbounded amount of downstream data. That failure mode — *state
divergence between sender and receiver* — will reappear in Chapter 37 as a TCP
sequence number problem and in Chapter 19 as a spanning tree topology problem.
Baudot's name survives in the **baud**, the unit of symbol rate we meet in
Chapter 4.

**ASCII (1963).** Seven bits, 128 characters, designed by an ANSI committee under
Bob Bemer. The design is full of deliberate cleverness that rewards a look: digits
`0`–`9` are 0x30–0x39, so masking off the low nibble converts a digit character to
its numeric value. Uppercase `A` is 0x41 and lowercase `a` is 0x61 — they differ
in exactly one bit, bit 5, so case conversion is a single OR or AND. The first 32
codes are control characters, including several — `SOH`, `STX`, `ETX`, `EOT`,
`ACK`, `NAK`, `SYN` — that are pure telecommunications: start of header, start of
text, end of text, acknowledge, negative acknowledge, synchronous idle. ASCII was
designed for the wire, not the page.

The eighth bit was originally a **parity bit**, which is the simplest possible
error detection: set it so the total number of 1s is even (or odd, by agreement).
A single-bit error changes the parity and is detected. A double-bit error is not.
Chapter 15's CRC is the industrial-strength descendant of this idea.

**The code page catastrophe (1980s–1990s).** Once the eighth bit was freed from
parity duty, 128 more slots became available — and roughly every country filled
them differently. ISO 8859-1 for Western Europe, 8859-5 for Cyrillic, Windows-1252
which is *almost but not quite* 8859-1, and dozens of others. The same byte value
0xE9 was `é` in one agreement, `Щ` in another, and a box-drawing character in a
third.

The result was two decades of mojibake — text rendered as garbage because the
reader's agreement differed from the writer's. Notice again: no error was
signalled. The bytes arrived perfectly. The transmission was flawless. The
*agreement* was mismatched, and the failure surfaced as content rather than as a
fault. This is exactly the endianness failure of §2.3 and exactly the VLAN
mismatch of Chapter 20, in a different costume.

**UTF-8 (1992).** Designed by Ken Thompson and Rob Pike, reportedly sketched on a
placemat in a New Jersey diner in September 1992, and it is a small masterpiece.
It is a variable-length encoding of Unicode: 1 byte for ASCII, 2–4 bytes for
everything else. Its properties are worth listing because they show what good
protocol design looks like:

- **ASCII compatibility.** Any valid ASCII file is a valid UTF-8 file. Deployment
  cost for the existing world: zero.
- **Self-synchronising.** Continuation bytes always start with the bits `10`, and
  leading bytes never do. Drop into the middle of a UTF-8 stream and you can find
  the next character boundary by scanning at most three bytes. Compare Baudot,
  where a lost shift corrupts everything after it.
- **No embedded nulls.** A UTF-8 string never contains a zero byte except as a
  genuine null, so C string handling survives.
- **Order-preserving.** Byte-order sort equals code-point sort.

UTF-8 is now over 98% of the web. It won on properties, not on committees, and it
is the encoding this book assumes throughout.

## The same bits, four meanings

To make the principle concrete, take the thirty-two bits

```
11000000 10101000 00001010 01000110
```

and read them under four different agreements:

- **As an IPv4 address:** `192.168.10.70`.
- **As an unsigned 32-bit integer, big-endian:** 3,232,238,150.
- **As an unsigned 32-bit integer, little-endian:** 1,175,103,680.
- **As four ASCII characters:** `À¨` followed by a newline and `F` — mostly
  unprintable, because those byte values are not ASCII text.
- **As two IEEE 802.1Q VLAN tag halves, or as part of a MAC address, or as a
  fragment of a JPEG:** whatever the surrounding agreement says.

None of these readings is more correct than the others. Correctness is a property
of the *pair* — bits and agreement — never of the bits alone.

## Where the agreement comes from

If meaning is agreement, the practical question becomes: how does the receiver
know which agreement to apply? Networking answers this in three ways, and you will
see all three repeatedly.

**By position.** "The first six bytes of an Ethernet frame are the destination
address." No marker; the layout is fixed by the standard and both ends have read
it. Fast, compact, and completely inflexible — you cannot add a field without
breaking every existing implementation.

**By a type field.** "Bytes 12–13 tell you how to read everything after them."
The EtherType 0x0800 means IPv4, 0x86DD means IPv6, 0x0806 means ARP. This is
**self-describing** framing, and it is how layering is physically implemented:
each layer's header contains a field naming the protocol of the layer above.
Chapter 23 makes this explicit.

**By negotiation.** "Let's agree on the encoding before we start." TLS negotiates
a cipher suite; HTTP negotiates content type and encoding; autonegotiation on an
Ethernet port settles speed and duplex. Flexible, and it introduces a new failure
class — negotiation failure, and worse, *successful negotiation of a bad
agreement*, which is what a downgrade attack is (Chapter 62) and what duplex
mismatch is (Chapter 66).

## What breaks here

Every failure in this section has the same shape and none of them announce
themselves:

- **Encoding mismatch** — text arrives as mojibake, a JSON payload fails to parse,
  a filename becomes unreadable. Symptom: content is wrong, transport is fine.
- **Endianness mismatch** — a port number reads as 20480, an address as
  70.10.168.192. Symptom: values that are structurally valid and semantically
  absurd.
- **Type field mismatch** — a frame's EtherType says IPv4 and the payload is
  something else; the receiver parses garbage as a header. Symptom: malformed
  packet counters incrementing with no other explanation.
- **Silent negotiation to the wrong agreement** — both ends "succeed" and then
  behave differently. Symptom: works, but badly, and consistently.

The diagnostic principle that covers all four: **when data is structurally valid
but semantically wrong, suspect the agreement, not the channel.** No amount of
cable testing will find these. Chapter 63 formalises the instinct; you can start
using it now.
