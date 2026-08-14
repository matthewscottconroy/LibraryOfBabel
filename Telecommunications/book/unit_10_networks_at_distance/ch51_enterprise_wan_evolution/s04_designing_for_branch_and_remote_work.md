# 51.4 Designing for the Branch and Remote Work

The branch office design questions now have a companion set for a person at a kitchen
table, and the interesting observation is that they are largely the same questions with the
budget removed.

## Sizing a branch

Start from what people do, not from what is available.

| Activity | Per user | Note |
|---|---|---|
| **Voice (VoIP)** | **~100 kb/s** | **jitter and loss matter; bandwidth does not** |
| **Video call, HD** | **1.5–3 Mb/s** | **and it is now the dominant driver** |
| Cloud office (M365, Workspace) | 0.5–2 Mb/s | bursty |
| Web and SaaS | 1–5 Mb/s | bursty |
| Backup / sync | **whatever you give it** | **schedule it** |
| **Guest / IoT** | varies | **give it its own VLAN and a cap** |

**A worked estimate for a 50-person branch:**

| | |
|---|---|
| **Concurrent video calls** (25% of staff) | 13 × 2.5 Mb/s = **33 Mb/s** |
| **Voice** (10% concurrent) | 5 × 0.1 = **0.5 Mb/s** |
| **Everything else**, 50 users at 1.5 Mb/s average | **75 Mb/s** |
| **Sum** | **~109 Mb/s** |
| **With 40% headroom** | **~150 Mb/s** |

**Two remarks about that arithmetic.**

Video is now the constraint, and it changed the sizing rules within about two years around
2020. A branch sized in 2018 is undersized now, and the cause is not more users.

Upstream matters far more than it used to. A video call is symmetric. A branch on a
service with 500 Mb/s down and 30 Mb/s up is constrained by the 30, and the headline figure
is misleading. This is Chapter 49 §49.2's DOCSIS asymmetry becoming an enterprise problem.

> **Size the upstream first.** It is the number that will fail, and it is the number nobody
> looks at.

## Redundancy at the branch

The question is what an hour of outage costs, and the answer determines everything else.

| Site type | Cost of an hour down | Design |
|---|---|---|
| **Retail store** | **lost trading, directly measurable** | **two independent links, automatic failover** |
| **Office** | reduced productivity | **primary + LTE backup** |
| Warehouse / logistics | **operations stop** | two links, possibly two carriers |
| **Clinical / industrial** | **safety-relevant** | **two links, two carriers, two routes, tested** |
| Small satellite office | inconvenience | **one link; people go home** |

And "two links" requires the same scepticism as everywhere else in this book:

> Two circuits from two carriers may share a duct, a pole, a cabinet, a building entry and a
> local exchange. Chapter 50 §50.3's shared risk link groups, at branch scale. Ask where
> the fibre enters the building — the commonest single point of failure is a single duct
> through a single wall.

LTE/5G backup is the pragmatic answer for most sites, because it is genuinely
independent of the fixed infrastructure — different medium, different plant, different
failure modes. The caveats are worth stating:

- **Test it monthly**, or it will not work when needed
- Capacity is a fraction of the primary — decide in advance what still works
- **Antenna placement matters** — a modem in a comms room in the basement will not perform;
  Chapter 42's arguments apply
- **Data caps** — an unattended failover that runs for a week produces a memorable invoice

## The local breakout decision

The question §51.2 raised and this section must answer: should branch traffic go straight
to the Internet?

| | **Backhaul to a hub** | **Local breakout** |
|---|---|---|
| Latency to cloud | **poor** | **good** |
| Bandwidth on the WAN | **consumed** | **freed** |
| Security inspection | **one place, well managed** | **many places, or cloud-delivered** |
| Egress IP addresses | **few, stable** | **many, changing** |
| Cost | high | **low** |
| Operational burden | **central** | **distributed, unless SASE** |

**The egress IP point is underrated.** Many SaaS platforms, partners and banking systems
restrict access by source IP address. Local breakout gives every branch a different,
frequently dynamic address, and that breaks those restrictions. The answer is either a
cloud security service with stable egress addresses, or backhauling the specific traffic that
needs a fixed source.

**The defensible position for most organisations:**

> Break out well-known SaaS traffic locally, inspected by a cloud security service.
> Backhaul everything else. It captures most of the benefit, keeps the inspection point
> singular, and leaves a clear place to put exceptions.

## The remote worker

The change 2020 accelerated and did not cause, and the honest framing is that the remote
worker is a branch office with a budget of zero and no site survey.

