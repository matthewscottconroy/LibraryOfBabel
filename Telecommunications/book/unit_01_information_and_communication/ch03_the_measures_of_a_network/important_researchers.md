# Chapter 3 — The People

**Stuart Cheshire.** British-born computer scientist, at Apple since the late
1990s and the principal designer of Bonjour/mDNS (Chapter 39) and a co-author of
the Multicast DNS and DNS Service Discovery standards. His 1996 essay *It's the
Latency, Stupid* — written while a doctoral student at Stanford, and prompted by a
modem vendor's marketing claim — is the clearest short statement of why bandwidth
and latency are different quantities, and remains in print on his website
unchanged. He is also responsible for the link-local addressing (169.254.0.0/16)
that Chapter 27 covers, and for a long campaign against the buffer sizing
practices that produce the bufferbloat of Chapter 66.

**Agner Krarup Erlang (1878–1929).** Danish mathematician and engineer at the
Copenhagen Telephone Company. Between 1909 and 1917 he founded queueing theory
from scratch, in order to answer a question his employer actually had: how many
circuits does an exchange need so that calls are rarely blocked? His formulas — the
Erlang B and Erlang C — are still used for capacity planning in telephony and call
centres, and the unit of offered traffic bears his name. The ρ/(1−ρ) behaviour in
§3.2 is the simplest member of the family he started. Chapter 12 uses his
blocking formula directly.

**Leonard Kleinrock (b. 1934).** American engineer, professor at UCLA. His 1962
MIT doctoral thesis applied queueing theory to message-switched communication
networks, producing the first analytic treatment of delay in a packet-switched
network — work that preceded and informed the ARPANET, whose first node was
installed in his laboratory in 1969. The extent to which his thesis constitutes
"the invention of packet switching" has been publicly disputed (notably by Donald
Davies's supporters and by Baran); what is not disputed is that he supplied the
mathematics of queueing delay that this chapter uses. Chapter 13 revisits the
priority argument.

**Matthew Mathis.** American network researcher, at the Pittsburgh
Supercomputing Center and later Google. His 1997 paper with Semke, Mahdavi and Ott,
"The Macroscopic Behavior of the TCP Congestion Avoidance Algorithm," derived the
inverse-square-root relationship between loss and throughput that §3.3 tabulates.
He also led the development of the Web100/Web10G TCP instrumentation work, which
made per-connection diagnosis possible, and the pathdiag tools for finding exactly
the misconfigurations described in §3.4.

**Van Jacobson (b. 1950).** American network researcher, at Lawrence Berkeley
National Laboratory and later Cisco, PARC and Google. His response to the 1986
congestion collapse produced the algorithms that made TCP survivable (Chapter 38),
and RFC 1323 — the window scaling and timestamps extensions discussed in §3.4 —
was written with Bob Braden and Dave Borman. He also wrote `traceroute`, `tcpdump`,
and the Berkeley Packet Filter that underlies both, which makes him responsible
for a remarkable fraction of both the mechanisms and the tools in this book.

**Bob Braden (1934–2018).** American computer scientist at USC/ISI, longtime
editor of the RFC series and co-author of RFC 1323. He also wrote RFC 1122 and
1123, the *Requirements for Internet Hosts* documents that codified what an
Internet implementation must actually do — arguably the most load-bearing pair of
documents in the suite, and the reason independent implementations interoperate at
all.

**John Nagle.** American engineer whose 1984 RFC 896, *Congestion
Control in IP/TCP Internetworks*, identified both the small-packet problem and the
congestion collapse risk two years before collapse actually happened. Nagle's
algorithm — coalescing small writes to reduce the overhead computed in §3.1 — is
in every TCP stack, and its interaction with delayed acknowledgement is a
classic performance trap that Chapter 37 covers. He wrote the RFC while working
on networking for Ford Aerospace.
