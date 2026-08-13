# Chapter 15 — Frames and Hardware Addresses

A wire carries voltage. It does not carry messages, and it certainly does not carry
*your* message as distinct from anybody else's. Everything in this chapter follows
from taking that seriously.

Suppose two machines share a cable and one has 40 kilobytes to send. It could
simply transmit for the requisite number of microseconds and stop. Three things go
wrong immediately.

**Nobody else can transmit for the entire duration.** On a 10 Mb/s shared segment
that is 32 milliseconds of exclusive use, which is an eternity if someone else has
one urgent packet to send. Chopping the transmission into pieces creates gaps in
which others can interleave.

**A single error destroys everything.** If one bit is corrupted 39 kilobytes in, the
receiver must discard the whole transmission and the sender must repeat all of it.
With small pieces, a corrupted piece costs only that piece.

**The receiver cannot find the boundaries.** When does the message start? When does
it end? A continuous voltage stream has no punctuation, and without punctuation
the receiver cannot even tell that a transmission has occurred.

So we chop the data into pieces and we wrap each piece in a structure that says
where it begins, who it is for, who sent it, what it contains, and whether it
arrived intact. That structure is a **frame**, and this chapter builds one, field
by field, from the reasons each field exists.

## The address problem, and the choice that shaped everything

Framing is mechanical. Addressing is a design decision with consequences that run
the length of this book, and Ethernet made a choice in 1980 that is worth
examining before we meet its details.

Every Ethernet interface ever manufactured has a 48-bit address, assigned at the
factory, globally unique, and unrelated to where the device is. The first 24 bits
identify the manufacturer — the **Organisationally Unique Identifier**, purchased
from the IEEE — and the remaining 24 are the manufacturer's serial number. There
are 2⁴⁸ ≈ 281 trillion possible addresses, which in 1980 seemed inexhaustible and
has so far proved to be.

This is a **flat** address space. The address tells you nothing about location. It
is a name, not a coordinate.

Compare a postal address, which is hierarchical: country, city, street, number.
That structure is what makes postal sorting possible — a sorting office in Sydney
does not need a list of every address in France, only a rule that says "France goes
in that bag." Hierarchy permits **aggregation**, and aggregation is what makes
large systems scale.

MAC addresses permit none. There is no rule that summarises a set of them, because
two adjacent addresses may be on opposite sides of the planet. Therefore any device
that must know where a MAC address lives must know it *individually*, by
observation, in a table — and the table has a hard size limit set by the switch's
memory.

That single property is the reason Chapter 17's switches have finite address
tables, the reason a broadcast domain cannot grow indefinitely, the reason VLANs
exist, and ultimately the reason Chapter 24's IP addresses had to be invented with
hierarchy built in. **Flat addressing works beautifully at local scale and cannot
be made to work globally**, and the entire structure of Units VI and VII is a
response to that fact.

Which raises the obvious question: why choose flat addressing at all? Because it
requires no administration. Plug in a device and it works, with a unique address,
with nobody assigning anything. In 1980, for a network in one building, that was
exactly the right trade — and it is still exactly the right trade, which is why
we have both address systems and why Chapter 18 exists to reconcile them.

## What this chapter does

§15.1 develops framing from the three problems above, covering frame delimiting,
the minimum and maximum size questions, and why Ethernet's minimum frame is
64 bytes — a number that encodes the speed of light and the maximum length of a
cable last manufactured decades ago.

§15.2 covers the MAC address in detail: its structure, the OUI and how to look one
up, the unicast/multicast and universal/local bits, broadcast, and MAC address
randomisation on modern mobile devices, which broke a great deal of network
management practice.

§15.3 walks the Ethernet II frame field by field — preamble, SFD, destination,
source, EtherType, payload, FCS — and contrasts it with the IEEE 802.3 length-field
variant, explaining how a receiver distinguishes them.

§15.4 covers error detection: parity, checksums, and the CRC-32 that Ethernet
actually uses, including what it does and does not detect, and why the frame is
*discarded* rather than corrected.

## By the end you will be able to

- Explain why data is framed, giving three independent reasons.
- Decode a MAC address: identify the OUI, look up the vendor, and read the
  multicast and locally-administered bits.
- Identify every field of an Ethernet frame in a hex dump.
- Explain the origin of the 64-byte minimum and the 1,518-byte maximum, and what
  a runt and a giant are.
- Explain what a CRC detects, what it misses, and why a failed FCS results in a
  silent discard — and therefore which counter to read when frames are vanishing.
- Explain, in terms of aggregation, why flat addressing cannot scale globally.

## Where this sits in the argument

This is the first chapter in which we build something that works. Chapter 16 adds
the rules for sharing the medium, Chapter 17 adds the device that makes it
efficient, and Chapter 18 reconciles this addressing scheme with the global one
still to come.
