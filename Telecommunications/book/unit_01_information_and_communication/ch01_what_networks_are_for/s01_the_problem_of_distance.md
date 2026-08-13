# 1.1 The Problem of Distance

Every technology in this book exists to defeat one thing, and it is worth being
precise about what that thing is, because the precision pays off later.

The thing is not *distance*, exactly. It is the fact that **matter takes longer to
move than information needs to take**. If a horse could carry a letter from London
to New York in a microsecond, there would be no telegraph, no telephone, no
Internet, and no reason for you to be reading this. The problem is a mismatch
between the speed at which we can physically transport a message-bearing object
and the speed at which decisions need to be made.

## Three eras of the same problem

Consider how the mismatch has been attacked.

**Transport the object faster.** For most of human history this was the only
option, and enormous effort went into it. The Persian royal road, with its relay
riders and stations at intervals of a day's ride, moved a message the 2,700 km
from Susa to Sardis in about seven days — a pace that Herodotus considered
miraculous and which works out to roughly 4.5 metres per second, averaged. The
Pony Express, in 1860–61, achieved about ten days from Missouri to California,
which is a comparable figure, twenty-three centuries later. This approach has a
hard ceiling: you are limited by the speed of the fastest thing you can make move,
and by 1860 that was still a horse.

**Send a signal instead of an object.** Signal fires, semaphore towers,
heliographs. The Chappe optical telegraph, built across France from 1792 onward,
eventually ran 556 towers over 4,800 km; a message could cross from Paris to
Lille — 230 km — in about two minutes on a clear day, which is a speed of roughly
2 km/s, four hundred times faster than the Persian riders. Note what has changed:
nothing physical travels the route. What travels is a *pattern*, reproduced at
each tower by a person with a telescope and two mechanical arms.

The Chappe system also has the two properties that will define every network in
this book. It is a **relay network** — no link spans the full distance, and each
node regenerates the signal. And it has a **capacity**: about two symbols per
minute per tower, from a vocabulary of 92 positions, which works out to a data
rate we could compute exactly once Chapter 4 gives us the tools. (It is about
0.2 bits per second. The line from Paris to Lille could carry roughly one page of
text per day. Napoleon still found it decisive.)

**Send the signal through a medium that is already fast.** This is where we live.
Electricity in copper, light in glass, radio in air. The propagation speed is a
substantial fraction of *c*, the speed of light in vacuum, and this is not a
number that further engineering will improve.

## The floor you cannot get under

That last point deserves emphasis, because it produces the single most persistent
misunderstanding in network engineering.

Light in vacuum travels at 299,792,458 metres per second, exactly, by definition
of the metre. In optical fibre, with a refractive index of about 1.468 at the
1550 nm wavelength used for long-haul transmission, the signal propagates at
roughly

$$v = \frac{c}{n} = \frac{3.00 \times 10^8}{1.468} \approx 2.04 \times 10^8 \ \text{m/s}$$

or about 204 kilometres per millisecond. In copper the figure is similar —
typically 0.64 to 0.77 *c*, depending on the cable's insulation, a ratio the
cable industry calls the **velocity factor**.

So: New York to London is about 5,585 km great-circle. Real submarine cables are
longer, because they route around continental shelves and existing infrastructure;
a realistic fibre path is around 6,600 km. At 204 km/ms that is

$$t = \frac{6{,}600}{204} \approx 32 \ \text{ms}$$

one way, and about 65 ms for a round trip, before any equipment does anything at
all. Measured round-trip times on that route are typically 70–80 ms, and the
extra 5–15 ms is the routers, the amplifiers, the regenerators and the queues.

Here is the point. **No amount of money buys you a 20 ms round trip from New York
to London.** Not a faster router, not a fatter pipe, not a better ISP. The
physics forbids it. The best conceivable cable — a perfectly straight one, in
vacuum — would give 37 ms round trip, and you cannot dig a straight tunnel
through the Earth's crust across the Atlantic. This is why high-frequency trading
firms spent tens of millions of dollars in 2010–2015 laying microwave relay chains
between Chicago and New Jersey: microwave through air is faster than light through
glass (air's refractive index is essentially 1.0), and shaving 4 milliseconds off
a 13 ms path was worth that money to them. They were not buying bandwidth. They
were buying the difference between *c/1.468* and *c*.

We will return to this in Chapter 3, where we separate latency from bandwidth
formally, and in Chapter 38, where we discover that this one number — the round
trip time — silently governs how fast a TCP connection can possibly go.

## Why not just shout louder?

Which brings us back to Wildman Whitehouse and his two thousand volts.

The instinct that more power means better communication is deeply ingrained and
almost always wrong. It is wrong for a reason that will recur in this book in at
least six different technical costumes, so let us get it right the first time.

Increasing transmit power increases the **signal**. But in nearly every real
system, the thing limiting you is not the absolute strength of the signal — it is
the **ratio** of signal to whatever else is arriving with it. And in many systems,
turning up the power increases the interference too, either because your own
signal is reflecting back at you, or because your neighbours are doing the same
thing, or because you have pushed a component out of its linear operating range
and it is now generating distortion products that were not there before.

Whitehouse's specific problem was none of these; his was worse. The 1858 cable was
a long capacitor. A sharp pulse entering one end emerges at the other as a slow
smear, and successive pulses smear into each other — what we now call
**intersymbol interference** and will meet properly in Chapter 6. Raising the
voltage makes the smear taller, not sharper. The smearing is a property of the
cable's geometry and materials, and Thomson's law of squares — that signalling
speed falls as the square of cable length — described it correctly. The fix was
Thomson's mirror galvanometer, an instrument sensitive enough to read a tiny,
clean signal, and a signalling rate slow enough that the smears did not overlap.

The general form of the lesson: **when a channel is not delivering, first find out
what is actually limiting it.** Sometimes it is power. Very often it is noise,
distortion, timing, congestion, or a configuration error, and every watt you add
to those problems makes them worse. In Chapter 42 we will meet the wireless
version of this, where an administrator responds to poor coverage by turning every
access point to maximum transmit power and thereby destroys the network's capacity
entirely. It is the same mistake. It is always the same mistake.

> **Network+ note.** The exam will not ask you about Whitehouse. It will ask you
> to choose a remedy for a described symptom, and the distractors will
> systematically include the "more power / bigger pipe" answer. Recognising that
> family of wrong answers is worth more than any single fact in this chapter.
