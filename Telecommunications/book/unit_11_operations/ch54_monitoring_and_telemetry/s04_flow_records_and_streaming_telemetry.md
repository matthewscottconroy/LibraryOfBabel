# 54.4 Flow Records and Streaming Telemetry

**An interface counter tells you the link is at 90%. It does not tell you what is on it**, and
that is almost always the next question.

## What a flow record contains

**A flow is a unidirectional sequence of packets sharing a key** — usually the five-tuple
(Chapter 35 §35.3).

| Field | |
|---|---|
| **Source and destination address** | |
| **Source and destination port** | |
| **Protocol** | |
| **Ingress and egress interface** | **which is what makes it a network record rather than a host one** |
| **Byte and packet counts** | |
| **Start and end timestamps** | |
| **TCP flags seen** | |
| **Next hop, AS numbers, DSCP** | depending on version |

**From which you can answer**: **which hosts, which applications, which conversations, which
direction, and how the mix has changed.**

> **This is the tool that turns "the circuit is full" into "the circuit is full because one host
> is uploading 40 GB to a cloud storage service"** — which is a statement someone can act on.

## The three families

| | **NetFlow v9** | **IPFIX** | **sFlow** |
|---|---|---|---|
| Origin | **Cisco** | **IETF standard** (RFC 7011) | **InMon / sFlow.org** |
| Model | **flow cache on the device** | same, **template-based and extensible** | **packet sampling** |
| Records | **aggregated per flow** | aggregated per flow | **sampled packet headers + counters** |
| Accuracy | **exact if unsampled** | exact if unsampled | **statistical** |
| Device cost | **higher — maintains state** | higher | **very low — no state** |
| Vendor support | Cisco and others | **broad** | **broad, especially in switching silicon** |

**The architectural difference is the important one.**

**NetFlow and IPFIX maintain a flow cache**: the device tracks active flows in memory, updating
counters, and **exports a record when the flow ends or a timer expires.** **Exact, and it costs
memory and CPU proportional to the number of concurrent flows.**

**sFlow samples packets**: **1 in N packets is copied, its header truncated, and sent to the
collector**, along with periodic interface counters. **Stateless, so it costs almost nothing and
is commonly implemented in the forwarding ASIC** — **and it is statistical, so small flows may
be missed entirely.**

> **NetFlow answers "exactly how much did this conversation transfer?" sFlow answers "what is
> the traffic mix, approximately?"** **Both are useful; they are not substitutes.**

**IPFIX is NetFlow v9 standardised and made extensible.** **Its template mechanism means a
device can export fields the collector has never seen** and describe them — which is how
vendor-specific and application-aware fields are carried. **New deployments should use IPFIX.**

## Sampling, and what it costs you

**Sampling is unavoidable at high rates**, and its consequences are worth understanding rather
than accepting silently.

**At 1:N sampling, a flow of $p$ packets yields about $p/N$ samples.**

| Sampling | A 1,000-packet flow | **A 5-packet flow** |
|---|---|---|
| **1:1** | 1,000 samples | **5 samples** |
| 1:100 | 10 samples | **0.05 — usually invisible** |
| **1:1000** | **1 sample** | **invisible** |
| 1:10000 | **0.1 — usually invisible** | invisible |

> **Sampling is accurate for elephants and blind to mice.** **Capacity analysis is fine at
> 1:1000; security analysis is not**, because the reconnaissance scan, the DNS exfiltration and
> the command-and-control beacon are all small flows.

**Which drives the deployment rule:**

| Purpose | Sampling |
|---|---|
| **Capacity planning, top talkers** | **1:1000 or coarser is fine** |
| **Billing / chargeback** | **unsampled, or a known and documented rate** |
| **Security analysis** | **unsampled where possible** |
| **A 100 Gb/s core link** | **sampled, because there is no alternative** |

**And record the sampling rate with the data.** **A collector that scales sampled counts back up
without knowing the rate produces confident, wrong numbers**, and this is a common
misconfiguration.

## Collection architecture

```
   ┌────────┐              ┌───────────┐         ┌──────────┐
   │ Router │──── UDP ────▶│ Collector │────────▶│ Analysis │
   │ Switch │   2055/4739  │  (receives│         │  + store │
   │  FW    │              │  & decodes)│        └──────────┘
   └────────┘              └───────────┘
       ▲                         ▲
   flow cache            **template state** — a collector
   or sampler            that missed the template cannot
                         decode the records that follow
```

**Three practical points that cause real problems:**

**Export is UDP and unacknowledged.** **Flow records lost in transit are lost silently**, and
they are most likely to be lost during congestion — which is when you want them. **Put the
collector close, and monitor for gaps.**

**Templates must arrive before the data.** **NetFlow v9 and IPFIX send templates periodically**;
**a collector that restarts must wait for the next template before it can decode anything.**
**Set the template interval short enough that a restart costs seconds rather than minutes.**

**Volume is substantial.** **2,000 flows per second is 173 million records a day**, and at
around 50 bytes each that is **about 9 GB per day before indexing.** **Plan storage and
retention deliberately**; the usual arrangement is **full records for days, aggregated
summaries for months.**

## Privacy, which is real

**Flow data is a detailed record of who communicated with whom, when, and how much.**

> **It does not contain content, and it does not need to.** **Metadata alone establishes that a
> device contacted a medical service, a legal service, a job site, or a political organisation**
> — and it does so with timestamps.

**In many jurisdictions this is personal data**, with obligations about **purpose limitation,
retention period, access control and subject access.**

**The practical implications:**

