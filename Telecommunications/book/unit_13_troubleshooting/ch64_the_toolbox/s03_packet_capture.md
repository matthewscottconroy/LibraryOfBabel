# 64.3 Packet Capture

The tool of last resort and first principles. Everything else in this chapter reports what
a device believes; a capture shows what actually crossed the wire.

## When to capture, and when not to

Capturing is expensive in time and produces a great deal of data, so it is not the first
tool.

| Reach for a capture when | Do not when |
|---|---|
| **The devices' own counters disagree with the symptom** | **a `show` command would answer it** |
| **You need to see what is actually in a packet** | you need to know whether a port is open |
| **A protocol is behaving unexpectedly** | **you have not yet checked the obvious** (Chapter 63 §63.2) |
| **You need proof for a third party** — a vendor, a carrier | |
| **The fault is intermittent** and you can capture continuously | |

> A capture is evidence in a way that a device's summary is not. "The firewall says it
> permitted it" and "the packet arrived on the far side" are different claims, and when a
> vendor disputes a fault, the capture ends the conversation.

## Where to capture

The most important decision, and getting it wrong wastes the exercise.

```
   Client ──── Switch A ──── Router ──── Firewall ──── Switch B ──── Server
      ▲           ▲                          ▲                          ▲
      1           2                          3                          4
```

Capture at both ends of the suspect element, because the question is almost always "did
it arrive?"

| Position | Shows |
|---|---|
| **On the client** | **what the client actually sent and received** |
| **On the server** | **what arrived, and what was sent back** |
| **Both, simultaneously** | **whether the middle lost, delayed or modified it** |
| **A SPAN/mirror port** | traffic on a switch, without touching the endpoints |
| **A network TAP** | **the same, passively, without a switch's CPU involved** |

**SPAN versus TAP is worth knowing:**

| | **SPAN / mirror port** | **TAP** |
|---|---|---|
| Cost | **free — a configuration** | hardware |
| **Drops under load** | **yes — the switch prioritises forwarding** | **no** |
| **Sees errors and runts** | **no — the switch discards them first** | **yes** |
| Bidirectional at line rate | **not if both directions are mirrored to one port** | yes |
| Effect on the switch | **CPU or fabric load** | none |

> A SPAN port mirroring both directions of a 1 Gb/s link to one 1 Gb/s port drops traffic
> whenever the total exceeds 1 Gb/s — which is routinely. If your capture shows loss, check
> whether the loss is in the network or in your capture, and this is a real and common error.

## tcpdump

On the device, in a terminal, and it is on almost everything.

```
   $ tcpdump -i eth0 -nn -s 0 -c 100 'host 10.9.0.5 and port 443'
```

| Flag | Does | Note |
|---|---|---|
| **`-i`** | interface | **`-i any` on Linux captures all** |
| **`-n`** / **`-nn`** | **no name resolution** | **always use it** — DNS lookups slow the capture and pollute it |
| **`-s 0`** | **full packet** | default on modern versions; snaplen otherwise truncates |
| **`-c N`** | count | bounded |
| **`-w file.pcap`** | **write** | **do this and analyse elsewhere** |
| **`-r file.pcap`** | read | |
| **`-v` `-vv`** | verbosity | |
| **`-e`** | **Ethernet header** | **essential for Layer 2 problems** |
| **`-A`** / **`-X`** | ASCII / hex payload | for plaintext protocols |

The ring buffer, for intermittent faults (Chapter 63 §63.3):

```
   $ tcpdump -i eth0 -nn -s 0 -W 20 -C 100 -w /var/cap/trace.pcap 'port 443'
   #  20 files, 100 MB each, rotating — 2 GB total, running for days
```

And a time-limited capture, which is the safer default: `-G 3600 -W 24` rotates hourly
and keeps a day.

## Filters, and the distinction that matters

> **There are two filter languages and confusing them wastes hours.**

| | **Capture filter (BPF)** | **Display filter (Wireshark)** |
|---|---|---|
| Applied | **before capture** | **after** |
| Syntax | `host 10.9.0.5 and port 443` | `ip.addr == 10.9.0.5 && tcp.port == 443` |
| Purpose | **reduce what is written to disk** | **find things in what you have** |
| **Cannot be changed later** | **correct** — you did not capture it | can be changed freely |

**Capture broadly, filter narrowly.** A capture filter that is too tight discards the packet
that would have explained everything, and you cannot get it back.

### Capture filters worth knowing

```
   host 10.9.0.5                       one host, either direction
   src host 10.9.0.5                   from it
   net 10.20.5.0/24                    a subnet
   port 443                            either direction
   portrange 8000-8100
   tcp port 443 and host 10.9.0.5      combined
   not port 22                         exclude your own SSH session — ESSENTIAL
   icmp                                
   arp                                 
   vlan                                 tagged frames
   'tcp[tcpflags] & tcp-syn != 0'      SYNs — connection attempts
   'tcp[tcpflags] & (tcp-rst) != 0'    resets — who is refusing?
   ether host 00:1a:2b:3c:4d:5e        by MAC
```

> `not port 22` when capturing on a machine you are connected to over SSH. **Without it,
> your capture records itself recording itself**, and the file grows without bound.

### Display filters worth knowing