**What you do not control:**

| | |
|---|---|
| **The broadband service** | its capacity, its contention, its outages |
| **The Wi-Fi** | **and this is where most complaints originate** |
| **The household** | three other people streaming |
| **The physical environment** | and Chapter 45's coverage arguments all apply, unmeasured |

**What you do control:** the device, the identity, the client software, and the policy.

### The VPN model, and its limits

The traditional answer: a VPN tunnel from the laptop to the corporate network (Chapter 61),
and then everything as before.

**Its problems are structural rather than incidental:**

**Tromboning, again.** The user's traffic to Microsoft 365 goes through the corporate VPN
concentrator and back out — §51.1's problem, relocated to the individual. In March 2020
this broke a great many organisations at once, when concentrators sized for 10% of staff met
100% of them.

Split tunnelling fixes the capacity problem and dilutes the security model. Some traffic
goes direct, some goes through the tunnel — and the inspection point now sees only part of
what the user does.

**And the model's premise has expired.** The VPN exists to place the user "on the network",
which was meaningful when the applications were on that network. They are not. A VPN that
connects a user to a data centre so they can reach a cloud service is doing work for no
reason.

### Zero trust as an architecture

> If the user is not on your network and the application is not in your data centre, a
> perimeter is not merely inadequate. It is irrelevant.

**The replacement principle:** grant access per application, per session, based on
authenticated identity and verified device posture, with no network-level access implied.

| | **VPN** | **Zero trust access** |
|---|---|---|
| Grants | **network access** | **application access** |
| Trust basis | **being connected** | **identity + device + context** |
| Lateral movement | **possible once in** | **not implied** |
| Application visibility | **everything on the subnet** | **only what is granted** |
| User experience | connect, then work | **it is just there** |

Which is a genuinely better model, and Chapter 59 develops it properly. The points that
belong here are the WAN consequences:

- The remote worker needs no path to the data centre, so no concentrator capacity is
  required for cloud traffic
- The legacy applications that do live in the data centre still need something, and that
  something is usually a broker or a residual VPN — the migration is long and partial
- Device posture checking becomes load-bearing, which means endpoint management is now a
  network dependency

## Supporting what you cannot see

The practical problem, and it is now a large fraction of enterprise support effort.

A user says "the network is slow." They are at home. You have no visibility.

**A defensible triage order:**

1. **Wired or wireless?** — "Please plug in a cable" resolves a large fraction, and it
   is diagnostic either way
2. Is it slow for everything, or one application? — separates access from service
3. A speed test to a nearby server — establishes the access link's condition
4. **Their own devices** — someone in the household is uploading
5. The client's own diagnostics — modern access clients report the selected point of
   presence, its latency, and the device's posture
6. Only then, the corporate side

**And the organisational answer is instrumentation:** **digital experience monitoring** agents
on the endpoint that measure the path continuously, so the answer to "was it slow?" is
data rather than recollection. Chapter 54 §54.4 covers this class of tool.

> The support model that assumed you owned the network no longer applies, and the
> replacement is not better remote hands. **It is measurement at the endpoint**, because that
> is the only place left that you control.

## What breaks here

A branch sized for its download speed and struggling on video. **The upstream.** Check it
first, always.

**LTE backup failing when the primary fails.** It was never tested, or the SIM was
deactivated for non-use, or the antenna is in a basement. Monthly testing is not optional.

A SaaS platform rejecting a branch after local breakout. **Source IP restriction.** Expected
and predictable; plan the egress addressing before enabling breakout.

**Everyone's VPN slow at 9 a.m.** Concentrator or licence capacity, and it is a sizing
problem rather than a network one.

A remote user with a good speed test and poor application performance. Latency or loss,
not bandwidth — and quite possibly their Wi-Fi (Chapter 45 §45.4's association ladder applies
unchanged). A speed test measures the wrong thing.

**Two "diverse" branch circuits failing together.** The same duct into the building. Ask
where the fibre enters, physically.

**Failover working and voice calls dropping.** The tunnel re-established with a different
source address and the sessions did not survive. This is expected unless the design
addresses it, and it should be tested rather than assumed.

> **Network+ note.** Objective 1.8, 4.1 and the general design material. Over-learn: **remote
> access uses VPN or zero-trust network access**; split tunnelling sends only corporate
> traffic through the tunnel; redundant WAN links should use different providers and
> paths; and **cellular is a common backup for a branch.** The redundancy-requires-real-
> diversity point is examined as a concept and is worth holding as a habit.
