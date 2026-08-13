# 55.3 Lifecycle, Patching and EOL

**The calendar, which most organisations manage badly** — and the failure is always the same
shape: **a date that was known for years arrives as a surprise.**

## The four dates

| Date | Meaning |
|---|---|
| **General availability** | you can buy it |
| **End of sale (EoS)** | **you can no longer buy more** |
| **End of software maintenance** | **no more bug fixes; security fixes may continue** |
| **End of support / end of life (EoL)** | **no patches, no replacements, no help** |

**End of support is the one that matters**, and **it is the field in the asset inventory
(Chapter 53 §53.2) that turns documentation into planning.**

**The typical intervals:**

```
   ─────────────────────────────────────────────────────────────▶
   GA          EoS announced    EoS        EoSW-M           EoL
   │           │                │           │                │
   │◀── 3–5 y ─▶│◀── 6–12 mo ──▶│◀── 2–3 y ─▶│◀──── 2–3 y ───▶│
                                 ▲
                    you can still buy it right up to here,
                    and the clock is already running
```

> **A device bought in the last month before end of sale has the same end-of-support date as one
> bought four years earlier.** **Purchasing a product late in its life buys you a shorter
> asset**, and the purchase price rarely reflects it. **Check the EoL date before buying, not
> after.**

## The uncomfortable arithmetic

**Why "we will deal with it when it happens" does not work:**

| | |
|---|---|
| **Support after end of sale** | **typically 5 years** |
| **Budget cycle** | **annual** |
| **Business case and approval** | **1–3 months** |
| **Procurement and lead time** | **1–6 months** (and supply chains have been worse) |
| **Design, build, migrate** | **3–12 months for a core refresh** |
| **Total from decision to complete** | **6–18 months** |

> **An organisation that discovers its core switches went out of support last quarter has a
> problem with no fast solution**, and **the failure was in the inventory rather than in the
> switches.**

**The practical rule:**

**Report on end-of-support dates 24 months out, into the annual budget process.** **Not 6
months, and not as an operational item** — **as a line in next year's budget**, because that is
the only mechanism that produces money.

**And "we will run it out of support for a while" is a legitimate decision**, provided it is a
decision: **documented, risk-assessed, with compensating controls** (segmentation,
Chapter 60 §60.4) **and a date.** **It becomes indefensible only when it is a discovery rather
than a choice.**

## Firmware: the genuine dilemma

**Stated honestly, because it is not simple and it is usually presented as if it were.**

| **Upgrading risks** | **Not upgrading risks** |
|---|---|
| **new defects** — and networking firmware is not lightly tested | **known vulnerabilities remain** |
| **an outage window** | |
| **behaviour changes** — defaults, deprecated features | **the defect that has already bitten someone else** |
| **feature regressions** | **incompatibility with newer equipment** |
| **hardware that no longer supports the new release** | **falling off supported versions entirely** |

**Both columns are real.** **An engineer who upgrades everything immediately will introduce
defects; one who never upgrades will eventually be exploited.**

### Why network equipment is a special case

**Two properties make network firmware vulnerabilities unusually serious:**

> **They are frequently pre-authentication and remotely exploitable** — a malformed packet to a
> management service, or to a routing protocol, **requiring no credentials at all.**

**And the device sits where it can see and redirect everything.** **A compromised switch is not
one compromised host; it is a position from which to observe or modify all traffic passing
through** (Chapter 62).

**Which is why "it's only a switch" is the wrong instinct**, and why network equipment
vulnerabilities have repeatedly produced the class of incident that reaches the news.

### A defensible policy

**Three tracks, with stated maximum delays:**

| Track | Trigger | Maximum delay |
|---|---|---|
| **Routine** | **planned cadence** | **every 12–18 months**, in a maintenance window |
| **Security — high severity** | **CVSS ≥ 7 affecting an exposed service** | **2 weeks**, expedited window |
| **Security — critical, exploited** | **known exploitation in the wild** | **72 hours**, emergency change |
| Defect-driven | you have hit the bug | as required |

**And the version selection rule, which is the part people get wrong:**

> **Do not run the newest release. Run the one that has been in the field long enough for its
> problems to have been found by someone else.**

**Vendors label this** — Cisco's "suggested release", Juniper's "recommended", and equivalents —
**and the labelled version is typically several months behind the latest.** **It is the right
default**, and deviating from it should require a reason (a specific fix you need, or hardware
that requires a newer release).

**Practical additions:**

- **Read the release notes**, specifically the **open caveats** and **behaviour changes**
  sections. **They are long and they are where the surprises are documented.**
