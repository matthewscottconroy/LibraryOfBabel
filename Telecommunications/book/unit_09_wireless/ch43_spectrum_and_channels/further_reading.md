# Chapter 43 — Further Reading

## Regulation

**Your national regulator's spectrum allocation table.** Ofcom (UK), FCC (US),
BNetzA (Germany), ACMA (Australia). Find yours and read the 2.4, 5 and 6 GHz entries —
channel availability, EIRP limits, DFS requirements and indoor/outdoor restrictions.
Exercise F6, and most people have never looked.

**FCC Part 15**, particularly §15.247 and §15.407.
The US rules for unlicensed operation. Dry, and it is the actual law rather than a
summary of it.

ETSI EN 300 328 (2.4 GHz) and **EN 301 893** (5 GHz).
The European equivalents, including the listen-before-talk and TPC requirements that differ
from the US rules.

Marcus, M. — his writing and talks on the ISM decision.
The engineer who made §43.1's decision, on why it met resistance and what he thinks
regulators still get wrong. Worth finding for the perspective of someone who opened a band
without knowing what would be built there.

ITU-R and the WRC-19 / WRC-23 final acts.
Where the 6 GHz allocations were decided, and why they differ by region.

## Wireless engineering

Coleman, D. & Westcott, D. — *CWNA Certified Wireless Network Administrator Study
Guide*.
The best practical book for this unit, again. The RF, spectrum and channel-planning
chapters are exactly this chapter's material at working depth.

Coleman, D., Westcott, D. & Harkins, B. — *CWAP Certified Wireless Analysis
Professional*.
For the analysis side: reading captures, interpreting utilisation and retry figures, and
distinguishing the interference types of §43.4.

Gast, M. (2005). *802.11 Wireless Networks: The Definitive Guide*, 2nd ed. O'Reilly.
Dated on standards and excellent on mechanism. Gast's *802.11ac: A Survival Guide* is
short, free from O'Reilly, and the clearest explanation of channel bonding and its
consequences.

**Bardwell, J. — the WLAN analysis papers.**
On what RSSI actually measures, why vendors' figures differ, and how to interpret survey
data. The antidote to wireless folklore, and freely available.

## Applied

Survey your own environment (exercise F1). **WiFiAnalyzer** (Android, open source),
**Wireless Diagnostics** (macOS — hold Option, click the Wi-Fi menu, then Window → Scan), or
**WiFi Explorer** (macOS/Windows).

Record every network, its channel and its width. Then assess: is the channel plan
sensible? Are there partial overlaps? Is anyone using 80 MHz in a crowded band? Most people
find at least one thing that should not be there.

Measure channel utilisation and retry rate at a quiet time and a busy one (exercise F2).
Enterprise access points report both; `iw dev wlan0 survey dump` on Linux gives utilisation.

**Get a spectrum analyser.** The cheapest useful option is an **RTL-SDR dongle (~£25)** with
`gqrx`, `SDR++` or `SDRangel`. It will not measure absolute power accurately and it will
show you the shape of the band, which is the point.

> **Watch a microwave oven on a waterfall display.** The 50% duty cycle at 2.45 GHz
> (§43.4) is unmistakable, and after seeing it once you will diagnose "the wireless breaks
> at lunchtime" instantly.

Check whether your access points already have spectrum analysis. Cisco CleanAir, Aruba
Spectrum Analysis, Mist and others include a dedicated radio. Many organisations own this
capability and have never enabled it.

**[tools/perfcalc.py](../../../tools/perfcalc.py) `noise`:**

```bash
python3 tools/perfcalc.py noise --bandwidth 20M --nf 5
python3 tools/perfcalc.py noise --bandwidth 160M --nf 5
```

The 9 dB difference between them is §43.2's width penalty, and computing it yourself is
the fastest way to internalise why wide channels cost range.

**`iw` on Linux**, for the parts GUI tools hide:

```bash
iw list                          # what your adapter supports, including channels
iw dev wlan0 scan | grep -E 'SSID|freq|signal'
iw dev wlan0 survey dump         # channel utilisation, noise
iw reg get                       # your regulatory domain
```

**`iw reg get` is worth running once.** If it says `country 00: DFS-UNSET`, your adapter is
using the most restrictive global defaults and may be refusing channels it could use.

**Lab 32** in this book's [labs/](../../../labs/) directory surveys a real environment,
builds a channel plan for a floor, then deliberately misconfigures partial overlap and
measures the throughput difference against a correct 1/6/11 plan — which makes §43.2's
argument empirical rather than asserted.

## For the certification-minded

Objective 2.4 expects frequency bands, channels and interference, and this chapter is
examined heavily.

The single most certain question: how many non-overlapping channels are there in
2.4 GHz? Three — 1, 6 and 11.

Eight more things worth over-learning:

1. 2.4 GHz: better range and penetration, 3 channels, more interference.
2. 5 GHz: more channels, higher throughput, shorter range, DFS on many channels.
3. 6 GHz: the most spectrum, the shortest range, recent clients only.
4. Wider channels give more throughput and fewer channels.
5. DFS means vacating a channel when radar is detected.
6. 2.4 GHz interferers: microwave ovens, Bluetooth, cordless phones, wireless cameras.
7. Co-channel interference is contention; adjacent-channel is corruption.
8. Regulatory domain determines available channels and power.

And the three operational facts worth more than the objective:

Partial overlap is worse than sharing a channel. 1/6/11, not 1/3/5/7/9/11 — and knowing
*why* lets you defend it.

**In dense deployments, narrow the channels.** More channels beats more megahertz per
channel, and it is counter-intuitive enough that you will have to explain it.

A survey showing a clear channel that does not work means a non-Wi-Fi interferer, and no
Wi-Fi tool will find it.
