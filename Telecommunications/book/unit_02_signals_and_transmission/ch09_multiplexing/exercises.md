# Chapter 9 — Exercises

## A. Recall

**A1.** Name the four physical multiplexing techniques and state what resource each
divides.

**A2.** Why do FDM systems need guard bands, and what does OFDM do instead?

**A3.** Derive the DS0's 64 kb/s from the sampling theorem and the quantisation
choice.

**A4.** State the composition and rate of a T1 and an E1. Why do they differ?

**A5.** What resource does statistical multiplexing divide, and how does that differ
from the other four?

## B. Apply

**B1.** An analog carrier system allocates 4 kHz per voice channel for a 3.1 kHz
signal. Compute the guard-band overhead as a percentage. Then compute the total
bandwidth of a mastergroup (600 channels) and compare with the 2.52 MHz figure in
§9.1.

**B2.** Verify the T1 rate: 24 channels × 8 bits + 1 framing bit, at 8,000 frames
per second. Then compute the framing overhead as a percentage, and the E1
equivalent.

**B3.** DS2 is 6.312 Mb/s but 4 × DS1 is 6.176 Mb/s. Account for the 136 kb/s
difference. Explain why it exists and what operational problem it creates.

**B4.** 100 users, 2 Mb/s each when active, 8% activity. Compute: the capacity a
reserved scheme requires; the expected aggregate load; the mean and standard
deviation of the number simultaneously active; and a provisioned capacity giving
six standard deviations of headroom. State the multiplexing gain.

**B5.** Repeat 9.9 for 1,000 users and for 10,000, keeping the per-user figures
the same. Tabulate the gain in each case and explain the trend in terms of √*n*.

**B6.** Verify your answers with the tool:

```bash
python3 tools/simnet.py statmux --users 100 --rate 2 --activity 0.08 --link <your figure>
```

**B7.** A CDMA system uses a 3.84 Mchip/s spreading rate to carry a 12.2 kb/s
voice codec. Compute the processing gain in dB. If the receiver requires 6 dB of
SNR after despreading, what SNR can it tolerate before despreading?

**B8.** A DWDM system has 80 channels at 200 Gb/s on 50 GHz spacing. Compute the
total capacity and the total optical bandwidth occupied. Express the spectral
efficiency in bits per second per hertz and compare with a Wi-Fi channel's.

## C. Analyse

**C1.** Show that the coefficient of variation of the number of simultaneously
active users falls as 1/√*n*, and use this to explain rigorously why statistical
multiplexing gain increases with population size. Then identify the assumption in
the binomial model that fails in practice and describe a realistic scenario where
it does.

**C2.** Statistical multiplexing gives up admission control. Construct an argument
that this loss is more serious than the loss of guaranteed bandwidth, using the
1986 congestion collapse and the behaviour of a modern video conferencing service
under contention as evidence. Then argue the opposite position.

**C3.** GPS signals arrive roughly 20 dB below the thermal noise floor and are
decoded successfully. Explain how, computing the processing gain required. Then
explain what this implies about the relationship between Shannon's capacity formula
and spread spectrum — specifically, whether spread spectrum beats Shannon or
operates within him.

**C4.** An EDFA amplifies all DWDM channels together, and its gain is not flat
across the band. Explain what happens to the surviving channels when one channel
fails, and why dynamic gain equalisation is necessary. Then explain why
dispersion-shifted fibre, designed to put zero dispersion at 1550 nm, turned out to
be a poor choice for DWDM.

**C5.** Erlang solved the trunk-dimensioning problem in 1909 and the packet
switching community rediscovered the same mathematics in the 1960s. Compare the two
formulations: what is the resource, what is the unit of demand, what is the failure
mode, and what does each system do when demand exceeds supply? Identify what is
genuinely different and what is the same problem renamed.

## D. Design

**D1.** You are dimensioning the Internet uplink for a 400-bed student residence.
Measurements from a comparable building:

- Peak simultaneous active users: 260 (19:00–23:00)
- Mean per-active-user demand at peak: 4.2 Mb/s
- 95th percentile per-user demand: 22 Mb/s
- Once a month, a game or OS update produces a correlated peak in which
  ~180 users pull simultaneously at their maximum line rate for 40 minutes.

Determine the capacity to purchase. Your answer must: compute what a reserved
scheme would require; compute the statistically multiplexed figure with stated
headroom; address the correlated-peak case explicitly and say whether you would
size for it; and state which of §9.3's given-up properties the residents will
notice and when. Justify the final number against cost.

## E. Troubleshoot

**E1.** A regional ISP serves 8,000 subscribers on a mix of 100 Mb/s and 500 Mb/s
plans, over a 40 Gb/s transit circuit. The circuit is provisioned at a 1:30
oversubscription ratio, which has been comfortable for two years.

Over six weeks, complaints rise. The evidence:

- Circuit utilisation: 34% mean, 71% peak at 20:00 — both up from a year ago but
  not alarming.
- 95th-percentile latency to a reference host has risen from 14 ms to 61 ms,
  entirely between 19:30 and 22:30.
- Packet loss during that window: 0.4%.
- Flow data shows a single video service now accounting for 47% of evening bytes,
  up from 18%.
- The complaints are disproportionately from customers on the 500 Mb/s plan.

Explain what has happened in terms of §9.3's assumptions. State specifically which
assumption of the multiplexing model has been violated and why the change in
traffic mix caused it. Explain why the higher-tier customers complain more. Then
give three responses ranked by cost, and state which of them addresses the cause
rather than the symptom.
