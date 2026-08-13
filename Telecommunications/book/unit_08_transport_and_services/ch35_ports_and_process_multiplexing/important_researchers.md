# Chapter 35 — The People

**Jon Postel (1943–1998).** The **port number registry**, maintained personally for nearly
three decades.

**This is the contribution this chapter rests on**, and it is entirely unglamorous. Ports
work because when you connect to port 25 anywhere on Earth you reach a mail server, and
that is true because one person, and then one organisation, kept an authoritative list and
adjudicated claims to it.

**There is no protocol enforcing it.** §35.3's convention has no mechanism behind it at
all — it is a published list that everyone chose to follow, sustained for forty-five years.

**RFC 1700** (1994), *Assigned Numbers*, was Postel's compendium of every registry: port
numbers, protocol numbers, EtherTypes, hardware types. It ran to 230 pages and was for
years the single most-consulted document in networking.

**When Postel died in 1998**, the transition of IANA's functions to an institution took
years, because it emerged that one person had been doing something requiring an
organisation. Chapter 23's notes make the general point; the port registry is the clearest
instance.

**Joyce K. Reynolds (1952–2015).** Co-maintained the assigned-numbers registries with
Postel for two decades and continued after his death. **The work of adjudicating claims —
deciding who gets a number, refusing duplicates, retiring dead assignments — is
administrative, endless, and the reason the namespace stayed coherent.**

**Bill Joy (b. 1954) and the Berkeley CSRG.** The **socket API** (Chapter 21 §21.2), and
therefore §35.2's model of what a socket is.

**The decision that matters here** is that a socket is a **file descriptor**. `read`,
`write`, `close` and `select` work on sockets exactly as on files, which meant that in 1983
every existing Unix programmer already knew most of the network API.

**And the design is still visible in the failure modes.** `CLOSE-WAIT` accumulating
because an application did not call `close()` is a *file-descriptor* leak, diagnosed with
`lsof`, bounded by `ulimit -n` — all of it inherited from the decision to make network
endpoints look like files.

**Michael Larsen and Ted Faber.** **RFC 6335** (2011), which formalised the three ranges
of §35.3 — well-known, registered, and dynamic — and consolidated the assignment procedures.

**It is worth noticing how late this is.** The ranges had been in use since the early
1980s; the document that states them normatively appeared in 2011. **Much of networking's
foundational structure was convention long before it was specification**, which is
Chapter 23's rough-consensus principle showing its age.

**Marshall Kirk McKusick, Robert Watson and the modern BSD/Linux socket implementers.**
`SO_REUSEPORT`, and the mechanism by which several processes share one listening socket so
a server can use every CPU core.

**A small change with a large consequence:** before it, a multi-core server needed one
process to accept and then distribute, which was a bottleneck. After it, the kernel hashes
each incoming connection's five-tuple across the listening processes — **the tuple of §35.2,
used as a load-balancing key inside a single machine.**

**The same idea as ECMP and link aggregation** (Chapters 29 and 19), applied at a scale of
centimetres.

**The `ss` authors — Alexey Kuznetsov and the iproute2 maintainers.** `ss` replaced
`netstat` because `netstat` reads `/proc/net/tcp` and parses text, which on a machine with
a hundred thousand sockets takes minutes. **`ss` uses a netlink socket and asks the kernel
directly.**

**More importantly, `ss` exposes what `netstat` never did:** the per-connection congestion
window, RTT estimate, retransmission counts and congestion-control algorithm of §35.4's
`ss -tni`. **Diagnostic information that previously required a packet capture is now one
command**, and this is a genuine improvement in what a person can find out about a running
system.

**Vic Abell.** **`lsof`**, first released in 1994 and maintained by him for over
twenty-five years.

*"What process has this port open?"* is a question asked constantly and answered by
essentially nothing else portably. `lsof -i :443` works on Linux, macOS, the BSDs, Solaris
and AIX, which in 1994 mattered enormously and still does when you are on an unfamiliar
system.

**A single-maintainer tool that became infrastructure**, which is a recurring shape in this
book — `ping`, `traceroute`, `tcpdump`, `lsof`, all originally one person solving their own
problem.

**The unnamed authors of the "reserved ports need root" decision.** §35.3's privilege rule
came from BSD in the early 1980s, for a threat model — untrusted users on a shared
timesharing machine — that describes almost no modern deployment.

**It is worth carrying as an example of a security control outliving its threat model.**
It now provides essentially no protection, imposes real friction on containers and
unprivileged services, and persists because changing it would break assumptions in an
unknowable amount of software.

**Security controls are easy to add and nearly impossible to remove**, and this one has
been generating deployment complexity for forty years in defence of a machine nobody runs
any more.
