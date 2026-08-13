# 6.4 Crosstalk and Interference

The fourth thing the world can do to your signal: let somebody else's into it.

## Why twisted pair is twisted

This fact is stated in every networking course and explained in almost none, which
is a pity, because the explanation is elegant and it makes several later facts
inevitable rather than arbitrary.

**The signal is differential.** A twisted pair does not carry a voltage relative to
ground. It carries a voltage *between the two conductors*, and the receiver
subtracts one from the other. If conductor A is at +1 V and conductor B at −1 V,
the received signal is 2 V.

**Interference is common-mode.** An external field — a nearby power cable, a radio
transmitter, a motor's switching transient — couples into both conductors. If the
two conductors are held physically close and are symmetrically placed with respect
to the interfering field, it induces **nearly the same voltage in both**.

**The subtraction cancels it.** If interference adds +0.3 V to both conductors,
the signal becomes +1.3 V and −0.7 V. The difference is still 2 V. The interference
has vanished from the received signal without any filtering, any shielding, or any
processing.

**And twisting is what makes the coupling equal.** Without twisting, one conductor
of the pair is consistently closer to the interfering source than the other, so it
picks up more, and the difference does not cancel. Twisting ensures that over any
length of more than a few centimetres, each conductor spends equal time nearer and
further from any external source. The coupling averages out to be equal, and the
cancellation works.

The measure of how well this works is the **common-mode rejection ratio**, and a
good balanced pair achieves 40–60 dB — a factor of ten thousand to a million in
interference power.

Two consequences follow directly, and both appear in practice:

**Untwisting the pair at the connector destroys the effect locally.** Every
termination guide insists on untwisting no more than 13 mm for Cat5e and less for
higher categories, and this is why. It is not fussiness; it is the length over
which the cancellation stops working.

**A split pair defeats it entirely.** If a technician wires pin 1 to pin 1 and pin
2 to pin 2 but uses one wire from the orange pair and one from the green pair as a
signal pair, every pin is connected correctly end to end. A continuity tester
passes it. And the two conductors carrying that differential signal are not twisted
together, so there is no common-mode rejection at all. The cable works at 10 and
100 Mb/s, where margins are generous, and fails or performs erratically at 1 Gb/s.
Lab 02 builds one deliberately, because meeting this once is worth a chapter of
reading.

## The crosstalk measurements

Pairs in one jacket also couple into *each other*, and the standards specify this
carefully because it is usually the binding constraint on twisted pair rather than
attenuation.

**NEXT — Near-End Crosstalk.** Measured at the *same* end as the transmitter: how
much of the transmitted signal appears on an adjacent pair at the point where it is
strongest. This is the dominant impairment on short links, because the disturbing
signal has not been attenuated at all while the wanted signal on the victim pair
may have travelled the full length.

**FEXT — Far-End Crosstalk.** Measured at the far end. Less severe, because the
disturbing signal has been attenuated along the way. **ELFEXT** or **ACR-F**
normalises it against the victim's own attenuation, which is the more meaningful
figure.

**PSNEXT / PSFEXT — Power Sum.** Crosstalk from *all* other pairs combined rather
than the worst single one. This is what matters for 1000BASE-T and above, which use
all four pairs simultaneously — every pair is being disturbed by three others at
once.

**Alien crosstalk.** Coupling between *different cables* in a bundle. Irrelevant at
lower rates and a genuine constraint for 10GBASE-T, which is why Cat6a specifies it
and why 10GBASE-T over Cat6 has a reduced distance limit that depends on how the
cable is bundled.

**ACR — Attenuation-to-Crosstalk Ratio.** The useful composite: the wanted signal's
level relative to the crosstalk. Since attenuation rises with frequency and
crosstalk generally worsens with frequency too, ACR falls at both ends and there is
a frequency beyond which the crosstalk exceeds the signal. That frequency is
effectively the cable's usable ceiling, and it is what a category rating encodes.

## Why the pairs have different twist rates

Open a Cat5e cable and measure the twists: the four pairs have visibly different
twist pitches, typically somewhere between 1.5 and 2 twists per centimetre and
differing by perhaps 20% between pairs.

