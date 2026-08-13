# 64.1 Reachability Tools

**Three commands, and knowing precisely what each proves and what it does not is worth more than
any number of options.**

## ping

**Send an ICMP echo request; expect an echo reply** (Chapter 34 §34.2).

```
   $ ping -c 4 10.9.0.5
   64 bytes from 10.9.0.5: icmp_seq=1 ttl=62 time=1.44 ms
   64 bytes from 10.9.0.5: icmp_seq=2 ttl=62 time=1.39 ms
   64 bytes from 10.9.0.5: icmp_seq=3 ttl=62 time=1.51 ms
   64 bytes from 10.9.0.5: icmp_seq=4 ttl=62 time=1.42 ms
   
   4 packets transmitted, 4 received, 0% packet loss
   rtt min/avg/max/mdev = 1.390/1.440/1.510/0.045 ms
```

**What a successful ping proves:**

| | |
|---|---|
| **Layers 1, 2 and 3 work, in both directions** | **the reply had to come back** |
| **The destination host is up and its stack is responding** | |
| **The path's round-trip time**, at that moment | |
| **Nothing along the path is filtering ICMP echo** | |

**What it does not prove — and this is the important list:**

> **A successful ping says nothing about whether the service you want is working.** **A host can
> ping perfectly with every application down**, and this is the commonest misreading of the
> result.

**And what a failed ping does not prove:**

| | |
|---|---|
| **The host may be up and filtering ICMP** | **very common** (Chapter 60 §60.1) |
| **A firewall may permit TCP 443 and drop ICMP** | routine policy |
| **The reply may be blocked, not the request** | **directionality** (Chapter 63 §63.3) |

> **"It doesn't ping" is not "it's down."** **Test the actual service before concluding
> anything.**

### The options that matter

| Option | Does | Use |
|---|---|---|
| **`-c N`** | **count** | **always** — an unbounded ping in a script is a mistake |
| **`-i N`** | interval | `-i 0.2` for a finer picture; requires privilege below 0.2 |
| **`-s N`** | **payload size** | **MTU testing** |
| **`-M do`** | **set Don't Fragment** | **MTU testing** (Chapter 66 §66.3) |
| **`-I <if\|addr>`** | **source interface or address** | **essential on multi-homed hosts** |
| **`-t N`** | set TTL | manual path probing |
| **`-f`** | flood | **only on your own equipment**, and it is a load test |
| **`-D`** | timestamps | correlating with logs |

