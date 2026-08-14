# 47.4 NFC, RFID and the Very Short Range

**At a few centimetres, the physics changes.** These technologies do not work by radiating a
wave across a distance — most of them operate in the near field, where the coupling is
magnetic rather than electromagnetic — and that difference explains both their properties and
their security model.

## Near field and far field

Chapter 42 assumed the far field, where an antenna radiates a propagating wave and power
falls as $1/d^2$.

Within roughly $\lambda/2\pi$ of an antenna, the behaviour is different: the fields are
reactive rather than radiating, energy is stored and returned rather than launched, and
coupling falls as $1/d^6$ for magnetic induction.

| Frequency | λ | Near-field boundary (λ/2π) |
|---|---|---|
| **13.56 MHz** (NFC) | 22 m | **~3.5 m** |
| 125 kHz (LF RFID) | 2.4 km | ~380 m |
| **868 MHz** (UHF RFID) | 35 cm | **~5.5 cm** |

So NFC at 13.56 MHz operates well inside the near field, and its coupling is magnetic
induction — a transformer with an air gap, not a radio link.

> The $1/d^6$ falloff is why NFC's range limit is sharp rather than gradual, and it is a
> genuine security property: at 10 cm the coupling is 64 times weaker than at 5 cm, and at
> 1 m it is essentially nothing.

## RFID

**Radio Frequency Identification** — and it is a family rather than a technology.

| | **LF** | **HF** | **UHF** |
|---|---|---|---|
| Frequency | **125–134 kHz** | **13.56 MHz** | **860–960 MHz** |
| Coupling | magnetic | magnetic | **backscatter (far field)** |
| Range | **~10 cm** | **~10 cm–1 m** | **1–12 m** |
| Rate | slow | moderate | fast |
| **Reads many tags?** | no | some | **yes — hundreds** |
| Liquid/metal tolerance | **good** | moderate | **poor** |
| Cost per tag | low | low | **very low** |
| Use | animal tags, access fobs, immobilisers | **cards, passports, NFC** | **supply chain, inventory** |

### Passive tags

The property that makes RFID work at scale: the tag has no battery.

**The reader's field powers the tag.** A passive tag harvests energy from the interrogating
field, uses it to run a tiny circuit, and replies.

In the near field (LF/HF), by load modulation: the tag changes its own impedance, which
the reader detects as a change in the load on its own coil. The tag does not transmit; it
modulates how much energy it absorbs.

**In the far field (UHF), by backscatter:** the tag reflects the reader's signal, changing
its reflection coefficient to encode data.

> A passive tag costs a few pence, lasts indefinitely, and needs no maintenance — which is
> why there are tens of billions of them, and why the technology succeeded where
> battery-powered alternatives did not.

**Active tags** have a battery and reach hundreds of metres; **semi-passive** tags use a
battery for the circuit and backscatter for communication. Both are for specific asset
tracking and are a small fraction of deployments.

### UHF and the anti-collision problem

A UHF reader may see hundreds of tags simultaneously, all trying to answer — which is
Chapter 16's medium-access problem in a population that cannot hear each other at all.

The solution is a slotted ALOHA variant (Chapter 16 §16.1): the reader specifies a number
of slots, each tag picks one at random and replies in it, collisions are detected and
retried, and the reader adjusts the slot count based on how many collisions it sees.

Reading a pallet of 200 tagged items takes a fraction of a second, and it is the capability
that made supply-chain RFID possible.

## NFC

A subset of HF RFID (13.56 MHz), standardised for consumer devices, with one significant
addition.

| Mode | Behaviour |
|---|---|
| **Reader/writer** | phone reads a passive tag |
| **Card emulation** | **phone pretends to be a card** — payments, transit, access |
| **Peer-to-peer** | two devices exchange data (largely superseded) |

Card emulation is the one that mattered commercially. It is how Apple Pay, Google Pay and
transit systems work: the phone presents itself to an existing contactless reader as a
card, requiring no change to the reader infrastructure.

Which is why contactless payment deployed so fast — Chapter 28's lesson again: the
technology that required nothing of the existing infrastructure won.

### The security model

NFC's security is not cryptographic. It is physical.

> **The protection is that an attacker must be within centimetres.**