This is deliberate. If all four pairs had the same pitch, they would maintain a
constant geometric relationship along the whole length — pair 1's conductors always
in the same position relative to pair 2's — and the coupling between them would
accumulate coherently rather than averaging out. Different pitches ensure the
relationship rotates, so the coupling from pair to pair also averages toward zero.

It is the same argument as the common-mode cancellation, applied between pairs
instead of between conductors, and it is why you cannot simply re-lay a cable's
pairs to a uniform twist and expect it to perform.

## Shielding, and when it helps

The notation, which is standardised in ISO/IEC 11801 and worth being able to read:

| Designation | Overall shield | Per-pair shield |
|---|---|---|
| **U/UTP** | none | none |
| **F/UTP** | foil | none |
| **S/FTP** | braid | foil per pair |
| **F/FTP** | foil | foil per pair |

Shielding attacks a different problem from twisting. Twisting rejects **common-mode
interference** by symmetry; shielding rejects it by **blocking the field** with a
conductive barrier.

Shielding helps substantially in electrically hostile environments — industrial
plant, near heavy machinery, in dense bundles at 10 Gb/s where alien crosstalk
binds — and it costs money, stiffness and installation care.

**And it can make things worse.** A shield must be grounded, at one end or both,
and the choice matters. Grounded at both ends with a potential difference between
the two grounds, the shield carries current — a **ground loop** — and becomes an
antenna injecting interference rather than blocking it. Grounded improperly or
left floating, it can pick up and re-radiate. Shielded installations require
attention to the earthing system that unshielded ones do not, and a badly earthed
S/FTP installation performs worse than a competent U/UTP one.

This is why the general recommendation for ordinary office environments is
unshielded twisted pair, properly terminated, and why shielded cable is specified
where the environment demands it and someone competent will earth it.

## Interference: the wireless case

On radio, "crosstalk" is called interference and it is not an impairment at the
margins — it is the dominant impairment, full stop.

Chapter 43 develops this properly. The preview, so that the connection is visible
here:

**Co-channel interference** comes from another transmitter on the same channel.
Both can decode each other's transmissions, so under CSMA/CA they defer politely
and share the airtime — capacity is divided, in an orderly way.

**Adjacent-channel interference** comes from a partially overlapping channel.
Neither can decode the other, so **neither defers**; both transmit simultaneously
and corrupt each other's frames. This is worse, and it is why Chapter 43 §43.2
insists that channel 3 is a worse choice than sharing channel 1.

**Non-Wi-Fi interference** — microwave ovens, video senders, cordless phones,
wireless cameras — raises the noise floor and is invisible to a Wi-Fi analyser,
which decodes only Wi-Fi. Finding it requires a spectrum analyser, and Lab 09 makes
the point directly.

The unifying observation with copper: **SINR, not SNR**, is the honest measure in
any shared medium. Interference degrades you identically to noise, and in the
2.4 GHz band it usually dominates thermal noise by a wide margin.

## What breaks here

**A cable that passes a continuity test and fails at gigabit.** Split pair. The
tester checks connections; it does not check twisting. A certifier measures NEXT
and finds it immediately.

**A run that degrades after someone "tidied" the terminations.** Untwisted too far
back at the connector.

**10GBASE-T that works in a loose bundle and fails in a tight one.** Alien
crosstalk. Cat6a specifies it; Cat6 does not, which is why its 10 Gb/s reach is
conditional.

**A shielded installation performing worse than the unshielded one it replaced.**
Earthing. Check whether the shield is bonded at both ends and whether the two
earths are at the same potential.

**Wireless that degrades when a neighbour "moves off your channel".** They moved
to a partially overlapping one. Ask which.

> **Network+ note.** Objective 5.2 lists crosstalk and EMI among cable
> connectivity issues, and objective 1.5 expects shielded versus unshielded media
> selection. The mechanism — differential signalling plus twisting giving
> common-mode rejection — is what makes both examinable facts derivable rather
> than memorised, and it is why the split-pair fault is such a good teaching
> example.
