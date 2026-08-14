# Chapter 34 — The People

**Jon Postel (1943–1998).** **RFC 792**, September 1981 — twenty-one pages specifying
ICMP, published alongside RFC 791 (IP) and never revised.

**The design decision worth noting** is the one in §34.1: an error message carries the
offending packet's IP header plus the first 8 bytes of payload. That number is not
arbitrary — it is exactly enough to include a TCP or UDP port pair, and therefore exactly
enough for the sender to attribute the error to a socket.

It is a layer violation, chosen deliberately, in 1981, because the alternative was an
error channel whose messages could not be acted upon. Chapter 21 §21.4's argument that
violations happen where the boundary costs more than it is worth, in its earliest form.

**Mike Muuss (1958–2000).** **`ping`**, written in an evening in December 1983 at the US
Army Ballistic Research Laboratory, to debug a network problem he was having.

He named it after sonar: emit a pulse, listen for the echo, and the delay tells you the
distance. **The backronym "Packet InterNet Groper" was applied later by someone else, and
he disliked it.**

His own account is characteristically unpretentious — he wanted to find out where the
packets were going, wrote a small program, and thought no more about it. It has shipped
with every operating system since, and is plausibly the most-executed network program in
history.

He was killed in a car accident in 2000. His original source is short, readable, and worth
finding — partly for the code, and partly because it is a reminder that a great deal of
essential infrastructure began as somebody solving their own immediate problem.

**Van Jacobson (b. 1950).** **`traceroute`**, 1987, and it is his third appearance in this
book after TCP congestion control (Chapter 38) and header compression (Chapter 21).

The insight is the reason it belongs in a chapter about diagnostics: TTL existed as a
safety mechanism to contain loops (Chapter 24 §24.4), and Jacobson noticed it could be
used as a **probe** — that deliberately causing a failure at a controlled distance and
reading the complaint would map the path.

**Nobody had done it because TTL was understood as protection rather than as an
instrument. The recurring pattern in Jacobson's work is that he builds the measuring
device first**, and the understanding follows from what it shows: `traceroute`, `tcpdump`,
`pathchar`, and the instrumentation that led to the congestion-control work.

**Jeffrey Mogul (b. 1958) and Steve Deering (b. 1955).** **RFC 1191** (1990), Path MTU
Discovery.

**They anticipated the black hole.** RFC 1191 explicitly discusses what happens when the
ICMP message does not return, and recommends a fallback in which a sender that gets no
response to repeated large packets reduces its estimate anyway.

**The fallback was widely not implemented.** The result is a failure mode that has been
independently rediscovered by tens of thousands of engineers over thirty-five years, and
§34.4 exists because of it.

**A specification that predicts its own failure mode and recommends a mitigation is doing
its job. An industry that implements the mechanism and skips the mitigation is not.**

**Matt Mathis and John Heffner.** **RFC 4821** (2007), Packetization Layer Path MTU
Discovery — the robust answer of §34.4.

**Their reasoning is the right one:** if a mechanism depends on a third party's
cooperation, and that third party frequently does not cooperate, **build a mechanism that
does not depend on them.** PLPMTUD probes using the transport's own traffic and its own
acknowledgements, so no firewall can break it.

It has been available for eighteen years and remains under-deployed, which is Chapter 28's
incentive problem in miniature: the endpoint that implements it is protected, so there is
at least a direct benefit — and it still took until stacks enabled it by default.

**The QUIC designers** took the lesson properly and built path MTU discovery into the
protocol (RFC 9000 §14), on the explicit reasoning that anything depending on middleboxes
behaving well would be broken by middleboxes.

**Ed Davies and Janos Mohacsi.** **RFC 4890** (2007), *Recommendations for Filtering ICMPv6
Messages in Firewalls*.

The document exists because people kept breaking IPv6 by applying IPv4 firewall habits
to a protocol where ICMP is load-bearing. It lists, type by type, what must pass and why.

It is the most useful firewall document in this book, and the fact that it needed to be
written is a comment on how deeply the "block all ICMP" reflex was embedded.

**Steve Bellovin (b. 1951).** For the counter-argument, made carefully. His work with
Cheswick and Rubin on firewalls is the authoritative treatment of what ICMP filtering
actually buys, and their conclusion — **rate-limit, filter selectively, do not block
wholesale** — is §34.1's position, from people whose professional interest is security
rather than convenience.

**That the strongest argument against blanket ICMP blocking comes from firewall authors
rather than from network engineers** is worth noticing, and worth citing when the argument
comes up in your own organisation.

**The `mtr` authors — Matt Kimball and Roger Wolff.** A small tool combining ping and
traceroute into something that answers a question neither could: **is this hop actually
losing traffic, or is it merely declining to talk about itself?**

The per-hop loss column, read with §34.3's rule, resolves more false alarms than any
other single piece of output in network operations. **A modest tool that removed a whole
category of wasted effort.**
