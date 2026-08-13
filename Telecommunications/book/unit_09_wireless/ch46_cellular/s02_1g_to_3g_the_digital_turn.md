# 46.2 1G to 3G: The Digital Turn

Three generations in twenty years, and the interesting transition is not the one people
expect. **The jump from analogue to digital mattered less than the jump from circuits to
packets** — and the second happened inside 3G rather than between generations.

## 1G — analogue, 1979

| | |
|---|---|
| Systems | **AMPS** (Americas), TACS (UK), NMT (Nordic) |
| Modulation | **FM**, exactly as broadcast radio |
| Channel | **30 kHz**, one call |
| Access | **FDMA** — one frequency per call |
| Security | **none whatsoever** |
| Data | none |

**A 1G call was an FM radio transmission**, and the consequences were exactly what that
implies:

**Anyone with a scanner could listen.** Not a vulnerability requiring skill — **a hobbyist
receiver tuned to the right frequency.** Several political scandals in the late 1980s and
early 1990s involved intercepted analogue calls, and the practice was widespread enough that
the US legislated against scanners covering those bands rather than fixing the system.

**Cloning was trivial.** The handset transmitted its identity in the clear at the start of
every call; **capture it, program another handset, and calls were billed to the victim.**
Fraud losses ran to hundreds of millions of dollars annually.

> **1G had no security because analogue FM offers nowhere to put any.** Encryption requires
> digital representation, which is the argument for the next generation independent of
> capacity.

## 2G — digital, 1991

**GSM in Europe, and it is the more consequential system by a wide margin.**

| | GSM | IS-95 (cdmaOne) |
|---|---|---|
| Access | **TDMA** — 8 timeslots per 200 kHz carrier | **CDMA** — codes on a 1.25 MHz carrier |
| Reuse | N = 4 or 7 | **N = 1** |
| Voice codec | 13 kb/s (full rate) | 8–13 kb/s variable |
| Security | **A5 encryption, SIM authentication** | encryption |
| Data | **GPRS 2.5G, EDGE 2.75G** | 1xRTT |
| Adoption | **~80% of the world** | mainly US, Korea |

**What digital bought:**

**Capacity.** Compressing voice to 13 kb/s and putting eight calls in one 200 kHz carrier is
**roughly three times the spectral efficiency of analogue.**

**Encryption.** Over-the-air encryption made casual interception impractical. **A5/1 has since
been broken** and was deliberately weakened for export at the time, **but it ended the scanner
era.**

**Authentication.** The **SIM** — Subscriber Identity Module — holds a secret key and
**proves the subscriber's identity cryptographically without transmitting the key.** Cloning
became hard.

**And the SIM is the underrated contribution.** It separated **the subscriber from the
handset**, which is why a European could change phones by moving a card and an American on a
CDMA network could not. **A design decision about identity that shaped two continents'
handset markets differently.**

### The message that was an afterthought

**SMS was specified as a way to use the signalling channel's spare capacity** — a 160-character
limit chosen because that is what fitted in the existing control-channel format.

**Nobody expected it to matter.** It was billed as a free or near-free extra, and it became a
substantial revenue stream and, for a decade, the dominant form of written communication for a
generation.

> **The feature added because there was spare room in a control channel outlasted most of the
> system's designed purposes.**

### 2.5G and 2.75G — packets arrive

**GPRS (2000)** added **packet switching** alongside GSM's circuits — **the first time a
cellular network carried IP** — at 40–100 kb/s.

**EDGE (2003)** improved the modulation to reach ~200–400 kb/s.