- **Upgrade a low-consequence device first** and leave it for a fortnight.
- **Standardise versions per platform and role.** **An estate running eleven versions of the
  same platform is an estate where every problem is unique.**
- **Keep the previous image on the device** where storage permits, so rollback does not require
  a network transfer at 03:00.
- **Check hardware compatibility before the window**, not during it — **a supervisor or a line
  card that the new release drops support for is a memorable discovery.**

## The end-of-support security problem

**A device past end of support receives no patches. Therefore:**

> **A vulnerability disclosed after that date is permanent**, and it will be found, because
> **researchers and attackers both scan for the versions that no longer receive fixes.**

**The compensating controls, in order of effectiveness:**

1. **Replace it.** Everything below is mitigation.
2. **Remove its exposure** — management on an isolated VRF or out-of-band network only
   (Chapter 60 §60.4)
3. **Segment what it can reach**
4. **Monitor it specifically** — a device you cannot patch is a device you should watch
5. **Document the accepted risk**, with an owner and a review date

**And note the timing asymmetry that makes this urgent:** **the end-of-support date is known
years in advance and the vulnerability is not.** **You cannot plan a response to a
vulnerability disclosed next March in a device you cannot patch. You can plan the replacement
now.**

## Beyond the device: the other lifecycles

**Three more calendars that produce outages, all of which belong in the same register.**

**Certificates.** **The most predictable outage in this book** (Chapter 58, Chapter 41 §41.2),
and they still happen. **Track expiry, alert at 60, 30 and 7 days, and automate renewal where
possible.**

**Support contracts and licences.** **A device with a lapsed contract cannot get a replacement
part**, and **a licence that expires may disable features** — some platforms degrade
substantially. **This is discovered during an incident with dispiriting regularity.**

**Circuits and commercial agreements.** **Auto-renewal at an unfavourable rate, or expiry with
no successor arranged.** Chapter 53 §53.2's circuit inventory carries the date.

**Cryptographic algorithms.** **The slowest lifecycle and the one nobody tracks.** **SHA-1,
TLS 1.0 and 1.1, 1024-bit RSA and DES have all gone from "standard" to "rejected by default"**,
and equipment that only supports them becomes unusable without failing. **Chapter 58 §58.4
covers the transition; the operational point is that it belongs on the same calendar.**

## Building the refresh plan

**The output of this section, and it is a document with dates and money in it.**

**From the inventory (Chapter 53 §53.2):**

| Device class | Count | EoL | Replace by | Est. cost |
|---|---|---|---|---|
| Core switch pair | 2 | **2027-04** | **2026-10** | £X |
| Access switches, gen 1 | 40 | 2028-01 | 2027-06 | £Y |
| Branch routers | 22 | 2029-06 | 2028-12 | £Z |
| Firewall pair | 2 | **2026-09** | **URGENT** | £W |

**Three columns beyond the obvious:**

**"Replace by" is earlier than EoL** — **by the migration duration, plus a margin.**

**Group by class, not by device.** **Replacing 40 access switches is one project**, and treating
them as 40 line items guarantees it is never funded.

**And carry the "what breaks if this fails and cannot be replaced" note** from the inventory,
because **that is the sentence that turns a technical date into a business decision.**

> **A refresh plan is a document that says what you will replace, when, and what it costs, over
> three years.** **Its purpose is to convert a series of surprises into a series of budget
> lines**, and that is the entire value.

## What breaks here

**A device past end of support discovered during a security review.** **The inventory did not
carry EoL dates**, or nobody reported on them. This is the failure this section exists to
prevent.

**A critical patch that cannot be applied.** **Past end of support.** Compensating controls, and
a replacement plan with a date.

**A firmware upgrade that introduced a worse problem.** **Latest release rather than suggested
release**, or **the open caveats were not read.**

**A rollback requiring an image that is not on the device.** **Keep the previous one.** A
40-minute transfer at 03:00 is avoidable.

**A line card unsupported by the new release, discovered in the window.** **Compatibility check
belongs before the window.**

**Eleven firmware versions across one platform.** **Every problem is now unique** and no
experience transfers. Standardise.

**A replacement part refused because the contract lapsed.** **The contract expiry date was in
nobody's calendar.**

**An expired certificate taking down a service.** **The most predictable outage there is**, and
it is still the most common self-inflicted one.

> **Network+ note.** Objective 3.2 covers lifecycle and patching. Over-learn: **EOL means no
> further support or patches**; **patches and firmware updates should be tested before
> deployment**; **a patch management policy defines the cadence**; and **unsupported systems
> should be isolated or replaced.** The EOL/EOS distinction is examined and is worth getting
> right.
