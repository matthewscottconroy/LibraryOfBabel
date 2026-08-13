# 3.2 Latency and Its Components

**Latency** is the time between sending something and its arrival. **Round-trip
time (RTT)** is the time until a response returns. Neither is a single quantity;
each is the sum of four independent components, and the entire practical value of
this section is that *you can usually tell which component is responsible from the
behaviour of the number*.

$$t_{\text{total}} = t_{\text{prop}} + t_{\text{trans}} + t_{\text{proc}} + t_{\text{queue}}$$

## Propagation delay: distance over speed

The time for the signal to physically cross the medium.

$$t_{\text{prop}} = \frac{d}{v}$$

where *v* is roughly 2.04 × 10⁸ m/s in fibre and 0.64–0.77 *c* in copper.

**Properties that identify it:** constant, independent of packet size,
independent of load, and irreducible. If your latency does not change when the
network is idle at 3 a.m., and does not change when you send a bigger packet, you
are looking at propagation delay and you cannot fix it.

**Magnitude:** ~5 µs/km in fibre. Across a data centre, nanoseconds. Across a
continent, ~20 ms. Across an ocean, ~30 ms. To a geostationary satellite at
35,786 km, **119 ms each way** — and a satellite hop involves ground-to-satellite
and satellite-to-ground, so a single hop is 238 ms and a round trip is at minimum
476 ms. That number is why GEO satellite internet feels the way it does, and no
amount of engineering will change it, which is precisely why the LEO
constellations of Chapter 49 exist at 550 km instead.

## Transmission delay: the time to clock the bits out

The time to push all of a packet's bits onto the wire, which is not the same thing
as the time for them to arrive.

$$t_{\text{trans}} = \frac{L}{R}$$

where *L* is the packet length in bits and *R* is the link rate in bits per second.

**Properties that identify it:** proportional to packet size, inversely
proportional to link rate, independent of distance.

A 1,500-byte frame (12,000 bits):

| Link rate | Transmission delay |
|---|---|
| 1 Mb/s | 12 ms |
| 10 Mb/s | 1.2 ms |
| 100 Mb/s | 120 µs |
| 1 Gb/s | 12 µs |
| 10 Gb/s | 1.2 µs |
| 100 Gb/s | 120 ns |

Two consequences worth internalising.

First, **this is what "faster link" actually buys you in latency terms** — and on
modern links it is almost nothing. Upgrading a 1 Gb/s link to 10 Gb/s removes
10.8 µs per hop. If your RTT is 80 ms, you have improved it by 0.03%. This is the
arithmetic behind "bandwidth does not fix latency," and it is worth being able to
produce on demand in a meeting.

Second, on *slow* links transmission delay dominates and packet size matters
enormously. On a 1 Mb/s link, a 1,500-byte frame occupies the wire for 12 ms. A
20 ms voice packet queued behind it arrives 12 ms late. This is **serialisation
delay**, it is why low-bandwidth links use fragmentation and interleaving, and it
is the origin of a whole class of QoS mechanisms in Chapter 52.

## Processing delay: what the device does

The time a node spends examining a packet and deciding what to do with it: check
the frame check sequence, look up the destination, decrement the TTL, recompute
the checksum, apply access lists.

**Properties:** small and roughly constant in hardware-forwarding devices;
potentially large and variable in software-forwarding devices or when a packet
takes a slow path.

**Magnitude:** a modern ASIC-based switch forwards in 1–5 µs (store-and-forward)
or ~500 ns (cut-through). A router doing deep packet inspection, TLS
interception, or NAT with a large translation table may take hundreds of
microseconds. A software router on a general-purpose CPU is somewhere between,
and highly load-dependent.

A specific trap: many devices forward normal traffic in hardware but process
*control-plane* traffic — including ICMP echo directed at the device itself — in
software, at low priority. This is why a router that shows 40 ms `ping` response
to its own interface may be forwarding transit traffic in microseconds. **A ping
to a router measures the router's CPU, not the path.** Ping *through* it instead.
Chapter 64 returns to this; it causes more false alarms than any other
measurement artefact in the field.

## Queueing delay: the one that varies

The time a packet spends waiting in a buffer for the link to become free. It is
the only component that depends on *other people's traffic*, and it is the reason
network performance is a statistical rather than a deterministic subject.

**Properties:** highly variable, load-dependent, and non-linear — this is the
important part.

Queueing delay does not rise gently with utilisation. It rises gently and then
explodes. For a simple M/M/1 model (Poisson arrivals, exponential service, one
server), the mean number waiting grows as

$$\frac{\rho}{1-\rho}$$

where ρ is utilisation. Tabulate it:

| Utilisation ρ | Relative queueing delay |
|---|---|
| 0.5 | 1.0 |
| 0.7 | 2.3 |
| 0.8 | 4.0 |
| 0.9 | 9.0 |
| 0.95 | 19.0 |
| 0.99 | 99.0 |

Going from 50% to 90% utilised — which looks like sensible use of a resource you
paid for — multiplies queueing delay by nine. This is why network capacity
planning targets 60–70% peak utilisation rather than 95%, a rule that looks
wasteful to anyone reading a utilisation graph without understanding this curve.
It is also why "the circuit is only 60% utilised, so it isn't the problem" is a
statement that must be checked against the *peak* rather than the five-minute
average: a link averaging 60% may be at 100% for three seconds every minute, and
those three seconds are where the user's video call lives.

The real Internet is not M/M/1 — traffic is bursty and self-similar rather than
Poisson, which makes the situation *worse*, not better, at a given mean
utilisation. But the shape of the curve is right and the lesson holds.

## Putting it together

A concrete decomposition. A 1,500-byte packet from Chicago to Frankfurt, roughly
7,000 km of fibre, crossing 14 routers, over links of 10 Gb/s, at moderate load:

| Component | Calculation | Value |
|---|---|---|
| Propagation | 7,000 km ÷ 204 km/ms | 34.3 ms |
| Transmission | 12,000 bits ÷ 10 Gb/s × 14 hops | 0.017 ms |
| Processing | ~5 µs × 14 hops | 0.07 ms |
| Queueing | variable | 0.5–15 ms |
| **One-way total** | | **~35–50 ms** |

Propagation is 98% of the fixed delay. Transmission is 0.05%. This is the normal
situation on any wide-area path, and it is why the productive question about a
WAN latency complaint is never "how fast is the link."

## Reading the number

The diagnostic payoff. Given a latency measurement, its *behaviour* identifies the
dominant component:

- **Constant, independent of load and packet size** → propagation. Irreducible;
  the only fix is a shorter path or a different medium.
- **Scales with packet size** → transmission. Fix: faster link, or smaller
  packets, or both.
- **Varies with load, especially at peaks** → queueing. Fix: more capacity, or
  queue management (Chapter 66), or QoS (Chapter 52).
- **Constant but implausibly large for the distance** → processing, or a path
  that is not the path you think it is. Run a traceroute; you may be going through
  another continent.
- **Fine to the destination, terrible to an intermediate hop** → almost certainly
  control-plane processing on that hop, not a real problem.

> **Network+ note.** N10-009 objective 5.5 expects `ping`, `traceroute`/`tracert`,
> and `mtr`-style tools, and the troubleshooting domain regularly presents latency
> symptoms. The most commonly mis-answered questions are those where a traceroute
> shows a high-latency intermediate hop with normal latency beyond it — which
> indicates a busy router CPU, *not* a slow link. Chapter 64 works through
> several.