- **Define and document the purpose** before collecting, and retain only as long as it serves
- **Restrict access**, and **log access to the flow system itself**
- **Aggregate for long-term retention** — counts and totals rather than per-conversation records
- **Consult whoever handles data protection in your organisation.** **They will not object to
  network monitoring; they will want it documented**, and doing so before rather than after is
  substantially cheaper.

**And note that flow data survives encryption.** **TLS hides the content; it does not hide that
the conversation occurred**, which is why flow analysis has become more rather than less
valuable as encryption spread (Chapter 62).

## Streaming telemetry

**The successor to polling, and the resolution to §54.2's tension.**

| | **SNMP polling** | **Streaming telemetry** |
|---|---|---|
| Direction | **manager pulls** | **device pushes** |
| Interval | **limited by polling load** | **sub-second, continuously** |
| Data model | **MIB — numeric OIDs** | **YANG — named, typed, structured** |
| Transport | UDP | **gRPC over HTTP/2, or NETCONF** |
| Encoding | BER | **protobuf or JSON** |
| Efficiency | **one request per object** | **one subscription, continuous stream** |
| Security | **v3, if configured** | **TLS, always** |

**The mechanism:**

```
   Collector ── subscribe: "/interfaces/interface/state/counters", 1 s ──▶ Device
   Collector ◀────────────── stream of updates, continuously ─────────────  Device
```

**Two properties change what is possible.**

**Sub-second granularity without polling load.** **The device sends what changed, when it
changed**, rather than answering the same question 8,640 times a day. **§54.1's microbursts
become visible.**

**Structured, named data.** **A YANG model is a schema with types and units**, so **the
collector knows that a field is a counter of octets rather than inferring it from a MIB
file.** **This is what makes automated analysis practical** (Chapter 70 §70.2).

**And it is what SNMP's replacement actually looks like:** **gNMI for telemetry and
configuration, YANG for the models, protobuf on the wire.**

**Adoption is uneven, and honestly so.** **Modern data centre and service provider equipment
supports it well; enterprise access switches frequently do not**, and **the model coverage
varies enormously between vendors** despite OpenConfig's efforts to standardise. **The practical
position for most organisations is streaming telemetry where it exists and SNMP everywhere
else**, for some years yet.

## Alerting: where monitoring succeeds or fails

**The most important part of the chapter, and the part most often absent.**

> **An alert nobody acts on is worse than no alert.** It consumes attention, **it trains people
> to ignore the channel**, and it provides false assurance that the system is being watched.

**Alert fatigue is not a soft problem.** **It is the mechanism by which real incidents are
missed**, and it has a documented role in serious outages across every industry that has looked
for it.

### The four rules

**Alert on symptoms users experience, not on every threshold.**

| Poor | Better |
|---|---|
| "CPU is 81%" | **"Users cannot reach the application"** |
| "Interface utilisation 85%" | **"Branch WAN saturated for 10 minutes"** |
| "Disk 78% full" | **"Log storage will fill in 6 days"** |

**Every alert must have an action.**

> **If the answer to "what do I do about this?" is "nothing, it clears itself", it is not an
> alert. It is a graph.**

**Put it on a dashboard. Do not send it to a person at 03:00.**

**Alert on trend and duration, not instantaneous values.**

**A link at 95% for five minutes is worth knowing. A link at 95% for two seconds is a backup
starting.** **Every threshold needs a duration**, and most need a hysteresis so they do not
oscillate.

**Review alerts that fired and were ignored, monthly, and delete or fix them.**

> **This is the single practice that keeps a monitoring system trustworthy**, and it is the one
> that is never done.

### A useful classification

| Class | Route to | Example |
|---|---|---|
| **Page** | **wake someone** | **service down, and users are affected now** |
| **Ticket** | **next working day** | a redundant power supply failed |
| **Dashboard** | **nobody, until asked** | utilisation, trends |
| **Log** | **searchable, no notification** | everything else |

**The discipline is that "page" is a small list**, and **adding to it requires justifying that a
human must act within minutes.** **Most things do not meet that bar**, and treating them as if
they do is how the bar stops being respected.

**And two alerts worth having that nobody configures:**

**Alert on the monitoring system itself.** **A silent monitoring system looks identical to a
healthy network.** **Dead-man's-switch alerting** — something that fires if the expected
heartbeat stops — is the answer.

**Alert on certificate and contract expiry.** **The most predictable outages in this book**, and
they still happen, **because nobody is watching a date.**

## What breaks here

**Flow data showing less traffic than the interface counters.** **Sampling, and the collector
does not know the rate**, or export packets are being lost.

**A small but important flow invisible.** **Sampling.** Security analysis needs unsampled data.

**A collector showing nothing after a restart.** **Waiting for templates.** Shorten the template
interval.

**Flow export contributing to the congestion it is measuring.** **It is UDP on the same path.**
Use a management path where possible.

**Streaming telemetry configured and the model returns nothing.** **Vendor model coverage.**
The path exists in OpenConfig and not in that platform's implementation.

**Alerts that everyone ignores.** **Delete them.** This is not a failure of discipline; **it is
information about which alerts were never actionable.**

**An outage discovered by a user.** **The monitoring measured devices rather than the service.**
Chapter 63's first question — "what does the user actually experience?" — should have been the
monitoring's first measurement.

**A silent monitoring system.** **Nobody noticed for three weeks.** Dead-man's-switch.

> **Network+ note.** Objective 3.1 covers flow data and alerting. Over-learn: **NetFlow records
> flow metadata including source, destination, ports and byte counts**; **sFlow uses sampling**;
> **flow data shows what traffic is present, which counters cannot**; and **alerts should be
> actionable and tuned to avoid fatigue.** The counters-versus-flows distinction is the
> examinable idea.
