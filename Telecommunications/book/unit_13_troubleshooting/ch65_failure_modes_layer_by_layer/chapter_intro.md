# Chapter 65 — Failure Modes, Layer by Layer

This chapter is a catalogue, and it is organised the way you will actually use it:
**by symptom**, with the causes that produce each symptom grouped by layer.

That inversion matters. Reference material is usually organised by cause — here is
what a duplex mismatch does, here is what a wrong VLAN does — which is the wrong index,
because you never begin an incident knowing the cause. You begin with a user saying
something is broken, and what you need is a mapping from *that* to the small set of
things that could produce it.

## The symptom index

The chapter's spine, and the thing worth internalising, is that a handful of symptom
patterns cover most faults, and each pattern eliminates most causes immediately.

**No link light.** Layer 1, always. Cable, port, transceiver, power, or the far device.
Nothing above Layer 1 can produce this, which makes it the easiest symptom in
networking and the reason it is worth checking first even when it seems beneath you.

**Link up, no communication at all.** Layer 2. Wrong VLAN, port in err-disabled state,
spanning tree blocking, 802.1X authentication failed, or a wrong speed/duplex forced
at one end. The port shows up and passes nothing, and every one of these is visible in
one `show interface` and one `show vlan`.

**Local works, remote does not.** Layer 3, and specifically the default gateway or the
route. The host can ARP for its neighbours and reach them; anything requiring a router
fails. Check the gateway address, check the router is alive, check the router has a
route back.

**Some remote destinations work, others do not.** Almost always a **wrong subnet
mask** (Chapter 25 §25.3), because the mask determines the local/remote decision per
destination, so a wrong one is right for some addresses and wrong for others. This
symptom is so characteristic that recognising it is worth more than most of this
chapter. Second candidate: a specific missing route.

**Address is 169.254.x.x.** DHCP did not answer (Chapter 27 §27.2, Chapter 40 §40.3).
Check, in order: is the DHCP server running, is the relay configured on this subnet's
router, is the scope exhausted, is the client on the VLAN you think it is.

**`ping` by address works, by name does not.** Everything below Layer 7 is working.
This is DNS, and one test eliminated six layers.

**Connects, then hangs on large transfers.** PMTUD black hole (Chapter 34 §34.4).
Confirm with `ping -s 1400` versus a small packet. This one is worth committing to
memory because the symptom is so specific and the cause is so consistently
misdiagnosed.

**Works intermittently, correlated with nothing obvious.** The hardest class. Candidates
in rough order of frequency: duplicate IP address, rogue DHCP server, marginal cable
with temperature dependence, spanning tree reconvergence, wireless interference, and a
failing power supply.

**Works, but slowly.** Chapter 66's entire subject, because nothing is broken and the
usual tests all pass.

## Physical

§65.1 covers the layer that produces the most misleading symptoms, because physical
faults are frequently *partial*.

A completely failed cable is easy. A **marginal** cable is the expensive one: it passes
a continuity test, it links up, it works at 1 Gb/s and fails at 10, it degrades when
the sun heats the riser in the afternoon, and it produces a low rate of CRC errors that
Chapter 3 §3.3 tells you is enough to cap TCP throughput at a fraction of capacity
while every dashboard shows a healthy link at full speed.

The counter signatures, which are the actual diagnostic content:

| Counter | Indicates |
|---|---|
| CRC errors / input errors | Frames arriving corrupted — cable, connector, EMI, or a failing transceiver |
| Runts | Frames below 64 bytes — usually collisions, or a duplex mismatch |
| Giants | Frames above the MTU — often an 802.1Q tag the far end did not expect (Chapter 20 §20.2) |
| Late collisions | Collision after the first 64 bytes — **a segment too long, or a duplex mismatch.** Never normal |
| Output drops | Egress queue full — congestion, not a physical fault |
| Input drops | Ingress buffer full — the device cannot keep up |

Fibre adds its own: dirty or damaged connectors (the most common fibre fault by a wide
margin, and the reason cleaning kits exist), bend radius violations, wavelength or
mode mismatch, and simple transmit/receive reversal.

## Data link

§65.2 covers VLAN and switching faults, and the four that account for most of them:

**Wrong access VLAN** — the port works perfectly, in the wrong broadcast domain.
**Native VLAN mismatch** on a trunk — two segments silently merged (Chapter 20 §20.3),
with no error and connectivity that should not exist.
**VLAN missing from a trunk's allowed list** — works locally on each switch, fails
between them, which is a confusing partial symptom.
**Spanning tree blocking an unexpected port** — usually because the root bridge was
elected by default (Chapter 19 §19.3) and is the oldest switch in the building.

Plus MAC table exhaustion, port security violations placing a port in err-disabled
state, and duplex mismatch — which appears here and again in Chapter 66 because it is
simultaneously a link fault and a performance fault.

## Network and routing

§65.3 covers Layer 3, where the faults are mostly arithmetic or absence.

Wrong mask (the selective-connectivity signature above). Wrong gateway. Missing route —
including the asymmetric case where traffic goes out and the return path does not
exist, which produces a one-way flow that is genuinely confusing until you check both
directions. Overlapping subnets. Duplicate addresses. NAT translation exhaustion. And
routing protocol adjacency failures, whose causes are a short and checkable list:
mismatched area, mismatched authentication, mismatched timers, mismatched MTU, or an
access list blocking the protocol.

## Transport and services

§65.4 covers the top, where the network is fine and the service is not.

**Blocked port** — `ping` works, the application does not. The distinguishing test is
`telnet host port` or `nc -zv host port`: a connection means the path is open and the
service is listening; a **refusal** means the path is open and nothing is listening; a
**timeout** means something is silently dropping. Those three outcomes are three
different diagnoses and Chapter 37's RST-versus-timeout distinction is exactly this.

Then: service not running, listening on loopback only (Chapter 38 §38.2), certificate
expiry, clock skew (Chapter 41 §41.3), DHCP scope exhaustion, DNS misconfiguration,
and authentication backend unreachable.

## By the end you will be able to

- Map a reported symptom to the small set of causes that could produce it.
- Recognise the wrong-mask signature, the APIPA signature and the PMTUD signature on
  sight.
- Read interface counters and name the physical cause each indicates.
- Diagnose the four common VLAN faults from their distinct partial symptoms.
- Distinguish refused, timed-out and successful connection attempts and state what
  each proves.
- Work systematically through an unfamiliar fault using the symptom index.
