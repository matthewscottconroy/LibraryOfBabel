# 15.4 Error Detection and the FCS

Chapter 6 established that the channel corrupts bits. This section is about
detecting it, and about the important thing Ethernet deliberately does not do.

## The escalating ladder

**Parity** — one bit set so the total count of ones is even (or odd, by agreement).
Detects any **single**-bit error; detects no double-bit error at all, since two
flips restore the parity. Chapter 2 §2.4 met it as ASCII's eighth bit. Cheap and
weak.

**Checksum** — sum the data words and transmit the sum. Detects more than parity and
has a specific weakness: **it is insensitive to reordering**, because addition is
commutative. Swap two words and the sum is unchanged. IP's header checksum
(Chapter 24 §24.2) is a one's-complement sum of this kind, and it is deliberately
weak because it is recomputed at every hop and speed mattered more than strength.

**Cyclic redundancy check** — treat the message as the coefficients of a polynomial,
divide by a fixed generator polynomial, and transmit the remainder. Detects far more
than either, is cheap in hardware, and is what Ethernet uses.

## How a CRC works

The message is a polynomial over GF(2) — arithmetic modulo 2, where addition is XOR
and there are no carries. The bit string `1101` is *x*³ + *x*² + 1.

The transmitter:

1. Appends *n* zero bits to the message (*n* = the generator's degree).
2. Divides by the generator polynomial, using XOR in place of subtraction.
3. Transmits the message with the **remainder** in place of the appended zeros.

The receiver divides the whole received frame by the same generator. **If the
remainder is zero, no error was detected.** If it is non-zero, the frame is
corrupted.

The mechanism is a shift register and a few XOR gates — a handful of transistors,
operating at line rate with no buffering, which is why it was chosen in 1980 and why
it remains in silicon at 400 Gb/s.

Ethernet uses **CRC-32**, generator polynomial

$$x^{32} + x^{26} + x^{23} + x^{22} + x^{16} + x^{12} + x^{11} + x^{10} + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1$$

usually written `0x04C11DB7`. The polynomial is not arbitrary; it was selected for
its detection properties.

## What CRC-32 detects

The guarantees, which are stronger than intuition suggests:

- **All single-bit errors.**
- **All double-bit errors**, given a suitable generator.
- **All odd numbers of bit errors**, because the generator has *x*+1 as a factor.
- **All burst errors up to 32 bits** — any contiguous run of corrupted bits shorter
  than the CRC itself.
- **99.99999998% of longer bursts** — the probability an arbitrary corruption
  produces a zero remainder is 2⁻³² ≈ 2.3 × 10⁻¹⁰.

The burst guarantee is the valuable one, because Chapter 6 §6.2 established that
real-world errors are **bursty** — impulse noise corrupts a run of bits, not a
scattered few. A code designed for random independent errors would be poorly matched
to the channel; a CRC is designed for exactly this.

**What it does not do: correct anything.** It is a detection code, and the frame is
discarded.

## Why Ethernet detects and does not correct

A deliberate decision, and it is Chapter 1 §1.4's recurring tradeoff in its clearest
form.

**Correction** requires enough redundancy to identify *which* bits are wrong — far
more than detection, which needs only to know that something is. A code correcting
*t* errors needs roughly 2*t* times the redundancy of one detecting *t*.

**The decision depends on the cost of retransmission:**

| Environment | Error rate | Retransmission cost | Choice |
|---|---|---|---|
| LAN | 10⁻¹² or better | microseconds | **Detect** |
| Wi-Fi | 10⁻³ to 10⁻⁵ | milliseconds, plus contention | **Both** |
| Long-haul optical | low but high volume | milliseconds | **Correct** (FEC) |
| Deep space | high | **40 minutes** | **Correct**, heavily |

On a LAN, errors are rare and retransmission is nearly free, so spending bandwidth
on correction would be waste. On a deep-space link, retransmission takes forty
minutes and correction is the only viable answer. Both are correct engineering, and
the difference is entirely in the retransmission cost.

Chapter 7 §7.3 noted that modern high-speed Ethernet has added **forward error
correction** — Reed-Solomon RS(544,514) in 400GBASE-R — precisely because at those
rates and reaches the raw error rate has risen enough to make correction worthwhile
again. The tradeoff moved because the channel did.

## The silent discard, and its diagnostic consequence

When the FCS fails, the frame is **discarded silently**. No error message, no
notification to the sender, no request for retransmission. Ethernet has no
link-layer retransmission at all.

Recovery, if any, is the transport layer's problem — TCP notices the missing data and
retransmits (Chapter 37 §37.3), or UDP does not and the application copes.

The consequence for diagnosis is important:

> **A link with a low rate of corruption shows no symptom except an error counter
> and degraded throughput.**

Chapter 3 §3.3's Mathis relation quantifies the degradation: 1% loss caps a single
TCP stream at under 2 Mb/s on an 80 ms path regardless of link capacity. So a
marginal cable produces:

- A link that is **up**, at **full negotiated speed**
- Every dashboard **green**
- `ping` succeeding
- Throughput at a fraction of expectation
- And a CRC error counter incrementing, which nobody looks at

**Reading interface error counters is therefore the diagnostic**, and it is why
Chapter 65 §65.1 makes them a table. The counters are the only visible evidence of a
fault that otherwise presents as "the network is slow".

## Reading the counters

| Counter | Meaning | Likely cause |
|---|---|---|
| **CRC / FCS errors** | Frame arrived corrupted | Cable, connector, EMI, failing transceiver |
| **Alignment errors** | Corrupted **and** not a whole number of bytes | Usually duplex mismatch |
| **Runts** | Under 64 bytes | Collision, or duplex mismatch |
| **Giants** | Over the maximum | Unexpected 802.1Q tag, or jumbo mismatch |
| **Input errors** | Aggregate of the above | Look at the breakdown |
| **Output drops** | Egress queue full | **Congestion, not a physical fault** |

The last row is the one most often misread. **Output drops are not errors.** They
mean the queue was full when a packet arrived, which is congestion — Chapter 13
§13.3's normal behaviour — and the remedy is capacity or queue management, not a
cable.

CRC errors and output drops appearing together suggests two separate problems.

## Where else CRCs appear

Once you recognise the mechanism you will find it everywhere: in every Ethernet
frame, in 802.11 frames, in USB packets, in PCI Express, in SATA, in every disk
sector, in ZIP and PNG files, in QR codes, and in the ITU-T checksums used
throughout telecommunications.

Different generator polynomials — CRC-8, CRC-16, CRC-32, CRC-64 — chosen for the
expected error patterns and the required strength, and all the same mechanism.

## A caution about relying on it

Two, and both matter in practice.

**A CRC detects accidental corruption, not deliberate modification.** An attacker who
alters the payload simply recomputes the CRC. It provides **integrity against
accident**, not against intent, and Chapter 57 §57.2 makes this distinction the basis
of why cryptographic MACs exist.

**A frame that passes the FCS is not guaranteed correct end to end.** The FCS covers
one hop. A store-and-forward switch verifies the FCS, then **recomputes it** on
egress — so corruption occurring *inside* the switch, in its memory or its
forwarding path, is covered by a freshly computed valid FCS and is invisible to the
receiver.

This is not hypothetical. Stone and Partridge's 1998–2000 measurements of real
Internet traffic found that end-to-end checksum failures occurred at rates far above
what link-layer error rates predicted, and traced a substantial fraction to
middleboxes and host software rather than to the wire.

It is one of the reasons TCP's checksum exists despite Ethernet's CRC, and it is the
end-to-end argument (Chapter 23 §23.4) applied to integrity: **only an end-to-end
check verifies end-to-end correctness.**

## What breaks here

**Incrementing CRC errors on a link that is up at full speed.** Physical layer.
Chapter 6's four impairments, and the cable, connector, transceiver or EMI source
that produced them.

**Alignment errors with CRC errors.** Usually duplex mismatch (Chapter 66 §66.2).

**Output drops mistaken for a cable fault**, or CRC errors mistaken for congestion.
Different counters, different causes, different remedies.

**Assuming the FCS protects the payload from tampering.** It does not, at all.

**Assuming a clean FCS means clean data.** It means clean on that hop, computed
after whatever the previous device did to it.

> **Network+ note.** Objective 5.2 expects CRC errors, runts, giants and their
> interpretation. The three things to carry: **CRC detects and does not correct**;
> **a failed frame is discarded silently, so the counter is the only evidence**; and
> **output drops are congestion, not corruption** — which is the distinction that
> most often separates a correct diagnosis from a wasted cable replacement.
