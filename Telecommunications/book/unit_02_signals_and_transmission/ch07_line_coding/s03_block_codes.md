# 7.3 Block Codes

The second family. Instead of guaranteeing a transition in *every* bit, block codes
map groups of data bits onto slightly larger groups of transmitted bits, choosing
the mapping so that the transmitted patterns have the properties we need.

The overhead is the size difference, and the whole history of the family is a
history of that difference shrinking.

## 4B/5B

Map every 4 data bits onto a 5-bit code word. There are 16 possible inputs and 32
possible outputs, so we choose 16 of the 32 — specifically, the 16 with the best
transition properties.

The selection rule: **no code word has more than one leading zero, and none has
more than two trailing zeros.** Concatenate any two valid code words and you can
never get more than three consecutive zeros. Clock recovery is therefore guaranteed
within three bit periods, by construction, whatever the data.

Part of the table:

| Data | Code | | Data | Code |
|---|---|---|---|---|
| 0000 | 11110 | | 1000 | 10010 |
| 0001 | 01001 | | 1001 | 10011 |
| 0010 | 10100 | | 1010 | 10110 |
| 0011 | 10101 | | 1011 | 10111 |
| 0100 | 01010 | | 1100 | 11010 |
| 0101 | 01011 | | 1101 | 11011 |
| 0110 | 01110 | | 1110 | 11100 |
| 0111 | 01111 | | 1111 | 11101 |

**Overhead: 5/4, so 25%.** Efficiency 80%. To carry 100 Mb/s of data you signal at
125 Mbaud.

**The spare code words are not wasted.** Sixteen of the 32 patterns are unused by
data, and several are assigned as **control symbols**:

| Symbol | Code | Meaning |
|---|---|---|
| `I` | 11111 | Idle |
| `J` | 11000 | Start of stream, part 1 |
| `K` | 10001 | Start of stream, part 2 |
| `T` | 01101 | End of stream, part 1 |
| `R` | 00111 | End of stream, part 2 |
| `H` | 00100 | Halt / error |

This is a substantial benefit and it is easy to miss. **A control symbol cannot be
confused with data**, because its bit pattern is not a legal data code word. That
solves the frame-delimiting problem — how does a receiver know where a frame starts
— without needing an escape mechanism or a reserved data value. Chapter 15 §15.1's
framing problem is partly solved here, at the physical layer, for free.

The remaining unused patterns are **invalid**, and receiving one is a detectable
error. Free error detection, at no additional cost.

100BASE-TX combines 4B/5B with MLT-3 (§7.2): 4B/5B supplies the transition density,
MLT-3 reduces the fundamental frequency by a factor of four, and the result fits in
Cat5's 100 MHz. FDDI used 4B/5B with NRZI over fibre.

## 8B/10B

Widmer and Franaszek's 1983 IBM code, and one of the most successful pieces of
coding engineering ever done. It appears in 1000BASE-X, Fibre Channel, PCI Express
1.0 and 2.0, SATA, DisplayPort, and InfiniBand.

Map 8 data bits onto 10 transmitted bits. Same 25% overhead as 4B/5B, and
substantially better properties.

**Running disparity** is the mechanism that makes it work, and it is worth
understanding because the idea generalises.

Each 8-bit input maps to **two** possible 10-bit outputs: one with more `1`s than
`0`s (positive disparity), one with more `0`s than `1`s (negative disparity). Some
inputs map to a single balanced output used in both cases.

The encoder maintains a **running disparity** state — a single bit tracking whether
the accumulated excess of `1`s over `0`s is currently positive or negative — and
for each symbol chooses whichever of the two encodings **corrects** the current
imbalance.

The result: the running sum never strays more than a small bounded amount from
zero, ever, for any input whatsoever. **DC balance is guaranteed by construction,
not statistically**, and the maximum run length is 5 bits.

Structurally the code is 5B/6B for the low five bits plus 3B/4B for the high three,
which makes the implementation a pair of small lookup tables rather than one large
one — an engineering choice that mattered a great deal in 1983.

**Control symbols** again: 8B/10B defines twelve `K` characters, of which `K28.5`
(the comma) is the important one. Its bit pattern `0011111010` or `1100000101`
contains a sequence that **cannot occur anywhere in any concatenation of data code
words**, including across code word boundaries. A receiver scanning a bit stream
can therefore find the comma and thereby establish where the 10-bit boundaries are.

