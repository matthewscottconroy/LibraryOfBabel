# 12.1 The Operator and the Crossbar

## What a circuit is

Before the mechanisms, the definition, because everything in this chapter follows
from it.

> A **circuit** is a dedicated end-to-end path, established before communication
> begins, held for its duration, and released afterwards.

Three phases, always: **setup**, **data transfer**, **teardown**. And between setup
and teardown, the path is yours — physically, in the early implementations, and
exclusively in all of them.

That exclusivity is what a circuit buys and what it costs. Nobody else can use your
path, which means nothing anyone else does can affect you; and your path is
unavailable to anyone else, which means it is wasted whenever you are silent.

## The human switch

The first exchange opened in New Haven, Connecticut, in January 1878, with
twenty-one subscribers. A caller cranked a magneto to signal the exchange, an
operator answered, the caller named the person they wanted, and the operator
connected the two subscriber lines with a **patch cord**.

That is circuit switching in its purest form. A physical copper path, metal to
metal, from one telephone to another, established by a human being and held until
they pulled the cord.

It worked, and it had three problems that shaped everything after it.

**It did not scale.** Operator effort grows with call volume, and the number of
subscribers an exchange could serve was limited by how many cords one operator could
manage and how many operators would fit in the room. AT&T's internal projections in
the early twentieth century, frequently quoted, observed that at the growth rates
then current, every woman in the United States would eventually need to be a
telephone operator. The figure is apocryphal in its precise form and directionally
correct.

**It was slow.** Ten to thirty seconds to set up a local call.

**It was not neutral**, which is where Almon Strowger comes in.

## Strowger's grudge

Strowger was an undertaker in Kansas City, and in 1888 he became convinced that the
local telephone operator — whose husband, in most tellings, was a competing
undertaker — was diverting calls intended for his business.

His response was not to complain to the exchange. It was to design, patent and
manufacture an automatic exchange that removed the operator from the process
entirely.

The **Strowger switch** is a rotating, stepping selector. The caller's dial sends
pulses — one for each digit — and each pulse advances a wiper arm one position
around a bank of contacts. Dial `4` and the wiper steps four positions; the wiper
is now touching the fourth line. A second stage handles the next digit, and so on.

The first installation went into service in La Porte, Indiana, in 1892. Variants
were still switching calls somewhere in the world into the 1990s — a hundred years
of infrastructure originating in a grudge about funeral bookings.

Two things about it are worth extracting.

**The dial is a signalling protocol.** Interrupting the loop current a specific
number of times, with defined make/break ratios and inter-digit pauses, is a
protocol with syntax, semantics and timing (Chapter 1 §1.3) — and it is the first
one most people ever used. The word "dial" survives on telephones that have not had
one for forty years, which Chapter 8 §8.1 noted as a general pattern.

**The signalling and the conversation share the same wires.** The pulses travel on
the same pair that will carry the voice. This is **in-band signalling**, and §12.3
covers both why it was natural and why it eventually had to be abandoned.

## Crossbar

Step-by-step switching has a structural weakness: the path through the exchange is
determined by the digits as they arrive, so if the selected outlet is busy the call
fails even when another path to the same destination exists. The switch cannot look
ahead.

**Crossbar** switching, introduced by Bell in the 1930s and dominant from the 1940s,
separates the two functions. A **matrix** of horizontal and vertical bars, with
crosspoints at the intersections, provides the switching fabric; a separate
**common control** subsystem — a marker — receives all the digits, works out a free
path through the matrix, and then closes the appropriate crosspoints.

The advantages are substantial:

- **Path selection considers the whole matrix**, so a call is blocked only when no
  path exists rather than when one particular path is busy.
- **The control logic is shared** across many calls rather than being replicated in
  every selector, which makes it economic to make it more capable.
- **Fewer moving parts per call**, so far greater reliability.

Crossbar is also where **stored-program control** eventually arrived: the 1ESS
switch, deployed by Bell from 1965, replaced the marker's relay logic with a
computer. Once the control was a program, the exchange gained features — call
forwarding, three-way calling, speed dialling — that were software rather than
wiring, and the modern conception of a telephone network as a programmable system
begins there.

## The exchange hierarchy

Chapter 11 §11.4's hierarchy argument, applied to telephony seventy years before
the LAN.

Every subscriber connects to a **local exchange** (central office) by a dedicated
pair — the **local loop** — which is theirs alone and is never shared. Local
exchanges connect to **tandem** exchanges, which connect to **toll** exchanges, and
so on upward through a five-level hierarchy in the classical North American plan.

```
                    Regional centre
                   ╱               ╲
          Sectional centre      Sectional centre
              ╱        ╲
      Primary centre   Primary centre
          ╱      ╲
    Toll centre  Toll centre
        ╱   ╲
   End office  End office
     │  │  │
   subscribers
```

The mechanisms are exactly §11.4's:

**Aggregation** — many subscriber lines share fewer inter-exchange trunks, because
not everyone calls at once. §12.4's Erlang mathematics is what determines how many
fewer.

**Summarisation** — the telephone number is hierarchical. Country code, area code,
exchange code, subscriber number, read left to right from most general to most
specific. An exchange routing a call examines only enough digits to decide the next
hop, exactly as a router examines a prefix (Chapter 29 §29.3). The North American
Numbering Plan's structure is a routing hierarchy in disguise.

**Containment** — a failure in one region does not propagate.

**Alternate routing** was the refinement: a call is offered to the most direct
trunk group first, and on finding it full, to progressively less direct paths. This
is load-dependent routing between exchanges, in production from the 1950s, and it
prefigures the dynamic routing of Chapter 31 by decades.

## The local loop, and why it still matters

The dedicated pair from the exchange to the premises is the most
consequential piece of infrastructure in this book, for a reason that has nothing
to do with telephony.

It exists, in copper, to virtually every building in the developed world. It was
installed over a century, at enormous cost, and it is already paid for.

That is why DSL exists (Chapter 49 §49.1). The telephone companies' great asset in
the broadband era was not their switches or their protocols; it was that they had
already dug the trenches and pulled the copper. Every technology in Chapter 49's
last-mile discussion is either an attempt to reuse that plant or an attempt to
compete with something already installed.

Chapter 49's economics — the path costs and the capacity is nearly free — is a
statement about this loop above all else.

## What breaks here

**A circuit's failure mode is total.** There is no partial circuit. A cut, a failed
crosspoint, or a lost path drops the call entirely — no degradation, no reduced
quality. This is a genuine difference from packet networks, where degradation is
the normal failure mode.

**Blocking rather than degradation** under overload. When trunks are full, new calls
are refused and existing ones are unaffected. §12.4 quantifies it; §13.4 contrasts
it with what a packet network does instead, which is degrade everyone.

**Setup latency.** A circuit cannot carry data until it is established, and
establishment takes time — seconds in the electromechanical era, still hundreds of
milliseconds in SS7. For a two-hour call this is irrelevant. For a 200-byte
transaction it is absurd, and that absurdity is a substantial part of Chapter 13's
argument.

> **Network+ note.** N10-009 expects the circuit-switched versus packet-switched
> distinction and expects the PSTN as context for WAN technologies. The three
> phases — setup, transfer, teardown — and the exclusivity of the reserved path are
> the load-bearing facts; everything in §12.2 through §12.4 elaborates them.
