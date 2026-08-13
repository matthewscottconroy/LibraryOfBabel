# 16.1 ALOHA and the Hawaiian Packet

The University of Hawaii is distributed across several islands, and in the late
1960s that posed a problem no existing technology solved economically.

Norman Abramson wanted terminals on Kauai, Maui and Hawaii to reach the central IBM
360 on Oahu. Leased telephone lines between islands were expensive, slow, and — for
a research budget — prohibitive. What Hawaii had instead was a great deal of unused
UHF spectrum and clear line of sight across a great deal of ocean.

**ALOHAnet** became operational in June 1971, using two 100 kHz channels at 407 and
413 MHz. And in building it, Abramson's team had to answer a question nobody had
needed to answer before.

## The question

Many stations share one radio channel. **None of them can hear each other** — they
are on different islands, transmitting toward a central site, and their signals do
not reach one another.

Chapter 1 §1.2 noted that Shannon's model has one transmitter and one receiver, and
that everything about shared media is outside it. Here is the first practical
instance of the gap: *when should a station transmit?*

The prior art was polling — a central controller asks each station in turn whether it
has anything to send. It works, it is deterministic, and it scales badly: with *n*
stations, each waits for *n*−1 others before its turn, and most polls return nothing
because most stations are idle most of the time. For terminal traffic, which
Chapter 13 §13.2 established is overwhelmingly idle, polling spends nearly all its
capacity asking questions whose answer is no.

## The answer

**Transmit whenever you have something to send.**

That is it. No coordination, no permission, no schedule. A station with a packet
sends it immediately.

If two stations transmit at overlapping times, both transmissions are destroyed at
the receiver. The central site acknowledges packets it received correctly; a station
that receives no acknowledgement within a timeout concludes its packet was lost,
waits a **random** interval, and retransmits.

The randomness is essential and is the part worth dwelling on. If both stations
waited a fixed interval, they would collide again, and again, forever. Randomising
breaks the symmetry — the two stations choose different waits and one succeeds.

**Transmit, detect failure, back off randomly, retry.** That shape is in every
shared-medium protocol since, and it is the durable contribution.

## The efficiency, computed

Abramson's analysis, and it is worth following because the result is famous and the
derivation is short.

Let *G* be the **offered load** in packets per packet-transmission-time — so *G* = 1
means stations collectively attempt one packet's worth of transmission per packet
time.

A packet transmitted at time *t* is destroyed if any other transmission begins in
the interval from *t*−*T* to *t*+*T*, where *T* is the packet time. That is a
**vulnerable period of 2*T*** — twice the packet duration, because a transmission
starting up to one packet time earlier is still in progress when yours begins.

Assuming Poisson arrivals, the probability of no other transmission in a window of
2*T* is *e*⁻²ᴳ. So the throughput is

$$S = G e^{-2G}$$

Differentiate and set to zero: the maximum is at *G* = 0.5, giving

$$S_{\max} = 0.5 \, e^{-1} = \frac{1}{2e} \approx 0.184$$

**18.4%.** Eighty-two per cent of the channel is lost to collisions and to the idle
time that keeps collisions from being worse.

## Slotted ALOHA

Roberts's 1972 refinement: require that transmissions begin only at **fixed slot
boundaries**, with all stations synchronised to a common clock.

A packet can now collide only with one starting in the *same* slot, not with one
starting at any overlapping moment. The vulnerable period halves from 2*T* to *T*:

$$S = G e^{-G}, \qquad S_{\max} = \frac{1}{e} \approx 0.368$$

**36.8%.** Twice as good, for the cost of distributing a clock.

Both figures are reproducible:

```bash
python3 tools/simnet.py aloha
```

