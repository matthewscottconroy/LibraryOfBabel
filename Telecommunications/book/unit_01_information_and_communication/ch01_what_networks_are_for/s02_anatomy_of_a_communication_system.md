# 1.2 The Anatomy of a Communication System

In July and October of 1948, the *Bell System Technical Journal* published, in two
parts, a paper by a thirty-two-year-old mathematician named Claude Elwood Shannon.
It was called "A Mathematical Theory of Communication," and it did something that
had not been done before: it separated the *engineering* problem of communication
from the *meaning* of what was being communicated, completely and permanently.

The paper's second paragraph contains one of the most consequential sentences in
the history of technology:

> "Frequently the messages have *meaning*; that is they refer to or are correlated
> according to some system with certain physical or conceptual entities. These
> semantic aspects of communication are irrelevant to the engineering problem."

*Irrelevant to the engineering problem.* This is not a dismissal of meaning; it is
a division of labour, and it is the reason the same cable can carry a symphony, a
bank transfer, and a photograph of a cat without knowing or caring which. Every
layer boundary in this book descends from that sentence.

On the paper's first page is a diagram. It has six boxes. We are going to use it
for the whole book.

## The six components

```
┌────────┐ message ┌─────────────┐ signal  ┌─────────┐ received┌──────────┐ message ┌─────────────┐
│ SOURCE │ ──────▶ │ TRANSMITTER │ ──────▶ │ CHANNEL │ ──────▶ │ RECEIVER │ ──────▶ │ DESTINATION │
└────────┘         └─────────────┘         └────┬────┘  signal └──────────┘         └─────────────┘
                                                  │
                                             ┌────▼────┐
                                             │  NOISE  │
                                             └─────────┘
```

**The information source** produces a message. Note that Shannon is careful here:
the source *selects* a message from a set of possible messages. That framing —
communication as selection from a known set — is the seed of everything in
Chapter 2 and Chapter 4.

**The transmitter** converts the message into a signal suitable for the channel.
This is the box that does the most work and gets the least credit. In a telegraph
it is a key and a battery; in a modem it is a modulator; in your laptop it is a
network interface controller that will, in Chapter 7, turn bits into voltage
transitions and, in Chapter 8, turn them into shifts of a carrier's phase.

**The channel** is the physical medium the signal traverses. Copper, glass, air,
vacuum, or in Shannon's own list, "a band of radio frequencies, a beam of light."
The channel is entirely indifferent to your intentions and has properties —
bandwidth, attenuation, propagation delay — that constrain everything.

**The noise source** is what Shannon added that his predecessors had left implicit.
Noise is not a malfunction. It is a permanent, unavoidable, thermodynamically
guaranteed feature of every physical channel that has ever existed or ever will.
Any resistor above absolute zero generates noise; the electrons jiggle. You cannot
design it away. You can only design *around* it, and Chapter 4 quantifies exactly
how much you can get for how much.

**The receiver** performs the inverse of the transmitter: it reconstructs the
message from the received signal. Crucially, "the received signal" is not "the
signal" — it is the signal plus noise, attenuated, distorted, and delayed. The
receiver's job is inference under uncertainty, which is why Thomson's mirror
galvanometer beat Whitehouse's brute force, and why a modern DSL modem contains
more computation than the entire Apollo Guidance Computer.

**The destination** is the person or thing the message is for.

## Locating real systems on the diagram

The diagram's value is that it applies at every scale, and being able to place a
system on it is a diagnostic skill, not an academic exercise.

**A Wi-Fi client sending a frame.** Source: an application. Transmitter: the
802.11 radio, performing OFDM modulation onto a 5 GHz carrier. Channel: about
twenty metres of air, two plasterboard walls, and a microwave oven. Noise: thermal
noise in the receiver's front end, plus the neighbour's access point on the same
channel, plus the microwave. Receiver: the access point's radio. Destination: the
AP's bridging logic.

**A fibre link between two data centres.** Source: a switch's forwarding engine.
Transmitter: a pluggable optical module converting electrical signalling to light
at 1310 nm. Channel: 40 km of single-mode fibre. Noise: amplified spontaneous
emission from optical amplifiers, plus dispersion smearing pulses into each other.
Receiver: a photodiode and clock-recovery circuit. Destination: the far switch.

**A voice call over the traditional telephone network.** Source: a larynx.
Transmitter: a carbon or electret microphone, then a codec sampling at 8 kHz.
Channel: the local loop, then a digital trunk. Noise: hum, crosstalk from adjacent
pairs, quantisation error introduced by the codec itself. Receiver: the far
handset. Destination: an ear.

Notice something about that third example: **the transmitter introduced noise.**
Quantisation error — the difference between the true analog voltage and the
nearest of 256 representable levels — is created by our own equipment, not by the
world. A great deal of engineering effort in this book goes into noise we
ourselves manufacture: quantisation error, jitter in a clock, intermodulation in
a non-linear amplifier, interference from our own reflections.

## What Shannon's separation bought us

The immediate practical consequence of declaring meaning irrelevant is that the
transmitter and receiver can be designed by people who have never met the source
and destination. That sounds obvious now. It was not.

Before 1948, communications engineering was largely organised *by application*.
Telegraph engineering, telephone engineering, and radio engineering were separate
disciplines with separate journals and separate assumptions, because each was
optimised end-to-end for its particular content. Shannon's framing said: there is
one problem here, not three, and its parameters are bandwidth, noise, and rate.

That is the intellectual ancestor of the layering we will formalise in Unit V.
When we get to the OSI model and argue that an application should not need to know
whether it is running over copper or fibre, we will be applying a principle first
stated as a sentence about semantics in a Bell Labs journal.

## The one thing the diagram omits

Shannon's model is a *point-to-point* model. One source, one destination, one
channel. It is complete and correct for that case, and it is the case that
Units I and II examine.

But look at what it cannot express. It has no notion of *which* destination —
there is only one. It has no notion of a shared channel with several transmitters
contending for it. It has no notion of a message passing through an intermediate
node that must decide where to send it next. It has no notion of two conversations
sharing a wire.

Those omissions are the entire subject of Units III through VIII. Addressing
exists because there is more than one possible destination. Media access control
exists because there is more than one transmitter on a channel. Routing exists
because there are intermediate nodes with choices. Multiplexing exists because
conversations must share.

So the model is not wrong; it is a base case. Every remaining chapter of this
book adds one clause to it and deals with the consequences.

> **Network+ note.** N10-009 does not test Shannon's model by name. It tests the
> ability to reason about where in a transmission chain a fault lies — which is
> precisely the skill this diagram gives you. When a question describes a symptom
> and asks for the most likely cause, the productive first move is to ask which
> of the six boxes could produce that symptom, and eliminate the rest.
