# 8.4 OFDM and Modern Radio

Everything so far assumed one carrier. This section is about using hundreds at
once, and about the problem that forced the change.

## The problem: multipath

A radio signal reaches the receiver by several paths — direct, plus reflections off
walls, floors, filing cabinets, cars, the ground. The paths have different lengths,
so the copies arrive at different times.

The spread between the earliest and latest significant copy is the **delay
spread**, and it is a property of the environment:

| Environment | Typical delay spread |
|---|---|
| Small office | 50 ns |
| Large office, factory floor | 100–300 ns |
| Urban outdoor | 1–3 µs |
| Hilly or mountainous terrain | up to 20 µs |

Now compare that against the symbol period. A single-carrier system at 100 Msymbols
per second has a symbol period of **10 nanoseconds**. With a 100 ns delay spread,
each symbol's reflections arrive **ten symbol periods later** — spilling into ten
subsequent symbols.

That is intersymbol interference (Chapter 6 §6.3) of a severity that no reasonable
equaliser can undo. A single-carrier system in an indoor environment at high rate
simply does not work, and this was the wall that early wireless LAN designs hit.

## The OFDM answer

Instead of one carrier at 100 Msymbols/s, use **many carriers each running slowly**.

Split the channel into *N* narrow subcarriers. Divide the data among them. Each
subcarrier now carries symbols *N* times longer, so the same 100 ns delay spread is
a small fraction of a symbol period rather than ten times it.

For 802.11a with a 20 MHz channel:

- **64 subcarriers**, spaced 312.5 kHz apart
- 48 carry data, 4 are pilots for tracking, 12 are unused guard and DC
- Symbol period: **3.2 µs**, compared with 10 ns for the single-carrier equivalent
- A 100 ns delay spread is now **3% of a symbol period** rather than 1,000%

Intersymbol interference has been reduced from catastrophic to negligible, by
arithmetic rather than by cleverness.

## Why "orthogonal"

Packing hundreds of carriers into one channel would seem to guarantee interference
between them. It does not, and the reason is the *O* in OFDM.

Space the subcarriers by exactly the **reciprocal of the symbol period**. Then over
one symbol period, each subcarrier completes an exact whole number of cycles more
than its neighbour, and the integral of any two different subcarriers multiplied
together is **exactly zero**.

They are orthogonal. Each can be recovered with no contribution from any other,
even though their spectra overlap substantially. This is the same orthogonality
that let *I* and *Q* share a carrier in §8.2, applied across frequency instead of
within one carrier.

The spectral picture is striking: each subcarrier is a sinc function whose peak
sits exactly on the nulls of every other subcarrier's sinc.

```
     ╱╲    ╱╲    ╱╲    ╱╲    ╱╲
    ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲  ╱  ╲
   ╱    ╲╱    ╲╱    ╲╱    ╲╱    ╲
   ──────•─────•─────•─────•──────
         ↑     ↑     ↑     ↑
   each peak sits on its neighbours' nulls
```

The spectra overlap and the information does not, which is why OFDM is far more
spectrally efficient than the guard-banded FDM of Chapter 9 §9.1.

## The cyclic prefix

Orthogonality holds only if the receiver integrates over exactly one symbol period.
Multipath violates this, because a reflection arriving mid-symbol carries part of
the *previous* symbol into the integration window.

The fix is elegant. Before transmitting, **copy the end of each symbol and paste it
onto the front**. That copy is the **cyclic prefix**, and it is discarded at the
receiver.

```
   ┌──────────────────────────────┐
   │        OFDM symbol           │
   └──────────────────────────────┘
   ┌───┬──────────────────────────┐
   │ CP│        symbol            │      CP = copy of the symbol's tail
   └───┴──────────────────────────┘
     ↑
   reflections land here and are thrown away
```

Provided the delay spread is shorter than the cyclic prefix, every reflection lands
within the prefix, which the receiver discards. The remaining integration window
contains one clean symbol period and orthogonality is preserved.

In 802.11a the cyclic prefix is 800 ns for a 3.2 µs symbol — **20% overhead** — and
it tolerates a delay spread up to 800 ns, which covers essentially every indoor
environment. Later standards added a **short guard interval** of 400 ns, reducing
the overhead to 11% and increasing the rate by about 11%, at the cost of tolerating
less multipath. That is the "short GI" option in Wi-Fi configuration, and it is a
straightforward trade you can now reason about: it works well in small rooms and
poorly in large reflective ones.

## Why OFDM is computationally cheap

Generating hundreds of carriers with hundreds of oscillators would be absurd. It is
not what happens.

