# Chapter 10 — Transmission Media

In 1966, at Standard Telecommunication Laboratories in Harlow, England, a
thirty-two-year-old engineer named Charles Kao published a paper with George
Hockham arguing something that the optical industry of the day considered
faintly absurd: that glass fibre could carry telecommunications signals over long
distances, and that the only thing preventing it was **impurities**.

The best optical glass then available attenuated light by about 1,000 dB per
kilometre. To put that in the decibel terms of Chapter 4: after one kilometre,
the fraction of light remaining is 10⁻¹⁰⁰. There is no receiver, no amplifier, and
no amount of transmitted power that recovers a signal from that. Everyone knew
glass was hopeless for transmission.

Kao's insight was that this figure was not a property of glass. It was a property
of the iron, copper and hydroxyl contamination in glass. Pure silica, he argued,
should manage 20 dB/km — and 20 dB/km is entirely workable, since it means 1% of
the light survives a kilometre, which a good detector can read.

He then spent several years travelling to glass manufacturers trying to convince
them to make purer glass. Corning succeeded in 1970. Modern single-mode fibre
attenuates at about **0.17 dB/km** at 1550 nm, which is a hundred times better than
Kao's target and roughly six thousand times better than the glass of 1966. At that
loss, light travels 80 kilometres and retains 4% of its power — which is why the
amplifier huts along a long-haul route are spaced where they are. Kao received the
Nobel Prize in Physics in 2009.

That story is the chapter in miniature. **The properties of a medium are not
fixed facts about the universe; they are the current state of a manufacturing
art**, and a medium that is hopeless in one decade is the backbone of civilisation
in the next. Anyone who tells you confidently what copper can and cannot do should
be asked what year they learned it: Category 5 was specified in 1991 to 100 MHz,
and in 2016 the 2.5GBASE-T standard extracted 2.5 Gb/s from the same cable.

## What this chapter is for

Four media, and one decision procedure.

The four media — twisted pair, coaxial cable, optical fibre, and free space — are
covered with the numbers that let you choose between them: attenuation per unit
distance, usable bandwidth, maximum reach, cost per metre, susceptibility to each
of Chapter 6's impairments, and the connector and termination practices that
determine whether a theoretically adequate link actually works.

The decision procedure in §10.5 is the point of the chapter. In practice you are
never asked "what is the bandwidth of Cat6a." You are asked "we need to connect
the warehouse to the main building, it is 240 metres away, across a yard with
three-phase machinery in it, and the budget is limited" — and the answer requires
you to weigh distance against interference against cost against future-proofing
against who will terminate it.

The single most common failure in that decision, worth flagging before we start:
**installation cost dominates material cost, and material cost dominates the
difference between cable grades.** The labour to pull a cable through a building is
the same whether the cable is Cat5e or Cat6a. The difference in material cost is a
few tens of pence per metre. Therefore the correct default is to install the best
cable you can terminate properly, because the one thing you will not want to do in
six years is pull it again. Engineers who optimise the wrong term in that equation
create expensive problems for their successors, and this is one of the few places
where "buy the better one" is genuinely the right engineering answer rather than
laziness.

## Notes on what has changed recently

Three things have shifted enough since older textbooks that they are worth
flagging up front:

**Copper has been repeatedly resurrected.** 2.5GBASE-T and 5GBASE-T (IEEE 802.3bz,
2016) run multi-gigabit over existing Cat5e and Cat6 at 100 m, using the PAM-4 and
DSP techniques of Chapter 7. The installed base of Category 5e in the world is
enormous, and this standard exists specifically to avoid replacing it.

**Coaxial cable did not die.** It survives in cable television and DOCSIS
(Chapter 49), where hybrid fibre-coax plants serve hundreds of millions of
households, and in every antenna feed and every data centre's direct-attach copper.
It merely disappeared from the LAN.

**Power over Ethernet changed the calculus.** Since 802.3bt (2018) delivers up to
90 W over the same four pairs that carry data, "which medium" is now sometimes
decided by whether the far end needs power — which fibre cannot supply. An access
point, camera or phone at the end of a copper run needs no local outlet; the same
device on fibre needs an electrician. This routinely decides the choice, and it is
absent from most older treatments.

## What this chapter does

§10.1 covers twisted pair: why it is twisted, the category ladder from Cat3 to
Cat8 with what each was designed for, shielding (UTP/FTP/S/FTP) and when it helps
or hurts, T568A/B pinouts, and PoE.

§10.2 covers coaxial cable: construction, impedance and why 50 Ω and 75 Ω both
exist, connectors, and where coax remains the right answer.

§10.3 covers optical fibre: total internal reflection, the core/cladding
structure, single-mode versus multimode and the OM/OS designations, wavelength
windows, connectors and polish types, splicing, and the loss budget calculation
that decides whether a run will work.

§10.4 covers free space: the spectrum as a regulated resource, what each band is
good for, and the point-to-point microwave, satellite and optical wireless cases.

§10.5 gives the decision procedure, worked against several realistic scenarios.

## By the end you will be able to

- Explain physically why twisting rejects interference and why pairs in one jacket
  have different twist rates.
- Choose a cable category from a required rate, distance, and environment, and
  justify it.
- Compute a fibre loss budget from launch power, receiver sensitivity, fibre
  attenuation, splice and connector losses, and a safety margin.
- Distinguish single-mode from multimode by application, and explain why multimode
  is distance-limited.
- Identify the common connectors and know which media and applications they belong
  to.
- Make and defend a media recommendation for a stated scenario, addressing cost,
  distance, interference, power delivery, and future capacity.
