# Chapter 35 — Important Concepts

**The gap IP leaves** *(§35.1)* — A packet arrives at the machine and **nothing says which
program it is for**. The IP header has a protocol field, which names TCP or UDP, not a
conversation or a process. **Without ports, a machine could run exactly one network
application.**

**The requirement, derived** *(§35.1)* — An identifier **in the packet** (nothing external
can help); **fixed by convention for servers** (a client must know it before making
contact, and negotiating would need a prior conversation with its own identifier);
**arbitrary for clients**, needing only local uniqueness; **small**, because it is in every
packet; and **numerous enough** for thousands of simultaneous conversations.

**16 bits** *(§35.1)* — 65,536 values. Extravagant in 1981, occasionally binding now
(NAT capacity, ephemeral exhaustion). **The smallest field that works** — 8 bits would be
exhausted by the well-known services alone.

**The two-level address** *(§35.1)* — **The IP address gets the packet to the machine; the
port gets it to the program.** Layer 3 is host-to-host; Layer 4 is process-to-process.

**The demultiplexing chain completed** *(§35.1)* — **EtherType → Protocol → Port →
process.** Every layer has exactly one field naming who gets the payload, and the chain
terminates at a process.

**Ports are per-protocol** *(§35.1)* — **TCP/53 and UDP/53 are different ports**, because
the protocol field is consulted first. In firewall rules they are two separate rules, and
forgetting one is a common error.

**What a port is not** *(§35.1)* — **Not a physical thing** — it is a number in a header.
**Not owned by a protocol** — the numbers are convention. And **"open" is a property of a
host, not a network**: a closed port answers with a refusal, a blocked one is silent, and
they are different faults.

**The five-tuple** *(§35.2)* — **(protocol, src IP, src port, dst IP, dst port).** Every
field is in the packet, so a host identifies a connection from the packet alone. **A
thousand clients on port 443 are distinguishable because their address and port differ.**

**What follows from the tuple** *(§35.2)* — **How one server port serves many clients**;
**why NAT works** (it rewrites two of the five and records the mapping); **why stateful
firewalls are possible** (the tuple is the state); **why one client opens many
connections**; and **why ECMP and load-balancer hashing keep a flow on one path** (the
tuple is the flow identifier). **Five numbers, and half of Unit VIII follows.**

**Listening versus connected sockets** *(§35.2)* — A listener has a local address and port
and no peer; a connected socket has all five fields. **`0.0.0.0:443` accepts on every
interface; `127.0.0.1:5432` accepts only locally** — and that difference is a security
control.

**The binding rule** *(§35.2)* — **Two sockets may not bind the same (protocol, address,
port) triple.** `127.0.0.1:8080` and `10.0.0.5:8080` can coexist; **`0.0.0.0:8080`
conflicts with both**, because it claims every address. This is why "address already in
use" sometimes appears when the port looks free.

**`accept()` returns a new socket** *(§35.2)* — The listener keeps listening. **A server
with 1,000 clients has 1,001 sockets**, all sharing one local port. **Which is why "the
server ran out of ports" is almost always wrong** — a server does not consume a port per
client; a *client* making thousands of outbound connections may.

**UDP has no connections** *(§35.2)* — So there is no teardown, and **a NAT or firewall
must guess when a flow has ended** — hence timeouts, hence the keepalives long-lived UDP
applications send.

**The three ranges** *(§35.3)* — **0–1023 well-known** (IANA, strict); **1024–49151
registered** (IANA on request, advisory); **49152–65535 ephemeral** (nobody).

**Two entries worth special note** *(§35.3)* — **Port 53 uses both UDP and TCP** — TCP for
large responses and zone transfers, and permitting only UDP breaks DNSSEC. **Port 443 now
carries UDP too**, for QUIC — and blocking it makes browsers fall back to TCP so everything
works slightly worse with nobody noticing.

**The privilege rule** *(§35.3)* — Binding below 1024 required root **for social reasons**:
in a multi-user system it meant a service on a well-known port had been started by an
administrator. **Its security value is now essentially nil**, and it survives as convention
and deployment friction.

**Ephemeral ranges differ by platform** *(§35.3)* — **Linux 32768–60999**; Windows, BSD and
RFC 6335 use 49152–65535. Linux's range **overlaps the registered range**, so an outbound
source port may be somebody's registered service number.

**Ephemeral exhaustion** *(§35.3)* — ~28,000 ports with a 60-second `TIME-WAIT` gives about
**470 new outbound connections per second, sustained**. Beyond that, `EADDRNOTAVAIL` —
which does not obviously mean "out of ports". **The real fix is connection reuse**;
widening the range and `tcp_tw_reuse` help; and **a second destination address doubles the
space, because the destination is part of the tuple.**

**The convention has no enforcement** *(§35.3)* — **Running SSH on 2222 reduces scanning
noise and provides no security.** **Port-based firewall rules are weak**, which is why
malware command-and-control uses 443 and why next-generation firewalls inspect content.
**`nmap` reporting "443/tcp https" is inferring from convention.** The port number is
enormously useful and it is not evidence.

**Reading a port in context** *(§35.3)* — Which range; **source or destination**; which
protocol; and is it really that service. **The service is whichever end has the low,
stable, well-known number** — and on the reply they swap.

**`ss -tlnp` is the command** *(§35.4)* — What is listening, on which addresses, and which
process. **Always use `-n`** — otherwise it resolves every address, and if DNS is what is
broken, it hangs.

**SYN-SENT means the peer is not answering** *(§35.4)* — Silence, not refusal. **A RST
would have produced no socket at all.** Accumulating SYN-SENT is a network problem, not an
application one.

**CLOSE-WAIT is an application bug** *(§35.4)* — **The peer sent FIN and our application
has not called `close()`.** A growing count is a file-descriptor leak that ends with the
service refusing connections hours later.

**TIME-WAIT is usually fine** *(§35.4)* — Thousands on a busy server is normal. **It
matters only on the client side**, where it consumes ephemeral ports. `tcp_tw_reuse` is
safe; **`tcp_tw_recycle` broke clients behind NAT and was removed from Linux.**

**The queues assign blame** *(§35.4)* — On a **listener**: `Recv-Q` is connections waiting
to be accepted and `Send-Q` is the backlog limit, so **a non-zero `Recv-Q` means the
application is not accepting fast enough** and an overflow gives clients timeouts rather
than refusals. On an **established** socket: **`Recv-Q` blames the local application** (it
is not reading) and **`Send-Q` blames the network or the peer** (it is not acknowledging).

**`ss -tni`** *(§35.4)* — Per-connection RTT, congestion window, retransmissions and
algorithm. Chapter 38's material, live, without a capture.

**The diagnostic sequence** *(§35.4)* — Is the process running; **is it listening and
where**; does it work locally; does it work remotely; is it a firewall. **Steps 3 and 4
together eliminate the entire application in two commands** — working locally and not
remotely means the bind address or the network, never the application.
