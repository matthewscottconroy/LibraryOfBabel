# Lab 09 — Wireless Survey and Analysis

**Corresponds to:** Chapters 42, 43, 44, 45
**Week:** 9
**Time:** 120 minutes

---

## Objectives

- Measure signal, noise and SNR, and demonstrate that SNR rather than signal
  predicts throughput.
- Survey a real space and map coverage against measured rate.
- Identify channel overlap and demonstrate that partial overlap is worse than
  co-channel.
- Observe rate adaptation as a constellation choice, in real time.
- Demonstrate that raising transmit power degrades a multi-AP network.
- Compute a link budget and compare it with measurement.

---

## You will need

- A Wi-Fi analyser. A phone application (WiFiman, WiFi Analyzer, AirPort Utility
  with scanning enabled) is sufficient for most of this.
- A **spectrum analyser** if available — the lab explains what it shows that the
  analyser cannot, and it is worth borrowing one for twenty minutes.
- At least one access point you can configure: channel, width and power.
  Two is much better.
- A laptop with `iw`/`iwconfig` (Linux) or the equivalent, and `iperf3`.
- A tape measure or a floor plan with a scale.
- `perfcalc.py` from [tools/](../tools/).

**Fallback:** Parts 1, 2 and 5 need only a phone analyser and one AP. Part 3
needs two APs; if you have one, demonstrate it at the front of the room.

---

## Procedure

### Part 1 — What is actually on the air

**1.** Open the analyser and record, for every network visible from where you are
sitting:

| SSID | BSSID | Band | Channel | Width | Signal (dBm) |
|---|---|---|---|---|---|

**2.** Count how many networks share each 2.4 GHz channel. Count how many are on
channels other than 1, 6 and 11.

**3.** Note the **noise floor** the analyser reports on each band, if it reports
one. Compute the theoretical floor and compare:

```bash
python3 ../tools/perfcalc.py noise --bandwidth 20M --nf 6
```

**4.** If a spectrum analyser is available, look at 2.4 GHz for sixty seconds and
record anything that is **not** Wi-Fi. Identify it if you can — a microwave oven
is a broad hump on a mains-synchronised duty cycle; a video sender is a constant
narrow carrier; a cordless phone hops.

---

### Part 2 — Signal, noise, and what predicts throughput

**5.** Associate to your test AP. At a fixed position, record:

```bash
iw dev <iface> link          # signal, bitrate, MCS
iw dev <iface> station dump  # more detail including signal average
```

Record: signal (dBm), noise if reported, negotiated bitrate, and MCS index.

**6.** Measure actual throughput:

```bash
iperf3 -c <server> -t 20
```

**7.** Repeat steps 5 and 6 at **five positions** of increasing distance and
obstruction. Tabulate: position, distance, signal, SNR, negotiated rate, measured
throughput.

**8.** Plot throughput against signal, and throughput against SNR. Note which is
the better predictor.

**9.** At the furthest usable position, compute the Shannon capacity for your
measured SNR and channel width, and compare with the negotiated PHY rate and with
measured throughput:

```bash
python3 ../tools/perfcalc.py shannon --bandwidth 20M --snr-db <your SNR>
```

Record all three numbers and the ratios between them.

---

### Part 3 — Channels and overlap

**10.** Set AP1 to 2.4 GHz channel 1. Measure throughput from a client.

**11.** Set AP2 to **channel 1 as well** — co-channel. Run traffic on both
simultaneously and measure each client's throughput.

**12.** Now set AP2 to **channel 3** — partial overlap. Repeat.

**13.** Finally set AP2 to **channel 6** — non-overlapping. Repeat.

**14.** Tabulate all three configurations: AP1 throughput, AP2 throughput, total,
and any retransmission or error counters you can read.

**15.** In each case, note whether the two APs can *hear* each other in the
analyser.

---

### Part 4 — The power experiment

**16.** With two APs on non-overlapping channels, set both to a **low** transmit
power — the minimum your equipment allows, or around 8–11 dBm.

**17.** Walk the space and record where each AP is the stronger, and where a
client roams between them. Measure throughput at three positions.

**18.** Now set both APs to **maximum** transmit power.

**19.** Repeat step 17 exactly. Record: does the roaming point move? Does a client
in the far corner still associate to the near AP? What happens to the measured
throughput at your three positions?

