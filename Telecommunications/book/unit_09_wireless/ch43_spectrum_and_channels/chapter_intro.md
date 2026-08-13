# Chapter 43 — Spectrum and Channels

Wi-Fi exists because of a decision that was, at the time, essentially an act of
tidying up.

In 1985, the United States Federal Communications Commission opened several bands for
unlicensed use, provided that devices used spread-spectrum techniques and kept their
power low. These were the **ISM bands** — Industrial, Scientific and Medical — and
they had been set aside decades earlier for equipment that generated radio energy as
a *by-product* rather than as a communication signal. The 2.4 GHz band in particular
was allocated largely because that is where microwave ovens operate.

So the band was regarded as spectral wasteland. It was noisy, it was full of
industrial equipment, and nobody with a licence wanted it. Michael Marcus, the FCC
engineer who drove the decision, has described it as making rubbish land available
for anyone willing to build on it.

What was built on it was Wi-Fi, Bluetooth, Zigbee, cordless telephones, and a
substantial fraction of the modern world's local connectivity. It is now among the
most economically valuable spectrum in existence, and the reason it is crowded is
precisely the reason it was available.

## Licensed and unlicensed, and the tradeoff

**Licensed spectrum** is exclusive. A cellular operator paying billions for a band
gets legal protection: nobody else may transmit there, interference is an enforceable
offence, and the operator can therefore plan capacity with confidence and deploy high
power.

**Unlicensed spectrum** is free and shared. Anyone may transmit within the power and
behaviour rules, nobody has priority, and interference is not merely permitted but
guaranteed. You must accept it, and you must not cause harmful interference to
licensed users.

The tradeoff is the whole story of enterprise wireless. Licensed spectrum gives
predictability at enormous cost and with a licensing process measured in years.
Unlicensed gives immediate, free deployment and no guarantees whatsoever — which is
why a Wi-Fi network's performance depends on your neighbours' behaviour, and why
"the Wi-Fi got worse and we changed nothing" is a coherent and common report.

Chapter 46's private 5G is, viewed this way, an attempt to buy licensed-band
predictability for enterprise use — and its adoption rate is a live experiment in how
much that predictability is worth.

## The channel arithmetic

The single most-cited piece of Wi-Fi folklore is "use channels 1, 6 and 11," and §43.2
derives it rather than repeating it, because the derivation generalises and the rule
does not.

The 2.4 GHz band offers 14 numbered channels (11 usable in North America, 13 in most
of Europe), spaced **5 MHz apart**. An 802.11 transmission occupies approximately
**22 MHz**. Twenty-two does not fit in five.

The consequence is that adjacent channel numbers overlap heavily. Channel 1 and
channel 2 are almost the same channel. For two transmissions not to overlap at all,
their centre frequencies must differ by at least 22 MHz, which is five channel
numbers:

$$1, \quad 1+5 = 6, \quad 6+5 = 11$$

**Three non-overlapping channels.** That is all 2.4 GHz has ever offered, and it is
why the band is unusable for any dense deployment: a building with more than three
access points must reuse channels, and neighbouring networks are using the same three.

The important part is what this reveals about **partial overlap**, which is worse than
it sounds. Two networks on the same channel can hear each other and will politely
take turns under CSMA/CA (Chapter 44 §44.2) — sharing the capacity, but sharing it
in an orderly way. Two networks on *partially* overlapping channels cannot decode
each other, so neither defers; they simply raise each other's noise floor and corrupt
each other's frames. **Co-channel interference costs you throughput; adjacent-channel
interference costs you the link.** Putting an access point on channel 3 "to avoid the
neighbours on 1 and 6" is therefore actively worse than sharing channel 1 with them,
and it is one of the most common well-intentioned mistakes in the field.

## The three bands, honestly compared

| | 2.4 GHz | 5 GHz | 6 GHz |
|---|---|---|---|
| Non-overlapping 20 MHz channels | 3 | up to 25 (region-dependent) | up to 59 |
| Range for equal power | best | moderate | shortest |
| Wall penetration | best | moderate | poorest |
| Congestion | severe | moderate | minimal (currently) |
| Non-Wi-Fi interference | severe | little | very little |
| Client support | universal | near-universal | Wi-Fi 6E/7 only |
| Regulatory complexity | none | DFS on many channels | AFC for standard power |

Two entries deserve comment.

**DFS** — Dynamic Frequency Selection — applies to a large portion of the 5 GHz band
that is shared with weather and military radar. Devices must listen for radar and
vacate the channel within a specified time if detected, which is legally required and
operationally awkward: a false detection moves your access point mid-shift, and some
client devices historically handled DFS channels badly enough that many
administrators avoid them entirely. That avoidance discards over half the available
5 GHz spectrum, which is a real cost, and §43.3 discusses when to reconsider it.

**The 6 GHz band** was opened for unlicensed use in the US in April 2020 and
progressively elsewhere, adding roughly 1,200 MHz — more than doubling the total
unlicensed spectrum available. It is currently uncrowded, contains no legacy devices
at all (only Wi-Fi 6E and 7 clients can use it), and mandates WPA3. It is the most
significant change to the wireless landscape in twenty years, and its principal
limitation is the shortest range of the three bands, which means more access points.

## The noise floor

§43.4 returns to Chapter 4 §4.3's −174 dBm/Hz and works out what a real 20 MHz
receiver sees: about −101 dBm thermal, plus 4–8 dB of receiver noise figure, giving a
practical floor around −93 to −95 dBm.

Anything above that is interference, and identifying its source is a real skill. A
spectrum analyser distinguishes a microwave oven (a broad hump across the middle of
2.4 GHz, on a 50% duty cycle synchronised to mains frequency), a video sender (a
constant narrow carrier), a cordless phone (frequency-hopping), and another Wi-Fi
network (bursty, with recognisable frame structure). A Wi-Fi analyser only sees the
last of these, which is why "the analyser shows nothing on this channel and it still
doesn't work" happens.

## By the end you will be able to

- Explain the licensed/unlicensed tradeoff and its consequences for enterprise
  design.
- Derive the 1/6/11 rule from channel spacing and occupied bandwidth.
- Explain why partial overlap is worse than co-channel, and correct the common
  mistake.
- Compare the three bands and choose appropriately for a stated deployment.
- Explain DFS and decide whether to use DFS channels.
- Compute a noise floor and distinguish interference sources from an analyser trace.
