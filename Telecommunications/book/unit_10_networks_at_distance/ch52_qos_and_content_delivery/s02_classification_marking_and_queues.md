# 52.2 Classification, Marking and Queues

Every QoS implementation, from every vendor, is the same three steps. Learning them once is
sufficient; the configuration syntax is the only thing that changes.

```
   ┌──────────┐   ┌────────┐   ┌──────────────────┐
   │ CLASSIFY │──▶│  MARK  │──▶│ QUEUE & SCHEDULE │
   └──────────┘   └────────┘   └──────────────────┘
    which class?   write it     act on it
                   in the
                   packet
```

## Classify

Deciding which class a packet belongs to, and there are four methods in increasing order of
cost and decreasing order of reliability.

| Method | Basis | Reliability |
|---|---|---|
| **Trust an existing marking** | **DSCP already set** | **best — if the trust boundary is right** |
| Access list | addresses, ports, protocol | **good for known servers, poor for dynamic ports** |
| **Deep packet inspection** | payload signatures | **increasingly useless — everything is encrypted** |
| Application recognition | heuristics, SNI, DNS | **approximate, and vendor-specific** |

Trusting an existing marking is the right answer, and it is the whole reason DSCP exists:
classify once, at the edge, and let every subsequent device read the answer.

> **Deep packet inspection's decline is worth noting.** In 2010 a router could identify most
> traffic by looking at it. With TLS 1.3 and encrypted SNI, and with QUIC hiding the
> transport header (Chapter 38 §38.4), **it largely cannot.** Classification is moving back
> to endpoint marking and to metadata, which is where it started.

## Mark

Write the classification into the packet so nothing downstream repeats the work.

The field is DSCP: six bits of the IP header's former Type of Service byte (Chapter 24
§24.2), giving 64 code points.

```
   ┌───────────────────────────────┐
   │  DSCP (6 bits)  │  ECN (2)    │   the ToS byte, redefined
   └───────────────────────────────┘
        64 values         Chapter 38 §38.3
```

The standard code points, and the ones you will actually use:

| Name | DSCP | Binary | Use |
|---|---|---|---|
| **Default / BE** | **0** | 000000 | **everything unclassified** |
| **EF** — Expedited Forwarding | **46** | **101110** | **voice** |
| **AF41** | 34 | 100010 | **interactive video** |
| AF31 | 26 | 011010 | call signalling |
| AF21 | 18 | 010010 | transactional data |
| AF11 | 10 | 001010 | bulk data |
| **CS6** | **48** | 110000 | **routing protocols — do not starve these** |
| CS1 | 8 | 001000 | **scavenger — less than best effort** |

The AF families follow a pattern worth knowing: AFxy — x is the class (1–4), y is the
drop precedence (1–3). AF11, AF12 and AF13 are the same class with increasing willingness
to be dropped, which lets a single class carry conforming and non-conforming traffic and
discard the latter first.

**And CS1, "scavenger", is underused.** It is a class that should be dropped before best
effort — for backups, software distribution, and anything that should use only what nobody
else wants. It is the least-known and most useful marking in the list.

At Layer 2, the equivalent is 802.1Q's three-bit PCP field (Chapter 20 §20.2), giving 8
values. Note that PCP lives in the VLAN tag, so an untagged frame cannot carry it — which
matters on access ports.

## The trust boundary

Where most QoS deployments fail, and it is a policy question rather than a technical one.

> If any host can mark its own traffic EF, every host eventually will, and the priority queue
> becomes the default queue.

**This is not hypothetical.** A user discovers that marking their traffic EF makes their video
call better; they tell a colleague; an application vendor ships a client that marks all its
traffic EF as a "performance feature"; and within a year the expedited queue carries most of
the load and provides no benefit to anything.

**The standard design:**

```
   ┌────────┐         ┌────────┐          ┌────────┐
   │ IP     │ trust   │ Access │  trust   │  Core  │
   │ phone  │────────▶│ switch │─────────▶│        │
   └────────┘  DSCP   └────────┘   DSCP   └────────┘
                          ▲
   ┌────────┐  DO NOT     │
   │ PC     │  trust ─────┘        ← the boundary
   └────────┘  (re-mark to 0, or
                classify locally)
```

| Device | Policy |
|---|---|
| **IP phone** | **trust** — it marks EF for media and CS3/AF31 for signalling, correctly |
| **PC / laptop** | **do not trust** — re-mark to 0, or classify by ACL |
| **PC behind a phone** | **the phone re-marks it** — which is why phones have a second port |
| **Server** | **trust selectively**, if the application marks correctly and you control it |
| **Wireless AP** | **trust**, after the AP has applied WMM mapping |
| **Everything inside the boundary** | **trust** |

The boundary must be decided explicitly and it is routinely forgotten, which produces a
network where markings exist and mean nothing.

**Two practical notes:**

**Carriers usually re-mark at the edge.** Your DSCP values do not survive into someone else's
network unless the contract says so — and most MPLS services map your markings to a small
number of carrier classes at ingress. Find out what the mapping is; it is in the service
description and nobody reads it.

