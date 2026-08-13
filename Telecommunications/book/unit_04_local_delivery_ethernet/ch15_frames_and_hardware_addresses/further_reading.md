# Chapter 15 — Further Reading

## Primary sources

**Metcalfe, R. M. & Boggs, D. R. (1976). "Ethernet: Distributed Packet Switching for
Local Computer Networks." *Communications of the ACM* 19(7): 395–404.**
The paper. Ten pages, and the frame format it describes is the one in your capture
window today. §4's discussion of the addressing scheme and §6's of the CRC are
directly relevant here; Chapter 16 uses the rest.

**Peterson, W. W. & Brown, D. T. (1961). "Cyclic Codes for Error Detection."
*Proceedings of the IRE* 49(1): 228–235.**
The CRC. Requires comfort with polynomial arithmetic over GF(2) and repays it: the
detection guarantees of §15.4 are *proved* here rather than asserted.

**Stone, J. & Partridge, C. (2000). "When the CRC and TCP Checksum Disagree."
*ACM SIGCOMM Computer Communication Review* 30(4): 309–319.**
**Read this one.** Measured evidence that a passing link-layer CRC does not imply
correct data, with the failure rates and the causes traced. Twelve pages, accessible,
and it will change how much you trust a green interface counter.

**IEEE 802.3-2022, Clause 3 (MAC frame structure) and Clause 4 (MAC operation).**
The authoritative frame definition, including the padding rules, the length/type
discrimination and the FCS computation. Freely available six months after
publication through the IEEE GET program.

**IEEE 802-2014, Clause 8 (MAC addresses).**
Where the I/G and U/L bits are defined, and where the address administration model
is set out.

## Books

**Spurgeon, C. & Zimmerman, J. (2014). *Ethernet: The Definitive Guide*, 2nd ed.
O'Reilly.**
Exactly what it says. Chapters 1–3 cover the frame, addressing and operation with
more detail than any other accessible source, and the book is unusually good on the
historical reasons for design decisions that look arbitrary.

**Perlman, R. (1999). *Interconnections*, 2nd ed. Addison-Wesley.**
Chapter 2 on data link issues, and the discussion of addressing throughout.
Perlman's treatment of why flat addressing does not scale — §15.2's argument — is
sharper than most, and she is characteristically direct about design decisions she
considers mistakes.

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1.* Addison-Wesley.**
Chapter 2 covers the link layer and shows real frames byte by byte. Its method —
state a mechanism, then show the actual trace — is what §15.3's decoding exercise is
imitating.

**Lin, S. & Costello, D. J. (2004). *Error Control Coding*, 2nd ed. Prentice Hall.**
Chapter 4 for cyclic codes, if you want §15.4's guarantees derived rather than
tabulated.

**Petzold, C. (2022). *Code*, 2nd ed. Microsoft Press.**
The chapters on error detection build the idea from parity upward with no
prerequisites, and are the place to start if §15.4's polynomial arithmetic was
opaque.

## Reference

**IEEE OUI registry, standards-oui.ieee.org.**
Searchable, free, authoritative. Look up the OUIs from your own captures — the
exercise takes two minutes and the habit is genuinely useful when an unexpected
device appears.

**Wireshark's `manuf` file.**
The same data, bundled, which is why Wireshark shows vendor names automatically. Worth
knowing where it comes from and that it can be stale.

**IANA EtherType registry.**
The authoritative list, though in practice the dozen values in §15.3's table cover
almost everything you will meet.

## Applied

**Any switch vendor's interface counter documentation.**
Cisco's, Juniper's and Arista's are all freely available and all explain what each
counter means on *their* platform, which differs in detail. §15.4's table is
generic; the platform documentation is what you consult during an incident.

**Wireshark's `frame` and `eth` dissector documentation.**
Useful for understanding what the tool is and is not showing you — in particular
that the preamble and SFD are absent, and that the FCS is often stripped by the
capture interface before Wireshark sees it.

## Tools

**`tcpdump -x` or `-xx`.** The exercise in §15.3 — decoding a frame by hand from
hex, then checking against Wireshark's dissection — is worth doing once properly.
`-xx` includes the link-layer header, which `-x` omits.

**A CRC calculator, or fifteen lines of Python.** Implementing CRC-32 from the
generator polynomial takes an afternoon and makes §15.4's mechanism concrete.
Corrupting a bit and watching the remainder become non-zero is more convincing than
any proof.

**Lab 04** in this book's [labs/](../../../labs/) directory works through frame
decoding, OUI lookup, and the MAC table observation directly.

## For the certification-minded

Objective 1.4 covers the Ethernet frame and MAC addressing. Objective 5.2 covers
interface errors — CRC, runts, giants — and their interpretation.

Four things worth over-learning:

1. **64 bytes minimum, 1,518 maximum, 1,522 tagged**, and that a runt is never
   normal.
2. **EtherType values** `0800`, `0806`, `86DD`, `8100`. You will read these in
   captures constantly.
3. **An odd first byte means multicast.** Immediately useful and rarely taught.
4. **Output drops are congestion; CRC errors are corruption.** Different causes,
   different remedies, and confusing them is the most common diagnostic error at
   this layer.
