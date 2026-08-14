# Chapter 47 — Important Concepts

Frequency hopping buys three things at once *(§47.1)* — Classic Bluetooth hops 1,600
times per second across 79 channels, which gives **interference resilience** (a corrupted hop
costs one packet, not the connection), mild rather than continuous interference to Wi-Fi,
and a modest obscurity benefit. **Adaptive Frequency Hopping** then learns which channels are
busy and excludes them — Chapter 42's Lamarr–Antheil idea in production.

BLE is not a slower Bluetooth *(§47.1)* — It is a radio designed to be off almost all the
time. Classic streams; BLE wakes, exchanges a few bytes, and sleeps. Connection setup
falls from ~100 ms to ~3 ms and idle current from milliamps to microamps, and the battery
life comes from the duty cycle rather than from efficiency.

BLE's advertising channels avoid Wi-Fi *(§47.1)* — 37, 38 and 39 sit in the gaps between
Wi-Fi channels 1, 6 and 11 (Chapter 43 §43.2), so discovery survives in a saturated band.
The same trick as 802.15.4's channels 15, 20, 25 and 26.

GATT makes BLE devices self-describing *(§47.1)* — Profile → service → characteristic, with
assigned UUIDs for standard services. A client can discover what a device offers without
prior knowledge, which is why a generic scanner app can interact with almost anything — and
why a device reveals a good deal before any pairing.

The coded PHY is Shannon's trade, explicitly *(§47.1)* — 125 kb/s with forward error
correction buys roughly four times the range of the 1 Mb/s mode, about 12 dB of link
budget. Spend bits on redundancy, buy distance (Chapter 4).

Bluetooth's security pattern is instructive *(§47.1)* — LE Secure Connections (4.2) uses
ECDH and is sound; the weakness is that the specification permits weak options for
compatibility — "Just Works" has no MITM protection — and implementations choose them.
The recurring shape throughout this book: a sound protocol with a permitted downgrade.

802.15.4's 127-byte frame shapes everything above it *(§47.2)* — 250 kb/s, 10–100 m, years
on a battery, and about 102 payload bytes after MAC header and security. Every design
decision in Zigbee and Thread follows from that number.

Choosing 802.15.4 channel 15, 20, 25 or 26 is the single most effective reliability
measure *(§47.2)* — They sit between Wi-Fi 1, 6 and 11. More effective than any other
change you can make in a building with Wi-Fi.

Zigbee's mesh improves as you add mains-powered devices *(§47.2)* — Bulbs, switches and
plugs route; battery devices cannot, because routing requires listening continuously.
Twenty bulbs are twenty repeaters — and a battery sensor at the edge has no such help.

Thread's central decision: devices are IPv6 hosts *(§47.2)* — Not a gateway that
translates, but an addressable Internet host. Zigbee reaches the Internet only through a
vendor hub that translates semantics and frequently depends on that vendor's cloud — which is
why the lights stop working when a company's service does.

6LoWPAN is compression plus adaptation *(§47.2)* — A 40-byte IPv6 header compresses to
2–3 bytes because most of it is derivable from context, link-layer fragmentation bridges
the 1,280-byte minimum MTU to a 127-byte frame, and mesh forwarding sits below IP.
It is what allows a coin-cell sensor to be a first-class Internet host.

Matter is an application layer, not a network *(§47.2)* — It runs over Thread, Wi-Fi,
Ethernet, and BLE for commissioning, and standardises the data model, commissioning,
device attestation, and local control. What it fixes is Zigbee's fragmentation: what a
light *is*, agreed across vendors.

Matter's plausibility is a network effect, not a technical one *(§47.2)* — Apple,
Google, Amazon and Samsung committed simultaneously, which had not happened before
(Chapter 28 §28.1). The specification is sound; the implementations have been uneven.

The trilemma: range, rate, power — pick two *(§47.3)* — Range + rate costs power
(cellular, Wi-Fi); rate + low power costs range (BLE, Thread); range + low power costs rate —
and that is LPWAN. The physics is Shannon's (Chapter 4 §4.2): to be heard further at the same
power, spread the energy over more time or accept lower SNR, and both mean fewer bits.

LoRa decodes below the noise floor *(§47.3)* — Chirp spread spectrum's correlation gain
works down to about **−20 dB SNR**. A LoRa signal invisible on a spectrum analyser is
decodable, and that is where the range comes from.

The spreading factor is the trade in one parameter *(§47.3)* — Each step adds ~2.5 dB of
link budget and doubles the time on air. SF12 reaches perhaps four times as far as SF7 and
takes thirty times as long to send the same message.

