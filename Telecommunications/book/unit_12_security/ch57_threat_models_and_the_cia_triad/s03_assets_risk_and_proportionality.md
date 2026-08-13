# 57.3 Assets, Risk and Proportionality

**This section's purpose is to prevent both of the standard failures: spending nothing because
it seems abstract, and spending everything because the attacks sound frightening.**

## Start with assets

**You cannot assess risk to something you have not identified**, and **the asset inventory of
Chapter 53 §53.2 is the starting point** — extended with two questions it does not ask.

| From the inventory | Added here |
|---|---|
| What it is, where, who owns it | **What would it cost if this were unavailable?** |
| Support status, firmware | **What would it cost if its data were disclosed or altered?** |

**And for a network specifically, the assets are not only devices:**

| Asset class | Examples |
|---|---|
| **Devices** | routers, switches, firewalls, access points, controllers |
| **Connectivity** | **circuits, and the service they provide** |
| **Data in transit** | **what crosses the network, and to whom it matters** |
| **Configuration and credentials** | **Chapter 55 §55.4's repository is among the most valuable things you own** |
| **The management plane** | **compromise it and you have compromised everything it manages** |
| **Documentation** | Chapter 53 — **and it is a map for an attacker as well as for you** |
| **Availability itself** | **for many organisations the network's value is entirely its availability** |

> **The management plane is the asset most often under-valued.** **A compromised network
> management system is a position from which to reconfigure every device in the estate** —
> Chapter 60 §60.4's out-of-band argument follows from this.

## Risk, and why the arithmetic is not the point

$$\text{risk} = \text{likelihood} \times \text{impact}$$

**Its value is not arithmetic precision — the numbers are estimates and everyone knows it — but
the discipline of being explicit.**

> **Writing down "we assess this as unlikely and catastrophic" forces a conversation that "we
> should really do something about that" does not.**

**The quantified form, where it is useful:**

| Term | Meaning |
|---|---|
| **SLE** — single loss expectancy | **what one occurrence costs** |
| **ARO** — annual rate of occurrence | **how many times a year** |
| **ALE** — annual loss expectancy | **SLE × ARO** |

**Worked:**

| Risk | SLE | ARO | **ALE** |
|---|---|---|---|
| **Ransomware event** | £500,000 | **0.05** (once in 20 years) | **£25,000** |
| **Branch outage, one day** | £12,000 | **2** | **£24,000** |
| Data breach | £2,000,000 | 0.02 | **£40,000** |
| **Laptop lost** | £3,000 | **12** | **£36,000** |

**Two things that table teaches immediately.**

**Rare and catastrophic can rank below frequent and moderate.** **Ransomware at £25,000 ALE
ranks below losing laptops at £36,000**, which is counter-intuitive and — for spending
decisions — correct.

**And the numbers are estimates that everyone will dispute**, which is fine. **Their function is
to make the disagreement specific.** "You think it is once in twenty years and I think once in
five" is a productive argument; "we should take ransomware seriously" is not.

**Where the arithmetic fails, and it must be said:** **it handles the tail badly.** **A risk
with a 2% annual chance of ending the organisation has an ALE that understates it**, because
**the organisation cannot average over twenty years — it only gets one.** **For existential
risks, use the arithmetic to rank and then override it deliberately**, and say that you are
doing so.

## The four responses

**All four are legitimate.**

| Response | Meaning | Example |
|---|---|---|
| **Mitigate** | **reduce likelihood or impact** | segmentation, patching, MFA |
| **Transfer** | **someone else bears the loss** | **cyber insurance, or a contract with an SLA** |
| **Accept** | **document that you chose to live with it** | **a legacy system past EOL, isolated and monitored** |
| **Avoid** | **stop doing the risky thing** | **decommission the service; do not collect the data** |

**Two notes on the ones people undervalue.**

**Avoidance is frequently the cheapest control and is rarely considered.** **The service nobody
uses, the data nobody needs, the port nobody has opened deliberately** — **removing them costs
nothing to run and cannot be attacked.** **"Do we need this at all?" should be the first question
and is usually the last.**

**Transfer is not what people think.** **Insurance transfers financial loss; it does not
transfer the outage, the regulatory obligation, the reputational damage or the work of
recovery.** **And insurers now require controls** — MFA, segmentation, offline backups — **as a
condition of cover**, which has done more to drive adoption than any technical argument.

> **Acceptance in particular is a valid engineering decision when it is explicit and
> documented.** **The failure is not accepting a risk; it is accepting it silently and later
> claiming nobody knew.**

