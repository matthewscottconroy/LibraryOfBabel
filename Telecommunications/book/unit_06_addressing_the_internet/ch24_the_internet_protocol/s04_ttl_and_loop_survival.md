# 24.4 TTL and Loop Survival

Chapter 19 established what a loop does to an Ethernet network: total collapse in under
a second, from one frame, with no self-limiting mechanism, because **an Ethernet frame
has no hop count**.

IP has one. This section is about the eight bits that make the difference, and about
the diagnostic tool they make possible.

## The field

**Time To Live**, 8 bits, and **it is a hop count**. The name is a historical
inaccuracy that has confused students for forty years.

RFC 791's intent was genuinely temporal: a measure in seconds, with a router holding a
packet longer than a second obliged to decrement by more than one. **No implementation
ever did this**, partly because measuring the holding time was awkward and partly
because nobody held packets that long. RFC 1812 formalised what everyone was already
doing: decrement by exactly one, per hop.

**IPv6 renamed it `Hop Limit`**, which is what it always was.

## The rule

```
   At every router:
      TTL ← TTL − 1
      if TTL == 0:
          drop the packet
          send ICMP Time Exceeded (type 11, code 0) to the source
```

**Note what is not on that list:** the source is not asked, the packet is not held, and
nothing is retried. The packet is destroyed and its sender is told.

## Why this saves the Internet

A routing loop is not an exotic failure. It happens routinely:

- During **convergence**, after a topology change, while different routers hold
  inconsistent views (Chapter 31 §31.4)
- From a **static route** pointing at a router that points back
- From **route redistribution** between protocols without filtering
- From a **misconfigured default route** on two devices pointing at each other

**Without TTL**, a packet entering such a loop would circulate until the loop was fixed
by a human. Every packet entering would join it. The loop's links would saturate, and —
exactly as in Chapter 19 §19.1 — the network would collapse and stay collapsed.

**With TTL**, a looping packet dies after at most 255 hops. A loop between two routers
kills a packet in a fraction of a second. The links carry the extra traffic briefly and
recover automatically the moment routing converges.

> **TTL does not prevent loops. It makes them survivable.** The distinction matters: a
> transient loop during convergence is normal and harmless, and permanent loops are a
> configuration fault that TTL contains rather than fixes.

**This is the design difference between Layer 2 and Layer 3**, and it is why Perlman's
argument for routing rather than bridging (Chapter 19, Chapter 67) is fundamentally
about this one field.

## Initial values

Set by the sending host, and it is a fingerprint:

| System | Initial TTL |
|---|---|
| Linux, macOS, modern BSD | **64** |
| Windows (all modern versions) | **128** |
| Cisco IOS, most network devices | **255** |
| Older Solaris | 255 |
| Some embedded stacks | 30, 32, 60 |

**Reading it backwards:** a `ping` reply with TTL 57 most likely started at 64 and
crossed **7** hops. TTL 122 started at 128 and crossed 6.

```
$ ping -c1 example.com
64 bytes from 93.184.216.34: icmp_seq=1 ttl=56 time=88.2 ms
                                          ^^^^^^
                              started at 64, crossed 8 hops,
                              and it is probably a Unix host
```

Free information in every reply. It is not authoritative — the initial value can be
changed — but it is right most of the time, and `nmap -O` uses it among other signals.

## Traceroute

The best-known deliberate use of TTL, and one of the great pieces of tool design.

**Van Jacobson, 1987**, and the trick is this: **send packets with deliberately small
TTLs and collect the errors.**

```
   TTL=1 →  first router decrements to 0, drops,
            returns ICMP Time Exceeded  →  you learn hop 1

   TTL=2 →  dies at the second router   →  you learn hop 2

   TTL=3 →  dies at the third           →  you learn hop 3
   …
   TTL=n →  reaches the destination, which returns something else
            (Port Unreachable, or Echo Reply)  →  done
```

Each hop is discovered by **deliberately causing an error and reading the reply's source
address.** Three probes per hop, by convention, to show variation.

