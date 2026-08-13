# Chapter 24 — The Internet Protocol

RFC 791, *Internet Protocol*, was published in September 1981, edited by Jon
Postel at USC's Information Sciences Institute. It is forty-five pages long, it has
been amended remarkably little in forty-five years, and it opens with a statement
of scope so restrained that it is almost an apology:

> "The internet protocol provides for transmitting blocks of data called datagrams
> from sources to destinations... There are no mechanisms to augment end-to-end
> data reliability, flow control, sequencing, or other services commonly found in
> host-to-host protocols."

Read that list again. **No reliability. No flow control. No sequencing.** IP will
attempt to deliver your packet. It may fail. It may deliver it twice. It may
deliver it out of order, corrupted, or after an arbitrary delay. It will not tell
you which of these happened. It is, in the standard phrase, a **best-effort**
service, and the phrase is honest rather than euphemistic.

This looks like a protocol that does not do its job. It is in fact a protocol that
does exactly one job and refuses all others, and the refusal is what made it win.

## Why best-effort was correct

Three arguments, and they compound.

**The end-to-end argument** (Chapter 23 §23.4). Reliability implemented in the
network is insufficient for any application that genuinely needs reliability,
because the failure modes that matter are not all in the network. Since the
endpoints must implement it anyway, the network's version is redundant for
correctness and justified only as an optimisation.

**The lowest common denominator argument** (Chapter 23 §23.1). IP must run over
every network technology that exists or will exist. If IP guaranteed delivery, then
every underlying network would have to support that guarantee — and some cannot.
A radio link with a 20% frame loss rate cannot promise delivery at any price. By
promising nothing, IP can run over anything, which is the entire point of the
hourglass.

**The complexity argument.** State in the network is expensive and fragile. A
router that must remember something about every conversation passing through it
cannot scale to the traffic volumes involved, and cannot survive its own reboot
without disrupting every conversation. A router that remembers nothing between
packets can be built out of very fast, very simple hardware, and can fail and
recover without anyone noticing. Chapter 29 shows just how simple the forwarding
decision is; the simplicity is a direct consequence of the refusal.

The applications that need reliability get it from TCP (Chapter 37), on the hosts,
where the end-to-end argument says it belongs. The applications that do not need it
— voice, video, DNS queries, game state — are not forced to pay for it. That
separation is a design decision of the first rank and it is why one protocol serves
both.

## The header, and what its fields reveal

§24.2 walks all twenty bytes. Each field is a small historical argument:

**Version** (4 bits) — the field that made IPv6 possible at all, and the reason a
receiver can tell the two apart before parsing anything else.

**IHL** (4 bits) — header length in 32-bit words, present because of options, and
the reason the header is variable-length and therefore slower to process than it
needed to be. IPv6 fixed this by making the header a fixed 40 bytes.

**Type of Service / DSCP** (8 bits) — originally a vague set of precedence and
delay/throughput/reliability bits that almost nothing honoured, redefined by
RFC 2474 as the Differentiated Services Code Point. This is the field on which all
modern IP quality of service depends, and Chapter 52 uses it.

**Total Length** (16 bits) — capping an IP packet at 65,535 bytes, a limit chosen in
1981 and now occasionally inconvenient at 400 Gb/s.

**Identification, Flags, Fragment Offset** (32 bits) — the fragmentation machinery,
which §24.3 argues was a mistake.

**Time to Live** (8 bits) — originally intended as seconds, universally implemented
as a hop count. §24.4.

**Protocol** (8 bits) — the self-describing type field of Chapter 2 §2.4, one layer
up: 6 means TCP, 17 means UDP, 1 means ICMP.

**Header Checksum** (16 bits) — covering the header only, recomputed at every hop
because the TTL changes. Removed entirely in IPv6 on the grounds that Layer 2 and
Layer 4 both check anyway — a rare instance of a standards body removing something.

**Source and Destination Address** (32 bits each) — the fields Chapters 25 through 28
are about.

## Fragmentation: an instructive mistake

Different link technologies have different maximum frame sizes. Ethernet carries
1,500 bytes of payload; some links carry less. If a 1,500-byte packet must cross a
link with a 576-byte limit, something must give.

IPv4's answer was to let **any router along the path** split the packet into
fragments, which are reassembled only at the final destination. It is a reasonable
design and it has been a persistent source of trouble:

- **Reassembly is expensive and stateful**, at the destination, which must buffer
  fragments and run a timer.
- **Loss of one fragment destroys the whole packet**, multiplying the effective loss
  rate — and the sender has no idea, since it never fragmented anything.
- **Fragments after the first carry no transport header**, so a firewall cannot see
  port numbers, which has produced a long and colourful history of evasion attacks.
- **It interacts badly with everything**: NAT, load balancers, and ECMP hashing all
  struggle with fragments.

IPv6 removed router fragmentation entirely. Only the source may fragment, and
routers that cannot forward a packet send an ICMP "Packet Too Big" instead. This
makes **Path MTU Discovery** mandatory rather than optional — and PMTUD depends on
ICMP being permitted, which many firewall administrators have historically blocked
on the theory that ICMP is dangerous. The result is the **PMTUD black hole**:
connections that establish perfectly and then hang the moment a large packet is
sent. Chapter 34 §34.4 and Chapter 66 both return to it, because it is still, in
2026, one of the most common and most misdiagnosed faults in networking.

## What this chapter does

§24.1 argues the best-effort model properly, including what it costs.

§24.2 walks the header field by field, with a real packet decoded in hex.

§24.3 covers fragmentation, MTU, MSS, and the interactions that make it a problem.

§24.4 covers TTL: its intent, its actual use as a hop count, what generates the
ICMP Time Exceeded message, and how `traceroute` (Chapter 34) exploits it.

## By the end you will be able to

- Explain the best-effort model and give three independent justifications for it.
- Decode an IPv4 header from a hex dump, field by field.
- Compute how a packet of given size will fragment across a link of given MTU, and
  state each fragment's offset and flags.
- Explain the relationship between MTU and MSS, and compute one from the other.
- Explain the PMTUD black hole, recognise its signature, and state the fix.
- Explain what TTL protects against and predict the TTL value observed at a
  destination.
