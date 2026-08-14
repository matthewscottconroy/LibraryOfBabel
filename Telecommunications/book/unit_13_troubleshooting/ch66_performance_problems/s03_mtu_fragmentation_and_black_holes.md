# 66.3 MTU, Fragmentation and Black Holes

One symptom identifies this section immediately, and it is worth stating before anything
else:

> **Small packets work. Large packets vanish.**

Pings succeed. SSH connects and then hangs when output scrolls. A web page's HTML loads and its
images do not. A file transfer starts and stalls. A VPN establishes and carries nothing.

All of them are the same fault, and it is diagnosed in one command.

## The mechanism

Chapter 24 §24.3 gave the protocol; this is what goes wrong.

```
   Host A ── MTU 1500 ── Router ── MTU 1400 ── Router ── MTU 1500 ── Host B
                                    (a tunnel, Ch 61 §61.1)

   1. Host A sends a 1500-byte packet with DF set (which everything does)
   2. The tunnel ingress cannot forward it and cannot fragment it
   3. It should send ICMP Type 3 Code 4: "Fragmentation Needed, MTU 1400"
   4. Host A should reduce its segment size and retry

   The black hole: step 3's ICMP is filtered somewhere (Ch 60 §60.1),
   so step 4 never happens, and Host A retransmits the same oversized
   packet until it gives up.
```

**And IPv6 removes the fallback entirely:**

> **IPv6 routers do not fragment.** **Only the source may**, so Path MTU Discovery is not an
> optimisation in IPv6 — it is mandatory, and **filtering ICMPv6 Packet Too Big breaks IPv6
> completely** (Chapter 60 §60.1).

## Diagnosing it in one command

```
   $ ping -M do -s 1472 -c 1 <destination>       # 1472 + 28 = 1500
   ping: local error: message too long, mtu=1500   ← local interface too small
   
   # or, across the problem path:
   (no reply at all)                                ← the black hole

   $ ping -M do -s 1372 -c 1 <destination>       # 1372 + 28 = 1400
   64 bytes from …: icmp_seq=1 ttl=61 time=14.2 ms  ← works
```

Bisect between the two to find the exact path MTU.

| `-s` value | + 28 | Result |
|---|---|---|
| 1472 | 1500 | **fails** |
| 1422 | 1450 | **fails** |
| 1372 | 1400 | works |
| 1397 | 1425 | fails |
| 1384 | 1412 | works |
| **1392** | **1420** | **works — and 1393 fails** |

**Windows:** `ping -f -l 1472 <dest>` — note that `-f` means Don't Fragment here, which
is `-M do` on Linux, and `-f` on Linux means flood. Confusing the two on a production
network is memorable.

## Where the MTU actually changes

A checklist, because the culprit is usually one of these.

| Cause | Typical MTU | Chapter |
|---|---|---|
| **IPsec ESP tunnel** | **~1442** | 61 §61.1 |
| **IPsec with NAT-T** | **~1434** | 61 §61.1 |
| **GRE** | **1476** | 61 §61.1 |
| **GRE over IPsec** | **~1418** | 61 §61.1 |
| **WireGuard** | **1420** | 61 §61.3 |
| **PPPoE** | **1492** | 49 §49.1 — **and this is very common on DSL** |
| **VXLAN** | **1450** | 67 §67.2 |
| **MPLS** | **−4 per label** | 50 §50.4 |
| **802.1Q tag** | **−4** | 20 §20.2 — **and some devices count it and some do not** |
| **A jumbo frame mismatch** | **9000 one side, 1500 the other** | |

The jumbo case deserves its own note because it fails differently:

> A device configured for a 9,000-byte MTU sending to one configured for 1,500 produces
> "giants" on the receiving interface (§66.2) **and the frames are discarded.** And jumbo
> frames must be configured consistently on every device in the path — every switch, every
> router, every host — because one device with the default breaks it for everyone.

## The fixes, in order of preference

### Set the interface MTU correctly

The tunnel knows its own overhead. Configure it.

```
   interface Tunnel0
    ip mtu 1400
```

Correct, and insufficient alone — because it only affects traffic originating on that
device, and the hosts behind it still send 1500-byte packets.

### Clamp the TCP MSS

The fix that works reliably and requires nothing of the endpoints.

```
   interface Tunnel0
    ip tcp adjust-mss 1360
```

The device rewrites the MSS option in passing SYN packets, so both ends negotiate a segment
size that fits — and neither end knows anything unusual happened.

$$\mathrm{MSS} = \mathrm{MTU} - 40 \quad\text{(IPv4: 20 IP + 20 TCP)}$$

