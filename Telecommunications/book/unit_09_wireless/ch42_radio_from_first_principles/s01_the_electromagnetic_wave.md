# 42.1 The Electromagnetic Wave

Everything wireless in this book rests on a physical phenomenon that carries no material
and needs no medium. **Understanding what is actually propagating is what turns wireless
troubleshooting from folklore into reasoning**, and it is worth the two pages.

## What oscillates

**An accelerating electric charge produces a changing electric field. A changing electric
field produces a magnetic field. A changing magnetic field produces an electric field.**

**And that is self-sustaining.** Each field regenerates the other, and the pair propagates
away from the source at the speed of light, **requiring nothing to travel through.**

```
        E (electric field)
             ↑
             │    ╱▔╲      ╱▔╲
             │  ╱    ╲   ╱    ╲
        ─────┼─╱──────╲─╱──────╲────▶  direction of travel
             │         ╳
             │       ╱   ╲
             ↓   ╱▔╲       ╱▔╲
        B (magnetic field, perpendicular to E)
```

**The two fields are perpendicular to each other and to the direction of travel**, and this
geometry is not incidental — it is what **polarisation** means (§42.2), and it is why
antenna orientation matters.

**Maxwell predicted this in 1865**, from four equations, before anyone had produced or
detected a radio wave. **Hertz demonstrated it in 1887**, generating and detecting waves
across his laboratory and — famously — seeing no practical use for them.

## The three related quantities

$$c = f \lambda$$

| | | |
|---|---|---|
| **c** | speed of light | **3 × 10⁸ m/s** in vacuum |
| **f** | frequency | cycles per second (Hz) |
| **λ** | wavelength | metres |

**Since *c* is fixed, frequency and wavelength are inversely related** — and this single
relationship determines most of what a radio system can and cannot do.

**Rearranged for the calculation you will actually do:**

$$\lambda = \frac{300}{f_{\text{MHz}}} \text{ metres} \qquad\text{or}\qquad \lambda = \frac{300}{f_{\text{GHz}}} \text{ millimetres}$$

**Worked:**

| Frequency | Wavelength | Where it appears |
|---|---|---|
| 88–108 MHz | ~3 m | FM broadcast |
| **900 MHz** | **33 cm** | LoRa, older cellular, some IoT |
| **2.4 GHz** | **12.5 cm** | Wi-Fi, Bluetooth, Zigbee, microwave ovens |
| **5 GHz** | **6 cm** | Wi-Fi |
| **6 GHz** | **5 cm** | Wi-Fi 6E / 7 |
| 28 GHz | 1.07 cm | 5G mmWave |
| 60 GHz | 5 mm | WiGig, 802.11ad |

**These numbers are worth carrying**, because antenna size, penetration and path loss all
follow from wavelength directly.

## Why frequency determines everything

**The three consequences**, and they are the whole of wireless engineering's central trade.

### 1. Higher frequency, more available bandwidth

**A channel of a given *fractional* width is wider in absolute terms at a higher
frequency.** A 20 MHz channel is a large fraction of the 2.4 GHz ISM band's 83.5 MHz and a
trivial fraction of the 6 GHz band's 1,200 MHz.

**So higher frequencies have room for more, and wider, channels** — which by Chapter 4's
Shannon limit means more capacity.

### 2. Higher frequency, worse propagation

**Free-space loss increases with frequency** (§42.3), and — more importantly for indoor
work — **absorption by materials increases sharply.**

| Material | 2.4 GHz | 5 GHz |
|---|---|---|
| Drywall | ~3 dB | ~4 dB |
| Wood door | ~3 dB | ~4 dB |
| **Brick** | ~6 dB | **~10 dB** |
| **Concrete** | ~12 dB | **~20 dB** |
| **Glass (coated / low-E)** | ~8 dB | **~15 dB+** |
| **Human body** | ~3 dB | **~6 dB** |
| Metal | **reflects — effectively total** | |

**The human-body row explains a real phenomenon:** a room that works when empty degrades
when full, **because water absorbs strongly at these frequencies** and people are mostly
water. **A conference room measured on a Sunday will not behave as measured on a Monday.**

**And the low-E glass row explains another:** modern energy-efficient windows have a metallic
coating that blocks infrared — **and radio.** A building with such glazing is a Faraday cage
for outdoor-to-indoor signal, which surprises people planning cellular coverage.

### 3. Higher frequency, smaller antennas

**An efficient antenna is sized to a fraction of the wavelength** (§42.2), so a 2.4 GHz
antenna is centimetres and a 900 MHz one is decimetres.

**Which is why a phone can have a dozen antennas and an FM radio needs a wire a metre
long.**

> **The fundamental trade: low frequency travels further and penetrates better but has
> little bandwidth; high frequency has enormous bandwidth and does not go far or through
> anything.**

**Every wireless technology in Unit IX is a position on that trade**, chosen for its
application — and once you can state the trade, the choices stop looking arbitrary.

## The spectrum

```
   3 kHz      3 MHz       3 GHz        300 GHz    ← frequency
   ├──────────┼───────────┼──────────────┤
    VLF/LF/MF     HF/VHF      UHF/SHF/EHF
   long wave    shortwave   Wi-Fi, cellular,
   AM radio     FM, TV      satellite, radar
   ├──────────┼───────────┼──────────────┤
   100 km      100 m        10 cm        1 mm    ← wavelength
```