```
$ traceroute example.com
 1  192.168.1.1        1.2 ms   1.1 ms   1.1 ms
 2  10.0.0.1           8.4 ms   8.2 ms   8.9 ms
 3  * * *
 4  203.0.113.1       12.1 ms  11.9 ms  12.4 ms
 5  93.184.216.34     88.2 ms  88.1 ms  88.4 ms
```

### Reading it correctly

Four things people get wrong, and they matter.

**`* * *` does not mean the packet was dropped.** It means the router did not *reply* —
which usually means ICMP Time Exceeded is rate-limited or filtered on that device.
**Traffic passes through it fine.** If hop 3 shows stars and hops 4 and 5 do not, hop 3
is forwarding perfectly and simply declining to introduce itself.

**Latency to intermediate hops is not path latency.** The value is the round trip to
*that router's control plane*, which is a slow CPU that treats generating ICMP as its
lowest priority. **A high figure at hop 4 followed by a low one at hop 5 is normal and
means nothing.** Only the final hop's latency is meaningful.

**The path may differ per probe.** ECMP (Chapter 29 §29.3) means successive packets may
take different routes, so the three probes at one hop may show three different routers.
`traceroute` with a fixed flow label, or `paris-traceroute`, addresses this.

**The reverse path is invisible.** Each ICMP reply travels back by its own route, which
may be entirely different. **You are measuring a round trip whose return half you cannot
see**, which is why asymmetric routing problems are so hard to diagnose from traceroute
alone.

### The implementations differ

| Tool | Probe | Why it matters |
|---|---|---|
| **Unix `traceroute`** | UDP to high ports (33434+) | Often filtered |
| **Windows `tracert`** | **ICMP Echo** | Often permitted where UDP is not |
| **`traceroute -I`** | ICMP Echo | Unix, matching Windows behaviour |
| **`traceroute -T`** | **TCP SYN to port 80/443** | **Gets through firewalls** |
| **`mtr`** | continuous, either | Shows loss and variance over time |

**When traceroute shows nothing and connectivity works, try `-T` to the port you
actually care about.** A firewall permitting TCP/443 and dropping UDP is completely
normal, and it makes the default traceroute useless while the path is perfect.

**`mtr` is the better tool for most purposes.** It probes continuously and reports
per-hop loss and jitter, which distinguishes a genuinely lossy hop from one that merely
rate-limits its own ICMP — a distinction a single traceroute cannot make.

## Other TTL uses

**GTSM** (Chapter 18 §18.4): send at TTL 255 and require receipt at 255, proving the
sender is on-link. Used by NDP and by BGP (Chapter 32 §32.2).

**Multicast scoping**: TTL was originally used to bound multicast distribution — TTL 1
for the local segment, 32 for the site, and so on. Administrative scoping largely
replaced it (Chapter 27 §27.3).

**Loop detection in tunnels**: a tunnel whose endpoints are reachable *through the
tunnel* creates a recursive loop, and TTL is what stops it consuming the router.

## What breaks here

**`* * *` in traceroute, then normal hops afterwards.** ICMP filtered or rate-limited at
that hop. **Not a fault.**

**High latency at a middle hop, normal at the end.** Control-plane priority. **Not a
fault.**

**Traceroute failing entirely while the application works.** UDP probes filtered. Use
`-I` or `-T`.

**TTL expired for a destination that should be close.** A routing loop. Look at hops
repeating in the output — a loop shows as an alternating pair of addresses.

**A destination unreachable with fewer than 255 hops available.** Some path exceeds the
initial TTL, which on a well-connected Internet essentially always means a loop rather
than genuine distance. The Internet's diameter is under 30 hops in practice.

> **Network+ note.** Objectives 1.4 and 5.5 expect TTL and `traceroute`/`tracert`.
> Over-learn: **TTL is a hop count, decremented at each router**; **at zero the packet
> is dropped and ICMP Time Exceeded is sent**; **traceroute works by incrementing TTL**;
> **`* * *` means no reply, not no path**; **Windows `tracert` uses ICMP, Unix
> `traceroute` uses UDP by default**. The last one is examined and explains a real
> operational difference.
