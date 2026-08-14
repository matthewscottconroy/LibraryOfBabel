# Chapter 65 — The People

This chapter is a catalogue rather than a history, and most of its faults have no
discoverable author. What it has instead is a set of people who documented failure modes
systematically when nobody else was, and a few whose designs created the faults.

**W. Richard Stevens (1951–1999).** The person who taught the industry to read packets.

*TCP/IP Illustrated, Volume 1* (1994) did something no other book had done: it showed the
actual packets, from actual captures, for every mechanism it described, and then explained why
they looked as they did.

> Before Stevens, the protocols were documented in specifications and the behaviour was
> folklore. After him, an engineer could read a capture and know whether what they were
> seeing was correct — which is the entire premise of Chapter 64 §64.3 and of this chapter's
> diagnostic method.

He wrote *UNIX Network Programming*, the three-volume *TCP/IP Illustrated*, and *Advanced
Programming in the UNIX Environment*, and all of them remain in print and in use thirty
years later.

**Stevens died in 1999, aged 48.** The second edition of Volume 1, by Kevin Fall, was published
in 2011 and is the current reference.

> His method is the transferable part: do not describe the protocol, show it working, then
> show it failing. **This chapter is organised on that principle**, and Chapter 64's
> recommendation to build a personal library of capture signatures is his practice.

**Radia Perlman**, again — and here for a fault rather than for a solution.

Spanning tree's convergence behaviour (§65.2) is a direct consequence of the 1985 design's
constraints (Chapter 19), and Perlman has been consistently clear that the 30-to-50-second
convergence was a compromise she would not have chosen with better hardware.

> The faults in §65.2 — the 45-second delay without PortFast, the topology change storms, the
> unexpected root — are the visible edges of a design that had to work on 1985 processors with
> no configuration. **Knowing why they exist makes them recognisable**, which is the argument
> for this book's historical material generally.

**Rich Seifert**, for the duplex mismatch.

Seifert's *The Switch Book* and his long participation in the IEEE 802.3 working groups make
him the person who documented most thoroughly why auto-negotiation behaves as it does — and
why the forced/auto combination fails in the specific way it does.

> The mechanism is worth knowing because it is counter-intuitive: a forced full-duplex port
> does not advertise anything, so the auto-negotiating port at the other end sees no
> advertisement, falls back to its default — half duplex — and links. The link comes up. The
> configuration looks reasonable at both ends. And the throughput collapses under load.

Seifert's larger point, made repeatedly in standards discussions, was that a negotiation
protocol with a "do not participate" option will produce mismatches — which is the same
argument as Chapter 58's about cryptographic downgrade, in a different decade and a different
layer.

The Bellcore and Telcordia reliability engineers, collectively, for the physical layer.

**The GR-series documents** — GR-326 for connectors, GR-1435 for fibre, and the wider body of
Bellcore reliability practice — are where the failure modes in §65.1 were catalogued, in
enormous detail, by people whose employer had to maintain telephone plant for forty years.

> **The "clean the connector first" advice is not folklore.** It is the conclusion of decades
> of documented field failures, and the fibre inspection standards exist because someone
> counted.

And the same tradition produced the environmental and physical failure catalogues that
§65.1's last table summarises — heat, water, vibration, rodents — each of which is in a
Bellcore document with statistics attached.

**The unnamed field engineers.**

This chapter's content came overwhelmingly from people who found these faults repeatedly and
told someone — on mailing lists, in vendor knowledge bases, in NANOG presentations, in the
comments of bug reports.

> "Check the transceiver power", "swap the patch lead", "look at the topology change counter",
> "it's the intermediate certificate", "check the clock" — **none of these has an author.**
> Each is the distilled result of a large number of people losing an afternoon and then
> writing it down.

**Which is Chapter 63 §63.4's argument, demonstrated:** the accumulated record is what makes
the second occurrence cheap, and this chapter is that record for the faults common enough to
have been written down many times.

## What this chapter's shape shows

**Two observations.**

**The faults cluster at the boundaries.** Not within a layer but between two elements that
disagree — duplex, native VLAN, MTU, mask, trunk allowed lists, certificate chains, clock
skew. Almost every fault in this chapter is two things that were configured independently
and do not match, which is Chapter 55 §55.1's argument for golden configurations and
Chapter 70's argument for generating configuration from a single source.

And the diagnostic value is concentrated in a few free observations. Interface counters,
transceiver power, the MAC table, the topology change counter, `ss -tlnp`, the clock. All
read-only, all instant, and between them they resolve the majority of this chapter.

> The engineer who is quick at this is not the one who knows more faults. It is the one who
> reads those six things first, every time, and that is a habit rather than knowledge.