| Band | Range | Character | Used for |
|---|---|---|---|
| VLF/LF | 3–300 kHz | follows the earth's curvature; penetrates seawater | submarine communication, navigation |
| MF | 300 kHz–3 MHz | ground wave by day, ionospheric by night | AM broadcast |
| **HF** | **3–30 MHz** | **reflects off the ionosphere — global range** | shortwave, amateur, aviation |
| VHF | 30–300 MHz | line of sight, good penetration | FM, TV, air traffic |
| **UHF** | **300 MHz–3 GHz** | **line of sight, decent penetration, useful bandwidth** | **Wi-Fi 2.4, cellular, TV** |
| **SHF** | **3–30 GHz** | **large bandwidth, poor penetration** | **Wi-Fi 5/6, satellite, radar** |
| EHF | 30–300 GHz | enormous bandwidth, blocked by almost anything | 5G mmWave, WiGig |

**The HF row is the interesting one**, because it violates the "higher is shorter range"
rule: **HF reflects off the ionosphere**, so a modest transmitter can be heard across an
ocean. **This is why shortwave broadcasting existed and why amateur radio operators care
about the sunspot cycle** — the ionosphere's reflectivity varies with solar activity.

**Above about 30 MHz the ionosphere stops reflecting and everything is line of sight**,
which is the regime all of Unit IX operates in.

## Modulation carries the information

**A pure sine wave at 2.4 GHz carries no information** — it is entirely predictable, so
receiving it tells you nothing you did not already know (Chapter 2 §2.1's argument).

**Information requires varying something**, and Chapter 7 covered the mechanisms:

| Vary | Called |
|---|---|
| Amplitude | ASK |
| Frequency | FSK |
| **Phase** | **PSK** |
| **Amplitude and phase together** | **QAM** |
| **Many subcarriers at once** | **OFDM** |

**Modern Wi-Fi uses OFDM with QAM on each subcarrier** (Chapter 8 §8.4), and Chapter 44
§44.4 covers what that means in practice.

**The relevant point here:** the **carrier** determines the propagation — how far it goes,
what it penetrates, what antenna it needs — **and the modulation determines the data rate.**
**They are separable, and confusing them causes real errors** in reasoning about coverage
versus throughput.

## What "signal strength" means

**Received power, measured in dBm** (Chapter 3 §3.2), and the numbers are worth memorising
because every wireless diagnostic reports them:

| RSSI | Quality | Usable for |
|---|---|---|
| **−30 dBm** | maximum achievable | you are next to the AP |
| **−50 dBm** | **excellent** | anything |
| **−60 dBm** | **good** | anything, including voice |
| **−67 dBm** | **the design target** | **voice and video — the standard threshold** |
| −70 dBm | fair | data, marginal for voice |
| **−80 dBm** | **poor** | barely usable |
| −90 dBm | **unusable** | at or below the noise floor |

**−67 dBm is the number to remember.** It is the conventional design target for a WLAN
carrying voice, and Chapter 45 §45.1's coverage design uses it.

**And it is logarithmic**, so the differences are larger than they look:

$$-60\ \text{dBm} = 10 \times \text{the power of } -70\ \text{dBm}$$

**Every 10 dB is a factor of ten in power. Every 3 dB is a factor of two.**

## Signal alone is not enough

**The most common misconception in wireless**, and correcting it early saves a great deal of
confusion.

**RSSI tells you how loud the signal is. It does not tell you whether it can be
understood.**

> **What matters is the *ratio* of signal to noise, not the absolute signal.**

$$\text{SNR} = \text{signal (dBm)} - \text{noise floor (dBm)}$$

**A signal of −60 dBm with a noise floor of −95 dBm gives SNR 35 dB — excellent.**

**The same −60 dBm with a noise floor of −70 dBm gives SNR 10 dB — poor**, and the client
will use a low data rate or fail entirely.

| SNR | Supports |
|---|---|
| **> 40 dB** | the highest modulation rates |
| **25 dB** | good rates |
| **20 dB** | **the practical minimum for reliable data** |
| 15 dB | low rates only |
| **< 10 dB** | **unusable** |

**Which is Chapter 4's Shannon limit applied directly:** capacity depends on bandwidth and
SNR, so **raising the noise floor destroys capacity exactly as reducing the signal does.**

**And this is why "the signal is strong and it does not work" is a coherent and common
complaint** — Chapter 43 §43.4 covers what raises the noise floor.

## What breaks here

**Strong signal, poor performance.** SNR, not RSSI. Something is raising the noise floor.

**A room that works when empty and fails when occupied.** Human bodies absorb, especially at
5 GHz.

**Outdoor coverage that does not reach indoors.** Low-E glazing, or concrete.

**5 GHz not reaching where 2.4 GHz does.** Expected — higher frequency, worse penetration.

**Confusing carrier frequency with data rate.** The carrier determines propagation; the
modulation determines rate.

> **Network+ note.** Objective 2.4 expects wireless fundamentals. Over-learn: **c = fλ**;
> **higher frequency means more bandwidth and worse propagation**; **2.4 GHz penetrates
> better and 5 GHz is faster**; **RSSI is measured in dBm and −67 dBm is the design
> target**; and **SNR matters more than absolute signal strength.** The frequency/range
> trade-off is examined in several forms.
