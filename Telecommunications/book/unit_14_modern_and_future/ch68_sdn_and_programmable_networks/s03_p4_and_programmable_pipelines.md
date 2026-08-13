# 68.3 P4 and Programmable Pipelines

**§68.2's central failure was that the hardware was not programmable.** **P4's answer is to make
it so** — **and it is a more radical proposal than OpenFlow was.**

## The fixed pipeline

**A conventional switch ASIC has a pipeline designed at manufacture.**

```
   Packet ─▶ [Parse ] ─▶ [ L2   ] ─▶ [ L3   ] ─▶ [ ACL  ] ─▶ [Queue] ─▶ Egress
             Ethernet     MAC        route       TCAM
             IP           table      table
             TCP
```

**Every stage knows what it is looking for.** **The parser understands Ethernet, IPv4, IPv6, TCP,
UDP and whatever else the designer included** — **and nothing else.**

> **Which means a new protocol requires new silicon.** **VXLAN took years to appear in hardware
> after it was specified; GENEVE is still not universal; and a protocol invented next year
> cannot be forwarded at line rate until a chip designed for it ships, which is a
> three-to-five-year cycle.**

**And it explains OpenFlow's difficulty precisely:** **OpenFlow asked for arbitrary matches
across arbitrary fields, and the hardware had specific tables for specific purposes.** **The
protocol described a switch that did not exist.**

## P4's proposition

> **Do not specify what the switch does. Specify a language in which what the switch does is
> written**, and **compile it onto the target.**

**P4 — Programming Protocol-independent Packet Processors — is a domain-specific language for
describing a packet processing pipeline.**

**A P4 program specifies four things:**

| | |
|---|---|
| **The parser** | **a state machine describing how to extract headers** |
| **The header formats** | **including ones nobody has invented yet** |
| **The match-action tables** | **what is matched, and what actions may be taken** |
| **The control flow** | **the order of the tables, and the conditions** |

**And a fragment looks like this:**

```
   header my_protocol_t {
       bit<16> flow_id;
       bit<8>  priority;
       bit<8>  ttl;
   }

   parser MyParser(packet_in packet, out headers hdr) {
       state parse_ethernet {
           packet.extract(hdr.ethernet);
           transition select(hdr.ethernet.etherType) {
               0x88b5: parse_my_protocol;      // an EtherType nobody uses
               0x0800: parse_ipv4;
               default: accept;
           }
       }
   }
```

> **The switch now understands a protocol that did not exist when the chip was manufactured**,
> and forwards it at line rate. **That is the whole argument.**

## What it enables that was not possible

**Four things, and the fourth is the one that has produced the most published work.**

### Protocols without new silicon

**A new encapsulation, a new tag, a research protocol, an organisation's own header** — **all
forwardable at line rate on existing hardware.**

### In-band network telemetry

**The application that has attracted most attention.**

> **Each switch appends its own state — queue depth, timestamp, egress port, hop latency — to
> the packet as it passes.** **The receiver extracts the complete path record from the packet
> itself.**

**Which answers a question no monitoring system can:** **not "what is the average queue depth on
switch 7?" but "what did this specific packet experience, at every hop, on this specific
journey?"** — **and it does it for every packet, without sampling** (Chapter 54 §54.4's
compromise, removed).

**Its cost is packet size** — **the telemetry accumulates as it traverses** — **and the usual
deployment is to instrument a sampled subset or to strip and export at the last hop.**

### Computation in the network

**Functions that were the endpoints' work, moved into the forwarding path:**

| | |
|---|---|
| **Load balancing** | **stateful, in the switch, at line rate** |
| **Consensus acceleration** | **Paxos and Raft primitives in the datapath** |
| **In-network aggregation** | **combining machine learning gradient updates at the switch** — **which is genuinely deployed in AI clusters** |
| **Caching** | key-value lookups answered by the switch |
| **DDoS mitigation** | **detection and filtering at line rate** (Chapter 62 §62.3) |

**The aggregation case is the one with a commercial deployment:** **an AI training cluster's
gradient exchange is a many-to-one reduction**, and **performing the arithmetic in the switch
removes a substantial fraction of the traffic** — which is why it appears in current fabric
designs (Chapter 67 §67.4).

### And it makes the pipeline auditable

**A P4 program is a complete, formal description of what the switch does to a packet** —
**which can be verified, tested and reasoned about**, in a way that a vendor's ASIC
documentation cannot.

## Where it actually runs

**Honestly, because P4 is a language and needs a target.**

| Target | Status |
|---|---|
| **Intel Tofino** | **the flagship programmable ASIC** — **and Intel announced its discontinuation in 2023** |
| **Smart NICs and DPUs** | **the growth area** (Chapter 67 §67.1) — Pensando, BlueField, and others |
| **`bmv2`** | **the software reference switch** — for development and teaching |
| **eBPF** | **P4 compiled to eBPF, running in the Linux kernel** |
| **FPGA** | research, and specialised deployment |

**The Tofino discontinuation is worth stating plainly** because it changed the field's
trajectory:

> **The most capable programmable switching ASIC was cancelled**, and **the reason was
> commercial rather than technical** — **the volume was not there against fixed-function
> merchant silicon, which is cheaper, faster and adequate for what most networks do.**

**Which is §68.2's lesson repeated:** **the incentive to commoditise is weaker than the incentive
to sell the existing product**, and **a technology that requires a hardware vendor to undermine
their own margin will struggle regardless of merit.**

**And the response has been that P4's centre of gravity moved to the host** — **DPUs, smart NICs
and eBPF** — **where the economics are different because the buyer is a server vendor rather
than a switch vendor.**

## The honest position

**Three statements.**

**The idea is right.** **A fixed pipeline is an arbitrary constraint imposed by manufacturing
economics, and a programmable one is strictly more capable.**

**The economics are hard.** **Fixed-function silicon is cheaper per port, uses less power, and
does what 95% of networks need.** **Programmability is worth paying for where it is used, and
most networks would not use it.**

**And where it is used, the value is large.** **Hyperscale data centres, AI fabrics, research
networks, network functions on DPUs, and telemetry** — **all real, all specialised, and none of
them is the enterprise network in your building.**

> **P4's likely legacy is not that switches became programmable but that the *idea* of the
> pipeline as a program became normal** — **in eBPF on hosts, in DPUs, and in the specification
> languages that describe forwarding behaviour.** **Which is the same shape as OpenFlow's
> legacy: the architecture survived and the product did not.**

## What breaks here

**A P4 program that compiles on `bmv2` and not on hardware.** **Target resource limits** — table
sizes, stages, memory. **The language is portable; the resources are not.**

**In-band telemetry inflating packets past the MTU.** **Predictable** (Chapter 66 §66.3), and it
is why telemetry is usually sampled or stripped at the last hop.

**A programmable switch deployed and used as a fixed-function one.** **The cost without the
benefit**, and it is a common procurement outcome.

**A hardware roadmap that depended on Tofino.** **It was discontinued.** This is a supply-chain
risk of exactly the kind Chapter 55 §55.3's lifecycle register exists for.

**A custom protocol forwarded at line rate on a P4 switch and dropped by everything else.**
**Expected.** Programmability is local.

> **Network+ note.** P4 is beyond Network+'s scope. The transferable content is: **a switch's
> capabilities are determined by its forwarding hardware**, **new protocols require hardware
> support to be forwarded at line rate**, and **this is why standards take years to become
> deployable.** Chapter 28's IPv6 transition and Chapter 67's VXLAN adoption are both instances
> of it.
