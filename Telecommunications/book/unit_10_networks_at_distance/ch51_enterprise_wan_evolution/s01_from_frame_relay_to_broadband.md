# 51.1 From Frame Relay to Broadband

Four WAN generations in thirty years, and each was displaced by the same force: the
traffic stopped going where the network was built to send it.

## The full-mesh problem

A leased line connects two sites. Connecting *n* sites to each other requires
$n(n-1)/2$ circuits, and this is Chapter 11's arithmetic arriving on an invoice.

| Sites | Full-mesh circuits |
|---|---|
| 4 | 6 |
| 8 | 28 |
| **20** | **190** |
| 50 | **1,225** |

At £500 per circuit per month, twenty sites cost **£95,000 a month** — before anyone has
sent a packet. So nobody built full meshes. They built hubs:

```
   Hub and spoke:                    The consequence:
                                     
      A     B                        A → B goes A → HQ → B
       \   /                         Two circuits, two hops,
        \ /                          and HQ is a single point
        HQ                           of failure for all of it
        / \
       /   \
      C     D
```

Which is 4 circuits instead of 6 for four sites, and 20 instead of 190 for twenty —
and every site-to-site conversation goes through headquarters.

> The hub-and-spoke topology that dominated enterprise networking for thirty years was not a
> design preference. It was a response to a pricing model, and every operational property it
> had — the HQ bottleneck, the doubled latency, the single point of failure — followed from
> that.

## Frame Relay

The first serious relief, from the early 1990s.

One physical circuit per site to the carrier's cloud, carrying many virtual circuits.

```
   Site A ──┐                        ┌── Site B
            │   ┌──────────────┐     │
   Site C ──┼───│ carrier's    │─────┼── Site D
            │   │ Frame Relay  │     │
   Site E ──┘   │   network    │     └── Site F
                └──────────────┘
     one physical access circuit each;
     PVCs between any pair, priced per PVC
```

| Term | Meaning |
|---|---|
| **PVC** | **permanent virtual circuit** — a logical path between two sites |
| **DLCI** | **the local identifier for a PVC** — significant only on your access link |
| **CIR** | **committed information rate** — what the carrier guarantees |
| **Burst** | traffic above CIR, carried **if capacity exists** |
| **DE** | **discard eligible** — the bit marking burst traffic as droppable first |
| **FECN / BECN** | congestion notification, forward and backward |

The DLCI is locally significant, exactly as an MPLS label is (Chapter 50 §50.4) and
exactly as ATM's VPI/VCI is. Three technologies, one idea, three decades.

**CIR is the interesting commercial mechanism.** You buy a 512 kb/s CIR on a 2 Mb/s access
circuit; traffic up to 512 kb/s is guaranteed, and traffic above it is marked DE and carried
if the carrier's network has room. The carrier oversubscribes deliberately and the
statistics work out — Chapter 9 again — and the customer gets far more than the CIR most of
the time and exactly the CIR when it matters.

> **FECN and BECN deserve a moment**, because they are explicit congestion notification
> (Chapter 38 §38.3) **arriving in 1990 and being almost entirely ignored.** The bits were
> defined, carriers set them, and **very little equipment did anything useful in response.**
> ECN's slow adoption in IP is the same story told again.

**ATM** competed with Frame Relay for the same role, with **fixed 53-byte cells**, genuine
quality-of-service classes, and far more complexity. It won in carrier cores for a decade and
lost at the enterprise edge, and Chapter 13 §13.3 covers why: the cell tax — 5 bytes of
header per 48 of payload, about 10% — was hard to justify once Ethernet framing was
everywhere.

## MPLS L3VPN, and what it actually sold

From the early 2000s, and it displaced both.

**The technical mechanism is Chapter 50 §50.4's.** What matters here is what the enterprise
was buying, because it was not bandwidth.

| What you got | Why it mattered |
|---|---|
| **Any-to-any without a mesh** | **the carrier's routing does it**; one circuit per site |
| **A contractual SLA** | **availability, latency, jitter and loss, with penalties** |
| **Separation** | overlapping RFC 1918 space, guaranteed private |
| **Class of service** | voice, video and data queued differently, end to end |
| **Someone to call** | **and a contract that makes them answer** |

> **The enterprise was buying a promise.** Chapter 13's account of what packet switching gave
> up explains exactly why that promise commanded a premium — and it was frequently ten times
> the per-megabit price of commodity Internet access.

A representative comparison, and the numbers are the argument:

| | **MPLS** | **Business broadband** |
|---|---|---|
| Bandwidth | **20 Mb/s** | **500 Mb/s** |
| Monthly cost | **£800** | **£80** |
| **Cost per Mb/s** | **£40** | **£0.16** |
| Install time | **6–12 weeks** | **3–10 days** |
| SLA | **yes, with penalties** | **best effort** |
| Latency guarantee | **yes** | none |

A 250-fold difference in cost per megabit is not a premium. It is a different market, and
it became indefensible to a finance director the moment a plausible alternative existed.

