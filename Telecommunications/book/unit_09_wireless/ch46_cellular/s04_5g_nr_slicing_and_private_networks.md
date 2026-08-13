# 46.4 5G NR, Slicing and Private Networks

**5G is three different systems sharing a name**, and separating them is the only way to
evaluate any claim made about it.

## The three service classes

| Class | Goal | Target | Reality |
|---|---|---|---|
| **eMBB** — enhanced Mobile Broadband | **faster** | 1–20 Gb/s peak | **deployed; this is what you have** |
| **URLLC** — Ultra-Reliable Low-Latency | **1 ms, 99.999%** | industrial control, remote surgery | **barely deployed** |
| **mMTC** — massive Machine-Type | **1 million devices/km²** | sensors at scale | **mostly served by LTE-M and NB-IoT** |

> **Consumer 5G is eMBB and nothing else.** The URLLC and mMTC promises justified much of the
> investment and the marketing, and **they are the parts that have not arrived.**

**Being clear about this is not cynicism** — it is necessary for assessing whether 5G solves
a problem you have.

## The bands, and the honesty problem

**5G NR operates across a very wide range, and the differences are enormous:**

| Band | Range | Speed | Coverage | Penetration |
|---|---|---|---|---|
| **Low (< 1 GHz)** | 600–900 MHz | **~100–250 Mb/s** | **tens of km** | good |
| **Mid (1–6 GHz)** | **3.5 GHz typical** | **200 Mb/s – 1 Gb/s** | **1–3 km** | moderate |
| **mmWave (24–47 GHz)** | 28, 39 GHz | **1–5 Gb/s** | **100–300 m** | **almost none** |

**And the marketing figures come from mmWave**, which covers a few hundred metres and **does
not pass through a window, a wall, a leaf, or a hand.**

**The demonstration videos of multi-gigabit 5G are real and were shot standing in line of
sight of a small cell.** Move round a corner and the connection drops to mid-band or LTE.

> **Mid-band is the useful 5G.** It is where most deployments are, it delivers a genuine
> several-fold improvement over LTE, and **it is not what the advertising showed.**

**Low-band 5G is barely faster than LTE** — it uses similar spectrum with a better air
interface, and the gain is perhaps 20–30%. **"5G" on a phone in a rural area frequently means
this.**

## What NR actually improves

**Setting aside the bands, the air interface has real advances:**

**Flexible numerology.** LTE's subcarrier spacing is fixed at 15 kHz; **NR supports 15, 30,
60, 120 and 240 kHz.** Wider spacing means shorter symbols, **which means lower latency** —
and it also tolerates the higher Doppler and phase noise of mmWave.

**Massive MIMO.** 64, 128 or 256 antenna elements at the base station, **with beamforming per
user** (Chapter 44 §44.4). At mid-band this is the single largest capacity contributor —
**it allows spatial reuse within a cell**, serving many users simultaneously on the same
frequency in different directions.

**Beam management.** At mmWave, **the beam is narrow enough that it must track the user**, and
the system continuously measures and switches beams. **A mmWave link is a steered pencil of
energy**, not a coverage area.

**Lower latency.** Shorter transmission intervals and the numerology above give **1–10 ms air
latency** against LTE's 30–50 — **though end-to-end latency is usually dominated by the
network beyond the radio**, which is §46.4's point about edge computing below.

**Better spectral efficiency.** LDPC and polar codes replace LTE's turbo codes, giving perhaps
30% more from the same spectrum.

## SA and NSA — the distinction that matters

**Most "5G" is not 5G.**

| | **NSA** — Non-Standalone | **SA** — Standalone |
|---|---|---|
| Radio | 5G NR | 5G NR |
| **Core** | **LTE's EPC** | **5G core (5GC)** |
| Control plane | **LTE** | 5G |
| Latency | **LTE's** | **improved** |
| **Network slicing** | **no** | **yes** |
| URLLC | no | possible |
| Deployment | **the majority** | growing |

> **NSA is 5G radio bolted onto an LTE core.** It delivers eMBB's speed and **none of the
> architectural features** — no slicing, no URLLC, and LTE's control-plane latency.

**Which means most claims about 5G's capabilities describe SA and most deployments are NSA**,
and asking which one is present is the first question about any 5G proposal.

## Network slicing

**The most interesting 5G idea, and the least deployed.**

**Create several logical networks on one physical infrastructure, each with its own
characteristics:**

```
   ┌────────────────────────────────────────┐
   │        one physical network            │
   ├──────────┬──────────┬──────────────────┤
   │ Slice 1  │ Slice 2  │    Slice 3       │
   │ eMBB     │ URLLC    │    mMTC          │
   │ consumer │ factory  │    meters        │
   │ high BW  │ 1 ms     │    tiny, cheap   │
   │ best     │ guaranteed│   massive count │
   │ effort   │ SLA      │                  │
   └──────────┴──────────┴──────────────────┘
```

**Each slice has its own scheduling policy, its own core network functions, and its own
service guarantees** — and a slice's traffic is isolated from another's.

**This requires SA**, and it requires the core to be **virtualised** (Chapter 67 §67.1), which
is why it arrived late.

**Its promise:** a factory buys a slice with guaranteed 1 ms latency; a utility buys one for
meters; consumers use the general slice — **all on the operator's existing infrastructure.**

