# Chapter 36 — Further Reading

## Primary sources

**RFC 768 — Postel, J. [Reed, D.] (1980). *User Datagram Protocol.***
**Three pages. Read the whole thing in one sitting** — it takes four minutes, and it is
the best available demonstration of how little a transport protocol is obliged to do.

**RFC 8085 / BCP 145 — Eggert, L., Fairhurst, G. & Shepherd, G. (2017). *UDP Usage
Guidelines.***
**If you will ever write a UDP application, this is required reading.** Unusual in the RFC
series for being advice to application authors, and its tone is that of people who have
watched the obligations of §36.4 be ignored for thirty years.

**RFC 3550 — Schulzrinne, H., Casner, S., Frederick, R. & Jacobson, V. (2003). *RTP: A
Transport Protocol for Real-Time Applications.***
The design that gives an application enough to detect loss and reordering and **nothing
more**, so it can decide for itself. §6 on RTCP is where the congestion response lives.

**RFC 4787 / BCP 127 — Audet, F. & Jennings, C. (2007). *NAT Behavioral Requirements for
Unicast UDP.***
Why §36.4's keepalives are necessary, and what a NAT should do that many do not.

**RFC 6935 / RFC 6936 — Eubanks, M., Chimento, P. & Westerlund, M. (2013). *IPv6 and UDP
Checksums for Tunneled Packets.***
The narrow, argued-for exception permitting a zero UDP checksum in IPv6. Worth reading as
an example of how a carve-out from a mandatory requirement gets justified.

**RFC 9715 — Fujiwara, K. & Vixie, P. (2025). *IP Fragmentation Avoidance in DNS over
UDP.***
DNS retreating toward smaller responses after two decades of fragmentation problems. The
practical conclusion of §36.4's fragmentation argument.

**RFC 9000 — Iyengar, J. & Thomson, M. (2021). *QUIC.***
Chapter 38 §38.4's subject, and relevant here for §14 (datagram size and path MTU) which
implements §36.4's rule as a protocol requirement.

## Books

**Stevens, W. R. (1994). *TCP/IP Illustrated, Volume 1*, chapter 11.**
UDP with real captures, including the fragmentation behaviour of §36.4.

**Stevens, W. R., Fenner, B. & Rudoff, A. (2003). *UNIX Network Programming, Volume 1*,
chapters 8 and 22.**
**Writing UDP applications properly**, including everything you must add yourself.
Chapter 22 in particular is a catalogue of the work §36.2 warns about.

**Kurose, J. & Ross, K. *Computer Networking*, chapter 3.**
The clearest textbook treatment of the TCP/UDP trade-off, with the reliable-data-transfer
construction built up step by step — which is a good way to see how much you would be
reimplementing.

**Fall, K. & Stevens, W. R. (2011). *TCP/IP Illustrated, Volume 1*, 2nd ed., chapter 10.**
The revision, covering IPv6's mandatory checksum and modern behaviour.

## On the hazards

**Rossow, C. (2014). "Amplification Hell: Revisiting Network Protocols for DDoS Abuse."
*NDSS*.**
**The systematic survey of amplification factors** — where §36.4's table comes from.
Fourteen protocols measured, and the results are worse than most people expect.

**Cloudflare's and Akamai's write-ups of the February 2018 memcached attacks**, including
the 1.35 Tb/s GitHub incident. Both are specific about the mechanism and the mitigation,
and both are honest about how easy it was.

**US-CERT TA14-017A**, on NTP `monlist` amplification. The advisory that drove the
remediation wave of 2014.

**RFC 5405** (the predecessor to 8085) and the discussion around it, for the argument about
whether congestion control should be a *requirement* or a *recommendation*. The conclusion
was requirement, and the reasoning is worth reading.

## Applied

**`tcpdump -nn udp` on a busy machine for thirty seconds.** You will find more than you
expect: mDNS, SSDP, NTP, DNS, DHCP renewals, and — if the machine browses the web —
**QUIC**. Identify every one.

**Compare a DNS query over UDP and TCP** (exercise F1):

```bash
dig example.com
dig +tcp example.com
# capture both and count packets and bytes
tcpdump -nn -c 40 'port 53'
```

**The packet-count ratio you measure is §36.1's argument**, produced by yourself.

**Write the UDP client and server of exercise F2**, then add loss with `tc`:

```bash
tc qdisc add dev lo root netem loss 2% delay 20ms reorder 5%
```

**Watch datagrams vanish and arrive out of order.** Then implement sequence numbers and
detect it. Then — exercise F3 — implement retransmission and congestion control, and
compare with TCP over the same conditions. **You will not beat TCP, and finding out why is
the point.**

**Measure your own DNS server's amplification factor:**

```bash
dig ANY example.com @yourserver | wc -c     # response size
# compare with the query size in a capture
```

**Then enable Response Rate Limiting and measure again.** Exercise F4.

**Check a VPN's keepalive behaviour.** Bring up a WireGuard tunnel without
`PersistentKeepalive`, leave it idle behind NAT, and time how long until it stops working.
Then set it and confirm the difference.

**`ss -unp`** for UDP sockets — and notice how much less there is to see than for TCP,
because there is no connection state to show.

**Lab 25** in this book's [labs/](../../../labs/) directory builds a UDP application,
subjects it to loss and reordering, has students implement reliability incrementally, and
then measures a deliberate amplification setup on an isolated network to make §36.4's
arithmetic concrete.

## For the certification-minded

Objective 1.4 expects UDP, the TCP/UDP comparison, and which applications use which.
Objective 4.2 expects amplification attacks.

Six things worth over-learning:

1. **UDP header is 8 bytes; TCP's minimum is 20.**
2. **UDP is connectionless and unreliable** — no handshake, no acknowledgement, no
   ordering, **no congestion control**.
3. **UDP protocols: DNS, DHCP, TFTP, SNMP, syslog, NTP, and voice/video (RTP).**
4. **TCP protocols: HTTP, HTTPS, SSH, FTP, SMTP, IMAP, POP3** — anything needing every
   byte.
5. **DNS uses both**, TCP for large responses and zone transfers.
6. **Amplification attacks exploit UDP's unverified source address**; DNS, NTP and
   memcached are the classic reflectors; **BCP 38 is the network-side defence.**

Expect a "which protocol would you use and why" question, and the reasoning that makes it
derivable: **UDP when the transaction is short, when timeliness beats completeness, when
the application can do better than TCP, or when you need one-to-many.**

And the point worth carrying past the exam: **choosing UDP is choosing to implement
congestion control yourself.** RFC 8085 is not optional advice, and an application that
ignores it is taking bandwidth from everyone else on the path.
