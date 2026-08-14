# Chapter 44 — Wi-Fi

IEEE 802.11 was ratified in 1997 and offered 2 Mb/s. Wi-Fi 7 (802.11be, 2024)
specifies a theoretical maximum in excess of 46 Gb/s. That is a factor of twenty-three
thousand in twenty-seven years, achieved on the same rubbish spectrum, and it is
worth understanding where the improvement came from — because it did not come from
one thing, and the four sources are all instances of principles this book has already
established.

**Wider channels** — 20 MHz to 320 MHz. Chapter 4 §4.4: capacity is linear in
bandwidth.

**Denser modulation** — from BPSK to 4096-QAM. Chapter 8 §8.3: more bits per symbol,
at the cost of requiring much better SNR.

**More spatial streams** — MIMO, from 1 to 16. This is the genuinely new idea, and it
is not on Chapter 4's curve at all, because each spatial stream is effectively a
separate channel occupying the same frequencies.

**Less overhead** — shorter guard intervals, frame aggregation, and OFDMA's ability
to serve several clients in one transmission rather than one at a time.

Notice that only one of those is a fundamentally new physical idea. The rest are the
systematic application of things Units I and II established.

## The name confusion, cleared up

| Marketing name | Standard | Year | Bands | Headline rate |
|---|---|---|---|---|
| — | 802.11 | 1997 | 2.4 | 2 Mb/s |
| — | 802.11b | 1999 | 2.4 | 11 Mb/s |
| — | 802.11a | 1999 | 5 | 54 Mb/s |
| — | 802.11g | 2003 | 2.4 | 54 Mb/s |
| Wi-Fi 4 | 802.11n | 2009 | 2.4, 5 | 600 Mb/s |
| Wi-Fi 5 | 802.11ac | 2013 | 5 | 6.9 Gb/s |
| Wi-Fi 6 | 802.11ax | 2021 | 2.4, 5 | 9.6 Gb/s |
| Wi-Fi 6E | 802.11ax | 2021 | + 6 GHz | 9.6 Gb/s |
| Wi-Fi 7 | 802.11be | 2024 | 2.4, 5, 6 | 46+ Gb/s |

The "Wi-Fi *n*" numbering was introduced by the Wi-Fi Alliance in 2018 because
consumers could not be expected to know whether `ac` was newer than `n`. It was a
sensible decision applied retroactively, which is why Wi-Fi 4 is from 2009.

Every headline rate in that table is a PHY rate under ideal conditions with maximum
streams and maximum channel width. Chapter 4 §4.4 and Chapter 3 §3.1 both explain
why real throughput is 40–60% of it at best, and a two-stream laptop on an 80 MHz
channel gets a fraction of a number quoted for an eight-stream access point on 320
MHz. §44.1 does the arithmetic explicitly, because the gap between the box and the
measurement is the single most common source of "faulty" wireless equipment.

## Why CSMA/CA and not CSMA/CD

Ethernet detects collisions (Chapter 16 §16.2). Wi-Fi cannot, for a reason that is
purely physical: a radio cannot listen while it transmits. Its own signal at the
antenna is perhaps a hundred billion times stronger than a distant station's, so
there is nothing to hear over it. Collision *detection* is unavailable.

So 802.11 uses collision **avoidance**: listen before transmitting; if the medium is
busy, wait; when it goes idle, wait a further random backoff before transmitting, so
that two stations that were both waiting do not both start at once. And because the
sender cannot tell whether the frame arrived, every unicast frame is
acknowledged — an explicit ACK from the receiver, at the MAC layer, for every
single frame.

That acknowledgement is a major overhead and it is why Wi-Fi's efficiency is
structurally lower than Ethernet's. It also creates a subtle performance property
worth knowing: because the medium is shared and half duplex, and every frame costs an
ACK, the slowest client on an access point degrades everyone. A legacy device
transmitting at 6 Mb/s occupies the medium for far longer than a modern one moving
the same data at 600 Mb/s, and during that time nobody else can transmit. This is the
"performance anomaly" of 802.11, and it is why disabling low data rates is standard
practice in dense deployments.

## The hidden node

The problem that CSMA/CA cannot solve on its own, and which distinguishes radio from
cable fundamentally.

Stations A and C are both in range of access point B, but not of each other — a wall
between them, or simply distance. A listens, hears nothing, and transmits. C listens,
hears nothing (because A is inaudible to it), and transmits. Both frames collide at
B. Neither A nor C has any way to know.

Carrier sense fails because the relevant question is whether the medium is busy at
the receiver, and the sender can only observe it at the sender.

The mechanism for this is **RTS/CTS**: A sends a short Request to Send; B replies with
a Clear to Send, which *C can hear*; C therefore defers. The exchange costs two extra
frames per transmission, which is why it is normally disabled and enabled only above
a size threshold, or in environments where hidden nodes are known to exist. §44.2
covers when it is worth it.

## The four-way handshake

§44.3 covers association and authentication, and the four-way handshake deserves
flagging because it is where wireless security actually happens.

The passphrase is never transmitted. Both sides derive a Pairwise Master Key from it,
then exchange nonces to derive a fresh **Pairwise Transient Key** for this session —
so every client on the same passphrase gets different encryption keys, and capturing
one session does not compromise another.

Two consequences worth knowing. Capturing the four-way handshake permits an **offline
dictionary attack** against the passphrase in WPA2, which is why passphrase length
matters enormously and why WPA3's SAE replaced it with a mechanism that is not
offline-attackable. And the KRACK vulnerability of 2017 exploited a flaw in the
handshake's state machine — not the cryptography — forcing nonce reuse. The lesson,
which Chapter 58 repeats: protocols fail at their state machines at least as often as
at their algorithms.

## By the end you will be able to

- Identify any 802.11 standard by name or number and state its bands and capabilities.
- Compute realistic throughput from PHY rate, channel width, streams and overhead.
- Explain why collision detection is impossible on radio and what replaces it.
- Explain the hidden node problem and decide whether RTS/CTS is warranted.
- Explain the 802.11 performance anomaly and justify disabling low data rates.
- Trace association, authentication and the four-way handshake in a capture.
- Explain MIMO, MU-MIMO and OFDMA, and say which of the three helps a single client.
