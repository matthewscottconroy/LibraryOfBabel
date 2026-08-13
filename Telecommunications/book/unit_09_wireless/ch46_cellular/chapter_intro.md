# Chapter 46 — Cellular

In December 1947, a Bell Labs engineer named Douglas H. Ring wrote an internal
technical memorandum proposing a solution to a problem that had made mobile telephony
a curiosity rather than a service.

The problem was capacity. Existing mobile radio used a small number of high-power
transmitters covering a whole city, and since two conversations cannot share a
frequency, the number of simultaneous calls equalled the number of available
channels — a few dozen for an entire metropolitan area. New York City's mobile
telephone service in the 1970s could support about twelve simultaneous calls, with a
waiting list years long.

Ring's proposal inverted the design. Instead of one powerful transmitter, use **many
low-power transmitters**, each covering a small area — a *cell*. Because each
transmitter is weak, the same frequency can be reused in another cell far enough away
that the two do not interfere. Capacity is no longer limited by the number of
channels; it is limited by the number of channels *multiplied by the number of times
you can reuse them*, which is limited only by how small you are willing to make the
cells.

The idea is a straightforward consequence of the path loss arithmetic in Chapter 42
§42.3 — signal falls with distance, so beyond a certain distance a transmitter is
below the noise floor and irrelevant. Ring had it in 1947. The technology to
implement it — in particular, the ability to hand a call from one cell to another
without dropping it, which requires real-time computation across a network of base
stations — did not exist until the late 1970s, and the first commercial cellular
system launched in Tokyo in 1979 and in Chicago in 1983.

Thirty-six years from idea to product. §46.1 covers the frequency reuse mathematics,
the cluster patterns, and the sectorisation that multiplies capacity further.

## The generations, as an argument rather than a list

Memorising the feature lists of 1G through 5G is not useful. Understanding what
problem each generation was solving, and what it gave up, is.

**1G (1979–1990s)** — analog voice, FDMA. Each call occupies its own frequency pair.
It worked, it was expensive, it had no security whatsoever (calls could be listened to
with a scanner, and cloning a handset's identity was trivial), and capacity was
limited by the number of frequencies.

**2G (1991–)** — **digital** voice, TDMA (GSM) or CDMA. This is the significant
transition, and it is the same one Chapter 12 §12.2 described for the wired network:
digitisation permits regeneration, compression, and encryption. GSM introduced the
SIM card, which separated subscriber identity from handset — an idea so obviously
right in retrospect that its absence in competing systems is hard to credit. Capacity
improved three-to-fivefold from voice compression alone. And SMS was added almost as
an afterthought, using spare capacity in the signalling channel, which is why it is
160 characters.

**2.5G/3G (2001–)** — **packet data**. GPRS and then EDGE overlaid packet switching on
the circuit-switched core; UMTS and CDMA2000 made data a first-class service. This is
Chapter 13's argument replayed: the cellular network discovered that reserving a
circuit for bursty data traffic was wasteful, and moved to statistical multiplexing.
3G is where the mobile Internet begins, and where the industry's revenue model began
its shift from minutes to bytes.

**4G / LTE (2009–)** — **all-IP**. The circuit-switched voice core is removed
entirely; voice becomes VoLTE, an application over IP, exactly like everything else.
OFDMA (Chapter 8 §8.4) replaces CDMA on the air interface. This is convergence
(Chapter 14 §14.4) completed for mobile: after a century, the telephone network has
become an application running on a packet network.

**5G NR (2019–)** — three distinct service classes rather than one: enhanced mobile
broadband, ultra-reliable low-latency communication, and massive machine-type
communication. mmWave bands offer enormous bandwidth over very short distances.
**Network slicing** creates logically isolated virtual networks over shared
infrastructure — which is, as §46.4 notes without embarrassment, an attempt to
reintroduce circuit-switching's guarantees (Chapter 13) using packet infrastructure,
and it is the most direct instance in this book of the pendulum swinging back.

## What is actually different about cellular

Three architectural features that Wi-Fi does not have, and which explain the
performance difference more than the radio technology does.

**Licensed spectrum.** The operator owns the band. No neighbours, no microwave ovens,
no contention with parties who did not agree to cooperate. Chapter 43's tradeoff,
resolved the other way, and it buys predictability that unlicensed cannot.

**Scheduled access.** A cellular base station *assigns* transmission opportunities.
There is no contention, no random backoff, no hidden node problem — the scheduler
decides who transmits when, with knowledge of everyone's queue and channel quality.
Compare Wi-Fi's CSMA/CA, which is a polite free-for-all. This is why cellular degrades
gracefully under load and Wi-Fi does not.

**Managed mobility.** Handover is network-controlled and designed for vehicles at
speed, with the core network maintaining session continuity. Wi-Fi roaming, per
Chapter 45, is a client-side decision with no such guarantees.

§46.4 covers **private 5G** in this light: enterprises deploying licensed or
shared-spectrum cellular in factories, ports and mines precisely to buy those three
properties for environments — moving machinery, large outdoor areas, hard reliability
requirements — where Wi-Fi's contention-based model struggles.

## By the end you will be able to

- Explain frequency reuse and compute the capacity gain from a given cluster size.
- Explain what problem each generation solved and what it traded away.
- Explain why 4G's all-IP core is the same convergence argument as Chapter 14's.
- State the three architectural advantages of cellular over Wi-Fi and explain each
  mechanistically.
- Explain network slicing and connect it to Chapter 13's packet-versus-circuit
  argument.
- Judge when private cellular is a better answer than Wi-Fi for a stated environment.
