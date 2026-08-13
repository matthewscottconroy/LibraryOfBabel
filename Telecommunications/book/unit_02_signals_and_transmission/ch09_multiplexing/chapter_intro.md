# Chapter 9 — Multiplexing

In 1874, Alexander Graham Bell was not trying to invent the telephone. He was
trying to invent the *harmonic telegraph* — a device that would send several
telegraph messages down one wire simultaneously by using tuned reeds vibrating at
different frequencies, each responding only to its own pitch. Western Union was
offering serious money for such a thing, because the cost of telegraph
infrastructure was overwhelmingly the cost of the wire, and any technique that
multiplied a wire's usefulness was worth a fortune.

Bell's harmonic telegraph worked poorly. But in the course of building it, on 2
June 1875, he and Thomas Watson noticed that a reed which had stopped vibrating
was still producing a faint sound at the far end — that the apparatus was
transmitting not just the presence of a tone but its *shape*. Within a year that
observation had become the telephone, and Bell had more or less abandoned the
multiplexing problem that started it.

The problem did not go away. It is, if anything, the central economic problem of
telecommunications, and every generation has re-solved it.

## The economics that force it

The reason multiplexing dominates this field is straightforward once stated: **the
cost of a communications link is overwhelmingly in the path, not in the capacity.**

Digging a trench across a city costs the same whether you lay one fibre or a
hundred and forty-four. Launching a satellite costs the same whether it carries
one transponder or forty. A submarine cable across the Atlantic costs several
hundred million dollars in ships, permits and cable, and the electronics at each
end that determine whether it carries 10 Tb/s or 250 Tb/s are a small fraction of
that. The marginal cost of capacity, once the path exists, is close to zero.

Therefore: get as many conversations as possible onto every path you have already
paid for. That is multiplexing, and the four physical techniques for it — plus one
that is not physical at all — are this chapter.

## The five answers

**Frequency-division multiplexing (FDM).** Give each conversation its own band of
frequencies. Everybody transmits all the time, in their own slice of spectrum.
This is broadcast radio, cable television, ADSL, and — in optical form — the DWDM
that carries the modern Internet. It requires modulation (Chapter 8) to shift each
signal to its allocated band.

**Time-division multiplexing (TDM).** Give each conversation the whole channel, in
turn, for a brief slot. Everybody uses all the frequencies, at different times.
This is the entire architecture of the digital telephone network: the DS0 from
Chapter 4 §4.2 combined into T1s and E1s and then into the SONET hierarchy of
Chapter 50, all by strict, rigid, clock-driven time slots.

**Code-division multiplexing (CDMA).** Give each conversation a distinct
mathematical code, and let everybody transmit simultaneously across the whole band.
Correlation at the receiver recovers each signal from what looks, to any other
receiver, like noise. This is 3G cellular, GPS, and — in a story worth telling —
an idea patented in 1942 by the actress Hedy Lamarr and the composer George
Antheil.

**Wavelength-division multiplexing (WDM).** FDM for light. Different colours down
one fibre, separated by prisms — or, in practice, by thin-film filters and arrayed
waveguide gratings. Chapter 50 shows how a single fibre pair carries 96 wavelengths
at 400 Gb/s each.

**Statistical multiplexing.** And then the one that is not a physical technique at
all: give the channel to whoever currently has something to send. No fixed
allocation, no reserved slot, no guarantee. This is packet switching, it is the
subject of Chapter 13, and it is the single idea that allowed data networks to beat
the telephone network on cost by such a margin that the telephone network
eventually migrated onto them.

## The argument this chapter is really making

The first four techniques all *divide a resource in advance*. Statistical
multiplexing does not, and the difference is the most important structural fact in
this book.

Here is the arithmetic, which §9.3 develops properly. Suppose 100 users each need
1 Mb/s when active, and each is active 5% of the time. Fixed allocation requires
100 Mb/s of capacity, permanently, and it is idle 95% of the time. Statistical
multiplexing over a 20 Mb/s link serves them with an overflow probability that is
vanishingly small — the binomial arithmetic gives roughly one chance in ten million
of more than 20 users being simultaneously active. You have bought one fifth of the
capacity and provided effectively the same service.

That factor of five, generalised and compounded, is why the Internet exists in the
form it does. It is also why the Internet cannot make the guarantees the telephone
network could, why quality of service (Chapter 52) is necessary and hard, and why
your video call occasionally stutters in a way that a 1985 telephone call never
did. The gain is real and so is the price.

## What this chapter does

§9.1 covers FDM: the guard band, the historical carrier telephone systems, and the
modern descendants.

§9.2 covers TDM: synchronous and asynchronous forms, framing and bit-stuffing, and
the digital hierarchy from DS0 to DS3 — the numbers that Chapter 12 and Chapter 50
both depend on.

§9.3 covers statistical multiplexing: the arithmetic above, done properly, with the
binomial calculation and the concept of statistical multiplexing gain, and an
honest account of what is given up.

§9.4 covers CDMA and WDM: spreading codes and orthogonality, and the optical case
with its channel spacing and amplifier bands.

## By the end you will be able to

- Explain why multiplexing is an economic necessity rather than a technical
  nicety.
- Distinguish the four physical multiplexing techniques by what resource they
  divide and how.
- Compute the capacity required for *n* users under fixed and statistical
  allocation, and quantify the multiplexing gain.
- Explain the DS0/T1/E1 hierarchy and where its numbers come from.
- State what statistical multiplexing gives up, and connect that loss to the QoS
  machinery of Chapter 52.
