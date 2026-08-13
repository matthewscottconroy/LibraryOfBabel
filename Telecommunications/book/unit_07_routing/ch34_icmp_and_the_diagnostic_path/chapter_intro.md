# Chapter 34 — ICMP and the Diagnostic Path

Chapter 24 established that IP promises nothing and reports nothing. A packet may be
discarded for a dozen reasons and IP itself has no mechanism to say so.

That is tolerable for the data path — the endpoints will notice, per the end-to-end
argument — but it is intolerable for *operations*. If a network is misconfigured, or
a link is down, or a packet is too large for the next hop, somebody needs to be told.

The **Internet Control Message Protocol**, RFC 792, is the channel for saying so. It
is carried inside IP packets (protocol number 1), which makes it structurally odd —
a protocol carried by IP whose entire purpose is to report on IP — and it is the
foundation of nearly every diagnostic tool in Chapter 64.

## The messages that matter

ICMP defines many types; a handful account for essentially everything you will meet.

**Echo Request (8) and Echo Reply (0).** The basis of `ping`. A host sends an echo
request; the target, if it is willing, replies. That is all.

**Destination Unreachable (3)**, with a code specifying which kind — and the codes
are diagnostically valuable in a way that the summary "unreachable" is not:

- **Code 0, Network Unreachable** — a router has no route. Sent by a router.
- **Code 1, Host Unreachable** — the final router could not reach the host on its
  local segment; usually an ARP failure (Chapter 18 §18.3).
- **Code 3, Port Unreachable** — the host exists, the packet arrived, and nothing is
  listening on that UDP port. This one proves a great deal: the whole path works and
  the *service* is absent.
- **Code 4, Fragmentation Needed and DF Set** — the packet is too large for the next
  link. This is the message on which Path MTU Discovery entirely depends, and §34.4
  is about what happens when it is blocked.
- **Code 13, Administratively Prohibited** — a firewall rejected it, and told you.

**Time Exceeded (11).** TTL reached zero. §34.3 shows how `traceroute` weaponises
this.

**Redirect (5).** "There is a better router on this segment for that destination."
Legitimate but also a spoofing vector, and usually disabled.

## What ping actually proves, and what it does not

`ping` is the most used and most over-interpreted tool in networking, and §34.2 is
mostly about calibrating what a result licenses you to conclude.

**A successful ping proves:** there is a bidirectional path at the IP layer between
you and that address, the target's IP stack is running, and it is willing to answer
echo requests. That is genuinely a lot — it exonerates Layers 1 through 3 in both
directions in one command.

**A successful ping does not prove:** that any application works, that TCP will
succeed, that performance is adequate, or that the path is the one you think it is.
A server whose web service has crashed pings perfectly.

**A failed ping proves almost nothing at all.** The list of reasons that do not mean
"the host is down" is long: a firewall dropping ICMP (extremely common, and the
default on Windows hosts not joined to a domain); a router rate-limiting ICMP; a
security policy prohibiting echo; a host configured not to respond. **"It doesn't
ping" is not a diagnosis**, and treating it as one is the single most common
beginner error in network troubleshooting.

The corollary that experienced engineers internalise: prefer a test that exercises
the thing you actually care about. If you need to know whether a web service works,
connect to port 443. `ping` tells you about `ping`.

## The traceroute trick

`traceroute` is the most elegant piece of engineering in this chapter, and knowing
how it works is what allows you to read its output correctly rather than
superstitiously.

Send a packet with **TTL = 1**. The first router decrements it to zero, discards it,
and sends back ICMP Time Exceeded — revealing its own address. Send another with
TTL = 2; the second router does the same. Increment and repeat until the destination
itself answers. Each router along the path identifies itself in turn, purely as a
side effect of the loop-prevention mechanism from Chapter 24 §24.4.

§34.3 covers the several implementations — classic Unix `traceroute` uses UDP to
high ports and detects arrival by Port Unreachable; Windows `tracert` uses ICMP
Echo; `traceroute -T` uses TCP SYN, which is often the only variant that gets
through a firewall — and, more importantly, covers the four ways its output is
routinely misread:

**Asterisks are not failures.** A hop showing `* * *` is usually a router configured
not to send Time Exceeded, or rate-limiting it. If subsequent hops respond, the path
is fine.

**High latency at one hop with normal latency beyond it is a control-plane
artefact**, not a slow link. Chapter 3 §3.2 said this; it is worth saying twice
because it generates more incorrect escalations to transit providers than any other
single misreading.

**Loss at an intermediate hop that does not propagate is not loss.** Same reason.

**The return path is invisible.** Each Time Exceeded takes its own route home, which
may be entirely different. A high RTT at hop 7 may be caused by something on the
return path from hop 7 that has nothing to do with your forward path. This is why
`mtr` in both directions, or a reverse traceroute from the far end, is worth the
trouble in a serious investigation.

## Path MTU discovery, and the black hole

The chapter's most practically important failure mode, and one that is still, in
2026, misdiagnosed constantly.

A host that wants to avoid fragmentation sets the Don't Fragment bit and sends
full-size packets. If a link along the path has a smaller MTU, its router discards
the packet and returns ICMP Type 3 Code 4 — *fragmentation needed* — including the
MTU it could accept. The sender reduces its packet size and continues. This is
**Path MTU Discovery**, RFC 1191, and it works beautifully.

Unless someone has blocked ICMP.

If that message is filtered, the sender never learns. It continues sending packets
that are silently discarded. Small packets — the TCP handshake, a short request —
get through fine. Large packets vanish. The observable symptom is distinctive and
maddening:

> **The connection establishes successfully and then hangs.** SSH connects and shows
> a banner, then freezes on the first large output. A web page's headers arrive and
> the body never does. `ping` works; `ping -s 1400` does not.

The cause is almost always a firewall administrator who blocked all ICMP on the
theory that ICMP is a security risk. It is a genuinely bad idea, it is still
extremely common, and RFC 4890 exists specifically to tell people which ICMP types
must be permitted. For IPv6 it is worse, because IPv6 removed router fragmentation
entirely (Chapter 24 §24.3) and therefore *depends* on ICMPv6 Packet Too Big — a
network that blocks ICMPv6 does not work at all.

## By the end you will be able to

- Identify the common ICMP types and codes and state what each tells you.
- State precisely what a successful ping proves and what a failed ping does not.
- Explain the TTL mechanism behind traceroute and name the three implementations.
- Read a traceroute correctly, including asterisks, intermediate latency and
  intermediate loss.
- Recognise the PMTUD black hole from its symptom within a minute, and state both
  the correct fix and the common workaround.
- Argue against a blanket ICMP block with specific consequences.
