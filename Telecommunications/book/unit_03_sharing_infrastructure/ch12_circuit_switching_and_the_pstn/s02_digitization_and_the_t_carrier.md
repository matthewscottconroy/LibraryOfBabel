# 12.2 Digitisation and the T-Carrier

The single largest improvement in the telephone network's history, and the source
of numbers that still appear in carrier contracts.

## The problem digitisation solved

Chapter 5 §5.1 established the mechanism; here is the consequence in its original
setting.

An analog long-distance circuit needs amplifiers, because attenuation accumulates
(Chapter 6 §6.1). An amplifier multiplies signal and noise together, so noise
accumulates too — and each subsequent span adds fresh noise on top of noise that is
now permanent.

The practical result: a transcontinental analog call in 1930 required exceptional
engineering care, cost a great deal, and still sounded like a transcontinental
analog call. Quality fell off with distance, smoothly, with no clear line anywhere.

Digitisation replaces amplification with **regeneration**. Each repeater decides
which discrete value was sent and emits a clean new signal, discarding the noise
entirely. Chain ten thousand regenerators and the last emits exactly what the first
did.

**Quality becomes independent of distance.** That is the change, and it is why a
digital call from London to Sydney sounds like a local one.

## The DS0, derived

The chain, assembled from Chapter 4 §4.2 and Chapter 9 §9.2 in one place because
this is where it belongs.

**Band-limit the speech.** Telephone-quality speech is filtered to about
300–3,400 Hz. That band carries essentially all the intelligibility, though it
loses enough of the higher formants that some consonants become confusable —
which is why "S" and "F" are hard to distinguish on a telephone and why phonetic
alphabets exist.

**Sample at 8 kHz.** Nyquist requires at least 2 × 3,400 = 6,800 Hz. Eight
thousand was chosen to leave a transition band for realisable anti-aliasing filters
(Chapter 5 §5.2's observation that nothing in engineering sits exactly at the
limit).

**Quantise to 8 bits, logarithmically.** A linear 8-bit quantiser gives poor
resolution to quiet passages, because the step size is fixed while the signal's
amplitude varies enormously. **Companding** compresses the signal before
quantisation and expands it afterwards, so the effective step size is proportional
to amplitude — fine steps for quiet sounds, coarse for loud.

Two incompatible curves:

- **µ-law** in North America and Japan
- **A-law** in Europe and most of the world

Both give roughly the dynamic range of a 12- or 13-bit linear quantiser using
8 bits. An international call crossing the boundary requires conversion, which is
one more entry in the list of T/E incompatibilities.

**Multiply:**

$$8{,}000 \ \text{samples/s} \times 8 \ \text{bits} = 64{,}000 \ \text{b/s}$$

That is the **DS0**, and it is the reference against which every voice codec since
has been measured. G.711 *is* the DS0. G.729 achieving comparable quality at 8 kb/s
is an eightfold improvement over it. Every VoIP bandwidth calculation starts here.

## T1: the first digital carrier

Bell deployed the **T1** carrier system in 1962, initially for short inter-exchange
trunks in Chicago, and its economics are worth stating because they explain why it
happened when it did.

An analog trunk carries one conversation per pair. A T1 carries **24** over two
pairs, using cable already in the ground, with repeaters every 6,000 feet — the
spacing of the existing loading coils, which is not a coincidence. For a telephone
company that owned a great deal of copper and needed more capacity between
exchanges in cities where digging was expensive, the arithmetic was immediate.

The frame, from Chapter 9 §9.2:

- 24 channels × 8 bits = 192 bits
- plus **1 framing bit** = 193 bits per frame
- × 8,000 frames per second = **1,544,000 b/s**

The framing bit carries a repeating pattern across successive frames — the
**framing pattern** — that a receiver hunts for to establish where frames begin.
0.5% overhead, and without it every channel's samples arrive in the wrong channel.

## Robbed-bit signalling, and where 56 kb/s came from

The T1 as originally designed had no separate signalling channel. Supervision — on
hook, off hook, ringing — was carried by **stealing the least significant bit of
every sixth frame** from each voice channel.