Time on air is the binding constraint, not data rate *(§47.3)* — 1% duty cycle = 36
seconds of transmission per hour. At SF12's 1.5 s per message that is 24 messages per hour,
total. LPWAN capacity planning is a duty-cycle budget.

The duty cycle constrains the gateway more than the device *(§47.3)* — A gateway's downlink
budget is shared across every device it serves. This is why **downlink is scarce**, why
Class A's model is a necessity rather than an optimisation, and why confirmed uplinks do not
scale.

A Class A device is unreachable by design *(§47.3)* — It transmits, opens two brief receive
windows, and sleeps. The network cannot reach it in between, and applications must be built
around that rather than against it.

LoRaWAN has no association, so mobility is free *(§47.3)* — A device transmits; every
gateway that hears it forwards, and the network server deduplicates. No handover, no roaming,
no connection to maintain — which is why it suits tracking.

Adaptive Data Rate is essential, not optional *(§47.3)* — The network server drives each
device to the lowest spreading factor that works, saving both battery and duty cycle.
Without it a dense deployment exhausts its airtime.

The deciding LPWAN question is usually not technical *(§47.3)* — Do you want to run the
network, or rent it? LoRaWAN is the only one you can own: no subscription, no carrier
roadmap dependency. NB-IoT gives licensed spectrum and national coverage, plus a per-device
subscription forever and exposure to the carrier's decisions — which §46.2's 2G shutdown
shows is a real risk.

Sigfox is a business-layer single point of failure *(§47.3)* — One commercial operator,
**receivership in 2022**, and customers with deployed devices discovered that a technology
evaluation that considers only the technology is incomplete.

LPWAN is for a few bytes, occasionally *(§47.3)* — The common design error is forcing a
kilobyte-per-minute application onto it, which produces duty-cycle exhaustion, battery drain
and a network that does not work.

Near field and far field are different physics *(§47.4)* — **Within roughly $\lambda/2\pi$**
the fields are reactive rather than radiating and coupling falls as $1/d^6$, not $1/d^2$.
At 13.56 MHz that boundary is **3.5 m**, so NFC is magnetic induction — a transformer with an
air gap — while at 868 MHz it is 5.5 cm, so UHF RFID is a genuine radio link.

The $1/d^6$ falloff makes NFC's range limit sharp *(§47.4)* — 64 times weaker at 10 cm
than at 5 cm, essentially nothing at a metre. A real security property, and not a
sufficient one.

A passive tag has no battery, and that is why there are billions *(§47.4)* — The reader's
field powers it. In the near field it replies by load modulation — changing its own
impedance, which the reader senses as a change in its coil's load; in the far field by
backscatter — changing its reflection coefficient. A tag that costs pence, lasts
indefinitely and needs no maintenance is the whole reason the technology succeeded.

UHF RFID's anti-collision is slotted ALOHA *(§47.4)* — Chapter 16 §16.1 again, in a
population that cannot hear each other at all: the reader announces slots, tags pick one at
random, and the reader adapts the slot count to the collision rate. It reads a 200-item
pallet in a fraction of a second.

Card emulation is why contactless deployed so fast *(§47.4)* — The phone presents itself
to existing readers as a card, requiring no infrastructure change. Chapter 28's lesson:
the technology that demanded nothing of the installed base won.

NFC's real security is tokenisation, not proximity *(§47.4)* — The phone transmits a
device-specific token and a per-transaction cryptogram, not the card number, with keys in a
secure element and user presence per transaction. A captured transaction cannot be replayed
and a compromised terminal learns nothing reusable.

The relay attack is the one physics does not prevent *(§47.4)* — Two attackers relay the
exchange in real time; the card is centimetres from a reader, the reader is centimetres from
a card, and they are a kilometre apart. Distance bounding — verifying proximity by
round-trip time — is the defence, and it is only now being deployed.

UWB's purpose is not data but distance *(§47.4)* — ≥500 MHz of bandwidth in nanosecond
pulses gives 5–10 cm ranging. Precision in time is precision in distance at 30 cm per
nanosecond. It gives cryptographically verifiable proximity, which defeats keyless-entry
relay attacks that signal strength — trivially spoofed by amplification — cannot.

The four short-range technologies are complementary, not competing *(§47.4)* — NFC for
proximity and payment, UHF RFID for cheap inventory at metres, BLE for powered sensors and
beacons, UWB for centimetre ranging. A modern phone contains all four.
