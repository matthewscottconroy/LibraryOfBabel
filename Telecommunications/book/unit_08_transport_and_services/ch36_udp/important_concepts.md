# Chapter 36 — Important Concepts

**RFC 768 is three pages** *(§36.1)* — David Reed, August 1980, never revised. **The most
restrained specification in the suite**, and the best introduction to what a transport
layer is *obliged* to do, because UDP does only that.

**The header** *(§36.1)* — **8 bytes, four fields**: source port (**optional, may be 0**),
destination port (**the demultiplexing key**), length (**minimum 8**), checksum.

**UDP's only essential contribution** *(§36.1)* — **It adds process-level addressing to IP**
and nothing else that is strictly required.

**The checksum is optional in IPv4 and that is a trap** *(§36.1)* — The IP header checksum
covers only the header, and Ethernet's FCS is recomputed at every hop. **With UDP's
checksum disabled, a bit flip inside a router reaches the application undetected.** **IPv6
makes it mandatory** because IPv6 removed the IP header checksum entirely — so UDP's is the
only integrity check between the link layer and the application.

**The pseudo-header** *(§36.1)* — UDP's checksum covers the IP addresses, so **UDP reads
the IP header**, and **a NAT must recompute it** after rewriting.

**Datagram boundaries are preserved** *(§36.1)* — **A `send()` of 100 bytes arrives as a
`recv()` of 100 bytes.** TCP does not do this — it is a byte stream, and the application
must frame its own messages. **This is a genuine feature, not merely an absence**, and it
is the most under-appreciated reason to choose UDP.

**Two packets versus eleven** *(§36.1)* — A DNS query and response over UDP is **two
packets and zero round trips before data**. The same over TCP is ~11 packets and **one full
RTT before the query is even sent** — which on a 50 ms path doubles the latency.

**The cost of reliability — one round trip** *(§36.2)* — TCP cannot send data until the
third packet of the handshake. **For a one-question transaction, the handshake doubles the
latency**, at every RTT from 1 ms to 600 ms.

**Head-of-line blocking** *(§36.2)* — A lost segment 5 blocks delivery of 6, 7 and 8 **even
though they have arrived**. Correct for a file transfer; **a disaster for live audio**,
because the held-back segments were the audio that should be playing now. **TCP's
retransmission does not merely fail to help real-time media — it actively harms it.**

**Retransmission of stale data** *(§36.2)* — TCP retransmits anything it sent, regardless
of whether it still matters, and **there is no interface for "forget byte 5, it has
expired"**.

**Connection state** *(§36.2)* — A TCP server with 100,000 connections holds 100,000
control blocks. **A stateless UDP server holds nothing**, which is why one machine answers
100,000 DNS queries per second.

**The four criteria for UDP** *(§36.2)* — (1) **the transaction is short**, ideally one
exchange; (2) **timeliness beats completeness** — the test is *if this arrives late, is it
still useful?*; (3) **the application can do reliability better than TCP can**, because it
knows things TCP cannot; (4) **one-to-many**, since there is no multicast TCP and cannot
be.

**The third criterion is the least understood** *(§36.2)* — TCP's reliability is
**general**. An application often knows that this frame is a keyframe, that this update
supersedes the last, that this request is idempotent. **QUIC is the industrial-scale
version of this argument.**

**When UDP is wrong** *(§36.2)* — Bulk transfer; anything needing every byte in order; and
**anything where you would end up implementing retransmission and ordering yourself.**
**"We used UDP because it is faster" is the most common bad reason** — it is faster because
it does less.

**The comparison has two surprising rows** *(§36.2)* — **UDP preserves message boundaries
and TCP does not**; **TCP suffers head-of-line blocking and UDP does not.** The list is not
simply "TCP has more".

**DNS** *(§36.3)* — Short, latency-critical, and **its retry logic is better than TCP's**:
it retries against a *different server*. **It uses TCP too**, for large responses and zone
transfers — a firewall permitting only UDP/53 breaks DNSSEC.

**DHCP** *(§36.3)* — **Could not use TCP.** The client has no address to receive a SYN-ACK
at and does not know who the server is. **UDP over broadcast is the only mechanism that
solves the bootstrap problem.**

