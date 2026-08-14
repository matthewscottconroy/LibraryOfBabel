# 34.2 Ping and Echo

`ping` is the first command anyone learns and the one whose results are most often
over-interpreted. This section is about what a successful ping actually proves — which
is less than people assume — and what each failure mode means.

## The exchange

Two messages.

```
   Sender  ──── ICMP Type 8, Echo Request  ────▶  Target
           ◀─── ICMP Type 0, Echo Reply    ────  
```

**The payload is arbitrary and is echoed back unchanged.** Typically 56 bytes of pattern
data, which with the 8-byte ICMP header gives the familiar **64 bytes** in Linux output.

The header carries an identifier and a sequence number, so that multiple concurrent
pings can be told apart and so that replies can be matched to requests.

## The name

Mike Muuss wrote `ping` in December 1983, in an evening, at the US Army Ballistic
Research Laboratory, to debug a network problem.

He named it after sonar — you emit a pulse and listen for the echo, and the time tells you
the distance. The backronym "Packet InterNet Groper" was invented later, and Muuss
disliked it.

His program is on every operating system ever shipped since. He died in a car accident in
2000; the source of the original still circulates and is worth reading for how small it is.

## What a successful ping proves

**More than nothing and much less than people assume.**

| Proven | Not proven |
|---|---|
| **Layer 1** — a physical path exists | any service is running |
| **Layer 2** — framing works both ways | the application works |
| **Layer 3** — routing works **in both directions** | **TCP or UDP will get through** |
| The host's IP stack is alive | performance is adequate |
| **A return path exists** | any port is open |
| Approximate round-trip latency | the firewall permits your traffic |

**The "in both directions" row is the underused one.** A reply means your packet arrived
**and** the target had a route back **and** the return path worked. **A successful ping
proves bidirectional reachability**, which eliminates a great deal (Chapter 22 §22.4's
method).

**And the right column is what people forget.** *"I can ping it, so the network is fine"*
is one of the most common wrong statements in operations. **Ping proves the network layer
works; nearly all real faults are above it.**

## What a failed ping proves

**Almost nothing**, and this asymmetry is the important part.

| Possible cause | |
|---|---|
| The host is down | plausible |
| The host is up and **ICMP is filtered** | **very common** |
| A firewall drops it | very common |
| Rate limiting discarded it | possible |
| The route exists one way only | possible |
| The host's ICMP handling is disabled | Windows Firewall does this **by default** |

**Windows blocks inbound ICMP echo by default.** So a Windows host that is running
perfectly, serving traffic, and reachable on every port you care about **does not answer
ping.** Concluding it is down is wrong, and it is a mistake made daily.

> **A successful ping is strong evidence. A failed ping is weak evidence.**

**When ping fails, test the thing you actually care about:**

```bash
# Does the port answer? (Chapter 22 §22.4)
nc -zv host 443
telnet host 443

# Is it there at Layer 2? (Chapter 18 §18.3)
arping -I eth0 192.168.1.50

# TCP-based path tracing, which firewalls usually permit
traceroute -T -p 443 host
```

**`arping` is the one to remember.** It proves presence at Layer 2 with no IP, no ICMP,
and nothing a firewall typically filters. **If `arping` succeeds and `ping` fails, the
host is definitely there and the problem is above Layer 2** — which is a large elimination
from one command.

## Reading the output

```
$ ping -c 4 example.com
PING example.com (93.184.216.34) 56(84) bytes of data.
64 bytes from 93.184.216.34: icmp_seq=1 ttl=56 time=88.2 ms
64 bytes from 93.184.216.34: icmp_seq=2 ttl=56 time=87.9 ms
64 bytes from 93.184.216.34: icmp_seq=3 ttl=56 time=88.4 ms
64 bytes from 93.184.216.34: icmp_seq=4 ttl=56 time=88.1 ms

--- example.com ping statistics ---
4 packets transmitted, 4 received, 0% packet loss, time 3005ms
rtt min/avg/max/mdev = 87.9/88.1/88.4/0.2 ms
```

Four things to read, and most people read only the last:

**The resolved address.** `ping` did DNS first. **If it resolves to something unexpected,
you have found the fault before sending a packet.**

**`ttl=56`.** Started at 64, crossed **8 hops**, and the initial value suggests a Unix host
(Chapter 24 §24.4). Free information in every reply.

**`icmp_seq`.** Sequence numbers. **Gaps mean loss; out-of-order arrival means multiple
paths** — ECMP (Chapter 29 §29.3), or something stranger.

