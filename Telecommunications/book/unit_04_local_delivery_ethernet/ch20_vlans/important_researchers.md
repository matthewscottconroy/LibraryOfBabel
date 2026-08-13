# Chapter 20 — The People

**W. Paul Sherer and the IEEE 802.1Q working group (1995–1998).** VLANs existed as
vendor-proprietary mechanisms for several years before standardisation — Cisco's ISL,
3Com's, Cabletron's, each incompatible with the others. A VLAN that could not cross a
multi-vendor boundary was of sharply limited use, and the committee's work was
principally one of reconciliation.

The design decision worth examining is the **tag's location**: inserted between the
source address and the EtherType, using a **reserved EtherType value** as its marker.
This was chosen so that the change is invisible to everything that does not need to
know. A switch that does not understand tags sees an unrecognised EtherType and
discards the frame — a clean failure rather than a corrupt interpretation. And crucially
**no endpoint had to change**: workstations, printers, servers and their drivers were
untouched, because access ports strip the tag before the frame ever reaches them.

**Backward compatibility was the constraint that shaped the standard**, and it is why
adoption was so fast. Compare with IPv6 (Chapter 28), which is technically superior to
what it replaces in every respect and has taken thirty years, precisely because it did
not preserve compatibility.

**The 12-bit VID decision.** Four thousand VLANs in 1998 was extravagant — the largest
campus networks used perhaps a hundred. The field size was chosen to fit the tag into
four bytes, which kept the frame within a size existing hardware could be persuaded to
accept.

It was the right call for its era and it is instructive about forecasting. The
constraint that broke it was not campus growth; it was **multi-tenancy**, a
requirement that did not exist in any recognisable form in 1998. Cloud providers with
tens of thousands of isolated customers exhaust 4,094 immediately, and VXLAN's 24-bit
VNI (Chapter 66) is the response. **The field was not too small for the problem it was
designed for; the problem changed.**

**The IEEE 802.1p working group.** The three-bit priority field, developed alongside
802.1Q and folded into it. Their contribution to the broader story is negative and
useful: 802.1p demonstrated that **marking traffic is easy and acting on markings is
hard**. The field was standardised quickly and widely implemented; consistent
end-to-end behaviour based on it never materialised, because it requires every device
along a path to agree on what each value means and to be configured accordingly.

Chapter 52 returns to this. The lesson generalises well beyond networking: **a
convention for expressing intent is not the same as a mechanism for honouring it.**

**Steve Deering (b. 1955).** Not directly a VLAN figure, but his consistent argument
against large Layer 2 domains — made from the IP side, over decades — is the
counterweight to this chapter's enthusiasm. His position is that VLANs allowed
organisations to defer subnetting decisions they should have made, and that
"stretched" Layer 2 across a campus or between data centres is an architectural
mistake that VLANs made possible. Chapters 25 and 67 give both sides.

**The bridging-versus-routing argument** runs through Chapters 17, 19, 20 and 67, with
Perlman and Deering on one side and operational convenience on the other, and it is
worth tracking as a thread rather than treating each chapter's version as separate.

**Anonymous, and the double-tagging discovery.** The double-tagging VLAN hop was
described publicly in the early 2000s, notably in work by **@stake** (the security
consultancy) and in Sean Convery's *Network Security Architectures* (2004). Convery's
treatment is the one that made switch security a standard topic rather than a
curiosity, and his framing is the one this chapter adopts: **the attack exploits
correct behaviour**, so it cannot be fixed by fixing an implementation. It has to be
designed around.

That framing — *specification-compliant and insecure* — recurs throughout Unit XII, and
double tagging is the cleanest early example in this book.

**Sean Convery.** Cisco, and the author of *Network Security Architectures*, which
systematised Layer 2 attacks and defences at a point when most security attention was
on firewalls and perimeters. His argument that **the access layer is a security layer**
— that a wall socket is an attack surface — is the reasoning behind every hardening
line in §20.3, and it took the industry a decade to internalise.

**The LLDP-MED working group (TIA-1057, 2006).** Media Endpoint Discovery: the
standard mechanism by which a telephone learns its voice VLAN, its power requirement
and its QoS policy from the switch, replacing Cisco's proprietary CDP for this purpose.

It is a small, unglamorous standard and it is the reason an IP telephone can be
unboxed, plugged in, and work — with the correct VLAN, the correct priority and the
correct power draw — without anybody configuring the telephone at all. **Zero-touch
provisioning for a category of device deployed in the hundreds of millions**, achieved
by a discovery protocol nobody thinks about. Cheshire's Zeroconf work (Chapter 18) has
the same character: infrastructure whose success is measured by how completely it is
ignored.
