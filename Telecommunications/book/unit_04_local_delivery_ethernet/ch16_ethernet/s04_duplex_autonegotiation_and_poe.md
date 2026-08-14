# 16.4 Duplex, Autonegotiation, and Power over Ethernet

## Full duplex, and the abolition of collisions

The most consequential change in Ethernet's operational history, and it
required no new physics.

**Twisted pair has separate pairs for each direction.** 10BASE-T and 100BASE-TX use
one pair to transmit and one to receive. On a shared hub they still operate half
duplex, because the hub repeats everything to everyone and collisions remain
possible.

Connect exactly **two** devices — a station and a switch port — and the situation
changes completely. There is one device at each end and separate paths between them.
Nobody else can transmit. **There is nothing to collide with.**

So:

- **Carrier sense is unnecessary.** Transmit whenever you like.
- **Collision detection is meaningless.** Collisions cannot occur.
- **Backoff never happens.**
- **Both directions operate simultaneously**, doubling the aggregate capacity.
- **The 64-byte minimum and the slot-time constraint become vestigial**, though both
  are still enforced.

IEEE 802.3x standardised full duplex in 1997, and it is why §16.3 could say that
CSMA/CD does not run on a modern network. Above 1 Gb/s the standard never
specified half-duplex operation at all.

**And it dissolved Token Ring's argument.** Chapter 16 §16.3 noted that Token Ring's
advantage was deterministic access under contention. Full duplex switched Ethernet
has no contention, so it is deterministic by construction, and the comparison ceased
to have a subject.

### Flow control

Full duplex introduced a new problem: a fast sender can overwhelm a slow receiver,
and there is no collision to slow it down.

**802.3x PAUSE frames** are the answer — a receiver sends a PAUSE frame asking the
sender to stop for a specified number of slot times.

It is a blunt instrument and is rarely enabled, for a good reason: **it pauses
everything**, including traffic destined for uncongested queues, and it can propagate
congestion backward through a network in a phenomenon called **head-of-line
blocking**. **Priority Flow Control** (802.1Qbb) refines it to per-priority pausing,
and is essential in the lossless data-centre fabrics of Chapter 71 §71.5 and
generally undesirable elsewhere.

## Autonegotiation

Two devices connect. They must agree on speed, duplex, and — where relevant — flow
control and pair usage. Autonegotiation is the mechanism, standardised in 802.3u
(1995) and extended since.

### How it works

The clever part is that it works **before any agreement exists**, using a signalling
method every Ethernet device already understood.

10BASE-T sends a **link test pulse** every 16 ms when idle, so the far end knows the
link is alive. Autonegotiation replaces the single pulse with a **fast link pulse
burst** — a train of pulses encoding a 16-bit word.

A 10BASE-T device that does not understand autonegotiation sees the burst as
ordinary link pulses and continues at 10 Mb/s half duplex. An autonegotiating device
recognises the encoding. **Backwards compatibility by construction**, which is why
the mechanism could be deployed incrementally.

The 16-bit word advertises capabilities. Both ends exchange them and each selects the
**highest common capability**, in a fixed priority order:

```
   10GBASE-T  >  5GBASE-T  >  2.5GBASE-T  >  1000BASE-T full  >  1000BASE-T half
   >  100BASE-TX full  >  100BASE-T4  >  100BASE-TX half
   >  10BASE-T full  >  10BASE-T half
```

Note that **full duplex always outranks half at the same speed**, which matters for
the failure mode below.

For 1000BASE-T and above, autonegotiation is **mandatory** — it also settles which
end acts as master for clock recovery, so the link cannot come up without it.

### The failure mode

The classic and much-misdiagnosed fault, and its mechanism is precise:

**One end is hard-coded. The other is left on autonegotiate.**

The autonegotiating end sends fast link pulses and receives none, because the
hard-coded end is not participating. Receiving no advertisement, it falls back to
**parallel detection**: it can detect the *speed* from the signalling it observes,
but it cannot detect duplex — nothing in the signal reveals it.

The standard specifies the fallback: **when duplex cannot be determined, assume half
duplex.**

So:

```
   Switch port:  hard-coded 100/full
   Host NIC:     autonegotiate → detects 100 Mb/s → assumes HALF duplex
   Result:       Switch full, host half. Link comes up. Both look correct.
```

Both administrators believe they configured correctly. The link is up at the right
speed. Nothing announces the problem.

### What a duplex mismatch does

The full-duplex end transmits whenever it likes. The half-duplex end, hearing
incoming traffic while it is transmitting, interprets it as a **collision**, aborts,
and backs off.

Consequences:

- **Throughput collapses** — typically to a few per cent of the link rate.
- **It gets worse as offered load rises**, which is diagnostic, because most
  performance problems degrade gracefully. This one accelerates downward.
- **The counter signature is unambiguous**: **late collisions** on the half-duplex
  end, alignment and CRC errors on both.