**`mdev` — the jitter.** 0.2 ms here, which is very stable. **A low average with a high
mdev is worse than a slightly higher average that is consistent**, for anything real-time
(Chapter 66 §66.1). The average alone hides the problem, and this is the most-ignored figure
in the output.

## Diagnosing with the options

```bash
# Path MTU discovery by hand — the most valuable ping variant
ping -M do -s 1472 host          # 1472 + 28 = 1500

# Flood ping — root only; use with care and never across a WAN
ping -f -c 10000 host

# Interval and count
ping -i 0.2 -c 100 host

# Choose the source address on a multihomed host
ping -I 10.1.1.5 host

# Set the TTL deliberately
ping -t 5 host

# Continuous with a timestamp, for correlating with an incident
ping -D host
```

`ping -M do -s N` is the one to learn (Chapter 24 §24.3). It sets DF and a payload
size, so a binary search finds the path MTU — and it is the diagnosis for §34.4's black
hole.

**On Windows:** `ping -f -l 1472 host` sets DF and size; `ping -t` is continuous (**not**
TTL, which is `-i`). The flags mean different things on the two platforms, and mixing
them up is a small, common irritation.

## What the errors mean

The distinctions here are the diagnostic value, and they map directly onto §34.1's
codes:

| Output | ICMP | Meaning |
|---|---|---|
| **Destination Host Unreachable** | type 3 code 1 | **A router has a route and ARP failed.** The host is not on the segment. |
| **Destination Net Unreachable** | type 3 code 0 | **No route.** A router has nothing for that network. |
| **Destination Port Unreachable** | type 3 code 3 | Only from UDP probes — the host is up, nothing listening |
| **Communication prohibited** | type 3 code 9/10/13 | **A firewall dropped it and said so** |
| **Time to live exceeded** | type 11 | A routing loop, or too few hops allowed |
| **Request timed out** *(no reply at all)* | — | **The least informative outcome** |
| **Network is unreachable** *(local error)* | — | **Your own host has no route** (Chapter 29 §29.4) |

**Two pairs are worth separating carefully.**

**"Host unreachable" versus "timed out".** The first is an *answer* — a router told you it
could not deliver. The second is *silence*. An answer is far more useful, because it
tells you a router was reached and where the delivery failed.

**"Network is unreachable" versus everything else.** That message is generated **by your
own machine before any packet is sent** — your routing table has no match (Chapter 29
§29.4). Nothing left your host. This is a configuration fault, not a network fault, and it
appears instantly rather than after a timeout.

## Ping sweeps and why they mislead

Scanning a range to find live hosts:

```bash
nmap -sn 192.168.1.0/24
fping -a -g 192.168.1.0/24
```

**Useful, and it undercounts**, for all of §34.2's reasons: Windows hosts, filtered
segments, and anything with a host firewall will not answer.

**A ping sweep finds hosts that answer ping.** It does not find hosts. Chapter 27 §27.4's
point about IPAM applies — it also does not find allocations, so a range that is
allocated and idle looks free.

## Continuous ping during a change

**The standard technique, and it is worth doing properly.**

```bash
ping -D -i 0.2 10.1.1.1 | tee /tmp/failover-$(date +%s).log
```

**Timestamped, five per second, logged.** During a failover test (Chapter 30 §30.3), the
gap in sequence numbers **is** the outage duration, measured rather than estimated.

**And ping alone is not enough for a failover test.** It proves the network layer
recovered; it does not prove the application did. Run a request loop against the actual
service alongside it — the two often differ, and the difference is what your users
experience.

## What breaks here

"I can ping it so the network is fine." Ping proves Layer 3. Nearly all faults are
above it.

"I can't ping it so it's down." Windows blocks echo by default, and firewalls filter
it constantly. Use `arping`, or test the port.

**Concluding from a ping sweep that a range is free.** It finds responders, not
allocations.

**Reading only the average latency.** Check `mdev`. Jitter is what breaks real-time
traffic.

**Using ping to size a maintenance window's impact.** Test the application too.

**Mixing up Windows and Linux flags.** `-t` is continuous on Windows and TTL on Linux.

> **Network+ note.** Objective 5.5 expects `ping`. Over-learn: **echo request is type 8,
> echo reply type 0**; **a successful ping proves bidirectional Layer 3 reachability and
> nothing above it**; **a failed ping proves very little, because ICMP is commonly
> filtered and Windows blocks it by default**; and **"destination host unreachable" is an
> answer from a router while "request timed out" is silence.** The last distinction is
> examined and is genuinely useful.