## What actually broke it

**Three forces, and they arrived together.**

### The traffic went elsewhere

MPLS assumed traffic flowed between branch and data centre. By 2015 it did not.

```
   The design:                      The reality:

   Branch ──MPLS──▶ Data Centre     Branch ──MPLS──▶ Data Centre ──▶ Internet
      (everything the                                              ──▶ M365
       office needed)                                              ──▶ Salesforce
                                                                   ──▶ AWS
                                       ▲                              │
                                       └──────── and back ────────────┘
```

**This is tromboning** — also called hairpinning — and it is bad in three separate ways:

**Latency.** Traffic from a Manchester branch to a Microsoft data centre in Dublin travels to
London, out through the corporate gateway, to Dublin, and all the way back. The direct path
is 350 km; the actual path is 1,100 km, and the added round-trip latency is 20–40 ms on
exactly the traffic most sensitive to it.

**Cost.** The expensive circuit carries traffic that was never going to the data centre, so
the enterprise pays MPLS prices to transport its Office 365 sessions.

**Capacity.** The central gateway becomes a bottleneck for every branch at once, and it must
be sized for the sum of them.

### The price gap became indefensible

**See the table above.** The argument "but the SLA" is legitimate and it does not survive
contact with a 250× ratio applied to traffic that mostly goes to the Internet anyway.

### Provisioning was too slow

Six to twelve weeks for an MPLS tail, against three to ten days for broadband.

For an organisation opening retail sites, that is strategic rather than inconvenient. A
retailer opening forty stores a year cannot wait three months per site, and "we can be
trading in a week" changes what the business can do.

> **Note that only one of the three forces is technical.** The traffic pattern changed;
> the other two are procurement and operations. The technology that displaced MPLS did not
> need to be better at anything MPLS was good at. It needed to be cheap and fast to install.

## The interim answers, and why they were insufficient

Enterprises tried the obvious things first, and it is worth knowing why they were not
enough.

**Split tunnelling at the branch.** Put an Internet circuit alongside the MPLS and send
cloud-bound traffic straight out. It solves the tromboning and it creates a security
problem — every branch is now an Internet edge, needing a firewall, patching, logging and
someone to manage it. Forty branches means forty firewalls.

**DMVPN and similar dynamic VPN overlays.** Cisco's DMVPN built on-demand IPsec tunnels between
spokes over Internet underlay, which is genuinely most of what SD-WAN does and predates it
by a decade. What it lacked was central policy, per-application steering, and continuous
path measurement — it was a routing mechanism, not a policy system.

**Policy-based routing.** Configure each router to send some traffic one way and some another.
It works, and it is configured per device, by hand, and it does not adapt when a path
degrades.

> **Each of these solved part of the problem.** SD-WAN's contribution (§51.2) was **not
> inventing any of the mechanisms** — the overlay, the tunnels and the policy routing all
> existed — but combining them with central control and continuous measurement.

## What remains of the old model

MPLS is not dead, and predictions of its death have been wrong for a decade.

**Where it is still the right answer:**

- Applications with a genuine latency or jitter guarantee requirement — trading, some
  industrial control, some healthcare
- Sites where the alternative is poor — rural sites where broadband is 10 Mb/s
  and unreliable
- **Regulatory requirements** that prohibit traffic traversing the public Internet
- As one underlay among several in an SD-WAN, which is the commonest outcome: MPLS for
  the traffic that needs it and broadband for the rest

And the honest position is that the typical modern enterprise WAN is hybrid, not because
hybrid is elegant but because different traffic has genuinely different requirements and one
transport does not serve all of them.

## What breaks here

A branch's Internet performance being poor while the MPLS circuit is idle. **Tromboning.**
Measure the path; the fix is local breakout.

A Frame Relay or MPLS circuit "at capacity" carrying mostly Internet traffic. **The same
diagnosis.** Look at what the circuit is actually carrying before ordering more of it.

**Traffic marked DE being dropped under load.** **Working as designed.** The CIR is what you
bought; the burst is a courtesy.

A new site waiting eight weeks for connectivity. Order a broadband circuit and an LTE
backup immediately, and treat the MPLS tail as an upgrade. This is now standard practice.

An SLA credit that does not cover the loss. **Read the SLA.** Carrier credits are
typically a percentage of the monthly circuit charge, which for a retail site losing a day of
trading is not compensation but a gesture. The SLA buys attention, not indemnity.

Site-to-site traffic that is slow via the hub. **The topology.** It was a pricing decision
in 1998 and it is still costing latency.

> **Network+ note.** Objective 1.2 and 2.1. Over-learn: Frame Relay and ATM are legacy
> packet-switched WAN technologies; **a leased line is dedicated**; MPLS provides carrier
> VPN services with class of service; **a CSU/DSU terminates a digital circuit**; and
> hub-and-spoke, full mesh and partial mesh are the WAN topologies. The topology
> arithmetic — $n(n-1)/2$ — is examined regularly.
