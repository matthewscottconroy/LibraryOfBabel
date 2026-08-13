# Lab 02 — Media, Signals, and Link Budgets

**Corresponds to:** Chapters 4, 5, 6, 10, 42
**Week:** 2
**Time:** 90 minutes

---

## Objectives

- Identify cable types, categories and connectors by inspection, and state what
  each is rated for.
- Terminate and test a patch cable, and distinguish what a continuity tester
  proves from what a certifier proves.
- Measure the throughput difference between links of different rates and account
  for the gap between rate and goodput.
- Compute a complete link budget and a Shannon capacity, and check both against
  a measurement.
- Observe attenuation's frequency dependence and connect it to cable category
  limits.

---

## You will need

- A selection of cables: Cat5e, Cat6, Cat6a if available; a coaxial patch lead;
  a fibre patch lead (multimode and single-mode if possible). Damaged and
  miswired examples are more useful than perfect ones.
- A cable continuity tester. A certifier if your institution has one — if not,
  §4 explains what you are missing and it is worth stating.
- Two hosts with `iperf3`, connected through a switch that can be forced to
  different speeds.
- `perfcalc.py` from [tools/](../tools/).

**Fallback:** Parts 1 and 4 need no equipment beyond the cables. Part 2 can be
demonstrated once at the front if there is only one tester. Part 3 needs two
hosts and any switch.

---

## Procedure

### Part 1 — Identification

**1.** Lay out every cable. For each, record: the type, the printed category or
specification (read the jacket — it is printed there), the connector, and what
standard it is rated for.

**2.** For each, state from Appendix B §B.14 the maximum distance and the highest
Ethernet standard it supports. Note where the answer is "it depends" — Cat6 runs
10GBASE-T to 55 m and 1000BASE-T to 100 m, and the distinction matters.

**3.** Examine the fibre leads. Identify single-mode from multimode by jacket
colour (yellow versus aqua/orange, conventionally) and by connector. Look into
the ferrule end — **not at a live transmitter** — and note the core diameter
difference if a magnifier is available.

**4.** For each cable, state one environment in which it would be the right
choice and one in which it would be wrong, with the reason.

---

### Part 2 — Termination and testing

**5.** Terminate a patch cable to T568B at both ends. Take your time; the pass
rate on first attempts is not high and that is itself informative.

**6.** Test it with the continuity tester. Record the result.

**7.** Now terminate a second cable **deliberately wrong** — swap the green and
orange pairs at one end only, producing a crossover. Test it.

**8.** Terminate a third with a **split pair**: pins connected correctly
end-to-end, but using one wire from each of two different twisted pairs for a
signal pair.

**9.** Test the split-pair cable with the continuity tester. **Record what it
reports.** Then, if a certifier is available, test it with that.

---

### Part 3 — Rate, goodput and measurement

**10.** Connect two hosts through the switch. Run:

```bash
iperf3 -s                      # on host A
iperf3 -c <host A> -t 30       # on host B
```

Record the throughput.

**11.** Force both switch ports to 100 Mb/s. Repeat. Then 10 Mb/s if the switch
supports it.

**12.** For each rate, compute the theoretical maximum goodput:

```bash
python3 perfcalc.py goodput --payload 1460 --rate 1G
python3 perfcalc.py goodput --payload 1460 --rate 100M
```

Compare with measurement. Compute the percentage of theoretical achieved.

**13.** Now measure with small packets:

```bash
iperf3 -c <host A> -u -b 100M -l 160 -t 20
```

and compute the theoretical goodput for a 160-byte payload. Compare.

---

### Part 4 — Budgets on paper

**14.** A proposed 5.8 GHz point-to-point link spans 8 km. Transmit power
20 dBm, 24 dBi antennas at both ends, 1 dB cable loss each end, 20 MHz channel,
receiver noise figure 6 dB, receiver sensitivity −80 dBm.

```bash
python3 perfcalc.py linkbudget --freq 5800 --distance 8 --tx-power 20 \
    --tx-gain 24 --rx-gain 24 --tx-loss 1 --rx-loss 1 \
    --bandwidth 20M --nf 6 --sensitivity -80
```

Record FSPL, received power, noise floor, SNR, Shannon capacity, and margin.

**15.** Recompute by hand for 16 km, using only the 6 dB rule from Chapter 42
§42.2. Check with the tool.

**16.** The client asks whether doubling transmit power would let the link reach
16 km. Answer with arithmetic, in one sentence.

---

## Expected observations

- **The split-pair cable passes the continuity test.** Every pin is connected to
  the correct pin at the other end, which is all a continuity tester checks. It
  will fail a certifier on crosstalk, and it will work at 10 and 100 Mb/s and
  fail or perform erratically at 1 Gb/s — because the untwisted signal pair has
  no common-mode rejection (Chapter 6 §6.4).
- **Gigabit measures ~940 Mb/s**, or about 99% of the 949 Mb/s theoretical.
  Anything materially lower is worth investigating.
- **100 Mb/s measures ~94 Mb/s.** The ratio holds because the overhead is
  proportional.
- **The 160-byte UDP test achieves far less than the rate suggests** — around 67%
  efficiency, because a third of the wire is header.
- **The 8 km link has roughly 15 dB of margin; the 16 km link has about 9 dB.**
  Doubling power adds 3 dB, so it does not recover the 6 dB that doubling the
  distance cost.

---

## Break it

**A.** Insert the crossover cable between the two hosts directly (no switch).
Does the link come up? Modern interfaces have Auto-MDI/MDI-X and will fix it
silently — which is why crossover cables have largely disappeared, and why a
student who learned about them in 2005 finds them mysteriously irrelevant.

**B.** Force one switch port to 100 Mb/s **full duplex** and leave the host on
autonegotiate. Run `iperf3` again and read the interface counters on both ends:

```bash
ip -s link show <iface>
```

Record what you see. You have just created a duplex mismatch, and Chapter 66 §66.2
predicts both the throughput collapse and the counter signature.

**C.** If you have a fibre lead, disconnect and reconnect it with a fingerprint on
the ferrule. If an optical power meter is available, measure before and after.

---

## Debrief

**1.** Your split-pair cable passed the continuity test. State precisely what
property the tester measures and what property it does not, and explain — with
reference to Chapter 6 §6.4 — why the cable fails at 1 Gb/s and works at 100 Mb/s.

**2.** You measured 940 Mb/s on a 1 Gb/s link. Show the arithmetic that accounts
for the missing 60 Mb/s, itemised by header. Then explain why a technician who
reports "the gigabit link is only doing 940" has misunderstood something.

**3.** Compute the efficiency of your 160-byte UDP test. A VoIP vendor claims 100
concurrent calls fit on a 10 Mb/s link at 64 kb/s per call. Evaluate the claim.

**4.** The 8 km link has adequate margin and the 16 km link is marginal. Using
only the 6 dB rules, state what *would* recover the margin at 16 km, and rank the
options by cost.

**5.** In Break-It B you created a duplex mismatch. Which counter incremented,
and why is that counter's name — "late collision" — a precise description of what
happened? Explain why the throughput got *worse* as `iperf3` ran harder, and why
that is diagnostic.

**6.** A colleague proposes Cat5e for a new installation because "it does gigabit
and gigabit is all we need". Construct the counterargument from Chapter 10's
chapter introduction, with the labour-versus-material figures made explicit. Then
construct the strongest case *for* Cat5e, and say which you would recommend and
under what condition your answer would change.
