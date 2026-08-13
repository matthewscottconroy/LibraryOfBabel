# 15.1 Why Frames Exist

Two machines share a cable and one has 40 kilobytes to send. The obvious approach —
transmit continuously until finished — fails for three independent reasons, and each
reason produces a different property of the frame.

## Reason one: fairness on a shared medium

On a 10 Mb/s shared segment, 40 kilobytes takes

$$\frac{40{,}960 \times 8}{10 \times 10^6} = 32.8 \ \text{ms}$$

of exclusive occupancy. For 32.8 milliseconds, nobody else can transmit anything —
including a station with one urgent 64-byte packet.

Chopping the transmission into pieces creates **gaps between them**, and in those
gaps other stations can interleave. The medium is shared in time at a granularity
the sender chooses, rather than in units of "whatever anyone happens to want to
send".

This is why there is a **maximum** frame size, and it is fundamentally a fairness
constraint rather than a technical one. Ethernet's 1,500-byte payload limit means no
station monopolises the medium for more than 1.2 ms at 10 Mb/s, or 12 µs at
1 Gb/s.

The same argument reappears in Chapter 52 §52.3 as **serialisation delay**: on a
slow link, a large frame in front of a voice packet delays it by the large frame's
transmission time, which is why low-bandwidth links use fragmentation and
interleaving.

## Reason two: bounding the cost of an error

Chapter 6 established that errors happen. Suppose the bit error rate is 10⁻⁶ — one
bit in a million, which is poor for modern copper and realistic for a marginal link.

**One 40 kB transmission** is 327,680 bits. The probability that it arrives intact is

$$(1 - 10^{-6})^{327{,}680} \approx 0.72$$

so 28% of attempts fail, and each failure costs the whole 40 kB retransmitted.

**Twenty-seven 1,500-byte frames** are 12,000 bits each. Each frame arrives intact
with probability

$$(1 - 10^{-6})^{12{,}000} \approx 0.988$$

so about 1.2% of frames fail, and each failure costs 1,500 bytes retransmitted.

The expected retransmission volume differs by more than an order of magnitude, and
the difference grows as the error rate rises. **Small units bound the cost of a
single error.**

This is why there is error detection *per frame* (§15.4) rather than per
transmission, and it is the same argument that Chapter 13 §13.1 made for Baran's
message blocks.

## Reason three: the receiver must find the boundaries

The most fundamental of the three, and the one that is easiest to overlook.

A wire carries a continuous stream of voltage transitions. Chapter 7's line coding
ensures there are always transitions, so the receiver can recover a clock and read
bits — but a stream of bits is not a message. Where does one message start? Where
does it end?

Without punctuation, the receiver cannot even determine that a transmission has
occurred, let alone extract it. **Framing is punctuation**, and every framing method
answers the same question: how does the receiver recognise a boundary?

## How frames are delimited

Four general techniques, and Ethernet uses two of them.

**By a length field.** The header states how many bytes follow. Simple, and it fails
badly: a corrupted length field means the receiver reads the wrong number of bytes
and loses synchronisation for everything after. IEEE 802.3's original frame used a
length field, and Ethernet II's EtherType occupies the same position (§15.3
explains how a receiver tells them apart).

