# Chapter 64 — The Toolbox

Every tool in this chapter answers a question. The skill is not operating the tools —
they are simple — but knowing which question you are asking, and therefore which
tool will answer it and what its answer licenses you to conclude.

That framing matters because the commonest error with these tools is
over-interpretation. `ping` fails and someone concludes the host is down. Traceroute
shows loss at hop 6 and someone opens a ticket with a transit provider. A cable tester
shows continuity and someone concludes the cable is fine. In each case the tool worked
correctly and the conclusion did not follow.

## Reachability

§64.1 covers `ping`, `traceroute`/`tracert`, and `mtr`, and it is largely a
consolidation of Chapter 34 with the emphasis on interpretation.

The points worth restating because they are the ones misapplied daily:

`ping` failing proves almost nothing — firewalls, rate limits and host policy all
block ICMP routinely (Chapter 34 §34.2). `ping` succeeding proves a great deal —
bidirectional Layer 3 reachability and a live stack — and proves nothing about any
application.

**Read all five numbers.** `min` approximates the path's irreducible delay; `avg −
min` estimates queueing; `max` reveals excursions; `mdev` proxies jitter; and the loss
percentage is the one everyone reads. Chapter 3 §3.3 made this point and it is the
single most useful habit in the chapter.

**Vary the packet size.** `ping -s 1400` versus `ping -s 64` distinguishes an MTU
problem from everything else in one test, and it is the fastest route to diagnosing the
PMTUD black hole of Chapter 34 §34.4.

Traceroute's intermediate hops lie about themselves and tell the truth about the
path. Loss or latency at hop *n* that does not persist at hop *n+1* is ICMP rate
limiting, not a fault. Only loss that propagates is real.

**`mtr` beats both** for intermittent problems, because it runs continuously and shows
per-hop statistics accumulating over time — which is what an intermittent fault looks
like and what a single traceroute cannot show.

## Names and addresses

§64.2 covers `dig`, `nslookup`, `host`, and the address-inspection commands.

`dig` is strongly preferred over `nslookup` and it is worth saying why: `dig`
shows you the actual DNS response — the flags, the authority section, the TTL, which
server answered — while `nslookup` shows a summary and hides exactly the details you
need. `dig +trace` walks the delegation from the root (Chapter 39 §39.2), which is the
tool that distinguishes a broken delegation from a broken record.

The technique that resolves most DNS incidents in one step: query the authoritative
server directly (`dig @ns1.example.com www.example.com`) and compare with what your
resolver returns. If they differ, it is caching. If they agree and are wrong, it is the
record. That single comparison eliminates half the possibilities.

The address side: `ip addr` / `ip route` / `ip neigh` on Linux, `ipconfig /all` on
Windows, and the ARP cache — where an `incomplete` entry is the signature of the
Chapter 18 §18.3 failure and is diagnostic on sight.

## Packet capture

§64.3 covers `tcpdump` and Wireshark, and it is the chapter's most important section
because capture is the tool that ends arguments.

Every other tool gives you a summary or an inference. A capture gives you what
actually crossed the wire, which is the ground truth against which every claim can be
checked. "The server never received the request" and "the server received it and did
not reply" are different problems with different owners, and a capture distinguishes
them in seconds where a discussion can consume a day.

The practical content:

**Capture filters versus display filters.** Capture filters (BPF syntax) decide what is
recorded and cannot be undone; display filters decide what is shown and can be changed
freely. Capture broadly, filter narrowly, unless volume forbids it.

**Where to capture matters more than how.** A capture at the client and a capture at
the server, taken simultaneously and compared, locate a fault to a segment. A capture
at only one end frequently cannot distinguish "not sent" from "not received."

**Getting the traffic.** Switched networks (Chapter 17) do not deliver other hosts'
frames to your port. A SPAN/mirror port, a network tap, or capturing on the endpoint
itself are the options, and each has a limitation worth knowing — SPAN ports drop
under load and are the first thing to doubt when a capture shows impossible loss.

**Reading a capture** — the filters that earn their keep (`tcp.analysis.flags`,
`tcp.stream eq N`, `http.request`, `dns`), Follow Stream, the Expert Information
panel, and the IO graph for spotting bursts. And the signatures from Chapter 37:
duplicate ACKs, retransmissions, zero windows, resets versus timeouts.

## Performance and discovery

§64.4 covers the rest.

**`iperf3`** for throughput, with the four options that matter (`-P` parallel, `-u`
UDP with jitter and loss, `-R` reverse, `-t` duration) and the warning from Chapter 3
§3.1 that a default single-stream ten-second test measures a single stream for ten
seconds and not the path's capacity.

**`nmap`** for discovery and port scanning — used to establish what is listening,
which is both a troubleshooting question and a security one, with the necessary note
that scanning networks you do not administer is at best rude and frequently unlawful.

**Physical layer tools**, which network engineers under-use because they are unfamiliar:

- A **continuity tester** proves the wires are connected. It does **not** prove the
  cable will carry gigabit — that requires a **certifier**, which measures attenuation,
  crosstalk, return loss and delay skew against the category's specification. The gap
  between these two instruments is where a great many intermittent faults live.
- A tone generator and probe finds which cable in a bundle is which.
- An **optical power meter** measures received light in dBm and compares it against the
  budget of Chapter 10 §10.3, which is how a dirty connector is found.
- An **OTDR** locates a fault along a fibre by distance, which is how you avoid
  excavating a kilometre of duct.
- A **Wi-Fi analyser** shows networks, channels and signal; a **spectrum analyser**
  shows *everything* radiating, including the non-Wi-Fi sources of Chapter 43 §43.4
  that a Wi-Fi analyser is blind to.

**Device `show` commands**, and specifically the interface counters — CRC errors,
runts, giants, input errors, output drops, collisions, late collisions — each of which
is a fingerprint of a specific fault from Chapter 6 and each of which Chapter 66 maps
to its cause.

## By the end you will be able to

- Choose the right tool for a stated question and say what its answer will and will
  not establish.
- Read all five `ping` statistics and derive queueing and jitter estimates.
- Use `dig` to distinguish a caching problem from a record problem from a delegation
  problem.
- Capture in the right place, filter effectively, and identify the standard TCP
  signatures.
- Measure throughput without producing a misleading result.
- Distinguish a continuity tester from a certifier and say when each is sufficient.
- Map an interface counter to the physical cause it indicates.