The Internet does not honour DSCP at all. Marking a packet EF and sending it to the
Internet achieves nothing, and may occasionally achieve something worse if some intermediate
network treats unexpected markings oddly. QoS is an intra-domain mechanism.

## Queue and schedule

**The part that actually changes behaviour.**

A router with QoS has several queues per interface and a scheduler deciding which to serve.

### Priority queueing

One queue is served before all others, always, until it is empty.

**Correct for voice**, and dangerous:

> A priority queue with no rate limit can starve every other queue completely. If the
> priority class offers more traffic than the link can carry, nothing else is ever served.

**So a priority queue is always policed.** `priority percent 10` or `priority 1000` — traffic
above the limit is dropped rather than allowed to consume the link. This is not optional
and omitting it is the classic QoS misconfiguration.

### Weighted fair queueing and CBWFQ

Each class gets a guaranteed share; unused capacity is shared among the classes that want
it.

```
   Class          Guarantee   When others are idle
   ─────────────────────────────────────────────────
   EF (voice)     10% (policed priority)
   AF41 (video)   25%         → may use more
   AF21 (data)    20%         → may use more
   BE (default)   40%         → may use more
   CS1 (scav)      5%         → may use more
```

"Guarantee" means a minimum, not a maximum, which is the property that makes it usable:
a class that is not using its share does not waste it, and a class that needs more may
borrow. This is statistical multiplexing (Chapter 9) with a floor under each tenant.

### Active queue management

The other half of the answer, and the part usually omitted.

A tail-drop queue drops packets only when full, which means it is full most of the time
under load — and a full queue is a queue with maximum delay. This is bufferbloat
(Chapter 66 §66.4).

| Discipline | Behaviour |
|---|---|
| **Tail drop** | **drop when full** — simple, and keeps the queue full |
| **RED / WRED** | **drop probabilistically as the queue grows** — signals TCP early |
| **CoDel** | **targets queueing *delay*, not queue length** — the modern answer |
| **FQ-CoDel** | **CoDel plus per-flow fairness** — the current default on Linux, and excellent |
| **CAKE** | FQ-CoDel plus shaping and DiffServ awareness — **best for a home or branch edge** |

> FQ-CoDel and CAKE do most of what a hand-built QoS policy does, automatically, by
> targeting delay rather than by classifying traffic. On a small edge link they frequently
> outperform a carefully configured class-based policy, **and they require no classification at
> all** — which sidesteps the entire trust-boundary problem.

This is a genuinely important development and it is under-taught. Where you control both
ends and the link is small, reach for FQ-CoDel or CAKE before building a DSCP policy.

## A worked policy

A 50 Mb/s branch circuit carrying voice, video, business applications and everything else:

| Class | Match | Mark | Queue |
|---|---|---|---|
| **Voice** | trusted EF from phones | **EF (46)** | **priority, policed to 5 Mb/s** |
| **Video** | trusted AF41 | AF41 (34) | 30% guaranteed |
| **Signalling** | SIP, H.323 | CS3 (24) | **2% guaranteed — small and essential** |
| **Network control** | routing protocols | **CS6 (48)** | **3% — never starve this** |
| **Business apps** | ACL by server | AF21 (18) | 30% guaranteed |
| **Scavenger** | backup servers | **CS1 (8)** | **1%, and it may borrow** |
| **Default** | everything else | 0 | remainder |

Two details in that table that matter more than they look:

**CS6 for routing protocols.** If your QoS policy starves BGP or OSPF, the adjacency drops,
the network reconverges, and the outage is self-inflicted. Give control traffic a small
guaranteed share and never police it aggressively.

5 Mb/s of policed priority for voice on a 50 Mb/s circuit. That is about 55 concurrent
G.711 calls — more than a branch needs — and the policer means that if something floods EF,
the damage is bounded at 10% of the circuit.

## What breaks here

**Markings not surviving across the WAN.** The carrier re-marked them. Read the service
description; the mapping is contractual.

**Everything in the priority queue.** **No trust boundary**, or an application marking itself.
Look at the actual DSCP distribution on the wire — `tcpdump -v` shows it — rather than at
the configuration.

Voice fine and the rest of the network degraded. **Unpoliced priority queue.** Add the
rate limit.

Routing adjacencies dropping when the link is busy. **Control traffic starved.** CS6, with
a guarantee.

**A queue configured and never used.** The classification does not match anything. Check the
class-map hit counters; a class with zero packets is a class that is not doing what you
think.

**Latency high with the link at 50%.** **Bufferbloat.** The queue is deep and always occupied.
FQ-CoDel or a shaper with a shorter queue, not more bandwidth (Chapter 66 §66.4).

QoS working on the LAN and not over wireless. Wireless has its own mechanism — WMM's
four access categories — and the DSCP-to-WMM mapping is a separate configuration that is
frequently left at a default that does not match your classes.

> **Network+ note.** Objective 2.1 and 3.2. Over-learn: DSCP is six bits in the IP header's
> ToS byte; **CoS/PCP is three bits in the 802.1Q tag**; **EF is used for voice**; **traffic
> is classified, marked and then queued**; and **a trust boundary determines whose markings are
> honoured.** The DSCP-versus-CoS layer distinction is examined regularly.
