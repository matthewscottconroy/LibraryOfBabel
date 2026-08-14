# Chapter 33 — The People

**Kjeld Borch Egevang and Paul Francis.** **RFC 1631** (1994), *The IP Network Address
Translator*.

**The document deserves to be read for its tone.** It does not oversell. It states that
NAT is a **short-term measure** pending a long-term solution, enumerates the architectural
damage — broken end-to-end addressing, protocols carrying addresses in payloads, the loss
of inbound reachability — and then recommends deployment anyway, because the alternative
was exhaustion before a replacement existed.

They were right about all of it, including the part they got wrong: they expected the
long-term solution to arrive in time.

**Paul Francis** went on to substantial work on Internet architecture, including PIP —
one of the IPv6 candidate proposals (Chapter 28) — and later research on privacy and on
NAT traversal. **He spent a career both creating the problem and working on its
consequences**, which is not a criticism.

**Pyda Srisuresh and Matt Holdrege.** **RFC 2663** (1999), which gave the terminology of
§33.2 — the taxonomy of NAT types, and the distinction between basic NAT and NAPT — and
**RFC 3022**, the revised specification.

**Naming things precisely is undervalued work.** Before RFC 2663 there was no agreed
vocabulary for the difference between what §33.2 calls static NAT, dynamic NAT and PAT,
and vendors used the terms inconsistently. The taxonomy made it possible to discuss the
behaviour at all — and, importantly, made it possible for STUN to classify what kind of NAT
it was behind.

**Jonathan Rosenberg.** **STUN** (RFC 3489, later 5389, now 8489), **ICE** (RFC 8445), and
much of the SIP work. He is the person most responsible for real-time media working across
NAT.

His achievement is unusual in character: **rather than fixing NAT or arguing against it,
he accepted it as a fact of the environment and built a discovery mechanism that works
regardless of what the NAT does.** ICE gathers every candidate path — direct,
STUN-discovered, TURN-relayed — tries them in parallel, and uses whichever works.

**Every browser video call on Earth runs ICE.** WebRTC would not exist without it, and the
entire remote-work infrastructure of the 2020s rests on a protocol family that exists
solely to work around an addressing shortage from the 1990s.

**Rohan Mahy, Philip Matthews and Jonathan Rosenberg.** **TURN** (RFC 8656) — the relay of
last resort.

**TURN is the admission of defeat, and it is essential.** When both endpoints are behind
symmetric NAT, no amount of cleverness produces a direct path, and the only remaining
option is to relay through a server that both can reach. **Ten to twenty per cent of calls
need it**, permanently, and somebody pays for that bandwidth in both directions for the
duration of every relayed call.

**Bryan Ford, Pyda Srisuresh and Dan Kegel.** *Peer-to-Peer Communication Across Network
Address Translators* (USENIX 2005) — the systematic treatment of hole punching.

Their paper is what made peer-to-peer possible across NAT at all, and it is worth
reading for the careful enumeration of which NAT behaviours permit it and which do not.
§33.3's table is theirs.

**Ford** is also worth reading on the broader consequence: that NAT converted a network of
peers into a network of clients, and that the applications which adapted did so by
centralising.

**Dan Wing and Alain Durand.** Much of the **CGNAT** and IPv6 transition work, including
**RFC 6598** (the `100.64.0.0/10` allocation) and the analyses of what carrier-grade NAT
costs.

RFC 6598's justification is worth reading because it is an admission: a fourth private
range was needed because providers had run out of addresses and could not use RFC 1918
without colliding with their own customers. **The document exists to solve a problem
created by the previous workaround**, which is how workarounds accumulate.

**Geoff Huston (b. 1954).** Again, for having measured what NAT actually did — the
consumption curves, the deferral of exhaustion, and the ongoing cost. His writing on NAT
is unsentimental in both directions: he does not defend it architecturally and he does not
pretend the Internet would have grown without it.

**The anonymous engineers of the home router industry.** Worth acknowledging, because the
practical experience of NAT for billions of people is a $30 box that does PAT, DHCP, a
firewall, and a wireless access point, and mostly works without configuration.

That it works at all is a real achievement, and the failures in this chapter —
hairpinning not implemented, UPnP opening holes, symmetric NAT defeating hole punching,
ALGs that corrupt traffic — are the seams of an enormously complex compromise implemented
cheaply, at scale, by people under commercial pressure. **Understanding the compromise is
more useful than being annoyed by the seams.**

**And the people who will remove it.** The mobile carrier engineers who deployed
464XLAT (Chapter 28 §28.4), the cloud providers now charging for IPv4, and the content
networks that went dual-stack — **none of them acting on architectural principle, all of
them acting on cost.**

Which is Chapter 28 §28.1's conclusion and this chapter's: **the argument that finally
works is the one with a number attached.**