**By a delimiter pattern.** A reserved bit sequence marks the boundary. The problem
is that the pattern might occur in the data, which requires either **bit stuffing**
(insert a 0 after five consecutive 1s, so the flag `01111110` cannot occur in data —
HDLC's method) or **byte stuffing** (escape the delimiter when it appears in data —
PPP's method).

**By a code violation.** Use a physical-layer symbol that cannot occur in valid data.
Chapter 7 §7.3 showed 4B/5B and 8B/10B reserving control symbols precisely for this,
and it is the cleanest solution because no data can ever be mistaken for a delimiter.

**By silence.** The gap between transmissions is itself the delimiter.

**Ethernet uses the last two together.** The **preamble** — 7 bytes of alternating
`10101010` followed by the **start frame delimiter** `10101011` — gives the receiver
a pattern to synchronise its clock against and a distinctive ending to mark the start
of real data. And the **interframe gap** — a mandatory 96 bit times of silence,
12 bytes at any rate — separates one frame from the next.

That interframe gap is not decoration. It gives receivers time to process a frame and
prepare for the next, and it is counted in every efficiency calculation in this book
(Chapter 3 §3.1's 1,538 bytes on the wire for a 1,460-byte payload includes it).

## The minimum size, and where 64 bytes comes from

Ethernet's **minimum frame is 64 bytes** — 512 bits — excluding preamble. This is
not a fairness or error-cost decision; it is a consequence of the speed of light and
a cable that has not been manufactured in decades.

The argument, from Chapter 16 §16.2 in advance:

For CSMA/CD to work, a transmitting station must **still be transmitting** when a
collision from the far end of the segment reaches it. If it has already finished, it
will never detect the collision, will assume success, and will not retransmit.

So the frame must occupy the wire for at least one **round-trip propagation time**
across the longest permitted segment.

The original 10BASE5 specification permitted 2,500 m with four repeaters. Signal
velocity in coax is about 2 × 10⁸ m/s, so:

$$t_{\text{one way}} = \frac{2{,}500}{2 \times 10^8} = 12.5 \ \mu s, \qquad t_{\text{round trip}} = 25 \ \mu s$$

At 10 Mb/s, 25 µs is 250 bits. The standard specified **512 bits**, roughly double,
to allow for repeater delays and transceiver latency.

512 bits is 64 bytes, and **that number is still enforced by the switch on your
desk**. A frame shorter than 64 bytes is a **runt** and is discarded. Padding is
added to reach the minimum when the payload is smaller — which is why a 20-byte
payload still occupies a 64-byte frame, and why small-packet efficiency is as poor
as Chapter 3 §3.1 computed.

There is no CSMA/CD on a modern full-duplex switched link. The constraint is
inherited, universally observed, and entirely vestigial — a fossil of a coaxial
cable, preserved because the frame format never changed (Chapter 16 §16.3's lesson
about stable interfaces).

## The maximum size, and jumbo frames

**1,500 bytes of payload**, giving a 1,518-byte frame or 1,522 with an 802.1Q tag
(Chapter 20 §20.2).

The figure was chosen in 1980 as a balance: large enough for reasonable efficiency,
small enough that a station does not monopolise the shared medium, and small enough
that the buffer memory required per frame was affordable at 1980 memory prices. The
last consideration is now irrelevant and the number has not changed.

**Jumbo frames** — up to about 9,000 bytes — raise efficiency from 94.9% to 99.1%
(Chapter 3 §3.1). They are not standardised by IEEE, they are widely supported, and
they carry a specific hazard: **every device on the path must agree**. A single
device that does not produces the black-hole failure of Chapter 66 §66.3 —
connections establish, small packets work, large ones vanish silently.

This is why jumbo frames are common inside data centres, where one team controls
every hop, and essentially absent across the Internet, where nobody does.

## What breaks here

**Runts** — frames under 64 bytes. On a modern link this means a collision (implying
a duplex mismatch), a corrupted frame, or a malfunctioning interface. Never normal.

**Giants and baby giants** — frames over the maximum. Usually an unexpected 802.1Q
tag at an administrative boundary (Chapter 20 §20.2), occasionally a jumbo frame
reaching a device that does not expect one.

**Inconsistent MTU on a path.** Chapter 66 §66.3's black hole, and the most common
consequence of enabling jumbo frames on some devices and not others.

**A missing interframe gap.** Some faulty transceivers transmit back to back; the
receiver drops frames it had no time to process, and the error counters show input
drops with no CRC errors.

> **Network+ note.** Objective 5.2 expects runts and giants as interface error
> types. The two numbers to carry are **64 bytes minimum and 1,518 maximum
> (1,522 tagged)**, and the reason the first exists — a round trip on a 2,500 m coax
> segment — is worth knowing because it explains why small-packet efficiency is so
> poor and why the figure never changed.