And that is a real protection — the $1/d^6$ falloff means eavesdropping requires proximity
that is socially conspicuous — but it is not sufficient alone, and the actual security comes
from layers above:

**Payment tokenisation.** A phone does not transmit the card number. It transmits a
device-specific token and a per-transaction cryptogram, so a captured transaction cannot be
replayed and a compromised terminal learns nothing reusable.

**Secure Element / TEE.** Keys live in tamper-resistant hardware that the operating system
cannot read.

**User presence.** Biometric or PIN confirmation per transaction.

**The attacks worth knowing:**

**Relay attacks** — the one the physics does not prevent. Two attackers, one near the
victim's card and one near a reader, relay the exchange over a network in real time. The
card is centimetres from *a* reader; the reader is centimetres from *a* card; and they are a
kilometre apart.

**Distance bounding** — measuring the round-trip time to verify proximity — is the defence, and
it is only now being deployed (in newer contactless standards and in UWB, below).

**Skimming** — reading a card from a pocket. Practical against unshielded older cards and
largely mitigated by tokenisation and by the transaction limits and cryptograms of modern
contactless.

## Ultra-Wideband

Worth a section because it is genuinely different and is being deployed rapidly.

| | |
|---|---|
| Band | **3.1–10.6 GHz** |
| Bandwidth | **≥ 500 MHz** — very wide, very low power spectral density |
| Method | **extremely short pulses** — nanoseconds |
| **Ranging accuracy** | **5–10 cm** |
| Range | 10–50 m |

UWB's purpose is not data. It is measuring distance.

Because the pulses are so short, the time of flight can be measured precisely — and
precision in time is precision in distance at 30 cm per nanosecond.

> UWB knows how far away something is, and — with several antennas — in which direction.

**Which enables things the technologies above cannot:**

**Secure keyless entry.** A car can verify that the key is actually within a metre, which
defeats the relay attack that has plagued keyless entry systems. This is UWB's most
commercially significant application.

**Precise indoor location** — centimetres, where Wi-Fi and Bluetooth manage metres.

**Item finding** — Apple's AirTag and equivalents, which direct you to an object rather than
merely telling you it is nearby.

**Automatic device handoff** — a phone knowing which speaker you are pointing at.

And the distance-bounding property is a genuine security primitive, not merely a
convenience: proximity that can be cryptographically verified rather than assumed from
signal strength (which is trivially spoofed by amplification).

## Comparing the short range

| | **NFC** | **UHF RFID** | **BLE** | **UWB** |
|---|---|---|---|---|
| Range | **< 10 cm** | 1–12 m | 10–100 m | 10–50 m |
| **Powered tag needed?** | **no** | **no** | yes | yes |
| Data rate | 424 kb/s | moderate | 1–2 Mb/s | high |
| **Position accuracy** | proximity only | **metres** | 1–5 m | **5–10 cm** |
| Cost per tag | pence | **fractions of a penny** | pounds | pounds |
| **Security by physics** | **yes** | no | no | **yes, verifiable** |
| Typical use | payment, access | inventory | sensors, beacons | ranging, keys |

And they are complementary rather than competing — a modern phone contains all four, using
each for what it is good at.

## What breaks here

**An NFC tag that does not read.** Metal or liquid behind it — LF and HF are coupled
magnetically and metal detunes the coil. Tags for metal surfaces have a ferrite spacer.

**UHF RFID failing near liquids.** Water absorbs strongly at 900 MHz (Chapter 42 §42.1).
Tagging bottles is a known hard problem.

A contactless card working at a longer distance than expected. Some readers are more
sensitive than the standard assumes; the range is not a hard cryptographic boundary.

**A keyless car stolen without the key.** A relay attack. UWB-based systems resist it; older
LF/UHF ones do not.

**A UWB device's ranging being inaccurate indoors.** Multipath (Chapter 42 §42.4) — UWB is more
robust than narrowband and is not immune.

**Reading hundreds of tags and missing some.** Anti-collision, orientation, or the tags being
shadowed by their own contents.

> **Network+ note.** Objective 2.4 mentions NFC and RFID. Over-learn: NFC operates at
> 13.56 MHz over a few centimetres and is used for payment and access; RFID tags may be
> passive (powered by the reader) or active; and **the short range is itself the security
> property.** The passive-tag concept is the useful part.
