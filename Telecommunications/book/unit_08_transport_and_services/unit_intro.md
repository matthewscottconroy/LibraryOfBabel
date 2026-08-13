# Unit VIII — Reaching the Application

A packet has arrived. It crossed nine networks, was forwarded by fourteen routers,
survived a NAT translation, and is now sitting in the network interface of the
machine it was addressed to.

It is not delivered.

The machine is running sixty processes. Four of them are listening for network
traffic. The IP header identifies the *host* and says nothing whatsoever about which
program on that host wanted this data. IP's job is finished and the packet is still
homeless.

That is the gap this unit opens with, and the answer — a small integer identifying
the intended process — is so simple that it is easy to miss how much depends on it.
Chapter 35 derives it before naming it, because a port number learned as a fact
("HTTPS is 443") is a fact you look up again, and a port number understood as the
solution to a problem you have felt is a concept you keep.

## The second problem

Having reached the right process, we discover that the service IP provides is
inadequate for most of what people want to do with a network.

IP loses packets, duplicates them, reorders them and corrupts them, by design
(Chapter 24 §24.1). A file transfer cannot tolerate any of that. A database
transaction cannot tolerate any of that. A web page cannot tolerate any of that.

So something must build reliability on top of unreliability — and by the end-to-end
argument (Chapter 23 §23.4) that something belongs on the hosts. **TCP** is it, and
Chapter 37 is the longest chapter in the book for good reason: turning a lossy,
reordering, best-effort datagram service into an ordered, reliable, flow-controlled
byte stream, between machines that cannot see each other and share no clock, is
genuinely hard. It took the field two decades to get right, and Chapter 38's
congestion control was added only after the network collapsed.

But not everything wants that. A voice packet that arrives late is worthless —
retransmitting it is not merely unhelpful but actively harmful, since it consumes
capacity to deliver something nobody can use. A DNS query is a single small
question with a single small answer, and paying a three-message handshake for it
would triple its cost. For these, **UDP** — Chapter 36, eight bytes of header and no
promises — is exactly right.

The existence of both is not a historical accident or a failure to converge. It is
the correct design: the transport layer offers two contracts, and applications
choose. §36.2 works through the choice properly.

## Then the services

With ports and transports in place, the unit turns to the protocols that make the
network usable by humans rather than by programs with addresses memorised.

**DNS** (Chapter 39) turns names into addresses. It is one of the most elegant
distributed systems ever built — a hierarchical, delegated, aggressively cached
database serving hundreds of millions of queries per second with no central point of
control — and it is also, as every operations engineer eventually learns, the cause
of a remarkable proportion of outages that initially look like something else. The
old joke is that it is never DNS, until it is DNS.

**DHCP** (Chapter 40) hands out addresses so that humans do not have to. It is the
reason you can carry a laptop into a building and have it work, and its failure
modes — an exhausted pool, a rogue server, a broken relay — produce symptoms that
are unmistakable once you have seen them once.

**Chapter 41** covers the rest of the zoo: HTTP and TLS, SSH, the file transfer
family, mail, directory, time, and the voice and management protocols. Ports are
attached to protocols only after each protocol's purpose is clear, which is the
opposite of how the port table is usually taught and, this book contends, the only
order in which it sticks.

## What to watch for

Two threads run through the unit and are worth flagging in advance.

**The tension between what the network knows and what it may know.** TCP's headers
are readable by any device on the path, and an enormous middlebox industry grew up
around that fact — load balancers, WAN optimisers, firewalls making decisions from
TCP state. QUIC (Chapter 38 §38.4) encrypts almost all of its transport metadata
specifically to end this, and the argument about whether that is a security
improvement or an operational catastrophe is live and interesting.

**The cost of a round trip, again.** Chapter 3 §3.4 established that round trips are
the expensive thing. Watch how much of this unit's engineering is devoted to
removing them: TCP Fast Open, TLS 1.3's one round-trip handshake, QUIC's zero
round-trip resumption, HTTP/2 multiplexing, DNS prefetching, connection reuse.
Almost every performance improvement in the last fifteen years of application
networking has been a round trip eliminated.

By the end of this unit you will be able to trace a complete web page load — DHCP,
ARP, DNS, TCP handshake, TLS handshake, HTTP request, response — and account for
every packet and every millisecond. That trace is the synthesis of everything from
Chapter 1 to here, and it is the exercise Chapter 41 closes with.
