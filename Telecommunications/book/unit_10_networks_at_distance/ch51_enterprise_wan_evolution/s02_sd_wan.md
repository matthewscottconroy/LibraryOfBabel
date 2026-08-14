# 51.2 SD-WAN

**Nothing in SD-WAN is new.** The overlay, the tunnels, the policy routing and the path
measurement all predate it. What is new is putting them under one central policy and
measuring continuously, and that turned out to be the whole difference.

## The architecture

```
   ┌──────────────────────────────────────────────────┐
   │              Orchestrator / Controller           │  ← policy, config,
   │           (cloud-hosted or on-premises)          │    monitoring
   └───────────────────────┬──────────────────────────┘
                           │ control plane
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────┴────┐        ┌────┴────┐        ┌────┴────┐
   │ Branch  │        │ Branch  │        │  Data   │
   │  edge   │        │  edge   │        │ centre  │
   └─┬──┬──┬─┘        └─┬──┬────┘        └────┬────┘
     │  │  │            │  │                  │
   MPLS│  LTE        bband LTE              MPLS
     │ bband           │  │                  │
     └──┴──┴═══ encrypted overlay tunnels ═══┴──────┘
              (IPsec, over every underlay)
```

Four components, and each maps to something already in this book:

| Component | What it is | Seen before |
|---|---|---|
| **Edge device** | at each site; terminates underlays, builds tunnels | |
| **Overlay** | **encrypted tunnels over any transport** | Chapter 67's overlays |
| **Controller** | **central policy; not in the data path** | Chapter 68's SDN |
| **Measurement** | **continuous probing of every path** | new, and it is the key |

> **SD-WAN is an overlay with a separated control plane and continuous telemetry.** Strip the
> marketing and that sentence is complete.

## Transport independence

**The first property, and the enabling one.**

Because every site-to-site path is an encrypted tunnel, the underlay is irrelevant.

| Underlay | Cost | Latency | Reliability | **Usable?** |
|---|---|---|---|---|
| MPLS | high | **guaranteed** | **high** | yes |
| Business broadband | **low** | variable | good | **yes** |
| Consumer broadband | **very low** | variable | moderate | **yes** |
| LTE / 5G | moderate | variable | good | **yes** |
| Satellite | high | **very high** | moderate | **yes, for backup** |

A site can use several simultaneously, and — this is the part that matters — all of them
carry traffic at once rather than one being an idle standby.

**Which changes the availability arithmetic.** Two independent links at 99.5% each give
**99.9975%** if genuinely independent — but the qualification does real work: a broadband
circuit and an MPLS tail from the same carrier down the same duct are not independent, and
Chapter 56 §56.2 takes this up properly.

## Application-aware steering

The second property, and the one users notice.

**Traditional routing decides by destination address.** SD-WAN decides by application,
user, time, and current path quality.

```
   Policy:
     voice        → lowest jitter path,     failover < 1 s
     M365         → direct local breakout
     backup       → cheapest path, off-peak only
     ERP          → MPLS only (compliance)
     everything else → best available
```

And the measurement that makes it possible runs continuously: each edge probes each path
several times a second, tracking latency, jitter, loss, and often MOS for voice.

**Two mechanisms exploit that measurement:**

**Path selection.** Send each application over the path currently meeting its requirement.

**Forward error correction and packet duplication.** For voice on a lossy path, send
redundant data — or duplicate the packets across two paths and de-duplicate at the far end.
It doubles the bandwidth for that flow and eliminates the loss, which for a 100 kb/s voice
call is a trade worth making every time.

> Duplicating a voice call across two paths is an absurd use of bandwidth and completely
> correct. Chapter 5's error-control argument: redundancy is cheap when the payload is
> small and the consequence of loss is large.

## Local breakout

The third property, and the one that pays for the project.

```
   Before:  Branch ──MPLS──▶ DC ──▶ Internet ──▶ M365
                    └─────── 40 ms of detour ────────┘

   After:   Branch ──▶ local Internet ──▶ M365
                    └── 8 ms ──┘
```

**Cloud-bound traffic leaves at the branch.** The MPLS circuit stops carrying it, the central
gateway stops being a bottleneck, and the latency penalty disappears.

And it creates the security problem that §51.4 and Chapter 60 must answer: every branch
is now an Internet edge. Forty branches means forty places where traffic enters and leaves,
each needing inspection, policy and logging.