For voice this is inaudible: one bit in eight, one frame in six, is a tiny
perturbation of the quantisation. For **data** it is fatal, because a bit is a bit.

So a data circuit over a robbed-bit T1 cannot use all 8 bits. It uses 7, giving

$$8{,}000 \times 7 = 56{,}000 \ \text{b/s}$$

**This is the origin of the 56 kb/s figure** that pervades older networking
material — 56 kb/s leased lines, 56 kb/s DDS, and the "56/64" distinction in carrier
tariffs. A "clear channel" 64 kb/s circuit requires a signalling arrangement that
does not steal bits, which is what **ESF** framing with a separate facility data
link, and later common-channel signalling (§12.3), provided.

Anyone who has wondered why 56 rather than the obvious 64 now knows, and it is a
good example of a number that looks arbitrary being the residue of a specific
engineering decision.

## E1 and the divergence

Most of the world outside North America and Japan uses **E1**:

- 32 timeslots × 8 bits = 256 bits per frame
- × 8,000 frames/s = **2,048,000 b/s**
- **Slot 0** carries framing; **slot 16** carries signalling for all 30 voice
  channels
- 30 usable voice channels

The structural difference matters beyond the channel count: **E1 has a dedicated
signalling slot from the outset**, so it never needed robbed-bit signalling and
never had the 56 kb/s problem. Its channels are clear 64 kb/s by design.

The incompatibility is genuine, expensive and permanent. It requires conversion at
every international boundary; it means equipment is built in two variants; and it
persists in carrier price lists and provisioning systems fifty years later. It is
the standard example of an early divergence that nobody can now afford to correct.

## The hierarchy above

Chapter 9 §9.2 covered the plesiochronous stack and the bit-stuffing that makes DS2
6.312 rather than 6.176 Mb/s. The consequence worth repeating here, because it
motivates Chapter 50:

**Extracting one DS0 from a DS3 requires demultiplexing the entire hierarchy.**
Stuffed-bit positions are known only level by level. For a carrier wanting to drop
a few channels at each town along a route, every intermediate site needs a full
multiplexer stack — which is expensive, occupies floor space, and consumes power.

SONET's synchronous design solved this and gave carriers **add-drop
multiplexing**, which is why it displaced the plesiochronous hierarchy. Chapter 50
§50.2 tells that story.

## The consequences that outlived the technology

Four numbers and one architecture from this section are still load-bearing:

**64 kb/s** is the reference for every voice codec, and the unit in which carrier
circuits are sold.

**1.544 and 2.048 Mb/s** define circuit sizes in contracts, and "a T1's worth of
bandwidth" remains a unit of thought for a generation of engineers.

**8 kHz sampling** is why VoIP packets are typically 10 or 20 ms of audio — 80 or
160 samples — which determines the packetisation overhead that Chapter 3 §3.1
computed and Chapter 52 §52.2 budgets for.

**µ-law and A-law** are in every VoIP gateway, converting between the digital
telephone network's representation and whatever the IP side is using.

And the architecture: **digitise at the edge, transport digitally, regenerate rather
than amplify.** That is what the modern network does, at every scale, and it was
established here.

## What breaks here

**A data circuit delivering 56 rather than 64 kb/s.** Robbed-bit signalling on a
circuit provisioned as clear channel. Historical, and still met on legacy links.

**Companding mismatch.** A µ-law endpoint talking to an A-law one produces audible
distortion — the signal is intelligible and unpleasant. A gateway must convert, and
one that does not is a real fault on international circuits.

**Loss of frame alignment.** Every channel corrupted simultaneously, with no partial
degradation, because the demultiplexer is delivering each channel's bits to the
wrong channel.

**Clock slips.** Two ends deriving timing from sources that differ cause buffers to
overflow or underflow, repeating or dropping a frame. Inaudible in voice,
destructive to data, and the reason carrier networks distribute timing so carefully.

> **Network+ note.** Objective 1.2's WAN technologies expect T1/E1 rates and channel
> counts, and DS0 as the underlying unit. The derivation — Nyquist, 8 kHz, 8 bits —
> makes them memorable, and **the 56 kb/s origin is worth knowing** because it
> explains a figure that otherwise looks like a typo.
