# Chapter 64 — The People

**Mike Muuss (1958–2000).** `ping`, written in one evening in December 1983.

**The circumstances are worth having.** Muuss was at the US Army Ballistic Research
Laboratory, and David Mills had mentioned measuring round-trip latency with ICMP echo
packets in conversation. Muuss wrote the program that night, to diagnose a specific
network problem he was having.

**The name is not an acronym.** Muuss was explicit about it:

> "I named it after the sound that a sonar makes, inspired by the whole principle of
> echo-location."

The backronym "Packet InterNet Groper" was applied later by someone else, and Muuss found it
irritating.

Its consequence is out of all proportion to its size. `ping` is on every operating system,
is the first command every network engineer learns, and its absence from a system is
remarkable. Muuss wrote perhaps a few hundred lines and it has been in continuous use for
over forty years.

> Muuss also worked on BRL-CAD and on ray tracing, and considered `ping` a minor piece of
> work. He was killed in a road accident in 2000, aged 42. His `ping` page — still
> hosted, and worth reading — includes the sentence "The best ping story I've ever heard was
> told to me at a conference…" and the story involves diagnosing a fault by listening to the
> speaker beep.

Van Jacobson (b. 1950), for the third time in this book — `traceroute`, 1987, and
`tcpdump`.

Traceroute's mechanism is a piece of lateral thinking worth admiring (Chapter 34 §34.3):
there is no protocol for "tell me the path", and Jacobson realised that TTL expiry already
produces exactly the required behaviour — each router that discards a packet identifies
itself.

> He did not add a feature to IP. He noticed that an existing error-reporting mechanism, used
> deliberately, was a path discovery protocol — which is the kind of observation that
> characterises his work.

`tcpdump`, with Craig Leres and Steven McCanne, is the other lasting one. And beneath it,
the more significant contribution:

Steven McCanne and Van Jacobson (1993). "The BSD Packet Filter."

BPF is a small virtual machine for packet matching, executed in the kernel, so that packets
that do not match are discarded without ever being copied to userspace. Which is what makes
capture on a busy interface feasible at all.

> **And BPF's second life is larger than its first.** eBPF — the extended version — is now the
> mechanism underneath Linux's networking, tracing, security and observability tooling
> (Chapter 67 §67.1's container networking, and much of modern kernel instrumentation). A
> packet-matching engine from 1993 became the general-purpose kernel extensibility mechanism,
> which nobody planned.

**Gerald Combs.** Ethereal, 1998 — renamed Wireshark in 2006.

Combs was a network engineer at a small ISP who needed a protocol analyser and could not
afford one. Commercial analysers cost thousands of dollars per seat, and the free tools
of the time decoded very little.

**He wrote one and released it.** The consequence was the dissector model:

> **Anyone can write a dissector for a protocol.** **Which means Wireshark decodes several
> thousand protocols**, including many that no commercial vendor would ever have implemented —
> obscure industrial protocols, vendor-proprietary formats, protocols with a hundred users.

No commercial product has ever approached that coverage, and it is a straightforward
consequence of the licence rather than of effort.

The 2006 rename was a trademark problem — Combs changed employer and the Ethereal name did
not travel with him — and the project moved wholesale within days, which is itself a
demonstration of what open development permits.

**Fyodor (Gordon Lyon).** `nmap`, 1997 — and the disclosure argument in its
sharpest form.

Nmap was published in *Phrack* magazine, and it made port scanning and OS fingerprinting
available to everyone, including people with no legitimate purpose.

Lyon's defence has been consistent for nearly thirty years:

> **Attackers already had these capabilities. Defenders did not.** A tool that reveals what
> your network exposes is a tool that lets you fix it, and the alternative — that only
> attackers know what is reachable — is worse.

**And the evidence supports him.** Nmap is standard in every security assessment, in every
audit, and in the toolkit of every competent network engineer, and the practice of scanning
your own perimeter regularly is now considered basic diligence rather than suspicious.

Lyon has also been notably principled about the tool's use — maintaining the licence,
resisting attempts to bundle it with malware or with commercial products that misrepresent it,
and publishing the "legal issues" documentation that §64.4 paraphrases.

Ajay Tirumala, Mark Gates and the NLANR/DAST group. `iperf`, from about 1999 — and
`iperf3` is a separate rewrite by ESnet.

**The tool's contribution is standardisation.** Before it, "how fast is this link?" was
answered by copying a file and dividing, which measures the disk, the filesystem, the
application and the network together.

> `iperf` measures the network and nothing else, and both ends run the same code — which
> means **two organisations can compare results**, and that is why carriers accept it as
> evidence and why it appears in contracts.

**Its weakness is §64.4's:** it is easy to run and easy to run wrongly, and the
single-stream result is quoted in disputes far more often than it should be.

## What this chapter's tools have in common

Four of the six were written by one person, for their own immediate problem, and given away.

`ping` in an evening. `traceroute` from an observation. Ethereal because the commercial ones
were too expensive. `nmap` in a magazine.

> **None of them was a product.** **Each solved a problem the author personally had**, and
> each became universal because the problem was universal and the solution was free.

And the second observation: they have all outlasted their assumptions. `ping` assumes ICMP
is permitted. `traceroute` assumes routers send Time Exceeded. Both assumptions are now
frequently false, and the tools are still the first two commands anyone runs — because a
partially-informative answer arriving in one second beats a complete answer arriving in twenty
minutes.

Which is the honest reason they persist, and it is worth knowing when you are reading their
output.
