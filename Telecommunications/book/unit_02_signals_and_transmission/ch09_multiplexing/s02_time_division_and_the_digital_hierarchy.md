# 9.2 Time-Division Multiplexing and the Digital Hierarchy

Give each conversation the whole channel, in turn, for a brief slot. Everybody uses
all the frequencies, at different times.

This is the architecture of the entire digital telephone network, it produced the
numbers that carrier contracts still quote, and it is the direct ancestor of the
SONET systems in Chapter 50.

## The mechanism

Divide time into **frames**. Divide each frame into *N* **slots**. Assign slot *i*
to conversation *i*, permanently, for the duration of the call.

```
   frame 1              frame 2              frame 3
  ┌──┬──┬──┬──┐        ┌──┬──┬──┬──┐        ┌──┬──┬──┬──┐
  │A │B │C │D │        │A │B │C │D │        │A │B │C │D │
  └──┴──┴──┴──┘        └──┴──┴──┴──┘        └──┴──┴──┴──┘
```

The multiplexer takes one sample from each input in turn and interleaves them; the
demultiplexer separates them by counting slots. Provided both ends agree on where
frames begin and how many slots there are, the arrangement is trivially correct.

Everything then depends on **synchronisation**, which is why the technique is
called *synchronous* TDM and why the framing bit in §9.2's T1 exists.

## From voice to the DS0

Chapter 4 §4.2 derived the number and Chapter 12 §12.2 traces its consequences;
here is the chain in one place.

Telephone speech is band-limited to about 3.4 kHz. Nyquist requires at least 6.8 kHz
sampling; **8 kHz** was chosen to leave a transition band for realisable
anti-aliasing filters. Each sample is quantised to **8 bits**, using a
logarithmic companding curve (µ-law in North America and Japan, A-law elsewhere)
that gives more resolution to quiet passages than a linear quantiser would.

$$8{,}000 \ \text{samples/s} \times 8 \ \text{bits} = 64{,}000 \ \text{b/s}$$

That is the **DS0**, and it is the atom from which everything above is built.

## T1 and E1

**T1** (North America, Japan) multiplexes **24 DS0s**:

- 24 channels × 8 bits = 192 bits per frame
- Plus **1 framing bit** = 193 bits per frame
- 8,000 frames per second (one sample from each channel per 125 µs)
- 193 × 8,000 = **1,544,000 b/s**

The framing bit is the synchronisation mechanism. Across a sequence of frames it
carries a repeating pattern that the receiver hunts for to establish where frames
begin. It is one bit in 193 — 0.5% overhead — and without it the whole scheme
fails, because a receiver that has lost frame alignment delivers every channel's
samples to the wrong channel.

**E1** (most of the rest of the world) multiplexes **32 timeslots** of which 30
carry voice:

- 32 slots × 8 bits = 256 bits per frame
- 8,000 frames per second
- 256 × 8,000 = **2,048,000 b/s**
- Slot 0 carries framing; slot 16 carries signalling for all 30 channels

The two systems are incompatible, the incompatibility is a genuine historical
accident, and it persists in carrier price lists and international circuit
provisioning to this day. An international call crossing the boundary requires
conversion, and the conversion is a real piece of equipment.

Note the structural difference beyond the channel count: **E1 has a dedicated
signalling slot; T1 originally stole bits from the voice channels** (robbed-bit
signalling, taking the least significant bit of every sixth frame). Robbed-bit
signalling is why a "64 kb/s" T1 channel often delivers only 56 kb/s for data — the
robbed bit is inaudible in voice and fatal to data, so data channels had to give up
the low bit entirely. This is the origin of the 56 kb/s figure that appears
throughout older networking material.

## The plesiochronous hierarchy

Above T1 and E1 the multiplexing continues recursively, and the resulting hierarchy
is called **plesiochronous** — "almost synchronous" — for a reason worth
understanding.

| North America | Rate | Channels | Europe | Rate | Channels |
|---|---|---|---|---|---|
| DS0 | 64 kb/s | 1 | E0 | 64 kb/s | 1 |
| DS1 (T1) | 1.544 Mb/s | 24 | E1 | 2.048 Mb/s | 30 |
| DS2 | 6.312 Mb/s | 96 | E2 | 8.448 Mb/s | 120 |
| DS3 (T3) | 44.736 Mb/s | 672 | E3 | 34.368 Mb/s | 480 |
| — | — | — | E4 | 139.264 Mb/s | 1,920 |

Look at DS2: 4 × 1.544 = 6.176 Mb/s, but the standard is 6.312. The extra
136 kb/s is not a rounding error.

**The reason is that the tributaries are not synchronised to one another.** Each T1
comes from a different exchange with its own clock, and those clocks differ by a
few parts per million. A multiplexer combining four of them cannot simply
interleave their bits, because they arrive at slightly different rates.

The solution is **bit stuffing**: the multiplexer runs slightly faster than the sum
of the tributaries and inserts dummy bits as needed, flagging where it did so in
overhead bits. The demultiplexer removes them.

This works, and it has a serious operational consequence: **to extract a single
DS0 from a DS3 you must demultiplex the entire hierarchy**. The stuffed bits'
positions are only known level by level, so there is no way to reach into a DS3 and
pull out one voice channel without unwrapping DS3 → DS2 → DS1 → DS0 and rebuilding
everything.

For a carrier wanting to drop a few channels at each town along a route, this is
ruinous — every intermediate site needs a full multiplexer stack.

That problem is what SONET and SDH were designed to solve, by making the whole
network **synchronous** to a common atomic reference so that a tributary's position
within a frame is fixed and known. Chapter 50 §50.2 covers it, and the capability it
buys — **add-drop multiplexing** — is the entire reason SONET displaced the
plesiochronous hierarchy.

## Statistical versus synchronous, previewed

Synchronous TDM assigns slot *i* to conversation *i* permanently. If conversation
*i* has nothing to send, **its slot is transmitted empty**. The capacity is
reserved and wasted.

For voice this was acceptable: a telephone call occupies its channel continuously
for its duration, so the reservation matches the usage. For data it is
catastrophic, because data is bursty — and §9.3 is the argument that follows.

**Statistical TDM** — also called asynchronous TDM — abandons the fixed assignment.
Slots are given to whoever has data, and each slot carries an identifier saying
whose it is. That identifier is the seed of the packet header, and the whole of
packet switching grows from it.

## What breaks here

**Loss of frame alignment.** A T1 that has lost frame sync delivers every channel's
data to the wrong channel — all channels corrupted simultaneously, with no partial
degradation. The counters are `LOF` and `LOS`, and carrier documentation is full of
them.

**A clock mismatch producing slips.** If two ends of a TDM link derive their timing
from sources that differ, buffers eventually overflow or underflow and a frame is
repeated or dropped — a **slip**. Slips are inaudible in voice and destroy data,
and they are why carrier networks distribute timing so carefully from a primary
reference source.

**Robbed-bit signalling breaking a data circuit.** A channel provisioned as
"64 kb/s clear" and delivered over a robbed-bit T1 gives 56 kb/s, and the symptom
is data corruption rather than a clean failure.

> **Network+ note.** N10-009 expects T1 and E1 rates and channel counts under the
> WAN objectives, and DS0 as the underlying 64 kb/s unit. The derivation — 8 kHz
> sampling × 8 bits, from Nyquist — is what makes those figures memorable rather
> than arbitrary, and the T1/E1 divergence is worth knowing as a real
> interoperability issue rather than trivia.
