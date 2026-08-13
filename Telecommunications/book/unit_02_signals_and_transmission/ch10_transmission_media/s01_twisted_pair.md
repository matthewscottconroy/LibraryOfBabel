# 10.1 Twisted Pair

The most-installed communications medium on Earth, by a very large margin, and the
one most likely to be at the end of the cable you are using now.

Chapter 6 §6.4 explained why it is twisted. This section is about what you can buy,
what each grade will carry, and how to choose.

## Construction

Two insulated copper conductors, typically 22 to 24 AWG, twisted together at a
specified rate. Four such pairs in a jacket makes the standard horizontal cable,
and the four pairs use **different twist rates** so that pair-to-pair coupling
averages out rather than accumulating (Chapter 6 §6.4).

**Solid** conductors for permanent horizontal runs — better electrical performance,
poor flex life. **Stranded** for patch cords — worse performance over distance,
survives being moved. Using stranded for a 90 m horizontal run, or solid for a
patch cord that gets flexed daily, are both mistakes that produce faults months
later.

## The category ladder

Each category is a **bandwidth specification** plus a set of transmission
parameters — attenuation, NEXT, PSNEXT, return loss, delay skew. It is not a data
rate, and Chapter 5 §5.3 argued why that distinction matters.

| Category | Bandwidth | Designed for | Runs today | Max |
|---|---|---|---|---|
| Cat3 | 16 MHz | 10BASE-T, voice | Voice, legacy | 100 m |
| Cat5 | 100 MHz | 100BASE-TX | (superseded) | 100 m |
| **Cat5e** | 100 MHz | 1000BASE-T | 1G, 2.5G | 100 m |
| **Cat6** | 250 MHz | 1000BASE-T with margin | 1G, 5G; 10G to 55 m | 100 m / 55 m |
| **Cat6a** | 500 MHz | 10GBASE-T | 10G | 100 m |
| Cat7 / 7a | 600 / 1000 MHz | 10G shielded | 10G | 100 m |
| Cat8 | 2000 MHz | 25G/40GBASE-T | 25G, 40G | **30 m** |

Three observations that matter more than the table.

**Cat5e is not obsolete.** 802.3bz (2016) extracted 2.5 Gb/s from it at 100 m by
encoding harder (Chapter 7 §7.4), on cable specified in 1999 for 100 Mb/s. The
enormous installed base of Cat5e is precisely why that standard exists.

**Cat6's 10 Gb/s figure is conditional.** 55 m is the headline, and it falls
further in a tightly bundled installation because Cat6 does not specify **alien
crosstalk** (Chapter 6 §6.4) and Cat6a does. "It depends how it is bundled" is an
unsatisfying answer and it is the correct one.

**Cat7 and Cat7a are ISO/IEC categories that TIA never adopted**, and they specify
non-RJ45 connectors (GG45, TERA) that essentially nothing uses. In North American
practice they are rare. Cat8 is real, is standardised by both bodies, and is a
30 m data-centre product rather than a horizontal cabling one.

## The cost argument, made properly

Chapter 10's introduction stated it; here are the numbers.

Installing a horizontal run means: pulling cable through ceilings and conduit,
terminating both ends, testing, labelling, and documenting. That labour is
**identical** whether the cable is Cat5e or Cat6a. Only the material differs.

A worked example for a 200-outlet office, at figures that are illustrative and
whose local values you should substitute:

| | Cat5e | Cat6a | Difference |
|---|---|---|---|
| Cable, ~55 m average × 200 | £1,700 | £3,400 | £1,700 |
| Connectors and patch panels | £1,200 | £1,900 | £700 |
| **Labour** (identical) | £14,000 | £14,000 | £0 |
| Testing and certification | £2,000 | £2,400 | £400 |
| **Total** | **£18,900** | **£21,700** | **£2,800** |

**A 15% cost difference on a fifteen-year asset**, for a fourfold increase in
bandwidth headroom. And the cost of pulling it again in eight years is the whole
£18,900 plus the disruption of working in an occupied building.

This is one of the few places in engineering where "buy the better one" is the
correct answer rather than laziness, and the argument is economic rather than
technical.

The counter-argument, which is real: **Cat6a is harder to terminate.** Larger
conductors, tighter tolerances, and less forgiveness for untwisting at the
connector. If the installation team is not competent with it, a badly terminated
Cat6a run performs worse than a well-terminated Cat5e one. Specify to the team you
have, which is Chapter 72 §72.1's point applied to cabling.

## Shielding

The designations from Chapter 6 §6.4: **U/UTP** (unshielded), **F/UTP** (foil
overall), **S/FTP** (braid overall plus foil per pair), **F/FTP** (foil overall plus
foil per pair).

