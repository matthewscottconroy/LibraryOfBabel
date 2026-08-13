# Chapter 42 — Radio from First Principles

In 1865 James Clerk Maxwell published a set of equations describing electricity and
magnetism, and noticed something that nobody had asked him to look for. The equations
predicted waves — self-sustaining oscillations of electric and magnetic field,
propagating through empty space — and when he computed their speed from the
electrical constants measured in laboratories, he got approximately 3 × 10⁸ metres
per second.

Which was, to within experimental error, the measured speed of light.

Maxwell's conclusion, stated in the 1865 paper, is one of the great sentences in
physics: *"We can scarcely avoid the inference that light consists in the transverse
undulations of the same medium which is the cause of electric and magnetic
phenomena."* Light is an electromagnetic wave. The prediction was made from
electrical measurements, by a man who had no way to generate such a wave and no way
to detect one.

Twenty-two years later Heinrich Hertz built the apparatus — a spark gap to transmit,
a loop with a gap to receive — and demonstrated in his Karlsruhe laboratory that the
waves existed and behaved exactly as predicted. Asked what use they might be, he is
reported to have said: *"It's of no use whatsoever... this is just an experiment that
proves Maestro Maxwell was right."*

Within a decade Marconi was sending signals across the English Channel. Hertz died in
1894 at thirty-six and never saw any of it.

## Why this chapter is here

Not for the history, though it is worth having. For the arithmetic.

Every wireless decision you will make — where to put an access point, whether a
point-to-point link will work, why the far corner of the warehouse has no coverage,
whether a proposed 5 km link needs a licence — reduces to a **link budget**: a sum, in
decibels, of everything that adds to the signal and everything that subtracts from
it, compared against what the receiver needs.

```
  Received power (dBm) = Transmit power (dBm)
                       + Transmit antenna gain (dBi)
                       − Cable and connector losses (dB)
                       − Path loss (dB)
                       − Obstruction losses (dB)
                       + Receive antenna gain (dBi)
                       − Receive cable losses (dB)
```

That is the entire chapter's practical output, and Chapter 4 §4.3 already gave you
the decibel arithmetic to compute it. Everything else in this chapter is establishing
where each term comes from and how large it is.

## The two facts about frequency

**Wavelength sets antenna size.** λ = *c*/*f*, and an efficient antenna is a
substantial fraction of a wavelength — a quarter wave is the usual compromise:

| Frequency | Wavelength | Quarter wave |
|---|---|---|
| 900 MHz | 33 cm | 8.3 cm |
| 2.4 GHz | 12.5 cm | 3.1 cm |
| 5 GHz | 6 cm | 1.5 cm |
| 28 GHz (5G mmWave) | 1.07 cm | 2.7 mm |

This explains a great deal at a glance: why 5 GHz antennas are physically smaller,
why a 900 MHz IoT device needs a comparatively large antenna, and why mmWave phased
arrays can pack dozens of elements into a few square centimetres — which is precisely
what makes beamforming practical up there and impractical at 900 MHz.

**Path loss rises with frequency.** The free-space path loss formula, which you will
use repeatedly:

$$\text{FSPL(dB)} = 32.45 + 20\log_{10} f_{\text{MHz}} + 20\log_{10} d_{\text{km}}$$

Two consequences fall straight out of the logarithms. **Doubling the distance costs
6 dB**, always, at any frequency — which means each doubling of range costs a
quadrupling of power, and is why range extension by transmit power is so unrewarding.
And **doubling the frequency also costs 6 dB**, which is the fundamental reason
5 GHz has shorter range than 2.4 GHz, and 6 GHz shorter still, and mmWave is measured
in hundreds of metres.

That second point is worth stating carefully, because it is often explained
incorrectly. Higher frequencies do not "penetrate walls worse" as a primary matter —
though absorption by materials does increase — they are simply subject to more
free-space loss for the same distance, and the antenna's effective aperture shrinks
with wavelength. The wall absorption is real and secondary.

## What the world does to the signal

§42.4 covers the propagation effects that make real environments unlike the free-space
formula:

**Reflection** off metal, concrete and water, which creates multiple copies of the
signal arriving at slightly different times — **multipath**. Historically this was
purely destructive, causing fading when copies arrived out of phase. MIMO (Chapter 44
§44.4) turns it into an asset by using the independent paths as independent channels,
which is one of the more elegant reversals in the field.

**Diffraction** around edges, which is why you have some signal around a corner.

**Absorption**, which is strongly material-dependent: plasterboard costs perhaps
3 dB, a brick wall 10–15 dB, reinforced concrete 20–30 dB, and a lift shaft
effectively infinite. **The human body absorbs 2.4 and 5 GHz well** — a room full of
people is measurably worse than an empty one, which is why a lecture theatre that
surveys perfectly on a Sunday fails on Monday morning.

**The Fresnel zone**, which is the chapter's most under-appreciated concept for
point-to-point work: clear line of sight is not sufficient. An ellipsoid around the
direct path must also be clear, and an obstruction intruding into it causes loss even
though you can see the far end. §42.4 gives the formula and works an example, because
"we can see the other tower, why doesn't it work" is a genuine and common question.

## What breaks here

- **Turning up transmit power to fix coverage.** 6 dB buys one doubling of distance
  and raises everyone's noise floor. Chapter 45 gives the correct answer.
- **Ignoring receive sensitivity.** A link budget must be compared against the
  receiver's requirement *at the desired data rate* — sensitivity for the lowest
  modulation is far better than for the highest, so a link that "works" may be
  working at a twentieth of the expected rate.
- **Forgetting the Fresnel zone**, and building a link that degrades in rain or when
  the trees grow.
- **Designing from a survey taken in an empty building.**

## By the end you will be able to

- Compute wavelength from frequency and predict antenna size.
- Compute free-space path loss for any frequency and distance.
- Construct a complete link budget and state the margin.
- Explain the 6 dB rules for distance and frequency doubling.
- Explain multipath and how MIMO exploits it.
- Compute Fresnel zone radius and determine whether an obstruction matters.