**A documented acceptance has four parts:** **the risk, the reason, the owner, and the review
date.** **Without the last two it is not an acceptance; it is a note.**

## Proportionality

> **The cost of the control should not exceed the expected loss it prevents.**

**A £50,000 control for a risk with an expected annual loss of £2,000 is not prudence but
innumeracy**, and **defending that position to a finance director is a skill worth
developing — because it is how security budgets are actually won.**

**The comparison must be done properly:**

$$\text{value of a control} = \text{ALE}_{\text{before}} - \text{ALE}_{\text{after}} - \text{cost of the control}$$

**Worked, for segmentation against ransomware:**

| | |
|---|---|
| ALE before | **£25,000** |
| **Estimated reduction in impact** | **80%** — the attacker reaches one segment, not everything |
| ALE after | **£5,000** |
| **Benefit** | **£20,000/year** |
| **Cost of segmentation** | **£40,000/year** (design, equipment, operational friction) |
| **Net** | **−£20,000** |

**So on this analysis segmentation does not pay** — **and this is where the honest engineer must
say three further things:**

**The estimate is soft.** An 80% impact reduction is a guess; so is the 0.05 ARO. **Sensitivity
matters: at ARO 0.1 the benefit doubles and the control pays.**

**Segmentation serves several risks at once.** **It also reduces insider impact, limits a
misconfiguration's blast radius, and satisfies compliance requirements.** **Costing a control
against one risk understates it**, and this is the commonest error in these calculations.

**And the tail argument applies.** **£500,000 may be the expected cost and the actual outcome
may be closure.**

> **The arithmetic is a tool for thinking, not an oracle.** **Its value is that it makes the
> assumptions visible and arguable**, which is precisely what "we should really do something"
> does not.

## Compliance is not security, and it is a real constraint

**Worth a short honest section, because the relationship is muddled everywhere.**

| | |
|---|---|
| **What compliance is good at** | **producing a floor**; **making funding available**; **forcing documentation** |
| **What it is bad at** | **being current**; **being proportionate to your actual risk**; **measuring outcomes rather than artefacts** |

**The failure mode is doing the thing the auditor checks rather than the thing that works** —
**quarterly vulnerability scans that nobody remediates, an annual penetration test whose findings
are still open a year later, a policy document nobody has read.**

**And the honest position is that compliance requirements are a constraint like any other.**
**They may be disproportionate to your risk and they are not optional**, so **satisfy them
efficiently and spend the remaining budget on what the risk assessment says.** **Arguing with a
regulator is rarely a good use of an engineer's time.**

## Making it a document that gets used

**A risk register that is read has a specific shape.**

| Field | Why |
|---|---|
| **The risk, stated as a scenario** | **"an attacker who compromises a laptop reaches the finance server" — not "lack of segmentation"** |
| Likelihood and impact, with reasoning | **the reasoning is the useful part** |
| **Current controls** | what already reduces it |
| **Response chosen**, of the four | |
| **Owner** | **a named role** |
| **Review date** | |
| **What would change the assessment** | **the most useful field, and the rarest** |

**The scenario formulation matters.** **"Lack of segmentation" is a missing control, not a
risk** — and stating risks as missing controls produces a register that is a shopping list.
**Stating them as scenarios permits the question "is there another way to prevent this
outcome?", which is where the cheaper answer usually is.**

## What breaks here

**A control that costs more than the loss it prevents.** **Do the arithmetic and be willing to
say so.**

**A control costed against one risk and rejected.** **Most controls serve several.** Count them
all.

**A risk register full of missing controls rather than scenarios.** **It has become a shopping
list**, and it will be ignored.

**An acceptance with no owner and no review date.** **Not an acceptance.** It will be discovered
during an incident, when it becomes an accusation.

**Insurance treated as a control.** **It transfers money, not outages or obligations** — and the
insurer will require the controls anyway.

**Compliance activity displacing risk-driven activity.** **Satisfy the requirement efficiently
and spend the rest on the assessment**, and be clear which is which.

**An existential risk ranked by ALE.** **The arithmetic averages over occurrences the
organisation will not survive to have.** Override deliberately, and record that you did.

**A service that could simply be removed, defended instead.** **Avoidance is the cheapest
control and is asked about last.**

> **Network+ note.** Objective 4.1 and the risk material. Over-learn: **risk is likelihood
> times impact**; **the four responses are mitigate, transfer, accept and avoid**; **a risk
> assessment identifies assets, threats and vulnerabilities**; and **security controls are
> preventive, detective or corrective.** The four responses are examined; **the proportionality
> argument is the part that determines whether you are any good at this.**