**TFTP** *(§36.3)* — Lock-step acknowledgement gives **25 KB/s on a 20 ms path regardless
of link speed** — and it was right, because it must fit in a boot ROM where a TCP stack
does not.

**NTP** *(§36.3)* — **The transaction is a measurement, and TCP would corrupt it.** A
retransmitted packet arrives late by an unknown amount, and the client cannot distinguish
it from a slow one — so the delay measurement, and hence the clock, would be wrong. **A
lost NTP packet is simply skipped.**

**SNMP and syslog** *(§36.3)* — High volume, loss tolerable. **And syslog over UDP is a
real weakness**: it drops silently, **exactly when volume spikes**, and it is trivially
spoofable. **Use TCP or TLS syslog for anything security-relevant.**

**RTP** *(§36.3)* — Carries its own sequence number and timestamp so **the application
decides what to do about loss**: conceal an audio packet, skip a P-frame, **request a new
keyframe**. The application *does* retransmit — only what is worth retransmitting, which
TCP cannot distinguish.

**Three routing protocols, three answers** *(§36.3)* — RIP uses UDP; **OSPF and EIGRP use
neither** and run directly on IP with their own reliability; **BGP uses TCP**, because a
route once sent is held until withdrawn.

**QUIC uses UDP as a substrate** *(§36.3)* — Not for its unreliability, but because
deploying a new IP protocol is impossible and **middleboxes cannot ossify what they cannot
parse.**

**Amplification** *(§36.4)* — **No handshake means the source address is never verified.**
A small spoofed query produces a large response to the victim, **from legitimate servers**,
so it cannot be blocked by source. **DNS 28–54×, NTP `monlist` ~557×, memcached
10,000–51,000×** — which produced a **1.35 Tb/s** attack on GitHub in 2018.

**The obligations against amplification** *(§36.4)* — **Rate-limit responses**; **do not
run open resolvers or reflectors**; prefer designs where the response is not much larger
than the request. And for the network: **BCP 38**, whose non-deployment is the enabling
condition for the entire class.

**No congestion control** *(§36.4)* — **TCP slows down and UDP does not**, so on a congested
link **UDP's share grows and TCP's shrinks until TCP has almost nothing.** The well-behaved
flow is punished for behaving well. **This is structural, not hypothetical.**

**RFC 8085 / BCP 145** *(§36.4)* — If you write a UDP application you are obliged to read
it: **implement congestion control or restrict the rate; back off on loss; use a
retransmission timer of at least 1 s; avoid fragmentation.** The alternative is a
**circuit-breaker** — stop sending entirely rather than contribute to collapse.

**Fragmentation hits UDP harder** *(§36.4)* — **TCP segments to fit the path
automatically; UDP does not** — the application chooses the size, and a bad choice means IP
fragments. **Keep datagrams under 1,400 bytes**, or 1,200 to be safe. **DNS learned this
the hard way** and RFC 9715 now recommends ~1,232 bytes, a retreat toward the original 512
reasoning after twenty years.

**No delivery confirmation, and it fails worst under load** *(§36.4)* — Queues fill, UDP is
dropped, **and the monitoring that would have told you is the thing being dropped.**

**NAT timeouts and keepalives** *(§36.4)* — No connection means no teardown, so a middlebox
**must guess** when a flow ended. When it guesses wrong, **inbound packets have nowhere to
go and no error is generated at either end.** Hence WireGuard's `PersistentKeepalive = 25`,
IPsec NAT-T's 20 s, and QUIC's PING frames — **otherwise-pointless traffic sent permanently
because a middlebox might forget.**

**Spoofing** *(§36.4)* — **TCP's handshake provides weak authentication as a side effect**;
an off-path attacker cannot guess the initial sequence number. **UDP has no equivalent** —
a single spoofed packet is accepted. Hence DNS cache poisoning, forged syslog, spoofed NTP
that moves a clock and thereby breaks certificate validation.

**The summary** *(§36.4)* — **UDP is not simpler. It is simpler in the protocol and
correspondingly harder in the application.** The work does not disappear; it moves — to
somewhere with less scrutiny and fewer decades of accumulated fixes.