which computes the curves and confirms the peaks at 1/2*e* and 1/*e*.

## Why build something 82% wasteful?

Because the alternatives cost money and this cost nothing.

Abramson knew the figure — it is in the paper. The comparison was not against an
efficient protocol; it was against **leased telephone lines**, which were the only
alternative, and which cost more per month than the entire radio system cost to
build.

A channel used at 18% is a poor channel. A channel used at 18% that you already own,
against no channel at all, is an excellent channel. This is Chapter 10 §10.5's cost
question applied to protocols, and it is worth noticing that the "wrong" answer was
correct given the constraints.

The general lesson, which Chapter 16 §16.3 will make again about Ethernet itself:
**a design is evaluated against the available alternatives, not against an ideal.**

## The instability nobody expected

One property of ALOHA that is easy to miss and matters greatly.

Look again at *S* = *Ge*⁻²ᴳ. Throughput rises with load to a peak at *G* = 0.5 and
then **falls**. At *G* = 2, throughput is 0.037. At *G* = 3, it is 0.007.

So an ALOHA channel under increasing load does not saturate — it **collapses**.
More offered load produces more collisions produces more retransmissions produces
more offered load. The system has a positive feedback loop and no damping.

This is congestion collapse, in 1970, in a single-hop radio network. Chapter 38
§38.1's 1986 NSFNET collapse is the same phenomenon at internetwork scale, and it
took the same kind of fix: a backoff algorithm that responds to observed failure by
reducing offered load rather than merely retrying.

ALOHA's **binary exponential backoff** — double the mean wait after each successive
collision — is that damping, and it is what §16.2's CSMA/CD inherits.

## What ALOHA gave the field

Three things, and all three outlived the network.

**Random access as a viable architecture.** Before ALOHA, sharing a medium meant
coordination — polling, token passing, time slots. ALOHA showed that uncoordinated
transmission with collision recovery is not merely workable but often better, and
every contention-based protocol since is a descendant.

**The backoff idea.** Detect failure, wait a random interval, retry, and lengthen the
interval on repeated failure. In CSMA/CD (§16.2), in CSMA/CA (Chapter 44 §44.2), in
TCP's retransmission timers (Chapter 37 §37.3), and in essentially every distributed
system that retries anything.

**A worked example for Metcalfe.** Metcalfe read Abramson's paper as a graduate
student, used an improved analysis of ALOHA to rescue a rejected doctoral thesis, and
then — at PARC, with a cable instead of a radio channel — asked what could be
improved. §16.2 is the answer.

## Where ALOHA still runs

Not merely historical. Pure or slotted ALOHA appears wherever stations cannot hear
each other and traffic is sparse:

- **RFID tag reading** — many tags respond to a reader's query, collide, and back
  off. The anti-collision protocols in EPC Gen2 are ALOHA variants.
- **LoRaWAN** uplinks (Chapter 47 §47.3) — pure ALOHA, because the devices are
  battery-powered, cannot listen continuously, and transmit a few bytes a day.
- **Satellite random access channels**, for initial terminal access.
- **Cellular random access channels** — the RACH by which a handset first contacts a
  base station is a slotted-ALOHA procedure.

In each case the conditions that made it right in Hawaii still hold: stations that
cannot sense each other, sparse traffic, and a cost of coordination that exceeds the
cost of collisions.

## What breaks here

**Applying ALOHA where carrier sense is available.** If stations *can* hear each
other, not listening first wastes 80% of the channel for nothing.

**Fixed backoff.** Two colliding stations retry simultaneously, forever. The
randomisation is not an optimisation; it is what makes the protocol work.

**No backoff growth.** Without exponential lengthening, a heavily loaded channel
enters the collapse regime and does not recover.

**Assuming throughput rises with load.** Past *G* = 0.5 it falls, and the network's
observable behaviour is that offering more traffic delivers less.

> **Network+ note.** ALOHA is not examined. Its descendants are: **CSMA/CD** for
> legacy Ethernet and **CSMA/CA** for Wi-Fi (objectives 1.6, 2.3), both of which are
> ALOHA plus carrier sense. The transferable idea is the backoff shape — **detect,
> randomise, retry, lengthen** — which appears at four different layers in this book.