**An OFDM symbol is an inverse Fourier transform.** Take the *N* complex numbers
representing the constellation points for the *N* subcarriers, run an inverse FFT,
and the output is the time-domain waveform containing all *N* subcarriers
simultaneously. The receiver runs a forward FFT and recovers all *N* constellation
points at once.

This is why OFDM became practical exactly when it did. The idea dates to the 1960s
— Chang at Bell Labs patented it in 1966, and Weinstein and Ebert showed the FFT
implementation in 1971 — and it was unusable until digital signal processing became
cheap enough to run an FFT at the symbol rate. Cooley and Tukey's algorithm
(Chapter 5 §5.2) is what made it affordable, and the connection between those two
chapters is direct.

## The costs

OFDM is not free, and two of its costs shape real systems.

**Peak-to-average power ratio.** Hundreds of subcarriers with independent phases
occasionally align, producing a peak far above the average — typically 10–13 dB
above. The amplifier must be backed off to accommodate the peaks without
compressing them, which wastes power and generates heat.

This is a genuine problem for battery-powered transmitters, and it is why LTE's
uplink uses **SC-FDMA** — a variant with a lower PAPR — while its downlink uses
ordinary OFDMA. The base station has mains power and can afford the backoff; the
handset cannot.

**Sensitivity to frequency offset.** Orthogonality depends on the subcarrier
spacing being exact. A frequency error between transmitter and receiver destroys
it, producing **inter-carrier interference** in which every subcarrier leaks into
its neighbours. OFDM systems therefore spend pilot subcarriers and preamble time on
frequency synchronisation, and they are less tolerant of Doppler shift than
single-carrier systems — which matters for high-speed rail and is a live
consideration in 5G.

## Where OFDM is used

Essentially every high-rate system designed since 1995:

| System | Notes |
|---|---|
| Wi-Fi (802.11a/g/n/ac/ax/be) | 64 to 2,048 subcarriers depending on width and standard |
| LTE and 5G NR downlink | OFDMA — subcarriers allocated to different users |
| LTE uplink | SC-FDMA, for the PAPR reason above |
| DSL (ADSL, VDSL) | **DMT** — discrete multitone, OFDM by another name |
| DVB-T, DVB-T2, ISDB-T | Terrestrial digital television |
| DAB | Digital radio |
| DOCSIS 3.1 | Cable, from 2013 |
| Powerline networking | HomePlug and G.hn |

DSL's use is worth a note because it shows the technique's generality. VDSL2 divides
its band into up to 4,096 subcarriers, measures the SNR on **each one
individually**, and loads bits onto each according to what it can carry — 15 bits on
a clean low-frequency subcarrier, zero on one destroyed by a nearby AM radio
transmitter. It is Shannon's formula applied hundreds of times per line, and it is
why DSL rate falls smoothly with loop length rather than in steps: the high
subcarriers die one by one.

## OFDMA: from one user to many

802.11ax and 5G extend the idea. If subcarriers are independent, they need not all
serve the same user.

**OFDMA** — orthogonal frequency-division multiple access — allocates *groups of
subcarriers* to different users simultaneously. In Wi-Fi these groups are called
**resource units**, and a single transmission opportunity can serve four, eight or
more clients at once.

The gain is not raw throughput; it is efficiency with small frames. Chapter 44
§44.2 established that every Wi-Fi transmission carries substantial overhead — a
preamble, an interframe space, an acknowledgement — and that overhead is paid per
transmission regardless of how little data it carries. In a room full of devices
sending small packets, that overhead dominates. OFDMA lets one transmission carry
data for many clients, paying the overhead once.

This is why Wi-Fi 6's real-world advantage is largest in dense environments with
many small flows — a lecture theatre, an office — and smallest for one client
downloading one large file, which is exactly the case a naive benchmark measures.

## What breaks here

**Poor performance in a large reflective space with short guard interval enabled.**
The delay spread exceeds the cyclic prefix and orthogonality breaks down. Symptom:
high error rate at good signal strength. Fix: long guard interval.

**A constellation that will not lock on an OFDM link** — frequency offset
destroying orthogonality. Check the reference oscillator.

**An amplifier overheating on an OFDM transmitter.** PAPR backoff was insufficient,
or the design assumed a lower-order constellation than is in use.

**Wi-Fi 6 delivering no improvement in a single-client benchmark.** Expected. Its
gains are in dense multi-client scenarios, and measuring one client against one
access point measures the case OFDMA does not help.

> **Network+ note.** N10-009 expects awareness of OFDMA as a Wi-Fi 6 feature
> (objective 2.3) and expects channel width to affect throughput. The mechanism —
> many narrow subcarriers, orthogonal, with a cyclic prefix absorbing multipath —
> is what makes both facts follow rather than needing to be memorised, and it
> explains why the guard interval setting exists in every enterprise access point's
> configuration.
