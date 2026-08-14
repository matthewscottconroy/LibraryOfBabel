# 9.1 Frequency-Division Multiplexing

Give each conversation its own slice of the spectrum. Everybody transmits all the
time, at once, and never interferes because they occupy different frequencies.

This is the oldest multiplexing technique, it is what Bell was trying to build when
he invented the telephone instead (Chapter 9's introduction), and it is still in
use everywhere from radio broadcasting to the DWDM systems carrying the modern
Internet.

## The mechanism

Take *N* baseband signals. Modulate each onto a different carrier frequency
(Chapter 8), so each occupies a band centred on its own carrier. Sum them and
transmit the sum. At the receiver, a bank of band-pass filters separates them, and
each is demodulated back to baseband.

```
   ch1  ──[×f₁]──┐
   ch2  ──[×f₂]──┤
   ch3  ──[×f₃]──┼──[+]──▶ one medium ──▶[filter bank]──▶ ch1, ch2, ch3, ch4
   ch4  ──[×f₄]──┘
```

The requirement is that the bands do not overlap, and the practical requirement —
which is where the cost lies — is that they are separated by enough that real
filters can separate them.

## Guard bands, and why they cost so much

An ideal filter would pass everything inside its band and reject everything
outside, with a vertical transition. Real filters have a **transition region**: a
range of frequencies over which the response falls from passing to rejecting.

So adjacent channels must be separated by more than their own widths. The extra
space is a **guard band**, and it is pure waste — spectrum that carries nothing and
exists only so that imperfect filters can do their job.

The overhead is substantial. Analog telephone carrier systems allocated 4 kHz per
voice channel to carry a 3.1 kHz signal — **22% of the spectrum spent on guard
band**. FM broadcasting allocates 200 kHz per station for a signal occupying about
180 kHz. Early analog cellular allocated 30 kHz channels for a 3 kHz voice signal,
because the modulation spread it.

This is why Chapter 8 §8.4's OFDM is such an improvement: by making the subcarriers
*orthogonal* rather than merely non-overlapping, it eliminates the guard band
entirely. The spectra overlap and the information does not, and the 22% comes back.

## The historical instance: analog carrier telephony

The Bell System's L-carrier hierarchy is worth knowing because it is where the
technique was industrialised and because its structure prefigures the digital
hierarchy of §9.2.

| Level | Composition | Channels | Bandwidth |
|---|---|---|---|
| Voice channel | — | 1 | 4 kHz |
| Group | 12 voice channels | 12 | 48 kHz (60–108 kHz) |
| Supergroup | 5 groups | 60 | 240 kHz |
| Mastergroup | 10 supergroups | 600 | 2.52 MHz |
| Jumbogroup | 6 mastergroups | 3,600 | 16.984 MHz |

The construction is recursive: build a group, then treat the group as a signal and
modulate *it* onto a higher carrier to build a supergroup, and so on. Each stage
uses the same technique on the output of the previous one.

L-carrier systems on coaxial cable carried tens of thousands of simultaneous
conversations across continents from the 1940s onward, and the last of them were
retired in the 1980s as digital transmission displaced them for the reasons
Chapter 5 §5.1 gives — analog amplification accumulates noise; digital regeneration
does not.

## Where FDM lives now

**Broadcast radio and television.** AM at 10 kHz channel spacing, FM at 200 kHz,
terrestrial television at 6 or 8 MHz per channel. The entire broadcast model is FDM
with the receivers being the demultiplexers.

**Cable television and DOCSIS.** A coaxial plant carrying 750 MHz or more, divided
into 6 or 8 MHz channels, some carrying television and some carrying DOCSIS data
(Chapter 49 §49.2). The architecture is pure FDM and it is why a cable modem can
coexist with television service on one cable.

**DSL.** Voice below 4 kHz, upstream data from about 25 to 138 kHz, downstream from
138 kHz to 1.1 MHz. Chapter 5 §5.4 worked this; the splitter is a pair of filters
implementing the frequency division.

**Cellular.** Every operator holds licensed bands, and within them FDD systems use
separate frequencies for uplink and downlink — frequency division applied to
direction rather than to users.

**Wavelength-division multiplexing on fibre**, which is FDM at optical frequencies
and is §9.4's subject.

**OFDM**, which as §9.1 noted is FDM with the guard bands removed by orthogonality.
Whether OFDM "is" FDM is a definitional question; mechanically it is the same idea
executed with a mathematical constraint that eliminates the technique's main cost.

## The properties FDM has

Worth stating explicitly, because they contrast sharply with §9.3's statistical
multiplexing.

**Each channel has a guaranteed, continuous allocation.** Nobody else can use your
frequency, ever, whether or not you are transmitting.

**Delay is constant and low.** There is no queueing; your signal goes out
immediately.

**Analog-friendly.** FDM works equally well for analog and digital payloads, which
mattered enormously before digitisation and matters little now.

**Capacity is fixed at design time.** Adding a channel means re-planning the
frequency allocation.

**Idle channels waste their allocation completely.** A silent voice channel occupies
its 4 kHz exactly as much as an active one.

That last property is the fatal one for data traffic, and it is the whole of §9.3's
argument.

## What breaks here

**Intermodulation between channels.** If the shared medium or an amplifier is not
perfectly linear, channels mix and produce products landing in other channels
(Chapter 6 §6.2). This is why amplifier linearity is specified so carefully in
cable plant, and why an over-driven amplifier degrades *every* channel rather than
just the one being driven hard.

**Filter drift.** An analog filter's centre frequency moves with temperature and
component ageing, and a drifted filter admits part of the adjacent channel.
Largely a historical problem now that filtering is digital.

**Adjacent-channel interference**, which is the wireless form of the same problem
and which Chapter 43 §43.2 shows costing more than co-channel interference — because
partially overlapping transmitters cannot decode each other and therefore do not
defer.

> **Network+ note.** N10-009 does not use the term FDM directly. It does expect you
> to know that cable television and DOCSIS share a coaxial plant by using different
> frequency channels, and that DSL and voice coexist on one pair for the same
> reason (objectives 1.2, 2.4). Both are FDM, and knowing that connects two facts
> that otherwise look unrelated.