**1400-byte MTU → 1360 MSS.** For IPv6, subtract 60.

> This is why MSS clamping is deployed almost universally on tunnels, and it is the first
> thing to check when a tunnel exhibits this symptom.

Its limitation is exact: it fixes TCP and only TCP. UDP-based protocols — QUIC, some VPNs,
DNS over UDP with large responses, and any application using UDP directly — are unaffected,
and they must discover the MTU themselves or be configured.

### Permit ICMP Type 3 Code 4

Which should be done anyway (Chapter 60 §60.1), and cannot be relied upon, because the
filtering is frequently in a network you do not control.

### PMTUD black hole detection

**A host-side mitigation** — RFC 4821's Packetization Layer PMTUD. The host detects that
retransmissions of large segments are failing while small ones succeed, and reduces its segment
size without any ICMP at all.

```
   $ sysctl net.ipv4.tcp_mtu_probing=1     # enable on black-hole detection
```

It works, it is slow — several retransmission timeouts before it triggers — and it is off by
default on many systems. Turning it on is a reasonable defensive measure and not a fix.

## Fragmentation itself, and why it is avoided

When fragmentation does occur, it is expensive and fragile.

| Problem | |
|---|---|
| **Loss of one fragment loses the whole packet** | **and the loss rate multiplies by the fragment count** |
| **Reassembly costs memory and CPU** at the destination | |
| **Only the first fragment carries the transport header** | **so firewalls cannot classify the rest** (Chapter 57 §57.4) |
| **Overlapping fragments were an attack** | and are now generally dropped |
| **Many firewalls drop fragments outright** | which turns a performance problem into a failure |
| **IPv6 forbids router fragmentation** | |

> The correct approach is to avoid fragmentation rather than to make it work, which is why
> Path MTU Discovery exists and why MSS clamping is the practical remedy.

## The symptom catalogue

**Because recognising these saves the diagnosis entirely.**

| Symptom | |
|---|---|
| **`ping` works, `ssh` connects and hangs on the first long output** | **the classic** |
| **A web page's text loads and its images do not** | small request, large response |
| **A file transfer starts and stalls at a few kilobytes** | |
| **A VPN establishes and carries no useful traffic** | |
| **Email works and attachments fail** | |
| **A database connection opens and queries hang** | |
| **Everything works from the office and fails over the VPN** | **the tunnel's overhead** |
| **Works over IPv4, fails over IPv6** | **ICMPv6 filtered** |
| **Retransmissions of the same large segment, repeatedly, in a capture** | **the definitive evidence** (Chapter 64 §64.3) |

**And the capture signature is unambiguous:**

```
   14:22:01.104  A → B  [SYN] MSS=1460
   14:22:01.187  B → A  [SYN,ACK] MSS=1460
   14:22:01.188  A → B  [ACK]
   14:22:01.190  A → B  [PSH,ACK] len=1448        ← large segment
   14:22:01.390  A → B  [PSH,ACK] len=1448        ← retransmission
   14:22:01.790  A → B  [PSH,ACK] len=1448        ← again
   14:22:02.590  A → B  [PSH,ACK] len=1448        ← again, doubling
```

> The handshake completes — small packets — and the first large segment is retransmitted with
> exponential backoff and never acknowledged. **No ICMP arrives.** That pattern is an MTU
> black hole and nothing else.

## What breaks here

**`ping` works and everything else hangs.** **MTU.** One command settles it.

A tunnel that establishes and passes nothing useful. **MTU** (Chapter 61 §61.1), before
anything else.

MSS clamping configured and UDP applications still failing. It fixes TCP only.

**A web page loading without images.** Small request, large response.

**IPv6 broken entirely after a firewall change.** ICMPv6 Packet Too Big filtered — and IPv6
routers cannot fragment.

**Giants counted on one interface.** **Jumbo frame mismatch**, and every device in the path must
agree.

`ping -f` run on Linux expecting Don't Fragment. **It floods.** `-M do`.

**A path MTU that differs by direction.** Possible, and it happens with asymmetric routing
(Chapter 65 §65.3). Test both ways.

> **Network+ note.** Objective 5.4 covers MTU issues. Over-learn: **MTU mismatch causes
> fragmentation or dropped packets**; **the standard Ethernet MTU is 1500**; jumbo frames are
> typically 9000 and must be configured consistently end to end; PMTUD uses ICMP and
> breaks when ICMP is blocked; and **MSS clamping is a common remedy on tunnels.** The
> "small packets work, large ones fail" symptom is examined and is the fastest recognition in
> this book.
