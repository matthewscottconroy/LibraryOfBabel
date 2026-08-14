# 50.1 Leased Lines and the Digital Hierarchy

A leased line is a circuit you rent and nobody else uses, and for fifty years it was what
"a connection between two sites" meant.

## What made it different

| | **Leased line** | **Internet access** |
|---|---|---|
| Shared | **no** | **yes** |
| Rate | **fixed, guaranteed, symmetric** | best effort, variable |
| Latency | **fixed and low** | variable |
| Endpoints | **two, specified** | **anywhere** |
| Billing | **per circuit, per month, by distance** | by capacity |
| Availability | **contractual, with penalties** | usually none |

The distance term is the one that dates it. A leased line's price rose with the geographic
separation of its endpoints, because the carrier was genuinely reserving capacity along a
route — and this is Chapter 13 §13.1's circuit switching, priced honestly.

> You were not buying bandwidth. You were buying the exclusive right to a path, and its
> cost was the cost of not selling that path to anyone else.

## The hierarchy, from the sample upward

Everything here derives from one decision made in the 1960s: how to digitise a telephone
call (Chapter 12 §12.3).

$$\underbrace{4 \text{ kHz}}_{\text{voice}} \rightarrow \underbrace{8{,}000 \text{ samples/s}}_{\text{Nyquist}} \times \underbrace{8 \text{ bits}}_{\text{PCM}} = \boxed{64 \text{ kb/s}}$$

That 64 kb/s channel is the DS0, and it is the atom of the entire telephone network. Every
rate in this section is a multiple of it.

### The North American hierarchy

| Level | Composition | Rate |
|---|---|---|
| **DS0** | one voice channel | **64 kb/s** |
| **DS1 / T1** | **24 × DS0 + 8 kb/s framing** | **1.544 Mb/s** |
| DS2 | 4 × DS1 + overhead | 6.312 Mb/s |
| **DS3 / T3** | **28 × DS1** | **44.736 Mb/s** |

**The T1 arithmetic is worth doing once:**

$$24 \times 64 \text{ kb/s} = 1{,}536 \text{ kb/s}, \quad +\ 8 \text{ kb/s framing} = 1{,}544 \text{ kb/s}$$

One framing bit per 193-bit frame, 8,000 frames per second. The frame is 24 eight-bit
samples plus one bit, and that single bit carries the frame alignment pattern that lets the
receiver find the channel boundaries.

### The European hierarchy

| Level | Composition | Rate |
|---|---|---|
| **E1** | **32 timeslots × 64 kb/s** | **2.048 Mb/s** |
| E2 | 4 × E1 | 8.448 Mb/s |
| **E3** | **16 × E1** | **34.368 Mb/s** |

E1 uses 32 timeslots of which 30 carry voice: timeslot 0 carries framing and timeslot 16
carries signalling. So the signalling is in a channel of its own rather than stolen from the
voice channels.

### And the incompatibility is a real historical accident

The United States standardised on 24 channels in 1962 because that is what the available
transistor technology could multiplex reliably. Europe standardised later, on 30, having
better components and choosing a cleaner design — a dedicated signalling channel instead of
robbed bits.

Neither is wrong. They are simply different, and:

| Consequence | |
|---|---|
| **International circuits need conversion** | at every boundary, forever |
| **Equipment markets split in two** | and remained split |
| **Rates never align** | 1.544 and 2.048 have no useful common multiple |
| **Price lists still show both** | in 2020s carrier catalogues |

> Sixty years later, every transatlantic voice circuit still crosses a boundary between two
> incompatible framing conventions that exist because of what a 1962 multiplexer could do.
> Chapter 22's OSI lesson has a sibling here: standards adopted early enough to matter are
> adopted before you know enough to get them right.

## Robbed-bit signalling, and why 56 kb/s

**A detail with a long shadow.**

T1 carries no separate signalling channel, so the on-hook/off-hook state had to go
somewhere. The answer was to steal the least significant bit of every sixth frame from each
voice channel.

For voice this is inaudible — one bit of eight, one frame in six, on a companded sample.
For data it is fatal, because you cannot know which bit was stolen.