```
   tcp.flags.reset == 1                who sent a reset
   tcp.analysis.retransmission         retransmissions
   tcp.analysis.zero_window            the receiver stopped
   tcp.analysis.flags                  all of Wireshark's warnings
   http.response.code >= 400
   dns.flags.rcode != 0                DNS errors
   tls.handshake.type == 1             ClientHello — and the SNI is in it
   icmp.type == 3 && icmp.code == 4    Fragmentation Needed — the MTU signal
   frame.time_delta > 0.5              gaps
   eth.dst == ff:ff:ff:ff:ff:ff        broadcasts
```

## Reading a capture

A method, because an unstructured stare at 40,000 packets achieves nothing.

**1. Look at the summary first.** Wireshark's Statistics → Conversations and Protocol
Hierarchy. What is actually on this wire, in what proportion, between whom — and this
frequently identifies the problem before any individual packet is read.

**2. Find the failure, not the traffic.** Filter for resets, retransmissions, ICMP errors and
DNS failures, not for the application you are debugging. `tcp.analysis.flags` in one filter
shows Wireshark's entire opinion of what went wrong.

**3. Follow one conversation.** Right-click → Follow → TCP Stream. One flow, in order,
both directions — which is the unit of analysis, not the packet.

**4. Read the timing.** Set the time display to "seconds since previous displayed packet".
A 3-second gap before a retransmission is an RTO (Chapter 37 §37.3); a 30-second gap is a
timeout somewhere; a 200 ms gap is delayed ACK.

**5. Then read individual packets.**

### The signatures worth recognising immediately

| Pattern | Means |
|---|---|
| **SYN, SYN, SYN, no reply** | **nothing is listening, or it is filtered by a drop rule** |
| **SYN → RST** | **something actively refused** — a closed port, or a reject rule |
| **SYN → SYN/ACK → ACK → RST** | **the application accepted and then closed** — not a network fault |
| **Retransmissions of the same segment** | **loss, in one direction** — and which end retransmits tells you which |
| **Duplicate ACKs then a retransmission** | **fast retransmit** (Chapter 37 §37.3) — normal recovery |
| **Zero window** | **the receiver's application is not reading** — not a network problem |
| **ICMP Type 3 Code 4** | **fragmentation needed — an MTU problem, being reported** |
| **Large packets absent, small ones present** | **an MTU black hole** — the ICMP is being filtered |
| **TLS ClientHello, then nothing** | **the server rejected it, or the path failed after the first large packet** |
| **Everything present and the application still fails** | **the network is not the problem**, and you now have proof |

> The last row is one of the most valuable outcomes of a capture and is frequently the
> actual result: "the request arrived, the response was sent, both are correct" ends the
> network team's involvement with evidence rather than with assertion.

## Wireshark specifics worth knowing

**Time reference and delta.** Ctrl-T sets a packet as the time reference, so everything after
is relative to it.

**Expert Information** (Analyse menu). Wireshark's own list of everything it considers
anomalous, in one place, and it is the fastest first look at an unfamiliar capture.

**Decode As.** A protocol on a non-standard port is dissected as raw TCP; Decode As tells
Wireshark to treat it correctly.

**IO Graph.** Throughput over time, with filters — which turns a capture into the graph
that shows when the problem occurred.

**And decrypting TLS:** set `SSLKEYLOGFILE` before starting the browser or client, and point
Wireshark at the resulting file. This works for traffic you generate and not for traffic you
intercept (Chapter 60 §60.3's inspection is a different thing entirely).

## The legal and ethical part

**Short and not optional.**

> **A packet capture records other people's traffic.** In many jurisdictions capturing on a
> network you do not own, or capturing content rather than headers, is a criminal offence —
> and in most organisations it requires authorisation regardless of the law.

**The practical rules:** capture only what you need, prefer headers to payloads where they
suffice, store captures with access control and delete them when the incident is closed,
and know your organisation's policy before you need it. Chapter 54 §54.4's flow-data
privacy discussion applies here with more force, because a capture contains content.

## What breaks here

A capture showing loss that the network does not have. The SPAN port dropped it. Check
the mirror's oversubscription, or use a TAP.

A capture that missed the packet that mattered. The capture filter was too tight.
Capture broadly.

A capture file growing without bound on an SSH session. **`not port 22`.**

No Layer 2 information in a capture of a Layer 2 problem. **`-e`**, and note that some
capture points strip VLAN tags.

**Retransmissions everywhere and no fault found.** Check which end is retransmitting, and
capture at both ends — the loss is between your capture point and the other end.

**A zero window blamed on the network.** The receiving application is not reading. Not a
network fault.

Large transfers absent from a capture at the far end and present at the near end. **MTU**
(Chapter 66 §66.3).

A capture proving the network is fine and nobody believing it. Show them the two captures
side by side; it is why you captured at both ends.

> **Network+ note.** Objective 5.5 covers protocol analysers. Over-learn: **a protocol
> analyser captures and decodes traffic**; a port mirror or SPAN copies traffic to a
> monitoring port; a TAP is a passive hardware device that does not drop under load; and
> capture requires promiscuous mode to see traffic not addressed to the capturing host. The
> SPAN/TAP distinction is examined and the SPAN's drop behaviour is what you will meet in
> practice.