A **late collision** — detected after the first 64 bytes — is impossible on a
correctly configured link, because the slot time guarantees any real collision is
detected within it. Its presence means either a duplex mismatch or a segment
exceeding the maximum length, and on a modern network it is almost always the first.

### The remedy, and the folklore

**Let both ends autonegotiate.**

There is persistent folklore that hard-coding is "safer" or "more reliable". It was
defensible in about 1997, when early implementations were genuinely buggy. It has
been wrong for twenty-five years, and **hard-coding one end is the primary cause of
the fault it is supposed to prevent**.

If a link must be hard-coded — some carrier handoffs still require it — hard-code
**both** ends, identically, and document it.

## Power over Ethernet

Delivering power on the same cable that carries data, which frequently decides a
media choice that would otherwise favour fibre (Chapter 10 §10.5).

### The standards

| Standard | Name | Year | At source | At device | Pairs |
|---|---|---|---|---|---|
| 802.3af | PoE | 2003 | 15.4 W | 12.95 W | 2 |
| 802.3at | PoE+ | 2009 | 30 W | 25.5 W | 2 |
| 802.3bt Type 3 | PoE++ | 2018 | 60 W | 51 W | **4** |
| 802.3bt Type 4 | PoE++ | 2018 | 90 W | 71.3 W | **4** |

The gap between the two power columns is **loss in the cable**, which is why longer
runs deliver less and why the device figure is the one that matters when specifying
equipment.

### How it works

**Detection.** Before applying power, the source applies a low voltage and measures
the resistance. A compliant powered device presents a 25 kΩ signature. Anything else
— a laptop, a switch, an unpowered device — is not powered, which is essential:
applying 48 V to a device not expecting it would destroy it.

**Classification.** The source then determines the device's power class, either by
measuring a current signature or, for 802.3bt, by an explicit LLDP exchange. This
lets the source allocate only what is needed rather than reserving the maximum.

**Power delivery.** 44–57 V DC. In **Mode A** it rides on the data pairs as a common-
mode voltage — which works because the data is differential and Chapter 6 §6.4's
subtraction ignores anything common to both conductors. In **Mode B** it uses the
spare pairs. 802.3bt uses all four pairs.

That Mode A works at all is a direct and elegant consequence of differential
signalling: the power and the data occupy the same conductors and do not interact,
because they are encoded in orthogonal ways.

### Budgeting, and the arithmetic that must be done

**A switch's total PoE budget is usually far less than the sum of its ports'
maximums.**

A 48-port switch advertising 802.3at on every port with a 740 W supply can deliver
30 W to 24 ports, not 48. This is normal, it is in the datasheet, and it is
routinely missed.

```
   14 access points  × 25.5 W = 357 W
   22 cameras        × 12.9 W = 284 W
   51 IP telephones  ×  6.5 W = 332 W
                               ──────
                               973 W  → exceeds a 740 W budget
```

Options: a larger power supply, a second switch, redistribution, or LLDP-based
allocation so devices negotiate what they actually need rather than their class
maximum.

**The failure mode when the budget is exceeded** is that devices power up in port
order and the last ones stay dark, or the switch cycles them. The symptom is "some
access points work" and the cause is arithmetic.

### Second-order considerations

**Heat.** Power dissipated in the cable warms the bundle, which raises attenuation
(Chapter 6 §6.1) and reduces the effective maximum length. A densely bundled 802.3bt
installation in a warm riser is closer to the limit than the headline figures
suggest, and the standards account for it with derating tables.

**Availability.** PoE means **the switch's power is the telephone's power**. A
converged network (Chapter 14 §14.4) in which telephones lose power with the access
switch has made the switch's UPS part of the *voice* system's availability design —
and this is routinely forgotten until the first mains failure, when nobody can
telephone anyone to report it.

**Cable grade.** Higher-power PoE over thin-conductor cable dissipates more; 802.3bt
effectively requires Cat5e or better with 23 AWG conductors for full power at full
length.

## What breaks here

**Duplex mismatch**, from hard-coding one end. Late collisions, and throughput that
worsens under load.

**Collisions on a full-duplex link.** Should be zero. Any count is a fault.

**PAUSE frames enabled on a general-purpose network**, propagating congestion
backward and blocking uncongested traffic.

**A PoE budget that does not add up.** Some devices never power on, in port order.

**Telephones dying with the access switch.** Convergence's failure-domain
consolidation, and the UPS that was not specified.

**A device that will not take power.** Check detection signature, class, budget
remaining, and cable grade — in that order.

> **Network+ note.** Objectives 2.4 and 1.5 cover PoE standards and budgeting;
> objective 5.2 covers duplex mismatch. The three things to over-learn: **let both
> ends autonegotiate**; **late collisions are never normal**; and **PoE budgets are
> per switch, not per port**, so the arithmetic must be done before ordering.
