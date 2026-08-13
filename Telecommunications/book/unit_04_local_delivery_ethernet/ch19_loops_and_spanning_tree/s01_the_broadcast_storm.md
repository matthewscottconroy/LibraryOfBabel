# 19.1 The Broadcast Storm

Everything in Chapter 17 was correct and produced a network with a fatal flaw. This
section is about the flaw, and it is worth understanding viscerally before meeting the
fix, because the fix looks like enormous complexity until you have seen what it
prevents.

## The three sentences, revisited

The switch algorithm (§17.2):

1. **Learn** the source address against the arrival port.
2. **Forward** by the destination out the single port where it was learned.
3. **Flood** out every other port when the destination is unknown, broadcast, or
   multicast.

Rule 3 is necessary. A switch that did not flood unknown destinations could never
deliver a frame to a station that had not yet spoken, and broadcasts — ARP, DHCP —
would not work at all.

Rule 3 is also the problem.

## Two switches, two cables

Consider the most natural thing a competent engineer would build. Two switches, and
**two** cables between them, because one cable might fail and redundancy is
obviously good.

```
        ┌──────────────┐
   A ───┤   Switch 1   ├────┬────┐
        └──────────────┘    │    │
                            │    │   two links
        ┌──────────────┐    │    │
   B ───┤   Switch 2   ├────┴────┘
        └──────────────┘
```

Host A sends **one** broadcast frame — an ordinary ARP request for the gateway.

**What happens:**

1. Switch 1 receives it, floods it out **both** links to Switch 2.
2. Switch 2 receives **two copies**. Each is a broadcast, so each is flooded out every
   other port — including the *other* link back to Switch 1.
3. Switch 1 receives two copies back and floods each out every other port, including
   both links to Switch 2.
4. Switch 2 receives four copies…

The frame count doubles at each traversal. Within milliseconds, both switches are
forwarding at line rate, entirely with copies of one ARP request.

## Why it does not stop

Three properties combine, and **all three are required**:

**There is no hop count in an Ethernet frame.** Chapter 15 §15.3's header has a
destination, a source, a type field and nothing else. IP has a TTL for exactly this
reason (Chapter 24 §24.3) and Ethernet, designed for a single shared cable where a
loop was physically impossible, has no equivalent. **A looping frame circulates
forever**, because nothing in it counts.

**Flooding multiplies.** A frame arriving on one port leaves on *n*−1. With two paths
between switches the population grows exponentially rather than merely persisting.

**There is no state to detect it.** A switch has no memory of frames it has already
seen. Each arriving copy is, as far as the switch is concerned, a new frame.

The result is not degradation. It is **collapse, in under a second**, from a single
frame that nobody did anything wrong to send.

## The three simultaneous failures

A loop causes three distinct problems, and it is worth separating them because they
produce different symptoms.

**1. The broadcast storm.** Exponential replication of every broadcast and every
unknown-unicast frame until the links are saturated. Nothing else gets through.

**2. MAC table instability — "MAC flapping".** Host A's frames arrive at Switch 2 on
port 1 *and* on port 2, alternately. The learning rule (§17.2) says the source's
location is the arrival port, so the table entry is rewritten thousands of times per
second:

```
%SW_MATM-4-MACFLAP_NOTIF: Host aa:aa:aa:aa:aa:aa in vlan 1
  is flapping between port Gi0/1 and port Gi0/2
```

**That log message is a loop until proven otherwise.** It is the single most
diagnostic line in this chapter. Even unicast forwarding now fails, because the table
is wrong more often than it is right.

**3. Multiple frame delivery.** A station receives many copies of frames sent once.
Higher layers are not designed for this. TCP will cope; ARP caches will not; some
applications will behave in ways that make no sense at all.

## What it looks like from the operations desk

This matters because the symptom does not point at the cause.

- **Everything stops at once.** Not one application, not one VLAN — everything, on
  every host in the broadcast domain, within a second.
- **Switch CPU at 100%.** Broadcasts must be processed in software, so the control
  plane saturates.
- **You cannot log in to the switches.** Management traffic is on the same broadcast
  domain, and the management interface is now unreachable. **The tools you would use
  to diagnose it are the tools it has taken away**, which is why a console cable and
  an out-of-band management network stop being paranoia.
- **All port LEDs solid.** Not blinking. Constant traffic at line rate.
- **The network does not recover.** No timeout, no backoff. It stays down until
  someone physically removes a cable.

The last point is what makes loops different in kind from congestion. **A loop has no
self-limiting mechanism at all.**

## How they actually happen

Not by anyone deliberately building a redundant path and forgetting to configure
anything. In practice:

| Cause | Frequency |
|---|---|
| **A user plugs both ends of a patch cable into the same wall plate** | Extremely common |
| A small unmanaged switch under a desk, cabled to two ports | Common |
| Two wall ports patched together in a comms room "to test them" | Common |
| A miscabled uplink during a change | Common |
| STP disabled on a port by an engineer chasing a slow-boot complaint | Occasional |
| A hypervisor bridging two physical NICs on the same VLAN | Occasional |
| A wireless bridge pair forming an unexpected path | Occasional |

The top row is the one that brings down enterprises. **It requires no malice and no
expertise, and any user with a spare patch cable can do it.** This is why loop
protection is configured on *every access port* rather than only where loops seem
plausible.

## The two responses

There are exactly two structural answers.

**Do not build loops.** Attractive and inadequate. It requires perfect cabling
discipline forever, from everyone, including people who do not know what a loop is.
It also forfeits redundancy — and the reason the second cable was there was that the
first one might fail.

**Build loops and disable them logically.** Provide physical redundancy, then have the
switches themselves compute which links to block, leaving a loop-free active topology.
When a link fails, unblock a blocked one.

The second is what Perlman's algorithm does, and §19.2 develops it. But notice what it
requires: **the switches must cooperate to compute a global property of a topology
none of them can see**, using only messages exchanged with immediate neighbours. That
is a genuinely hard distributed-systems problem, and the fact that it has a compact
solution is why the algorithm is famous.

## What breaks here

**MAC flapping messages.** A loop. Investigate immediately; do not wait for the
storm.

**Total loss of connectivity on a segment, with switch CPU at 100%.** A loop.

**"It broke right after we patched something."** A loop. The correlation is nearly
perfect, and the first question during an outage should be *what changed physically*.

**A storm that survives removing the obvious cable.** There is a second path — often a
device bridging two networks (a hypervisor, a laptop with Wi-Fi and Ethernet on the
same VLAN, a wireless bridge).

> **Network+ note.** Objectives 2.3 and 5.2 expect switching loops, broadcast storms
> and their remedy. Two sentences to over-learn: **an Ethernet frame has no TTL, so a
> looping frame circulates forever**, and **MAC flapping in the logs means a loop**.
> Both are examined, and the second is the fastest real diagnosis in this chapter.
