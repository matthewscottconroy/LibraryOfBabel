# Chapter 40 — The People

Ross Finlayson, Timothy Mann, Jeffrey Mogul and Marvin Theimer. **RFC 903** (1984),
Reverse ARP.

The first answer to "how does a host learn its own address", and its limitations —
returns only an address, cannot cross a router — are what the next two protocols
successively removed. A useful thing to fail at, because the failures were specific and
each one named a requirement.

**Bill Croft and John Gilmore.** **RFC 951** (1985), BOOTP.

**The two decisions that mattered:** run over **UDP/IP** rather than a link-layer broadcast,
so a relay could forward it — and return the mask, gateway and boot file, not just an
address.

BOOTP's purpose was diskless workstations, which needed enough configuration to reach a
TFTP server and download an operating system (Chapter 36 §36.3). The whole of DHCP's
packet format, its ports, and its relay mechanism come from this document, and the
inheritance is why a BOOTP relay works for DHCP unchanged.

**John Gilmore** is better known for co-founding the Electronic Frontier Foundation and for
the observation that *"the Net interprets censorship as damage and routes around it"* —
which is a comment on Chapter 29's forwarding model, from someone who had built part of it.

**Ralph Droms (1955–2023).** **DHCP** — RFC 1531 (1993), and the definitive **RFC 2131**
(1997), which he authored and then chaired the working group for.

**The contribution is the lease.** BOOTP had solved configuration delivery; **Droms solved
allocation**, and the mechanism that makes dynamic allocation safe is the bounded grant with
renewal.

Its design is unusually careful about failure, and §40.2's renewal timers are the
evidence: T1 at 50%, T2 at 87.5%, and only then expiry. A protocol that continues working
for hours after its server has failed is not an accident; it is a deliberate choice to
make the common failure survivable.

Droms also worked extensively on DHCPv6 (RFC 3315, later 8415) — and DHCPv6's decision
**not** to supply a default gateway (§40.1) is his working group's, on the reasoning that
the router is the authority on whether it is a router. Defensible, correct, and the
cause of a great deal of confusion ever since.

He remained active in IETF work until shortly before his death in 2023.

**Ted Lemon.** The ISC DHCP implementation, which for twenty-five years was what most of
the Internet ran, and much of the DHCP failover protocol work.

ISC DHCP reached end of life in 2022, replaced by **Kea** — and the transition is worth
knowing about, because an enormous amount of documentation and institutional knowledge still
assumes the old server. Kea's decision to store leases in a database rather than a flat
file turns redundancy into a solved problem borrowed from elsewhere, which is a good
instinct.

**Michael Patrick.** **RFC 3046** (2001), the relay agent information option — option
82 of §40.4.

The idea is that the relay knows something the server cannot: which physical port the
client is on. Adding it to the request lets a server assign by location, which service
providers needed for per-subscriber assignment.

And it turned out to be the foundation of DHCP snooping's binding table — a security
mechanism built on a field added for provisioning. Chapter 18 §18.3's Dynamic ARP
Inspection depends on it, which means the defence against ARP spoofing rests, ultimately,
on a 2001 option for cable operators.

**The Cisco engineers who implemented DHCP snooping.** Not a standards contribution — it is
a vendor feature that became universal — and it is one of the more effective security
mechanisms in this book because of its cost profile.

> One line per switch, one trusted port, and an entire class of accident and attack
> disappears.

**Compare with DNSSEC** (Chapter 39 §39.4) or **BCP 38** (Chapter 27 §27.2): mechanisms that
are more principled, more thoroughly specified, and far less deployed — because they cost
more and their benefit is shared. DHCP snooping costs almost nothing and the benefit is
entirely local, and it is consequently everywhere.

The unnamed engineers of every rogue DHCP incident. Worth acknowledging, because §40.4's
accidental cases are the common ones and none of them involve malice.

A home router plugged into a wall socket to get more ports is the canonical case: it is
a switch, a router, a firewall and **a DHCP server**, and its LAN ports serve
`192.168.0.0/24` to anyone who asks. The person who plugged it in was solving a real
problem — not enough ports — and had no reason to know.

Which is the argument for DHCP snooping being on by default rather than configured after
the first incident, and it is the same argument as Chapter 19 §19.3's BPDU Guard: the
failure is caused by a reasonable action taken by someone without the knowledge to predict
it, so the defence must not depend on that knowledge existing.