**20.** From one position at the boundary, examine the client's negotiated rate in
both configurations.

---

### Part 5 — A link budget on paper

**21.** For your AP and a client position, gather: AP transmit power (from its
configuration), antenna gain (from the datasheet, typically 2–5 dBi for an
omni AP), distance, and frequency.

**22.** Compute the expected received power:

```bash
python3 ../tools/perfcalc.py linkbudget --freq 5200 --distance 0.02 \
    --tx-power 17 --tx-gain 3 --rx-gain 2 --bandwidth 20M --nf 6
```

(Adjust to your actual figures. Distance in kilometres — 20 m is 0.02.)

**23.** Compare the computed received power with the measured signal from step 5.
Record the discrepancy.

**24.** Account for it. Free-space path loss assumes free space; you are indoors.
Estimate the obstruction losses from Chapter 42 §42.4 — plasterboard ~3 dB, brick
10–15 dB, reinforced concrete 20–30 dB — and add them until the computation
matches. Record how many walls' worth of loss the discrepancy represents, and
check it against how many walls are actually in the way.

---

## Expected observations

- **Step 2:** the 2.4 GHz band is crowded, and there will be networks on channels
  other than 1, 6 and 11, put there by someone trying to avoid the crowd.
- **Step 8: SNR predicts throughput and raw signal does not.** Two positions with
  the same signal strength but different noise floors give very different rates.
  This is the lab's central finding.
- **Step 9:** measured throughput is typically 40–60% of the negotiated PHY rate,
  and the PHY rate is well below Shannon. Both gaps have explanations and you
  should be able to give them.
- **Step 14: co-channel is the *best* of the two-AP configurations after
  non-overlapping.** The two APs hear each other and take turns under CSMA/CA,
  sharing capacity in an orderly way. **Partial overlap on channel 3 is the
  worst** — neither AP can decode the other, so neither defers, and both corrupt
  each other's frames.
- **Step 19: maximum power makes things worse.** Clients hold associations to
  distant APs, negotiated rates at the boundary drop, and aggregate throughput
  falls even though every signal reading improved.
- **Step 23:** the free-space computation predicts a *stronger* signal than you
  measure, by 10–30 dB depending on the building.

---

## Break it

Already integrated — Part 3's channel 3 and Part 4's maximum power are the
deliberate faults, and they are deliberate because they are the two mistakes
practitioners actually make.

Two more if time allows:

**A. Set the AP to a 40 MHz channel width in 2.4 GHz.** Measure throughput, then
measure again with a second network active. Record the effect on both.

**B. Disable low data rates** (1, 2, 5.5, 11 Mb/s) on the AP if it supports it.
Measure aggregate cell throughput before and after with a mix of clients at
different distances.

---

## Debrief

**1.** Present your table from step 7 and your two plots from step 8. State which
of signal and SNR is the better predictor of throughput, and explain *why* with
reference to Chapter 4 §4.4. Then explain what a user's "full bars" actually
measures and why it can accompany terrible performance.

**2.** At your furthest position you recorded a measured SNR, a Shannon capacity,
a negotiated PHY rate, and a measured throughput. Give all four numbers and
account for **each** gap between consecutive figures.

**3.** Channel 3 performed worse than channel 1 despite "avoiding" the neighbour.
Explain the mechanism, distinguishing what happens when two transmitters can
decode each other from what happens when they cannot. State the practical rule in
one sentence.

**4.** Maximum transmit power improved every signal reading and degraded the
network. Give the four distinct mechanisms from Chapter 45's introduction, and
state which of them you observed directly and which you inferred.

**5.** Your link budget computation over-predicted the received signal by X dB.
Convert that into walls, using the material figures. Does the number match the
building? If not, propose what else is absorbing — and note that a survey done in
an empty room does not include the people.

**6.** A colleague proposes fixing poor coverage in a far office by raising the
nearest AP to maximum power. Write the reply you would actually send: three
sentences, one alternative proposal, and one measurement you would take first.

---

## Feeds the project

Deliverable 4 is due this week and requires a wireless design for Meridian's four
distinct spaces — reception, open-plan operations, training room and warehouse.
Parts 1, 3 and 4 of this lab are directly reusable: the channel plan must be
derived as in Part 3, the power position must be argued as in Part 4, and at least
one link budget must be computed as in Part 5.