**Specify shielded when:** the environment has severe EMI (industrial plant,
alongside high-current cable, near variable-speed drives); the installation is
dense enough that alien crosstalk binds at 10 Gb/s; or a security requirement calls
for reduced emissions.

**Specify unshielded otherwise**, and that covers most commercial buildings.

The reason for the default is the earthing burden. A shield must be bonded, and a
shield bonded at both ends across a potential difference carries current and
becomes an antenna — injecting interference rather than blocking it. A badly earthed
S/FTP installation performs measurably worse than a competent U/UTP one, and the
failure is not obvious.

If you specify shielded, specify the earthing explicitly and confirm someone
competent will implement it.

## Connectors and pinouts

The **RJ45** (properly the 8P8C modular connector) is universal for twisted pair
Ethernet.

**T568B** pinout, which is the common choice in most installations:

| Pin | Wire | Pair |
|---|---|---|
| 1 | White/Orange | 2 |
| 2 | Orange | 2 |
| 3 | White/Green | 3 |
| 4 | Blue | 1 |
| 5 | White/Blue | 1 |
| 6 | Green | 3 |
| 7 | White/Brown | 4 |
| 8 | Brown | 4 |

**T568A** swaps the orange and green pairs. Both work; **the only requirement is
consistency within an installation**, because a cable with A at one end and B at
the other is a crossover.

Two practical notes. Note that **pins 3 and 6 are one pair, split around pins 4 and
5** — this is a historical compatibility artefact with telephone wiring, and it is
why a technician wiring "straight across" without the standard produces a split
pair (Chapter 6 §6.4) that passes continuity and fails at gigabit.

And **untwist no more than 13 mm** at the termination for Cat5e, less for higher
categories. Beyond that the common-mode rejection stops working locally and NEXT
rises.

**Crossover cables** have essentially disappeared, because Auto-MDI/MDI-X is
universal on equipment made after about 2005. If you meet one in a drawer, it is
history rather than a spare.

## Power over Ethernet

Since 802.3af (2003) the same pairs can carry power, and this frequently decides a
media choice that would otherwise favour fibre.

| Standard | Name | At source | At device | Pairs |
|---|---|---|---|---|
| 802.3af | PoE | 15.4 W | 12.95 W | 2 |
| 802.3at | PoE+ | 30 W | 25.5 W | 2 |
| 802.3bt Type 3 | PoE++ | 60 W | 51 W | 4 |
| 802.3bt Type 4 | PoE++ | 90 W | 71.3 W | 4 |

The difference between the two columns is loss in the cable, which is why the
device figure is lower and why longer runs deliver less.

**Budgeting.** A switch has a total PoE budget, and it is usually less than the sum
of its ports' maximums. A 48-port switch advertising 802.3at on every port with a
740 W budget can supply 30 W to 24 ports, not 48. Work the arithmetic before
ordering:

```
  22 access points  × 25.5 W = 561 W
  22 cameras        ×  9.0 W = 198 W
  51 IP telephones  ×  6.5 W = 332 W
                              ──────
                              1,091 W  → needs two switches or a larger PSU
```

**Heat** is the second-order consideration: power dissipated in the cable warms the
bundle, which raises attenuation (Chapter 6 §6.1) and reduces the maximum length.
The standards account for it, and a densely bundled 802.3bt installation in a warm
riser is closer to the limit than the headline figures suggest.

**The design consequence:** a camera, access point or telephone at the end of a
copper run needs no local power outlet. The same device on fibre needs an
electrician, a socket, and a small power supply that will fail in five years. This
routinely decides the choice, and it is absent from older treatments because PoE did
not exist when they were written.

## What breaks here

**Stranded cable on a long horizontal run.** Higher attenuation; works at 1 Gb/s
over 40 m and fails at 90 m.

**Untwisting too far at the connector.** Raises NEXT; shows as a certifier failure
or as intermittent errors at gigabit.

**Split pair.** Passes continuity, fails at gigabit, and Chapter 6 §6.4 explains
why.

**Mixed T568A and T568B in one installation.** Produces crossovers where nobody
expects them. Auto-MDI/MDI-X hides it until it meets equipment that lacks it.

**A PoE budget that does not add up.** Devices power up in port order and the last
ones stay dark, or the switch cycles them. The symptom is "some access points work"
and the cause is arithmetic.

**Cat6 at 10 Gb/s in a tight bundle.** Alien crosstalk. Loosen the bundle, shorten
the run, or specify Cat6a.

> **Network+ note.** Objective 1.5 expects the category ladder, distances,
> connectors and the shielded/unshielded decision; objective 2.4 expects PoE
> standards and budgeting. The two things worth over-learning: **category is a
> bandwidth rating, not a data rate**, and **PoE budgets are per switch, not per
> port**.