So a data circuit on a T1 with robbed-bit signalling gets 7 bits per sample: 56 kb/s.

> **This is the same 56 kb/s as V.90's** (Chapter 49 §49.1), from the same cause. The
> number that defined dial-up's ceiling was set by a signalling decision made for analogue
> telephony three decades earlier.

**Clear-channel signalling** — moving signalling out of band entirely, which **SS7** did —
restored the full 64 kb/s, and by then 56 kb/s had been designed into a great deal of
equipment.

## Plesiochronous, and why that was a problem

The T1/E1 hierarchy is plesiochronous — "almost synchronous". Each multiplexer runs on
its own clock, close to the nominal rate but not locked to it.

Which means the rates do not quite match, and the multiplexer must insert **stuffing bits**
to make up the difference, recording in the overhead how many it inserted.

**And that has an expensive consequence:**

```
   To extract one DS0 from a DS3:

   DS3 ──▶ demux to 7 × DS2 ──▶ demux to 4 × DS1 ──▶ demux to 24 × DS0
                                                          │
                                                    take one, then
                                                    remultiplex everything
```

You must demultiplex the entire hierarchy to reach one channel, and multiplex it all back
up. For a carrier dropping a single T1 at each of twelve towns along a route, that is
twelve complete demux/remux cycles, each with its own equipment, power, floor space and
failure modes.

> This is the specific problem SONET was designed to solve (§50.2), and the solution — a
> single network-wide clock — is why "synchronous" is the word in the name.

## What remains

Leased lines have not disappeared; they have changed shape.

**Still in service:**

- T1/E1 for legacy PBX trunks, alarm circuits and SCADA — and they are being switched
  off, which is stranding equipment in the same way as Chapter 46 §46.2's 3G shutdown
- Point-to-point circuits where a guarantee is genuinely required — trading, broadcast
  contribution feeds, medical imaging
- **The pricing model**, which survives into Ethernet private line and wavelength services

**Replaced by:**

| Old | New |
|---|---|
| T1 / E1 | **Ethernet private line (EPL) at 10 Mb/s–10 Gb/s** |
| Fractional T1 | **committed rate on a shared port** |
| T3 | **1 GbE** |
| Multiple leased lines | **MPLS L3VPN (§50.4), then SD-WAN (Chapter 51 §51.2)** |

Ethernet private line is the modern leased line, and it kept the properties that mattered:
point to point, symmetric, guaranteed rate, a service-level agreement with penalties. What
it dropped was the distance-based pricing and the 64 kb/s granularity.

> **The requirement never went away.** Organisations that need a deterministic circuit still
> buy one; they buy it framed as Ethernet rather than as 24 telephone calls.

## What breaks here

A T1 showing errored seconds and no outage. Marginal line, a bad repeater, or a clocking
problem. Check which end is the clock source — two ends both configured as master produces
slips.

Slips on a circuit that otherwise looks clean. **Clock mismatch.** In a synchronised
network there is exactly one traceable reference, and something is not following it.

An E1 delivered where a T1 was ordered, or the reverse. **International circuits.** Someone
must convert, and the conversion is not free of framing implications.

A data circuit delivering 56 kb/s per channel instead of 64. **Robbed-bit signalling.**
Ask for clear channel.

**Alarms in the wrong direction.** T1/E1 alarms — AIS, RAI, LOF, LOS — indicate where the
fault is relative to you, and reading them correctly localises the fault to a span without a
site visit. They are among the better-designed diagnostics in telecommunications.

**A legacy circuit's renewal price rising sharply.** The carrier is trying to retire the
technology. This is a business signal, not a technical one, and the answer is to plan the
migration rather than to negotiate.

> **Network+ note.** Objective 1.2 and 2.1 touch WAN links. Over-learn: a T1 is 1.544 Mb/s
> and an E1 is 2.048 Mb/s; **a leased line is dedicated and not shared**; **a CSU/DSU
> terminates a T1** (Chapter 51 §51.1); and **the DS0 is 64 kb/s.** The rates are examinable
> and worth being able to derive rather than recall.
