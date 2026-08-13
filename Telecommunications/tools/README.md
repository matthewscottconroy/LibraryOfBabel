# Tools

Runnable Python companions to the textbook. No dependencies beyond the standard
library; every script runs with `python3 <script>.py --help`.

These exist for one purpose: **to let you check a claim rather than believe it.**
Every number the book asserts about capacity, latency, subnetting or channel
behaviour can be reproduced here. Where a tool disagrees with the text, the text
is wrong and should be reported.

They are not a substitute for hand working. A subnet calculator used *instead of*
understanding produces someone who cannot look at a routing table and see that
two entries overlap. Use them to verify, and to drill.

---

## `netcalc.py` — subnetting, VLSM, summarisation

Companion to Chapters 25–27 and Appendix A.

```bash
# Analyse a prefix, with the binary working shown
python3 netcalc.py subnet 192.168.10.70/27 --binary

# Split a block into equal subnets
python3 netcalc.py split 172.16.0.0/16 --into 8

# VLSM allocation, largest-first, from required host counts
python3 netcalc.py vlsm 172.16.0.0/16 2000 500 100 25 2 2

# Shortest covering prefix, with the binary common-prefix working
python3 netcalc.py summarise 198.51.100.0/24 198.51.101.0/24 \
                             198.51.102.0/24 198.51.103.0/24

# The local-or-remote decision a host makes for every packet (§25.3)
python3 netcalc.py local 192.168.10.70/27 192.168.10.100
```

The `local` subcommand is the most pedagogically useful: it shows both AND
operations side by side and states which address the host will ARP for. That is
the decision Chapter 25 §25.3 argues is the whole point of the subnet mask, and
seeing it done twice makes the wrong-mask symptom obvious.

---

## `subnet_practice.py` — unlimited drills with worked solutions

Companion to Chapters 2 and 26.

```bash
python3 subnet_practice.py                          # 10 mixed, interactive
python3 subnet_practice.py --topic subnet --count 25
python3 subnet_practice.py --sheet 20 --answers     # printable worksheet + key
python3 subnet_practice.py --sheet 20 --topic binary --seed 42
```

Topics: `binary`, `mask`, `subnet`, `design`, `summary`, `mixed`.

Every question comes with the **worked solution**, not just the answer — the
block-size derivation for subnetting, the place-value subtraction for binary
conversion, the common-prefix binary for summarisation. Use `--seed` to produce
the same worksheet twice (useful for handing out an assessed version and a
practice version).

Drill this until Chapter 26's operations take under thirty seconds without a
chart. Chapter 26 makes the argument for why that investment is worth making
exactly once; this is the mechanism.

---

## `perfcalc.py` — performance and link arithmetic

Companion to Chapters 3, 4 and 42.

```bash
# Bandwidth-delay product, and whether window scaling is needed
python3 perfcalc.py bdp --rate 1G --rtt 100

# Max single-stream throughput for a given window
python3 perfcalc.py window --window 64K --rtt 100

# Mathis: throughput ceiling from a loss rate
python3 perfcalc.py loss --mss 1460 --rtt 80 --loss 0.001

# Latency decomposed into its four components
python3 perfcalc.py latency --distance 7000 --hops 14 --rate 10G

# Ethernet goodput efficiency for a payload size
python3 perfcalc.py goodput --payload 1460 --rate 1G
python3 perfcalc.py goodput --payload 160 --rate 1G      # a voice packet

# Shannon capacity, with the curve at other SNRs
python3 perfcalc.py shannon --bandwidth 20M --snr-db 30

# Thermal noise floor for a bandwidth and receiver noise figure
python3 perfcalc.py noise --bandwidth 20M --nf 6

# A complete point-to-point link budget with margin and verdict
python3 perfcalc.py linkbudget --freq 5800 --distance 34 --tx-power 23 \
    --tx-gain 27 --rx-gain 27 --tx-loss 2 --rx-loss 2 \
    --bandwidth 20M --nf 6 --sensitivity -85

# Decibel conversions in both directions
python3 perfcalc.py db --ratio 400
python3 perfcalc.py db --db 26
python3 perfcalc.py db --dbm -65
```

`linkbudget` is the tool for Chapter 42's exercises and for the mountaintop link
in Chapter 4's design problem. It reports EIRP separately so you can check it
against a regulatory limit, and gives a verdict on the margin — because a link
with 4 dB of margin works in the lab and fails in the rain.

---

## `simnet.py` — reproducing the book's numerical arguments

Companion to Chapters 3, 9, 13 and 16.

```bash
# ALOHA's 18.4% and slotted ALOHA's 36.8% ceilings
python3 simnet.py aloha

# What carrier sense and collision detection actually buy
python3 simnet.py csma --stations 20 --load 0.8

# Statistical multiplexing gain — the argument that decided packet vs circuit
python3 simnet.py statmux --users 100 --rate 1 --activity 0.05 --link 20

# Queueing delay against utilisation: the rho/(1-rho) curve
python3 simnet.py queue

# Where Ethernet's 64-byte minimum frame comes from
python3 simnet.py minframe --length 2500 --rate 10M
```

`statmux` is the one to run before Chapter 13. It computes the capacity a
circuit-switched design would need, the capacity a packet-switched design
actually needs, and the probability that the second is insufficient — and the
gap between those numbers is the reason the Internet exists in the form it does.

`queue` is the one to run before any capacity-planning conversation.

---

## Suggested use by week

| Week | Topic | Tool |
|---|---|---|
| 1 | Performance vocabulary | `perfcalc.py latency`, `perfcalc.py bdp` |
| 2 | Signals, media, link budgets | `perfcalc.py shannon`, `noise`, `linkbudget` |
| 3 | Switching, multiplexing | `simnet.py statmux`, `simnet.py queue` |
| 4 | Ethernet | `simnet.py aloha`, `csma`, `minframe` |
| 6 | Subnetting | `subnet_practice.py`, `netcalc.py` — heavily |
| 7 | Routing, address plans | `netcalc.py vlsm`, `netcalc.py summarise` |
| 9 | Wireless | `perfcalc.py linkbudget`, `noise` |
| 10 | WAN, satellite | `perfcalc.py bdp --rtt 500`, `loss` |
| 13 | Performance troubleshooting | `perfcalc.py loss`, `goodput`, `window` |

---

## A note on verification

Every tool here was checked against the figures quoted in the text. A few worth
confirming yourself, because they are the ones the book leans on hardest:

- `perfcalc.py goodput --payload 1460 --rate 1G` → **949 Mb/s**, matching
  Chapter 3 §3.1's efficiency calculation and the ~940 Mb/s that `iperf3`
  reports on a healthy gigabit link.
- `perfcalc.py bdp --rate 1G --rtt 100` → **12.5 MB**, and the note that an
  unscaled window caps the path at **5.24 Mb/s** — Chapter 3 §3.4's table.
- `simnet.py aloha` → peaks at **0.1839** and **0.3679**, which are 1/2e and
  1/e exactly.
- `simnet.py statmux` → a **5× gain** with an overflow probability of about
  2 × 10⁻⁸, which is Chapter 13 §13.4's argument.

If you find a discrepancy between a tool and the text, the tool is easier to
audit — read the source, it is short — and the text should be corrected.
