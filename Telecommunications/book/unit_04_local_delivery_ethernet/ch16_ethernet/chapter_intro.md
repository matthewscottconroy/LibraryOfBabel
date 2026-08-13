# Chapter 16 — Ethernet

The University of Hawaii has campuses on several islands, and in the late 1960s
this posed a problem that no existing technology solved well. Norman Abramson
wanted terminals on Kauai, Maui and Hawaii to reach the central computer on Oahu.
Leased telephone lines between islands were expensive and slow. There was, however,
a great deal of unused radio spectrum and a clear line of sight across a great deal
of ocean.

Abramson's team built **ALOHAnet**, operational in June 1971, and in doing so had
to answer a question nobody had needed to answer before: if many stations share one
radio channel, and none of them can hear each other, when should a station
transmit?

Their answer was so simple that it sounds like a joke. **Transmit whenever you have
something to send.** If two stations transmit at once, both transmissions are
destroyed, and neither receives an acknowledgement — so both wait a *random*
interval and try again.

That is pure ALOHA, and its maximum channel utilisation is 1/(2*e*) ≈ **18.4%**.
Eighty-two per cent of the channel is lost to collisions and idle time. Abramson
knew this; it is in the paper. It was still better than the alternatives, because
the alternatives cost money and this cost nothing.

Slotted ALOHA — requiring transmissions to begin only at fixed slot boundaries —
doubles it to 1/*e* ≈ 36.8%. Still terrible. But the *shape* of the solution —
transmit, detect failure, back off randomly, retry — is the shape that survives,
and it is in every wireless standard and every Ethernet controller today.

## The memo

Robert Metcalfe read Abramson's ALOHA paper while a graduate student at Harvard.
His doctoral thesis had been rejected as insufficiently theoretical; he rewrote it
with an improved analysis of ALOHA's performance, and it passed.

In 1972 he joined Xerox's Palo Alto Research Center, where a problem was waiting:
PARC had built the Alto, arguably the first personal computer, and a laser printer
that could produce a page per second, and there was no way to connect the hundred
Altos to the printer at a rate that made the printer worth having.

On 22 May 1973 Metcalfe circulated a memo describing a solution. It took ALOHA's
random-access idea and added one thing the radio version could not have: **listen
before you transmit.** On a shared cable, unlike a radio channel, every station can
hear every other station. If the cable is busy, wait. That is *carrier sense*, and
it lifts utilisation from 18% to well over 90%.

He added a second refinement: **listen while you transmit**, and if what you hear
is not what you sent, a collision has occurred — abort immediately rather than
wasting the rest of the frame time. That is *collision detection*, and together
they give **CSMA/CD**: Carrier Sense Multiple Access with Collision Detection.

Metcalfe named it after the **luminiferous aether**, the hypothetical medium
through which nineteenth-century physicists believed light propagated — a
substance whose existence the Michelson–Morley experiment had disproved in 1887.
He chose it deliberately, as a joke about a passive medium that carries waves and
does not exist. It is the best-named technology in this book.

## The interesting question

Ethernet's technical history is worth knowing. But the question the chapter is
really built around is a different one, and it is more useful:

> **Why did Ethernet win, given that it was not the best design?**

Token Ring was deterministic — a station holding the token could transmit without
contention, so the maximum delay was bounded, which Ethernet could not promise.
Under heavy load Token Ring degraded gracefully and Ethernet degraded badly. IBM
backed it, and IBM in 1985 was not a company you bet against.

Ethernet was a shouting match with a random backoff timer. It could, in principle,
delay a frame indefinitely. Its worst case was unbounded.

And Ethernet won so completely that Token Ring hardware is now a museum piece.

§16.3 develops the answer, and it has three parts: Ethernet was **cheaper** at every
point in its history; it was **good enough**, because the pathological cases that
Token Ring guarded against were rare in practice and were eliminated entirely once
switching arrived; and — most importantly — its **interface stayed stable while its
implementation was replaced completely**. The frame format of 1983 is the frame
format of 2026. Everything else about the technology has been thrown away and
rebuilt: the medium, the coding, the topology, the arbitration, the duplex model,
the speed, four times over.

That is a general lesson about how technologies survive, and it is worth
extracting: **standardise the interface, not the mechanism.** Chapter 21 makes the
same argument abstractly, about layers.

## What this chapter does

§16.1 covers ALOHAnet: the problem, the protocol, the 18.4% analysis, and why the
random backoff idea is permanent.

§16.2 covers Metcalfe's memo, CSMA/CD in operation, the collision domain, the
binary exponential backoff algorithm, and the derivation of the 64-byte minimum
frame from the round-trip time of a maximum-length segment — a piece of arithmetic
that every network engineer should do once.

§16.3 covers the standards ladder: 10BASE5, 10BASE2, 10BASE-T, 100BASE-TX,
1000BASE-T, 10GBASE-T, 25/40/100/400G — what changed at each step, what did not, and
the "why did it win" argument.

§16.4 covers full duplex and the abolition of collisions, autonegotiation and its
failure modes, and Power over Ethernet from 802.3af through 802.3bt.

## By the end you will be able to

- Explain pure and slotted ALOHA and their utilisation limits, and identify the
  same backoff idea in Wi-Fi.
- Trace a CSMA/CD transmission including a collision, backoff, and retry.
- Derive the 64-byte minimum frame size from segment length and propagation speed.
- Explain what a collision domain is and why modern switched networks have one per
  port.
- Identify an Ethernet standard from its name (`10GBASE-SR`, `1000BASE-LX`) and
  state its medium, rate and reach.
- Explain autonegotiation, duplex mismatch, and the counter signature that reveals
  it.
- State the PoE standards and their power budgets, and compute whether a given
  switch can power a given set of devices.
