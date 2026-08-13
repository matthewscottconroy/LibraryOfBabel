# Chapter 36 — The People

**David P. Reed (b. 1952).** **RFC 768**, August 1980 — **three pages**, never revised, and
the shortest specification of any protocol in daily use on the Internet.

**Its restraint is the achievement.** The obvious thing to do in 1980, with TCP already
being designed, was to make the second transport protocol *do something*. Reed's insight
was that the useful thing was to do **almost nothing** — to expose IP's datagram service
with the one addition IP lacked, process addressing, and get out of the way.

**Reed is also one of the three authors of the end-to-end argument** (Chapter 23 §23.4),
and UDP is that argument expressed as a protocol: **the network provides the minimum, and
the endpoints build what they need.** A reliable protocol and an unreliable one, both over
the same IP, each chosen by the application — which is precisely what the 1978 TCP/IP split
(Chapter 23 §23.1) was for.

**"Reed's law"** — that the value of a network supporting group formation scales as 2ⁿ — is
his other well-known contribution, and unrelated.

**Danny Cohen (1937–2019).** The packet-voice work at ISI in the mid-1970s that produced
the argument for splitting TCP (Chapter 23 §23.1): **a retransmitted voice sample arrives
too late to play, so an application must be able to decline reliability.**

**UDP exists because Cohen demonstrated a need for it.** §36.2's second criterion — that
timeliness can beat completeness — is his observation, made empirically by trying to carry
speech over the ARPANET and finding that TCP made it worse.

**Karn and Partridge, Jacobson, and the TCP community.** They belong in this chapter by
contrast: **everything they built into TCP over thirty years (Chapter 38) is what a UDP
application must build for itself.**

**Reading §36.2's warning against reimplementing TCP is more persuasive after Chapter 38**,
where the accumulated corrections — Karn's algorithm, fast retransmit, SACK, congestion
avoidance, window scaling — show how much there is to get wrong.

**Lars Eggert, Godred Fairhurst and Greg Shepherd.** **RFC 8085 / BCP 145** (2017), *UDP
Usage Guidelines* — §36.4's obligations.

**The document exists because the obligations were being ignored.** It is unusual in the
RFC series for being **advice to application authors rather than a protocol
specification**, and its tone is that of people who have watched too many applications
send at a fixed rate into a congested network and call it a design decision.

**Eggert** was IETF Chair and has been consistently clear on the point §36.4 makes: **an
application without congestion control is imposing an externality on everyone sharing the
path**, and choosing UDP does not make that acceptable.

**Van Jacobson (b. 1950).** Again, and here for the reason congestion control is an
obligation rather than a nicety: **the 1986 collapse** (Chapter 38 §38.1) demonstrated that
a network of unresponsive senders does not degrade — **it stops**.

**Everything UDP does not do, TCP had to learn to do after the network failed.** §36.4's
insistence is the memory of that.

**Henning Schulzrinne, Steve Casner, Ron Frederick and Van Jacobson.** **RTP** (RFC 3550),
and the design that makes §36.3's argument concrete.

**RTP's contribution is that it does not try to be reliable.** It provides sequence numbers
and timestamps — **enough for the application to detect loss and reordering and decide what
to do** — and stops there.

**And RTCP is the part that matters for §36.4:** receivers report loss and jitter, and the
sender adapts. **Congestion response implemented in the application, because the transport
does not provide it** — which is exactly what RFC 8085 requires, arrived at seventeen years
earlier by people who had to make voice work.

**Schulzrinne** is also a principal author of SIP, and the combination — SIP for
signalling over TCP or UDP, RTP for media over UDP — is the architecture behind essentially
all Internet telephony.

**David Mills (1938–2024).** **NTP**, and §36.3's most interesting case.

**His insight was that the transport must not hide delay.** NTP's entire method is
measuring the round trip and inferring the offset, so **any mechanism that retransmits,
queues or reorders invisibly makes the measurement wrong** — and a wrong measurement is
worse than a missing one, because the clock is confidently set to the wrong time.

**"A man with one clock knows what time it is; a man with two is never sure"** is
attributed to him, and NTP's design — many servers, statistical filtering, discarding
outliers — is that observation made into a protocol.

Mills was also an early Internet architect at the University of Delaware and the author of
the fuzzball routers that were the ARPANET's first gateways.

**Jim Gettys and the memcached incident's aftermath.** Not the cause, but the clearest
demonstration of §36.4's amplification argument: **memcached had no business being on a
public address**, its protocol had no authentication, and the amplification factor was
four orders of magnitude.

**The 2018 GitHub attack — 1.35 Tb/s** — was mitigated in about ten minutes because
Akamai's scrubbing capacity existed, and the underlying enabling conditions — **spoofable
sources and an exposed reflector** — had both been documented for decades.

**Paul Ferguson and Daniel Senie**, once more, for **BCP 38**. §36.4's amplification
section is a catalogue of what their two-page recommendation would have prevented, and its
incomplete deployment after twenty-five years is the reason the catalogue keeps growing.

**Jana Iyengar and Martin Thomson.** **QUIC** (RFC 9000), and the reason this chapter's
framing is dated at its edges.

**QUIC uses UDP not for its unreliability but as a substrate** — and then implements
reliability, ordering, flow control and congestion control **better than TCP does for its
purpose**, because it can see the application's streams. Chapter 38 §38.4 is theirs.

**The lesson for this chapter:** the choice was never *reliable or not*. **It was who
implements reliability, and with what knowledge of what the application actually needs.**
