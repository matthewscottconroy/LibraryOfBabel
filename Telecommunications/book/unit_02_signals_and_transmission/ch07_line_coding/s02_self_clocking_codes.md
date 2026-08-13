# 7.2 Self-Clocking Codes

The first family of answers. These solve the clock recovery problem by
**construction** — the code guarantees a transition in every bit period, so the
receiver can never lose lock, whatever the data.

## Manchester encoding

Also called biphase-L. The rule, in one line:

> **Every bit is a transition in the middle of its bit period.**
> A `1` is a low-to-high transition; a `0` is a high-to-low transition.

(IEEE 802.3 uses that polarity; G. E. Thomas's original 1949 convention is the
opposite, and the ambiguity has caused enough confusion that a standard's text
always specifies which it means.)

An example. Encoding `1 0 1 1 0`:

```
data      1       0       1       1       0
        ┌───┐   ┌───┐   ┌───┐   ┌───┐   ┌───┐
        │   │   │   │   │   │   │   │   │   │
    ────┘   └───┘   └───┘   └───┘   └───┘   └────
         ↑       ↑       ↑       ↑       ↑
      low→high high→low low→high low→high high→low
```

Every bit period contains exactly one mid-bit transition. Additional transitions
occur at bit boundaries when two consecutive bits are equal, which is harmless.

**What it buys:**

- **A transition in every bit period, guaranteed**, regardless of the data. Clock
  recovery is trivial and cannot fail.
- **Perfect DC balance**, in every single bit. Each bit spends exactly half its
  period high and half low, so the running average is exactly zero over any whole
  number of bits — not merely on average, but exactly.
- **Simple to implement.** The encoder is an XOR of the data with the clock.
  In 1980 this mattered enormously.

**What it costs:**

Two signal levels per bit means the signalling rate is **twice** the data rate. To
carry 10 Mb/s of data you must signal at 20 Mbaud, and by §5.3's minimum-bandwidth
argument that needs roughly twice the bandwidth of raw NRZ.

**Efficiency: 50%.** Half the channel's capacity is spent on the timing guarantee.

That is why 10BASE-T runs on Category 3 cable specified to 16 MHz — a 10 Mb/s
Manchester signal has its fundamental at 10 MHz — and why 100BASE-TX could not
simply use Manchester on Category 5, which would have needed 100 MHz of clean
bandwidth for the fundamental alone plus harmonics.

## Differential Manchester

A variant that encodes information in the **presence or absence** of a transition
at the bit boundary, rather than in the direction of the mid-bit transition:

> There is always a mid-bit transition, used only for clocking.
> A **`0`** additionally has a transition at the start of the bit period.
> A **`1`** does not.

The advantage is **polarity independence**. If someone reverses the two conductors
of a differential pair — a genuinely common wiring error — Manchester's decoding
inverts and every bit is wrong. Differential Manchester is unaffected, because it
reads changes rather than levels.

Token Ring used it, and so do several industrial and instrumentation protocols
where field wiring is done by people who are not networking specialists and where
a reversed pair should not be a fault. It costs the same 50% efficiency.

## The other biphase codes

Worth recognising, not worth memorising.

**Bipolar AMI** (Alternate Mark Inversion) represents `0` as zero volts and `1`
as alternately positive and negative pulses. DC balance is perfect because
consecutive `1`s cancel. Clock recovery works from the `1`s — and fails on a long
run of `0`s, which is exactly the problem we were solving.

The fix was **B8ZS** (North America) and **HDB3** (Europe), which substitute a
deliberate code violation for a run of zeros. The receiver recognises the
violation — a pulse of the "wrong" polarity, which cannot occur in valid data —
and restores the zeros.

These appear in T1 and E1 circuits (Chapter 12 §12.2) and are worth knowing because
carrier documentation still specifies them, and because "B8ZS versus AMI" is
occasionally a real configuration mismatch on a legacy circuit.

**Return-to-zero (RZ)** returns to a neutral level in the middle of every bit,
guaranteeing transitions at the cost of needing three levels and twice the
bandwidth. Used in some optical systems where the pulse shape matters more than
the bandwidth.

**MLT-3** — used by 100BASE-TX — cycles through three levels (−1, 0, +1) on each
`1` and holds on each `0`. Its virtue is that it reduces the fundamental frequency
by a factor of four compared with NRZ, which is what let 100 Mb/s fit within Cat5's
100 MHz. It is combined with 4B/5B (§7.3), which supplies the transition density
that MLT-3 alone does not guarantee.

## Why self-clocking codes lost

The efficiency ladder in §7.1's table tells the story, and it is worth being
explicit about the reasoning rather than just the outcome.

Manchester's 50% efficiency was acceptable in 1983 because:

- Bandwidth on a 10 Mb/s coaxial segment was not the binding constraint.
- Reliable clock recovery *was* — the phase-locked loops and the digital signal
  processing that make statistical approaches safe did not exist cheaply.
- The encoder is an XOR gate, and gate count mattered.

By 1995 all three had changed. Cat5 cable was the installed base and its 100 MHz
was the binding constraint; PLLs were cheap and good; and gate count had stopped
mattering. So 100BASE-TX spent 25% on 4B/5B instead of 100% on Manchester, and
used MLT-3 to reduce the fundamental further.

By 2002, at 10 Gb/s, even 25% was unaffordable, and 64B/66B's 3.1% with a
statistical guarantee became the answer.

**The general shape:** as the cost of computation fell relative to the cost of
bandwidth, the industry moved from guarantees purchased with bandwidth to
guarantees purchased with processing. That trade recurs throughout this book —
Chapter 6 §6.3's equalisation is the same trade, and Chapter 50 §50.3's coherent
optics is the same trade again at a different scale.

## Worked example

Encode `1 1 0 1 0 0` in Manchester, then state the signalling rate required to
carry it at 10 Mb/s.

```
data:      1        1        0        1        0        0
          ┌──┐     ┌──┐  ───┐     ┌──┐  ───┐   ────
          │  │     │  │     │     │  │     │
     ─────┘  └─────┘  └─────┘─────┘  └─────┘────────
```

Six bits, twelve signal elements, so the signalling rate is 20 Mbaud for 10 Mb/s of
data. Fundamental frequency for an alternating pattern: 10 MHz. Cat3's 16 MHz
specification accommodates it with margin, and Cat3 is why 10BASE-T could run over
the telephone cabling already in buildings — which is the entire reason 10BASE-T
displaced coaxial Ethernet.

> **Network+ note.** Not examined directly. The connection worth carrying is that
> **10BASE-T runs on Cat3 and 100BASE-TX does not**, and the reason is the
> bandwidth that the encoding requires. That fact appears in the media objectives
> and is otherwise arbitrary.