The industry's answer is SASE — Secure Access Service Edge — which is SD-WAN plus
cloud-delivered security: traffic breaks out locally to a nearby cloud security point of
presence which applies firewall, URL filtering, malware inspection and data-loss policy,
and then goes on to its destination.

> SASE moves the security stack from forty branch appliances to a provider's edge, which is
> genuinely the right architecture for a distributed organisation and makes that provider a
> single point of failure and a single point of trust. Chapter 59 develops the identity
> question this raises.

## Zero-touch provisioning

**The fourth property, and the operational one.**

Ship a device to a site. Someone plugs in power and a network cable. It boots, calls home
using a certificate burned in at manufacture, authenticates, downloads its configuration, and
joins the overlay.

No engineer travels. No configuration is typed at the site.

For an organisation opening sites regularly this is transformative, and it is the argument
that most often wins the internal business case — not the bandwidth, not the cost, but that a
new site is a shipping problem rather than a project.

## What SD-WAN does not do

Vendors are not clear about this, so this section is.

It does not create an SLA on the public Internet.

> Measuring paths and choosing the best one is not the same as a carrier committing to a
> latency figure. When every available path is congested, SD-WAN picks the least bad one.

**For most traffic that is entirely sufficient.** For a small class — real-time trading,
some industrial control, some clinical systems — it is not, and the honest design keeps a
dedicated circuit for those and says so.

It does not remove the need for capacity. A congested 100 Mb/s link is congested. Steering
moves the problem; it does not solve it.

**It does not simplify operations, on balance.** It replaces per-device configuration with
central policy, which is better — and it adds a controller, an overlay, a measurement
system, and a vendor relationship. The complexity moved; it did not vanish, and the
skills required changed rather than reduced.

**It introduces a new dependency.** If the controller is unreachable, existing tunnels keep
forwarding — every serious implementation gets this right — but no policy change,
no new site, and no visibility. Ask any vendor exactly what happens during a controller
outage, and be sceptical of a vague answer.

**And it is a lock-in decision.** There is no interoperability between SD-WAN vendors.
An edge device from one vendor will not form an overlay with another's. Choosing an SD-WAN is
choosing a vendor for five to seven years, and it should be evaluated as such.

## Making the decision

**A defensible framework, rather than a vendor's:**

| Question | If yes |
|---|---|
| Is most traffic cloud-bound? | **strong case** |
| Are there many sites? | **strong case — the operational saving scales** |
| Is MPLS spend large relative to need? | **strong case** |
| Do sites open and close often? | **strong case** |
| Is there a hard latency SLA requirement? | **keep a dedicated circuit for that traffic** |
| Are sites few and static, with good MPLS pricing? | **the case is weak; do not assume otherwise** |
| Is there in-house capability to run an overlay? | **if not, budget for a managed service** |

> **SD-WAN is not automatically correct.** For a three-site organisation with satisfactory
> MPLS pricing and no cloud traffic, **it adds cost and complexity for no benefit**, and saying
> so is more useful than a migration plan.

## What breaks here

**Voice degrading despite SD-WAN steering.** Every path is congested. Steering has no good
option. Check whether the underlay has capacity before blaming the policy.

**Traffic taking an unexpected path.** **Policy evaluation order**, or the application was
misclassified. Check what the edge identified the flow as — deep packet inspection fails
on encrypted traffic and falls back to IP or DNS heuristics, which are frequently wrong.

An application misclassified after the vendor changed its address range. Extremely
common with SaaS. Application definitions are signature databases and they go stale.

Local breakout working and security policy not applying. The breakout bypassed the
inspection path. This is a serious misconfiguration and it is easy to make.

**A site online but not receiving policy.** The controller is unreachable. Forwarding
continues; management does not. Confirm this is the designed behaviour, in a test, before
you need it.

Poor performance on one path with all metrics green. Probe traffic is not user traffic.
Probes are small and frequent; a path can pass probes and fail a large transfer, especially
where a middlebox treats them differently.

**Everything working and the MPLS bill unchanged.** Nobody cancelled the circuits. This is
common enough to be worth stating: the saving is realised at contract renewal, not at
cutover, and the business case should say so.

> **Network+ note.** Objective 1.2 and 1.8 cover SD-WAN and SASE. Over-learn: **SD-WAN builds
> a software-defined overlay across multiple transports**; it selects paths based on
> application and current conditions; **it enables local Internet breakout**; and **SASE
> combines SD-WAN with cloud-delivered security.** The transport-independence idea is the
> examinable content.
