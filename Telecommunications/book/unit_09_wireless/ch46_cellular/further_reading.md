# Chapter 46 — Further Reading

## Primary and historical

Ring, D. H. (1947). "Mobile Telephony — Wide Area Coverage." Bell Labs internal
memorandum.
**The founding document.** Short, and remarkable for how much of §46.1 is already in it,
including cell splitting.

MacDonald, V. H. (1979). "The Cellular Concept." *Bell System Technical Journal*.
The system as designed, immediately before AMPS was deployed. The clearest statement of the
reuse arithmetic, and freely available.

**3GPP specifications** (3gpp.org).
Enormous and freely available. TS 36.300 (LTE overview) and TS 38.300 (NR overview) are
the entry points — architecture and principles rather than bit-level detail.

## Books

Dahlman, E., Parkvall, S. & Sköld, J. — *4G LTE-Advanced Pro and The Road to 5G*, and
*5G NR: The Next Generation Wireless Access Technology*.
**The standard texts**, by Ericsson engineers who worked on the standards. Unusually good on
why the choices were made, which specifications do not explain.

Rappaport, T. — *Wireless Communications: Principles and Practice*.
Chapter 42's recommendation, and its cellular chapters cover §46.1's reuse arithmetic
properly.

Sauter, M. — *From GSM to LTE-Advanced Pro and 5G*, 4th ed. Wiley.
The best single book for a network engineer rather than a radio engineer. Covers all
generations at a systems level, with the protocol flows.

Holma, H. & Toskala, A. — the WCDMA, LTE and 5G volumes (Nokia).
Detailed, and good on the practical deployment considerations.

## On private cellular and spectrum

CBRS Alliance / OnGo documentation, and the **FCC Part 96** rules.
The three-tier sharing model of §46.4, and the SAS mechanism. Read it alongside Chapter 43
§43.1's AFC — they are the same idea with different details, and comparing them is
instructive.

Ofcom's shared access licence framework (UK) and BNetzA's local 5G licensing
(Germany).
Two different national approaches to making private cellular accessible. Germany's is the
most enterprise-friendly regime in existence and is worth understanding as a model.

**GSMA and 5G-ACIA** material on industrial private networks.
Vendor-influenced and technically substantial, particularly on the industrial use cases where
private 5G genuinely wins.

## Applied

**Field-test your own connection.** Exercise F1:

- **Android**: `*#*#4636#*#*` in the dialler, or apps like **Network Cell Info**,
  **CellMapper**, **NetMonster**
- **iOS**: Field Test Mode via `*3001#12345#*`

Record RSRP, RSRQ and SINR, and relate them to Chapter 42 §42.1:

| Cellular | Wi-Fi equivalent | Good |
|---|---|---|
| **RSRP** | RSSI | > −90 dBm |
| **SINR** | SNR | **> 20 dB** |
| RSRQ | — | > −10 dB |

And the same lesson applies: RSRP is the signal and SINR is what determines the rate.
A phone with four bars and poor SINR is Chapter 43 §43.4's problem in a different band.

Determine your band and whether you are on SA or NSA (exercise F2). NetMonster and Network
Cell Info both report it, and most people find they are on NSA mid-band — which is the
useful 5G of §46.4 and not the advertised one.

**CellMapper** — crowd-sourced tower locations and band information. Useful for
understanding what is actually deployed around you, and for explaining coverage.

Speed and latency tests at several times of day (exercise F3). The variation is
congestion, and it is the cellular equivalent of Chapter 43 §43.4's channel utilisation.

Check the 2G/3G shutdown timetable for your country (exercise F4). Then think about what
depends on it — lift telephones, alarm panels, telemetry, older vehicles. This is a live
operational problem and it is being under-managed almost everywhere.

**Lab 35** in this book's [labs/](../../../labs/) directory records cellular metrics along a
route, correlates handovers with signal changes, compares LTE and 5G at the same locations,
and works the reuse arithmetic for a hypothetical deployment.

## For the certification-minded

Objective 2.4 mentions cellular, and it is examined lightly — the depth here exceeds the
requirement considerably.

Five things worth over-learning:

1. Cellular works by frequency reuse across many small cells.
2. Capacity is increased by making cells smaller (cell splitting).
3. **The generations**: 1G analogue, 2G digital, 3G data, **4G/LTE all-IP**, 5G with three
   service classes.
4. 5G's bands trade speed against range — mmWave fastest and shortest.
5. **Network slicing** creates logical networks with different characteristics.

And the three things worth more than the objective:

Ask which band and whether it is SA. Almost every claim about 5G depends on the answer,
and most deployments are mid-band NSA.

**Air latency is not end-to-end latency.** A 1 ms radio does not help if the application is
100 ms away.

Private cellular is right for some industrial sites and wrong for offices. The
deciding factors are area, mobility at speed, and whether determinism is actually required.
