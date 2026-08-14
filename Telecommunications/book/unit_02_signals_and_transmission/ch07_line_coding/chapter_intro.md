# Chapter 7 — Line Coding

Here is a problem that does not occur to most people until it is pointed out, and
which then turns out to have shaped every wired standard in this book.

You are receiving a stream of bits at one gigabit per second. To read them, you
must sample the wire once per bit — at the right instant, in the middle of each
bit, one billion times per second. Your clock and the sender's clock are separate
crystal oscillators in separate buildings, and no two crystals in the world run at
exactly the same rate. A typical crystal is accurate to ±50 parts per million,
which at one gigabit means your sampling point drifts by one full bit period after
about twenty thousand bits — two microseconds.

There is no wire carrying the sender's clock. There never is; a separate clock
line would double the cable cost, and at gigabit speeds it would arrive skewed
relative to the data anyway.

So the clock must be recovered from the data itself. The receiver watches for
transitions — edges where the voltage changes — and uses them to keep its sampling
point aligned. Every edge is a correction.

And now the problem is visible. **What if there are no edges?**

Send a hundred consecutive zeros as a hundred bit-times of low voltage, and the
receiver sees a flat line. It has nothing to synchronise against. After a few
thousand identical bits it no longer knows where one bit ends and the next begins,
and when the data finally changes it may have miscounted. Worse, some transmission
media — anything with a transformer or a capacitor in the path, which is nearly
everything — physically cannot pass a sustained DC level. The flat line does not
merely fail to carry timing; it decays.

**Line coding** is the answer: a transformation applied to the data before
transmission that guarantees the wire never goes too long without a transition and
never accumulates a DC offset, at the cost of some of the channel's capacity.

## The tradeoff, stated once

Every line code spends bandwidth to buy timing and DC balance. The question is
always *how much*, and the answer has changed dramatically as speeds have risen —
which makes the history of Ethernet's line codes an unusually clean illustration
of engineering under a moving constraint.

| Code | Used by | Overhead | Efficiency |
|---|---|---|---|
| Manchester | 10BASE-T Ethernet | 100% | 50% |
| 4B/5B | 100BASE-TX, FDDI | 25% | 80% |
| 8B/10B | 1000BASE-X, PCIe, SATA | 25% | 80% |
| 64B/66B | 10GBASE-R and above | 3.1% | 96.9% |
| 256B/257B | 200/400G Ethernet | 0.4% | 99.6% |

Manchester spends *half the wire* on timing. That was an entirely reasonable
decision in 1983 — the alternative was unreliable clock recovery, and bandwidth on
a 10 Mb/s coaxial segment was not the scarce resource. By the time 10 Gigabit
Ethernet was being designed, halving the effective rate was unthinkable, and the
engineering effort went into scrambling techniques that achieve transition density
statistically rather than by construction.

Notice the shape of that table. It is the same tradeoff Chapter 4 described
between redundancy and reliability, made at a different layer, and resolved
differently as the relative cost of bandwidth fell. This is what it looks like when
a field learns.

## The other half: more bits per symbol

The chapter's second theme runs in the opposite direction. Having established that
we sometimes spend capacity on timing, we look at how to *gain* capacity by using
more than two voltage levels — PAM-4, PAM-8, PAM-16 — which is Chapter 4's
bits-per-symbol arithmetic applied to baseband copper.

The tradeoff here is exactly the one §2.1 predicted: four levels in the same
voltage range means one third the spacing between adjacent levels, which means
roughly 9.5 dB more SNR required for the same error rate. Modern high-speed copper
accepts that deal because bandwidth on the cable is the binding constraint and SNR
is recoverable through equalisation and forward error correction. 400GBASE-DR4
uses PAM-4 per lane, GDDR6X graphics memory adopted it, and 2.5GBASE-T runs
10GBASE-T's PAM-16 signalling scaled down over ordinary Cat5e, which is how that
standard managed to run 2.5× faster on cable never designed for it.

## What this chapter does

§7.1 demonstrates concretely why raw NRZ signalling fails: the clock drift
arithmetic above, worked properly, plus the DC balance problem and why transformers
care.

§7.2 covers the self-clocking codes — Manchester, differential Manchester — that
solve the problem by construction, and computes what they cost.

§7.3 covers block codes: 4B/5B, 8B/10B with its running disparity and control
symbols, and 64B/66B with scrambling, tracing the efficiency ladder above.

§7.4 covers multilevel signalling: PAM-4 and beyond, the SNR penalty, and how
2.5GBASE-T and 400G Ethernet use it.

## By the end you will be able to

- Compute how long a receiver can track a sender's clock without a transition,
  given a clock tolerance and a bit rate.
- Explain why AC-coupled media require DC-balanced codes.
- Encode a short bit sequence in Manchester and in 4B/5B by hand.
- Explain what 8B/10B's running disparity does and why control symbols are
  possible at all.
- Compute the overhead and efficiency of any *n*B/*m*B code, and explain why the
  industry moved from 25% overhead to 3%.
- State the SNR penalty of moving from NRZ to PAM-4 and explain the circumstances
  under which that trade is worth making.

## Where this sits in the argument

Chapter 5 gave us signals; Chapter 6 gave us what damages them. This chapter is
the first of two on what we do about it — encoding for baseband transmission —
and Chapter 8 covers the other, modulation onto a carrier. Together they are how
bits become something a wire or an antenna can carry.
