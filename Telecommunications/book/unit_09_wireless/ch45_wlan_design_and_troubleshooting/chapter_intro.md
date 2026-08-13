# Chapter 45 — WLAN Design and Troubleshooting

Here is the mistake, and it is made constantly, by people who should know better,
because it is the intuitive thing to do.

Coverage is poor in the back office. The obvious remedy is to increase the access
point's transmit power. So the administrator sets every AP in the building to
maximum, and the reported signal strength improves everywhere, and the network gets
*worse*.

The reasons compound, and each has been established earlier in this book:

**The client cannot shout back.** A laptop transmits at perhaps 15 dBm; an access
point at 23 dBm. Raising the AP's power extends the range at which the client can
*hear* the AP but not the range at which the AP can hear the client. The client
associates from a distance at which its own transmissions do not arrive cleanly, and
the link is asymmetric and broken — showing full bars.

**Every AP is now interference for every other AP.** With three non-overlapping
2.4 GHz channels (Chapter 43 §43.2), APs must reuse them. Higher power means each AP
is heard further, which means more APs share each channel's airtime under CSMA/CA
(Chapter 44 §44.2), which means everyone waits longer. Aggregate capacity falls.

**Clients stop roaming.** A client holds its association while the signal is
tolerable. Powerful APs remain tolerable far beyond where they are *good*, so clients
cling to a distant AP at a low data rate — and by the performance anomaly of
Chapter 44, a client at 6 Mb/s consumes airtime that everyone else is waiting for.

The correct answer is nearly the opposite: **more access points at lower power**.
Smaller cells, cleaner boundaries, more aggressive roaming, more total spectrum
reused. It costs more in hardware and it is the design that works.

That inversion is the chapter's organising theme. Wireless design intuitions
developed on wired networks are frequently backwards, and the corrections all come
from the shared-medium and half-duplex facts of Unit IX's introduction.

## Coverage or capacity

Every WLAN design answers one of two questions, and confusing them produces the wrong
network.

**Coverage design** asks: is there adequate signal everywhere it is needed? This is
the right question for a warehouse with forty handheld scanners across 20,000 square
metres. Few clients, large area. Place APs to eliminate gaps.

**Capacity design** asks: is there enough airtime for the number of clients in this
space? This is the right question for a 400-seat lecture theatre. The signal is fine
everywhere; the problem is that four hundred devices are contending for a shared
half-duplex medium. Place APs to divide the client population, use narrow channels to
maximise channel reuse, and turn the power *down* so cells stay small.

The lecture theatre designed for coverage gets one powerful AP and fails completely.
The warehouse designed for capacity gets a fortune in unnecessary hardware. §45.3
gives the sizing arithmetic for both: clients per radio, airtime per client, and the
application bandwidth budget.

## Surveys

§45.1 covers the three kinds, and the point is that they answer different questions
and are not substitutes.

**Predictive** — modelling software with a floor plan and wall materials. Cheap, done
before deployment, and only as good as the material assumptions. Useful for planning
AP count and rough placement.

**Passive** — walking the building with a receiver, listening. Shows what exists:
your APs, the neighbours', the coverage overlap, the noise floor. Essential for
validating a deployment and for diagnosing an existing one.

**Active** — associating and measuring actual throughput, loss and roaming behaviour.
The only one that tells you what a user will experience.

The most important survey practice, and the most often skipped: **survey with the
building populated**. Human bodies absorb 2.4 and 5 GHz measurably (Chapter 42 §42.4).
A lecture theatre surveyed empty on a Sunday and deployed to a full house on Monday
is a well-documented way to be surprised.

## Roaming

A client, not an access point, decides when to roam. This is worth stating plainly
because it explains why roaming problems are so often unfixable from the
infrastructure side: the AP can encourage, hint, and refuse, but the decision logic
lives in a driver written by the client's manufacturer and varies enormously between
devices.

The infrastructure can help. **802.11k** gives clients a neighbour report so they do
not have to scan blindly. **802.11v** lets the AP suggest a better AP. **802.11r**
(fast transition) pre-establishes keys so a roam does not require a full
reauthentication — which matters enormously for voice, where a 300 ms roam is an
audible gap and an 802.1X reauthentication can take far longer.

**Sticky client** behaviour — a device holding a weak association past the point of
usefulness — is the classic complaint, and the fixes are: reduce power so the old AP
becomes genuinely unusable, disable low data rates so the client is forced off, and
enable 802.11k/v so it knows where to go.

## The four complaints

§45.4 is structured around what users actually say, because they never say "my SNR is
inadequate."

**"It's slow."** Check SNR, not signal. Check the negotiated data rate. Check channel
utilisation — a clean signal on a saturated channel is slow. Check for legacy clients
dragging the cell down.

**"It keeps dropping."** Almost always roaming. Check whether the drop correlates with
movement; check for coverage holes at cell boundaries; check whether both bands are
configured consistently.

**"It works in the morning and not the afternoon."** Load, or a scheduled interference
source. Check channel utilisation over time. Check for a microwave.

**"It works on my phone but not my laptop."** A client-side difference: supported
bands, DFS channel support, driver, or antenna count. Establish which before touching
the infrastructure.

The general procedure, which is Chapter 63's method applied here: **establish whether
the problem is signal, noise, airtime, or client** — those four exhaust the space, and
each has a different remedy.

## By the end you will be able to

- Explain why raising transmit power usually degrades a WLAN, in four distinct ways.
- Distinguish coverage from capacity design and size an AP count for each.
- Choose the appropriate survey type and state what it will and will not reveal.
- Explain who decides to roam and what the infrastructure can do about it.
- Diagnose a wireless complaint by determining whether the cause is signal, noise,
  airtime or client.