That is **symbol alignment**, and it is a genuinely hard problem solved elegantly:
a receiver that starts listening mid-stream, with no idea where symbols begin,
recovers alignment by waiting for a comma.

**Error detection** comes free from two independent mechanisms: an invalid code
word, and a disparity violation. Between them they catch all single-bit errors and
a high proportion of multi-bit ones — at the physical layer, before the CRC.

## 64B/66B

At 10 Gb/s, 25% overhead means signalling at 12.5 Gbaud to carry 10 Gb/s. In 2002
that was the difference between an achievable design and an unachievable one.

64B/66B (IEEE 802.3ae) abandons the guarantee-by-construction approach entirely:

- Take **64 bits** of data.
- Prepend a **2-bit synchronisation header**: `01` means the payload is all data;
  `10` means it contains control information.
- **Scramble** the 64-bit payload with a self-synchronising scrambler using the
  polynomial *x*⁵⁸ + *x*³⁹ + 1.

**Overhead: 2/64, so 3.125%.** Efficiency 96.9%. To carry 10 Gb/s you signal at
10.3125 Gbaud.

Two things to note.

**The sync header is never scrambled**, and it is always `01` or `10` — never `00`
or `11`. So a transition is guaranteed at least every 66 bits, at the header, and
the receiver achieves block alignment by hunting for a bit position at which the
header is consistently `01` or `10` over many blocks. That is the same alignment
trick as the comma, done statistically.

**The payload's properties are statistical.** A scrambler makes long runs
*improbable* rather than impossible. With a 58-bit polynomial the probability of a
run long enough to matter is astronomically small for real data — but it is not
zero, and a deliberately chosen input can defeat it.

This is a real consideration rather than a theoretical one. Standards specify test
patterns designed to stress the scrambler, and there is a known class of
adversarial "killer packet" that can, in principle, produce pathological line
conditions. In practice the risk is accepted, because 3.1% overhead against 25% is
not a close call.

## 256B/257B and beyond

At 200 and 400 Gigabit Ethernet the same logic applied again. 256B/257B takes four
64B/66B blocks, notes that their four sync headers carry only 2 bits of real
information between them, and compresses them into a single bit plus, where needed,
a small amount of positional information.

**Overhead: 1/256, so 0.4%.** Efficiency 99.6%.

Modern high-speed Ethernet also layers **forward error correction** on top —
Reed-Solomon RS(544,514) in 400GBASE-R — which adds about 6% of its own. That is
not line coding; it is error correction, buying reach and tolerance rather than
timing. But it means the total encoding overhead at 400G is higher than the 0.4%
figure suggests, and it is worth knowing that the two mechanisms are separate and
serve different purposes.

## The ladder, and what it means

| Code | Year | Overhead | Efficiency | Guarantee |
|---|---|---|---|---|
| Manchester | 1983 | 100% | 50% | Construction |
| 4B/5B | 1995 | 25% | 80% | Construction |
| 8B/10B | 1998 | 25% | 80% | Construction |
| 64B/66B | 2002 | 3.1% | 96.9% | Statistical |
| 256B/257B | 2017 | 0.4% | 99.6% | Statistical |

Fifty per cent of the wire spent in 1983; less than half a per cent in 2017.

The engineering did not get *better* at providing the same guarantee more cheaply.
It changed what guarantee it was providing — from "no run longer than *n*, ever" to
"a run longer than *n* is so improbable we will accept the risk" — and it did so
because the receivers became good enough that the residual risk was smaller than
other risks already being accepted.

That is a recurring shape in engineering: a constraint that looks fundamental turns
out to be a design assumption, and the assumption is renegotiated when its cost
changes.

## What breaks here

**Comparing wire rate with data rate carelessly.** 1000BASE-X signals at
1.25 Gbaud to carry 1 Gb/s; 10GBASE-R signals at 10.3125 Gbaud to carry 10 Gb/s.
A datasheet quoting one and a specification quoting the other are not in conflict.

**Symbol misalignment on an 8B/10B link.** Produces a stream of invalid code words
rather than corrupted data, which is a distinctive counter signature — "code
violations" rather than CRC errors.

**Assuming a scrambled link has a hard run-length guarantee.** It does not, and
the standards' test patterns exist because someone thought about this.

> **Network+ note.** Not examined. The transferable point is that **the wire
> signalling rate exceeds the data rate**, by an amount set by the encoding, and
> that this is one of the several contributions to why a "1 Gb/s" link delivers
> 940 Mb/s of payload (Chapter 3 §3.1's other contributions are the frame headers).