**And this is the transition that mattered.** Not analogue to digital, but **circuit to
packet** (Chapter 13's argument): a circuit is held for the duration of a call whether or not
data is flowing, **and packet switching lets a device be "always on" while consuming resources
only when transmitting.**

## 3G — data as a design goal, 2001

**UMTS/W-CDMA in most of the world**, CDMA2000 in some.

| | |
|---|---|
| Access | **W-CDMA** — 5 MHz carriers, N = 1 reuse |
| Initial rate | 384 kb/s |
| **HSPA (3.5G)** | **14 Mb/s down** |
| HSPA+ | 42 Mb/s |
| Core | **still circuit-switched for voice**, packet alongside |

**The significant change is the air interface: CDMA.**

### How CDMA works, and why N = 1

**Every user transmits on the same frequency at the same time**, distinguished by a
**spreading code.**

```
   User A's data  ×  code A  ─┐
   User B's data  ×  code B  ─┼──▶ all summed, all on one frequency
   User C's data  ×  code C  ─┘

   Receiver × code A  →  A's data emerges; B and C average to noise
```

**The codes are orthogonal**, so correlating the received signal with one user's code
**recovers that user and suppresses the others.**

**Two consequences:**

**Frequency reuse of 1.** Every cell uses every frequency (§46.1) — **there is no channel plan
at all**, which removes the design problem that N = 7 imposed.

**Soft handover** (§46.1) becomes possible, because adjacent cells are on the same frequency.

**And one severe constraint: the near-far problem.**

**A handset close to the tower swamps one far away**, because the codes' orthogonality is
imperfect and a strong signal's residue exceeds a weak signal entirely.

> **So CDMA requires extremely fast, precise power control** — the network commands every
> handset's transmit power **1,500 times per second**, keeping every arriving signal at
> roughly equal strength.

**This is why CDMA handsets had poorer battery life than GSM ones**, and why the system is
sensitive in ways TDMA is not: **power control failure is not degradation but collapse**, since
one handset at excessive power removes capacity from everyone.

### Cell breathing

**A property with no equivalent in TDMA**, and it surprises people.

**In CDMA, capacity and coverage are the same resource.** Each additional user raises the
noise floor for everyone (they are all on the same frequency), **so as a cell fills, its usable
radius shrinks.**

```
   Lightly loaded:  ⬤ ⬤ ⬤ ⬤   large cells, gaps covered
   Heavily loaded:  ● ● ● ●    small cells, gaps appear
```

**So a busy cell covers less ground**, and users at the edge lose service **not because
anything failed but because the cell contracted.**

**Coverage holes that appear only at peak times are the signature**, and they cannot be fixed
by adjusting the cell — only by adding capacity.

## Why 3G disappointed

**It is worth being specific**, because the story is usually told as a straightforward
success.

**The spectrum auctions were catastrophic.** European 3G licences raised over **€100
billion** in 2000–2001 — the UK auction alone raised £22.5 billion — **and the operators who
paid could not recover it.** Several were financially damaged for a decade, and network
build-out was slowed by the debt.

**The handsets were poor.** Early 3G phones had bad battery life (the power control above) and
were expensive.

**The applications did not exist.** The business case assumed video calling, which **nobody
wanted**, and the actual demand — mobile web and later apps — **arrived with the iPhone in
2007, six years after the licences were sold.**

> **The technology was correct, arrived on time, and its business case was wrong about what
> people would use it for.** The demand that eventually justified it was for something the
> planners had not considered.

**And the lesson recurs** — Chapter 43's ISM bands succeeded because nobody predicted what
would be built there, and 3G's licences were priced on a prediction that was wrong.

## The generational summary

| | 1G | 2G | 2.5G | 3G |
|---|---|---|---|---|
| **Year** | 1979 | 1991 | 2000 | 2001 |
| **Signal** | analogue | **digital** | digital | digital |
| **Access** | FDMA | **TDMA / CDMA** | TDMA | **W-CDMA** |
| **Reuse** | 7 | 4–7 | 4–7 | **1** |
| **Data** | none | SMS | **40–200 kb/s packet** | **384 kb/s – 42 Mb/s** |
| **Voice** | circuit | circuit | circuit | **circuit** |
| **Security** | **none** | encryption + SIM | same | mutual authentication |

**Note the voice row.** **Voice remained circuit-switched through 3G** — the packet network
carried data and voice used a separate, circuit-oriented path. **Unifying them onto IP is
LTE's contribution**, and §46.3 is that story.

## What breaks here

**Coverage holes appearing only at peak times on a 3G network.** Cell breathing. Add capacity.

**A 3G handset with poor battery life.** Power control at 1,500 Hz. Inherent.

**2G/3G networks being switched off.** Happening now worldwide, and **it strands devices** —
alarm panels, lift telephones, telemetry, older vehicles' emergency call systems. **Chapter 47
§47.4's IoT deployments frequently depend on 2G**, and the shutdown is a real operational
event rather than a background detail.

> **Network+ note.** Objective 2.4 touches cellular generations. Over-learn: **1G analogue,
> 2G digital with GSM/CDMA, 3G data-oriented, and the packet-switching transition at 2.5G.**
> The generational data rates appear as recall items.