**Its difficulty:** **guaranteeing a slice's performance requires reserving resources**, and
reserved resources are unavailable to everyone else. **A slice with a hard latency guarantee
is a slice with capacity held idle for it**, and the economics of that are exactly the
economics that made circuit switching expensive (Chapter 13 §13.1).

> **Slicing is statistical multiplexing with per-tenant guarantees**, and the tension between
> those two is not a 5G problem — it is the oldest problem in this book.

## Private 5G

**The development most relevant to an enterprise network engineer**, and the one worth
watching.

**An organisation runs its own cellular network** on its own premises, using **licensed,
shared or unlicensed spectrum:**

| Region | Mechanism |
|---|---|
| **US** | **CBRS** (3.5 GHz) — three tiers, with a **SAS** database allocating access |
| **Germany** | **local 5G licences** at 3.7–3.8 GHz, cheap and per-site |
| UK | Ofcom shared access licences |
| Japan, others | local licence schemes |

**CBRS's tiered model is worth understanding** because it is Chapter 43 §43.1's AFC idea taken
further: **incumbents (naval radar) have priority; Priority Access Licence holders come next;
and General Authorized Access users take what is left** — with a **Spectrum Access System**
database arbitrating in real time.

**Why an organisation would want this:**

| | Wi-Fi | Private 5G |
|---|---|---|
| Spectrum | **unlicensed, contended** | **licensed or coordinated — predictable** |
| Coverage per AP | tens of metres | **hundreds of metres** |
| **Mobility** | roaming is client-decided (Ch 45 §45.2) | **seamless, network-controlled handover** |
| **Determinism** | best effort | **schedulable, with guarantees** |
| Cost | **low** | **high** |
| Client devices | **everything** | **modules, and expensive** |
| Expertise | common | **scarce** |

**Where it is genuinely the right answer:**

- **Large outdoor industrial sites** — ports, mines, refineries — where Wi-Fi would need
  hundreds of access points and its roaming would not cope with vehicles at speed
- **Automated guided vehicles and robotics** needing deterministic latency
- **Environments where interference is unacceptable** and unlicensed spectrum is too risky
- Sites where **coverage over a large area with few cells** is worth the cost

**Where it is not:** an office. **Wi-Fi is cheaper, every device has it, and the expertise
exists.** Private 5G in a carpeted office is a solution looking for a problem, and it is being
sold that way.

## Edge computing

**The part of 5G's latency story that is not about radio.**

**1 ms air latency is meaningless if the server is 100 ms away** (Chapter 3 §3.1's
propagation). **So low latency requires the application to be close** — which means moving
computation from central data centres to the network edge.

```
   Traditional:  device ──▶ tower ──▶ core ──▶ central DC ──▶ Internet
                 └────────────── 50–100 ms ──────────────┘

   MEC:          device ──▶ tower ──▶ edge compute
                 └────── 5–10 ms ──────┘
```

**MEC — Multi-access Edge Computing** — places servers at or near base stations.

**And it changes what a network operator is.** An operator with compute at thousands of edge
sites is **a distributed cloud provider**, competing with — and partnering with —
hyperscalers. **AWS Wavelength, Azure Edge Zones and Google's equivalents are exactly this**,
and the commercial arrangements are still being worked out.

**Chapter 69 §69.4 returns to it** as an architecture; the point here is that **5G's latency
claim is an end-to-end claim that the radio alone cannot deliver.**

## What to make of it

**An honest assessment, because this is a field with a great deal of marketing:**

**Real and delivered:** mid-band eMBB is a genuine several-fold capacity improvement; massive
MIMO works; the air interface is better; **and 5G's efficiency is what permits 2G and 3G
shutdown.**

**Real and limited:** mmWave works where it is deployed and covers very little; low-band 5G is
marginally better than LTE.

**Promised and not delivered:** URLLC at scale, network slicing in production, mMTC (which
LTE-M and NB-IoT serve adequately).

**Genuinely interesting and under-appreciated:** **private cellular**, which gives enterprises
access to a technology previously available only to operators, **and which for specific
industrial cases is the right answer where nothing else is.**

> **5G is a solid generational improvement sold as a revolution.** Assessing a specific
> proposal means asking: which band, SA or NSA, and which of the three service classes does
> the claim actually depend on?

## What breaks here

**5G showing on the phone and speeds like LTE.** Low-band, or NSA, or both. Expected.

**mmWave dropping when you move.** It does not penetrate or diffract. Expected.

**A 5G proposal promising 1 ms latency.** Ask whether it is SA, and where the application
runs. **Air latency is not end-to-end latency.**

**Slicing promised on an NSA deployment.** Not possible.

**Private 5G proposed for an office.** Usually the wrong tool. Ask what Wi-Fi cannot do.

**A device stranded by 3G shutdown.** §46.2 — and it is a large, current, under-appreciated
problem.

> **Network+ note.** Objective 2.4 expects 5G. Over-learn: **5G's bands trade speed against
> coverage, with mmWave fastest and shortest-range**; **5G's latency is lower than LTE's**;
> and **network slicing creates logical networks with different characteristics.** The band
> trade-off is the examinable content, and the SA/NSA distinction is worth knowing even
> though it is rarely examined.