**Windows:** `ping -n 4`, `-l <size>`, `-f` (Don't Fragment — **note that `-f` means something
entirely different from Linux's**), `-i <ttl>`, `-S <source>`.

### MTU testing, which is the highest-value use

```
   $ ping -M do -s 1472 -c 1 10.9.0.5      # 1472 + 28 = 1500
   PING 10.9.0.5 56(84) bytes of data.
   64 bytes from 10.9.0.5: icmp_seq=1 ttl=62 time=1.51 ms

   $ ping -M do -s 1473 -c 1 10.9.0.5
   ping: local error: message too long, mtu=1500
   
   # Across a tunnel:
   $ ping -M do -s 1472 -c 1 10.2.0.9
   (no reply — silently dropped: the PMTUD black hole)
   $ ping -M do -s 1400 -c 1 10.2.0.9
   64 bytes from 10.2.0.9: icmp_seq=1 ttl=62 time=14.2 ms
```

> **Add 28 to the `-s` value to get the IP MTU** — 20 bytes of IP header plus 8 of ICMP.
> **Bisect on the size until it fails**, and you have the path MTU (Chapter 66 §66.3).

**And note the difference between the two failures above:** **a local error names the MTU; a
silent drop is the black hole** — **the second means something upstream is discarding without
telling you** (Chapter 60 §60.1's ICMP filtering).

### Reading what comes back

| Response | Means |
|---|---|
| **Reply** | it worked |
| **`Destination Host Unreachable` from an intermediate** | **a router has no route or ARP failed** — **and note which router said it** |
| **`Destination Net Unreachable`** | a routing problem, further away |
| **`Destination Port Unreachable`** | **not from ping** — this is what UDP traceroute uses |
| **`Time to live exceeded`** | **a routing loop**, or a TTL set too low |
| **Request timeout** | **no response at all** — filtered, or genuinely down |
| **`Packet filtered` / admin prohibited** | **a firewall said so explicitly**, which is helpful |
| **Duplicate replies (`DUP!`)** | **a broadcast or a bridging loop** |

**The TTL in the reply is free information:** **it tells you how many hops away the responder
is**, given a guess at the initial value (64 for Linux, 128 for Windows, 255 for many network
devices). **`ttl=62` from a Linux host means two hops.**

## traceroute and tracert

**Send packets with increasing TTL; each router that decrements to zero returns Time Exceeded**
(Chapter 34 §34.3).

```
   $ traceroute -n 203.0.113.10
    1  10.20.0.1      0.412 ms   0.398 ms   0.401 ms
    2  10.0.0.1       1.221 ms   1.198 ms   1.211 ms
    3  198.51.100.1   8.441 ms   8.502 ms   8.398 ms
    4  * * *
    5  203.0.113.10  12.104 ms  12.088 ms  12.201 ms
```

**Three probes per hop, three round-trip times.**

**The crucial misreading, and it is universal:**

> **The latency shown at hop N is the round trip to hop N, not the latency of hop N.** **A high
> figure at hop 4 that returns to normal at hop 5 does not mean hop 4 is slow.** **It means hop
> 4's control plane was slow to generate an ICMP response** — which is deprioritised on every
> router, by design.

**And the second:**

> **Asterisks are not packet loss.** **They mean no ICMP Time Exceeded was returned**, which is
> usually a router configured not to send them, or rate-limiting them. **Traffic passes through
> that hop perfectly.**

**When to worry:**

| Pattern | Meaning |
|---|---|
| **Latency rises at hop N and stays high** | **something real changed at hop N** |
| **Latency rises at hop N and falls at N+1** | **control-plane deprioritisation.** Ignore |
| **`* * *` at one hop, replies after it** | **that hop does not respond.** Ignore |
| **`* * *` from hop N to the end** | **the path is broken at or after N** |
| **The same address repeating** | **a routing loop** |
| **The path changes between runs** | **ECMP** (Chapter 29 §29.3) — normal |

**The transport used differs, and it matters:**

| Platform | Default probes | Consequence |
|---|---|---|
| **Linux `traceroute`** | **UDP, high ports** | **frequently blocked**; use `-I` or `-T` |
| **Windows `tracert`** | **ICMP echo** | blocked where ICMP is |
| **`traceroute -T -p 443`** | **TCP SYN to port 443** | **gets through where the others do not** |

> **If traceroute stops and the destination is reachable, change the probe type before
> concluding anything.** **`traceroute -T -p 443` succeeding where UDP traceroute fails tells
> you the path is fine and the probes were filtered.**

**And the return path is invisible.** **Traceroute shows the forward path only** — **each Time
Exceeded travels back by whatever route the router chooses**, which may differ. **Asymmetric
routing is common and traceroute cannot show it**, which is why a traceroute from each end is
worth having.

## mtr

**Traceroute and ping combined, run continuously** — **and it is the right tool for intermittent
loss.**

```
   $ mtr -rwzbc 100 203.0.113.10
   HOST                          Loss%  Snt  Last   Avg  Best  Wrst StDev
   1. AS???  10.20.0.1            0.0%  100   0.4   0.4   0.3   1.2   0.1
   2. AS???  10.0.0.1             0.0%  100   1.2   1.3   1.1   4.8   0.4
   3. AS64512 198.51.100.1       12.0%  100   8.4   8.6   8.2  22.1   1.8
   4. AS64512 198.51.100.9       12.0%  100   9.1   9.3   8.9  24.0   2.1
   5. AS64500 203.0.113.10        0.0%  100  12.1  12.3  11.9  15.2   0.6
```

**Reading it correctly is the skill, and one rule dominates:**

> **Loss at an intermediate hop that does not appear at subsequent hops is not loss. It is ICMP
> rate limiting.**

**In the output above, hops 3 and 4 show 12% loss and hop 5 shows 0%.** **The traffic is not
being lost** — if it were, hop 5 could not be at 0%. **Those routers are rate-limiting their
own ICMP responses.**

**Loss that begins at hop N and persists to the destination is real.**

| Pattern | Real? |
|---|---|
| Loss at hop 3, none at 4 and 5 | **no — rate limiting** |
| **Loss at hop 3, and at 4, and at 5** | **yes — starting at or before hop 3** |
| Loss only at the final hop | **yes — the destination, or its last link** |
| Rising latency and loss together, at the same hop onward | **congestion** |

**Useful flags:** **`-r`** report mode, **`-c N`** cycles, **`-w`** wide, **`-z`** show AS
numbers, **`-b`** show both name and address, **`-T`/`-u`** TCP/UDP probes, **`-P`** port.

## Choosing among them

| Question | Tool |
|---|---|
| **Is it reachable at all?** | **`ping`** |
| **What is the path?** | **`traceroute`** |
| **Where is the loss?** | **`mtr`, over at least 100 cycles** |
| **What is the path MTU?** | **`ping -M do -s`, bisecting** |
| **Is the port open?** | **not these** — §64.4's `nc`, `nmap` or `Test-NetConnection` |
| **Is the service working?** | **not these** — use the application's own client |

**And the ordering that costs least:**

1. **`ping` the destination by IP** — one command, and it eliminates most of the stack
2. **`ping` by name** — Chapter 63 §63.3's highest-yield test
3. **`traceroute` if ping fails or is slow**
4. **`mtr` if the problem is intermittent**
5. **A port test** before concluding the network is at fault

## What breaks here

**A host that pings and whose service does not work.** **Ping proved Layers 1–3.** Test the
service.

**A host that does not ping and whose service works fine.** **ICMP is filtered.** Extremely
common.

**Traceroute showing a slow hop in the middle and a fast destination.** **Control-plane
deprioritisation.** Not a fault.

**Asterisks in the middle of an otherwise complete traceroute.** **That router does not respond
to probes.** Not a fault.

**Traceroute stopping entirely at hop 6.** **Change the probe type** before concluding the path
is broken.

**mtr showing 15% loss at hop 3 and 0% at the destination.** **Rate limiting**, not loss.

**MTU testing that succeeds locally and fails across a tunnel.** **The tunnel's overhead**
(Chapter 61 §61.1). Bisect to find the real value.

**`ping` from a multi-homed host testing the wrong path.** **Specify the source** with `-I`.

**A high `mdev`/StDev with a low average.** **Jitter** (Chapter 3 §3.3) — and it matters far
more than the average for voice.

> **Network+ note.** Objective 5.5 covers these directly. Over-learn: **`ping` tests
> reachability using ICMP echo**; **`traceroute`/`tracert` shows the path using TTL
> expiry**; **`pathping` and `mtr` combine both**; **`tracert` uses ICMP and Linux `traceroute`
> uses UDP by default**; and **a firewall may block ICMP, so a failed ping does not prove the
> host is down.** The last point is examined and is the one people get wrong in practice.
