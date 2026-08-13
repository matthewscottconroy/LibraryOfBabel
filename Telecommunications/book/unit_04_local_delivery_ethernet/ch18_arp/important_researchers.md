# Chapter 18 — The People

**David C. Plummer.** Wrote RFC 826 at MIT in November 1982 — four pages that have
never been revised. The specification's genericity (any hardware type, any protocol
type, explicit length fields) reflects a moment when it was genuinely unclear which
network and which internetwork protocol would win, and Plummer declined to guess. The
generality was used at the time, for Chaosnet and DECnet among others; today it is
vestigial, and the fields are constants.

What the document does not contain is any notion of authentication, and it is worth
being fair about why. ARP was written for a network of a few hundred machines
administered by people who knew each other, on a campus where physical access to the
cable *was* the trust boundary. The protocol is not naive about its threat model; it
has no threat model, because in 1982 the threat did not exist. **The failure is one of
context outliving design**, which is the most common way protocols become insecure.

**Jon Postel (1943–1998) and Joyce K. Reynolds (1952–2015).** Assigned and maintained
the hardware and protocol type numbers that ARP's HTYPE and PTYPE fields index — part
of the vast, unglamorous IANA registry work that made independent implementations
interoperate. Chapter 35 §35.3 returns to their role with port numbers.

**Smoot Carl-Mitchell and John S. Quarterman.** Authored RFC 1027 (1987), *Using ARP
to Implement Transparent Subnet Gateways* — the specification of proxy ARP. It solved
a real transitional problem: hosts whose IP implementations predated subnetting and
therefore believed their whole class network was local. The mechanism is a hack, was
understood as one, and outlived its purpose by three decades. It is a good example of
a transition mechanism that should have had an expiry date.

**Stuart Cheshire (b. 1965).** Apple, and the author of RFC 5227, *IPv4 Address
Conflict Detection* — the formalisation of what gratuitous ARP had been used for
informally. ACD specifies exactly how a host should probe before claiming an address
and how it should behave on discovering a conflict, replacing a folk practice with a
specification. Cheshire also created **Zeroconf/Bonjour** and co-authored the
bufferbloat work (Chapter 13); his consistent interest is in networks that work
without configuration, which is the same interest that produced IPv6's SLAAC.

**Thomas Narten, Erik Nordmark, William Simpson and Hesham Soliman.** The authors of
RFC 2461 (1998) and its revision RFC 4861 (2007) — Neighbor Discovery for IPv6. Their
achievement is less the individual mechanisms than the consolidation: address
resolution, router discovery, prefix advertisement, parameter distribution,
redirection and reachability detection had been six separate mechanisms in IPv4 (ARP,
ICMP router discovery, ICMP redirect, DHCP, and nothing at all for reachability), and
NDP makes them one protocol with a coherent state machine.

**Neighbour Unreachability Detection is Narten's contribution above all**, and it is
the part with the largest operational consequence: it is why IPv6 hosts recover from a
dead first hop without VRRP.

**Erik Nordmark**, separately, is responsible for a great deal of the IPv6 transition
work, and his repeated argument — that transition mechanisms must have failure modes
people can diagnose — is visible in DAD's design.

**Tuomas Aura (b. 1971).** Microsoft Research, and the inventor of **cryptographically
generated addresses** (RFC 3972): binding an IPv6 address to a public key by making
the interface identifier a hash of it, so that ownership can be proven by signature
with no certificate authority anywhere. It is a genuinely beautiful idea — the address
*is* the credential — and it is the foundation of SEND.

Its near-total non-deployment is worth sitting with. The mechanism is sound, the
threat is real, and the industry solved a sufficient fraction of the problem with
switch features (RA Guard, ND Inspection) that required no host changes at all.
**Deployability beat elegance**, as it usually does.

**Jari Arkko, James Kempf, Brian Zill and Pekka Nikander.** Co-authors of RFC 3971
(SEND) with Aura's CGA underneath. Arkko went on to chair the IETF, and his writing on
why security mechanisms fail to deploy — that the party who must act is rarely the
party who is harmed — is the clearest general statement of the pattern this chapter
illustrates twice.

**Vasil Sadovnikov, Alberto Ornaghi and Marco Valleri.** Authors of `arpspoof` (part
of Dug Song's dsniff suite, 1999) and `ettercap` (2001) respectively — the tools that
turned ARP spoofing from a theoretical observation into something any student could do
in an afternoon. Their effect on the field was substantial and largely positive: the
existence of trivially usable attack tooling is what forced switch vendors to
implement Dynamic ARP Inspection and what made "the LAN is a trust boundary" an
untenable position.

**Dug Song (b. 1975)**, whose dsniff suite included `arpspoof`, made the broader
argument explicitly — that switched networks were being sold as secure and were not —
and the tooling was the argument. It is a recurring pattern in security: the
demonstration is the advocacy.
